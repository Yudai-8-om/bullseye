use crate::db;
use crate::errors::BullsEyeError;
use crate::models::companies_model::{Company, NewCompany};
use crate::models::earnings_model;
use crate::models::earnings_model::NewEarningsReport;
use crate::models::forecast_models::NewForecasts;
use crate::models::metrics_model::{CurrentMetrics, NewCurrentMetrics};
use bullseye_api::model::Exchange;
use diesel::pg::PgConnection;

/// runs when handling new ticker data.
/// creates new company row for all 3 tables.
pub async fn get_company(
    ticker: &str,
    exchange: &Exchange,
    conn: &mut PgConnection,
) -> Result<Company, BullsEyeError> {
    let company_profile = bullseye_api::scrape_profile(ticker, exchange).await?;
    if let Some(company) = Company::load_if_existed(&company_profile, conn)? {
        Ok(company)
    } else {
        let new_company_entry = NewCompany::create_new_entry(
            &company_profile.company_name,
            &company_profile.industry,
            &company_profile.isin_number,
        );
        let new_company = new_company_entry.add_new_company(conn)?;
        let new_metrics_entry =
            NewCurrentMetrics::create_new_entry(new_company.id, exchange, ticker, "")?;
        new_metrics_entry.insert_new_metrics(conn)?;
        let new_forecast_entry = NewForecasts::create_empty(new_company.id);
        new_forecast_entry.insert_new_forecast(conn)?;
        Ok(new_company)
    }
}

/// runs after Q4 Earnings or for the initial update.
/// includes:
///     storing latest earnings data (TTM & Annual)
///     filling missing fields
///     updating estimates and current stock price
pub async fn update_earnings_all(
    company_id: i32,
    ticker: &str,
    exchange: &Exchange,
    conn: &mut PgConnection,
) -> Result<(), BullsEyeError> {
    let (earnings_enum_ttm, earnings_enum_annual, currency, earnings_date, price, next_yr_rev) =
        bullseye_api::scrape_all(ticker, exchange).await?;
    // let company_id = CurrentMetrics::get_company_id(ticker, get_exchange_string(exchange), conn)?;
    let ttm_entries = NewEarningsReport::create_new_entry(company_id, &currency, earnings_enum_ttm);
    let annual_entries =
        NewEarningsReport::create_new_entry(company_id, &currency, earnings_enum_annual);
    let is_ttm_entries_existed = earnings_model::insert_earnings_report_batch(ttm_entries, conn)?;
    let is_annual_entries_existed =
        earnings_model::insert_earnings_report_batch(annual_entries, conn)?;
    if is_ttm_entries_existed || is_annual_entries_existed {
        db::update_growths_batch(conn)?;
        db::update_ratios_batch(conn)?;
    }
    db::update_earnings_date(company_id, earnings_date, conn)?;
    db::update_estimate(company_id, next_yr_rev, conn)?;
    db::update_price(company_id, price, conn)?;
    Ok(())
}

/// runs  after Q1-Q3 Earnings.
/// includes:
///     storing latest earnings data (TTM)
///     filling missing fields
///     updating estimates and current stock price
pub async fn update_earnings_ttm(
    company_id: i32,
    ticker: &str,
    exchange: &Exchange,
    conn: &mut PgConnection,
) -> Result<(), BullsEyeError> {
    let (earnings_enum_ttm, currency, earnings_date, price, next_yr_rev) =
        bullseye_api::scrape_quarter_update(ticker, exchange).await?;
    let ttm_entries = NewEarningsReport::create_new_entry(company_id, &currency, earnings_enum_ttm);
    let is_entries_existed = earnings_model::insert_earnings_report_batch(ttm_entries, conn)?;
    if is_entries_existed {
        db::update_growths_batch(conn)?;
        db::update_ratios_batch(conn)?;
    }
    db::update_earnings_date(company_id, earnings_date, conn)?;
    db::update_estimate(company_id, next_yr_rev, conn)?;
    db::update_price(company_id, price, conn)?;
    Ok(())
}

/// updates earnings date and current stock price
pub async fn update_regular(
    company_id: i32,
    ticker: &str,
    exchange: &Exchange,
    conn: &mut PgConnection,
) -> Result<(), BullsEyeError> {
    let (earnings_date, price, next_yr_rev) =
        bullseye_api::scrape_regular_update(ticker, exchange).await?;
    db::update_earnings_date(company_id, earnings_date, conn)?;
    db::update_estimate(company_id, next_yr_rev, conn)?;
    db::update_price(company_id, price, conn)?;
    Ok(())
}

/// updates all metrics after earnings
pub fn update_metrics_ttm(
    comp_id: i32,
    conn: &mut PgConnection,
) -> Result<CurrentMetrics, BullsEyeError> {
    db::copy_latest_data(comp_id, conn)?;
    let latest_metrics = db::update_short_term_trends(comp_id, conn)?;
    db::update_price_target(comp_id, conn)?;
    db::update_guidance(comp_id, conn)?;
    Ok(latest_metrics)
}

/// updates all metrics after earnings. This only runs after Q4 Earnings.
pub fn update_metrics_annual(
    comp_id: i32,
    conn: &mut PgConnection,
) -> Result<CurrentMetrics, BullsEyeError> {
    db::copy_latest_data(comp_id, conn)?;
    db::update_short_term_trends(comp_id, conn)?;
    db::update_multi_yr_growth(comp_id, conn)?;
    let latest_metrics = db::update_long_term_trends(comp_id, conn)?;
    db::update_price_target(comp_id, conn)?;
    db::update_guidance(comp_id, conn)?;
    Ok(latest_metrics)
}
