use crate::db_operations;
use crate::errors::BullsEyeError;
use crate::models::earnings_model;
use crate::models::earnings_model::NewNominalEarnings;
use crate::models::metrics_model;
use crate::models::metrics_model::{NewNominalMetrics, NominalMetrics};
use bullseye_api::table;
use diesel::pg::PgConnection;

/// runs when handling new ticker data.
/// includes:
///     storing all recent earnings data (TTM & Annual)
///     filling missing fields
///     creating new ticker for the metrics table
pub async fn handle_new_ticker(
    new_ticker: &str,
    exchange: &table::Exchange,
    conn: &mut PgConnection,
) -> Result<(), BullsEyeError> {
    let (
        concat_statement_ttm,
        concat_statement_annual,
        currency,
        industry,
        earnings_date,
        price,
        next_yr_rev,
    ) = bullseye_api::scrape_init(new_ticker, exchange).await?;
    let ttm_entries: Vec<NewNominalEarnings> = concat_statement_ttm
        .into_iter()
        .filter_map(|x| NewNominalEarnings::create_new_entry(x))
        .collect();
    let annual_entries: Vec<NewNominalEarnings> = concat_statement_annual
        .into_iter()
        .filter_map(|x| NewNominalEarnings::create_new_entry(x))
        .collect();

    earnings_model::insert_stock_data_batch(ttm_entries, conn)?;
    earnings_model::insert_stock_data_batch(annual_entries, conn)?;
    db_operations::update_growths_batch(conn)?;
    db_operations::update_ratios_batch(conn)?;
    let new_entry = NewNominalMetrics::create_new_entry(
        exchange,
        new_ticker,
        &currency,
        &industry,
        earnings_date,
        price,
        next_yr_rev,
    )?;
    metrics_model::insert_new_ticker(new_entry, conn)?;
    Ok(())
}

/// runs  after Q4 Earnings.
/// includes:
///     storing latest earnings data (TTM & Annual)
///     filling missing fields
///     updating estimates and current stock price
pub async fn update_earnings_all(
    ticker: &str,
    exchange: &table::Exchange,
    conn: &mut PgConnection,
) -> Result<(), BullsEyeError> {
    let (concat_statement_ttm, concat_statement_annual, _, price, next_yr_rev) =
        bullseye_api::scrape_annual_update(ticker, exchange).await?;
    let ttm_entries: Vec<NewNominalEarnings> = concat_statement_ttm
        .into_iter()
        .filter_map(|x| NewNominalEarnings::create_new_entry(x))
        .collect();
    let annual_entries: Vec<NewNominalEarnings> = concat_statement_annual
        .into_iter()
        .filter_map(|x| NewNominalEarnings::create_new_entry(x))
        .collect();
    let is_ttm_entries_existed = earnings_model::insert_stock_data_batch(ttm_entries, conn)?;
    let is_annual_entries_existed = earnings_model::insert_stock_data_batch(annual_entries, conn)?;
    if is_ttm_entries_existed && is_annual_entries_existed {
        db_operations::update_growths_batch(conn)?;
        db_operations::update_ratios_batch(conn)?;
        db_operations::empty_earnings_date(ticker, table::get_exchange_string(&exchange), conn)?;
        db_operations::update_estimate(
            &ticker,
            table::get_exchange_string(&exchange),
            next_yr_rev,
            conn,
        )?;
    }
    db_operations::update_price(&ticker, table::get_exchange_string(&exchange), price, conn)?;
    Ok(())
}

/// runs  after Q1-Q3 Earnings.
/// includes:
///     storing latest earnings data (TTM)
///     filling missing fields
///     updating estimates and current stock price
pub async fn update_earnings_ttm(
    ticker: &str,
    exchange: &table::Exchange,
    conn: &mut PgConnection,
) -> Result<(), BullsEyeError> {
    let (concat_statement_ttm, _, price) =
        bullseye_api::scrape_quarter_update(ticker, exchange).await?;
    let ttm_entries: Vec<NewNominalEarnings> = concat_statement_ttm
        .into_iter()
        .filter_map(|x| NewNominalEarnings::create_new_entry(x))
        .collect();
    let is_entries_existed = earnings_model::insert_stock_data_batch(ttm_entries, conn)?;
    if is_entries_existed {
        db_operations::update_growths_batch(conn)?;
        db_operations::update_ratios_batch(conn)?;
        db_operations::empty_earnings_date(ticker, table::get_exchange_string(&exchange), conn)?;
    }
    db_operations::update_price(&ticker, table::get_exchange_string(&exchange), price, conn)?;
    Ok(())
}

/// updates earnings date and current stock price
pub async fn update_regular(
    ticker: &str,
    exchange: &table::Exchange,
    conn: &mut PgConnection,
    update_date: bool,
) -> Result<(), BullsEyeError> {
    let (earnings_date, price) = bullseye_api::scrape_regular_update(ticker, exchange).await?;
    if update_date {
        db_operations::update_earnings_date(
            ticker,
            table::get_exchange_string(&exchange),
            earnings_date,
            conn,
        )?;
    }
    db_operations::update_price(&ticker, table::get_exchange_string(&exchange), price, conn)?;
    Ok(())
}

/// updates all metrics after earnings
pub fn update_metrics_ttm(
    ticker: &str,
    exchange: &str,
    conn: &mut PgConnection,
) -> Result<(), BullsEyeError> {
    db_operations::copy_latest_data(ticker, exchange, conn)?;
    db_operations::update_short_term_trends(ticker, exchange, conn)?;
    let target = NominalMetrics::find(ticker, exchange, conn)?;
    target.update_price_target(ticker, exchange, conn)?;
    let updated_target = NominalMetrics::find(ticker, exchange, conn)?;
    updated_target.update_guidance(ticker, exchange, conn)?;
    Ok(())
}

/// updates all metrics after earnings. This only runs after Q4 Earnings.
pub fn update_metrics_annual(
    ticker: &str,
    exchange: &str,
    conn: &mut PgConnection,
) -> Result<(), BullsEyeError> {
    db_operations::copy_latest_data(ticker, exchange, conn)?;
    db_operations::update_short_term_trends(ticker, exchange, conn)?;
    db_operations::update_multi_yr_growth(ticker, exchange, conn)?;
    db_operations::update_long_term_trends(ticker, exchange, conn)?;
    let target = NominalMetrics::find(ticker, exchange, conn)?;
    target.update_price_target(ticker, exchange, conn)?;
    let updated_target = NominalMetrics::find(ticker, exchange, conn)?;
    updated_target.update_guidance(ticker, exchange, conn)?;
    Ok(())
}
