use crate::db;
use crate::errors::BullsEyeError;
use bullseye_api::table;
// use chrono::{Duration, Local};
use db::NewStockEntry;
use diesel::pg::PgConnection;

/// This function adds new ticker data in the database.
///
/// # Arguments
///
/// * `new_ticker` - The ticker you are adding
/// * `exchange` - Exchange enum
/// * `conn` - PgConnection(diesel) instance
///
/// # Returns
///
/// * `Ok(())` if the operation was successful.
/// * `Err(BullsEyeError)` if an error occurred during scraping or database insertion.
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
    let ttm_entries: Vec<NewStockEntry> = concat_statement_ttm
        .into_iter()
        .filter_map(|x| db::NewStockEntry::add_new_entry(x))
        .collect();
    let annual_entries: Vec<NewStockEntry> = concat_statement_annual
        .into_iter()
        .filter_map(|x| db::NewStockEntry::add_new_entry(x))
        .collect();
    db::insert_stock_data_batch(ttm_entries, conn)?;
    db::insert_stock_data_batch(annual_entries, conn)?;
    db::update_growths(conn)?;
    db::update_ratios_batch(conn)?;
    db::add_new_eval(
        new_ticker,
        exchange,
        &currency,
        &industry,
        earnings_date,
        price,
        next_yr_rev,
        conn,
    )
    .await;
    Ok(())
}

/// This function updates data upon earnings and price changes.
///
/// # Arguments
///
/// * `ticker` - The ticker you are updating
/// * `exchange` - Exchange enum
/// * `conn` - PgConnection(diesel) instance
///
/// # Returns
///
/// * `Ok(())` if the operation was successful.
/// * `Err(BullsEyeError)` if an error occurred during scraping or database insertion.
pub async fn update_earnings_all(
    ticker: &str,
    exchange: &table::Exchange,
    conn: &mut PgConnection,
) -> Result<(), BullsEyeError> {
    let (concat_statement_ttm, concat_statement_annual, _, price, next_yr_rev) =
        bullseye_api::scrape_annual_update(ticker, exchange).await?;
    let ttm_entries: Vec<NewStockEntry> = concat_statement_ttm
        .into_iter()
        .filter_map(|x| db::NewStockEntry::add_new_entry(x))
        .collect();
    let annual_entries: Vec<NewStockEntry> = concat_statement_annual
        .into_iter()
        .filter_map(|x| db::NewStockEntry::add_new_entry(x))
        .collect();
    let is_ttm_entries_existed = db::insert_stock_data_batch(ttm_entries, conn)?;
    let is_annual_entries_existed = db::insert_stock_data_batch(annual_entries, conn)?;
    if is_ttm_entries_existed && is_annual_entries_existed {
        db::update_growths(conn)?;
        db::update_ratios_batch(conn)?;
        db::empty_earnings_date(ticker, table::get_exchange_string(&exchange), conn)?;
        db::update_estimate(
            &ticker,
            table::get_exchange_string(&exchange),
            next_yr_rev,
            conn,
        )?;
    }
    db::update_price(&ticker, table::get_exchange_string(&exchange), price, conn)?;
    Ok(())
}

pub async fn update_earnings_ttm(
    ticker: &str,
    exchange: &table::Exchange,
    conn: &mut PgConnection,
) -> Result<(), BullsEyeError> {
    let (concat_statement_ttm, _, price) =
        bullseye_api::scrape_quarter_update(ticker, exchange).await?;
    let ttm_entries: Vec<NewStockEntry> = concat_statement_ttm
        .into_iter()
        .filter_map(|x| db::NewStockEntry::add_new_entry(x))
        .collect();
    let is_entries_existed = db::insert_stock_data_batch(ttm_entries, conn)?;
    if is_entries_existed {
        db::update_growths(conn)?;
        db::update_ratios_batch(conn)?;
        db::empty_earnings_date(ticker, table::get_exchange_string(&exchange), conn)?;
    }
    db::update_price(&ticker, table::get_exchange_string(&exchange), price, conn)?;
    Ok(())
}

pub async fn update_regular(
    ticker: &str,
    exchange: &table::Exchange,
    conn: &mut PgConnection,
    update_date: bool,
) -> Result<(), BullsEyeError> {
    let (earnings_date, price) = bullseye_api::scrape_regular_update(ticker, exchange).await?;
    if update_date {
        db::update_earnings_date(
            ticker,
            table::get_exchange_string(&exchange),
            earnings_date,
            conn,
        )?;
    }
    db::update_price(&ticker, table::get_exchange_string(&exchange), price, conn)?;
    Ok(())
}
