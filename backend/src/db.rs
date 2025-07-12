use crate::calculate;
use crate::errors::BullsEyeError;
use crate::helper;
use crate::metrics;
use crate::models::companies_model::Company;
use crate::models::earnings_model::EarningsReport;
use crate::models::forecast_models::Forecasts;
use crate::models::metrics_model::CurrentMetrics;
use crate::query;
use bullseye_api::model::Exchange;
use chrono::{Duration, Local};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error as DieselError;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection_pool() -> Result<Pool<ConnectionManager<PgConnection>>, BullsEyeError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager);
    match pool {
        Ok(val) => Ok(val),
        Err(_) => Err(BullsEyeError::DbPoolError),
    }
}

//TODO: add error
pub fn lookup_exchange(ticker: &str) -> Exchange {
    match &ticker[0..1].parse::<u64>() {
        Ok(_) => Exchange::TSE,
        Err(_) => Exchange::NASDAQ,
    }
}

/// extracts a specific column from the earnings table
pub fn extract_field<T, F>(data: &[EarningsReport], f: F) -> Vec<T>
where
    F: Fn(&EarningsReport) -> T,
{
    data.iter().map(f).collect()
}

/// joins all necessary tables for the list view
pub fn join_data(
    conn: &mut PgConnection,
) -> Result<Vec<(Company, CurrentMetrics, Forecasts)>, DieselError> {
    use crate::schema::companies::dsl::*;
    use crate::schema::current_metrics::dsl::*;
    use crate::schema::forecasts::dsl::*;
    let all_data = companies
        .inner_join(current_metrics)
        .inner_join(forecasts)
        .load::<(Company, CurrentMetrics, Forecasts)>(conn)?;
    Ok(all_data)
}

/// updates all missing ratios for all earnings data
pub fn update_ratios_batch(conn: &mut PgConnection) -> Result<(), DieselError> {
    use crate::schema::earnings_report::dsl::*;
    let target: Vec<EarningsReport> = earnings_report
        .filter(ratio_calculated.eq(false))
        .load::<EarningsReport>(conn)?;
    for i in target {
        i.update_ratios(conn)?;
    }
    Ok(())
}

/// updates all missing growth rates for all earnings data
pub fn update_growths_batch(conn: &mut PgConnection) -> Result<(), DieselError> {
    use crate::schema::earnings_report::dsl::*;
    let target = earnings_report
        .filter(growth_calculated.eq(false))
        .load::<EarningsReport>(conn)?;
    for i in target {
        i.update_yoy_growth(conn)?;
    }
    Ok(())
}

/// updates next earnings date after date is updated for the given ticker
pub fn update_earnings_date(
    comp_id: i32,
    earnings_date: Option<String>,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::forecasts::dsl::*;
    let latest_earning = EarningsReport::latest_quarter_data(comp_id, conn)?;
    let next_earnings = earnings_date
        .map(|date_str| helper::convert_date_from_string(&date_str).ok())
        .flatten();
    let valid_next_earnings =
        next_earnings.filter(|&date| date - latest_earning.period_ending >= Duration::days(90));
    query::update_forecasts_table(comp_id, next_earnings_date.eq(valid_next_earnings), conn)?;
    Ok(())
}

/// updates current stock price in the metrics table
pub fn update_price(
    comp_id: i32,
    price: Option<f64>,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::forecasts::dsl::*;
    query::update_forecasts_table(
        comp_id,
        (
            latest_price.eq(price),
            last_updated.eq(Local::now().date_naive()),
        ),
        conn,
    )?;
    Ok(())
}

/// updates next year revenue estimate in the metrics table
pub fn update_estimate(
    comp_id: i32,
    next_yr_rev: Option<f64>,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::forecasts::dsl::*;
    query::update_forecasts_table(comp_id, revenue_next_year.eq(next_yr_rev), conn)?;
    Ok(())
}

/// updates all multi-year growth rate columns in the metrics table
pub fn update_multi_yr_growth(comp_id: i32, conn: &mut PgConnection) -> Result<(), DieselError> {
    use crate::schema::current_metrics::dsl::*;
    let target = query::load_multiple_earnings_annual(comp_id, 4, conn)?;
    let net_interest_income_growth = extract_field(&target, |data| data.net_interest_growth_yoy);
    let net_interest_income_growth_ave =
        calculate::average_options(&net_interest_income_growth, true);
    let rev_growth = extract_field(&target, |data| data.revenue_growth_yoy);
    let rev_growth_ave = calculate::average_options(&rev_growth, true);
    let gp_growth = extract_field(&target, |data| data.gross_profit_growth_yoy);
    let gp_growth_ave = calculate::average_options(&gp_growth, true);
    let shares_change = extract_field(&target, |data| data.shares_change_yoy);
    let shares_change_ave = calculate::calculate_average_growth(shares_change);
    query::update_metrics_table(
        comp_id,
        (
            net_interest_income_growth_multi_year.eq(net_interest_income_growth_ave),
            revenue_growth_multi_year.eq(rev_growth_ave),
            gross_profit_growth_multi_year.eq(gp_growth_ave),
            shares_change_multi_year.eq(shares_change_ave),
        ),
        conn,
    )?;
    Ok(())
}

/// updates all short term trends columns in the metrics table
pub fn update_short_term_trends(
    comp_id: i32,
    conn: &mut PgConnection,
) -> Result<CurrentMetrics, DieselError> {
    use crate::schema::current_metrics::dsl::*;
    let target = query::load_multiple_earnings_ttm(comp_id, 8, conn)?;
    let nim_trend =
        metrics::get_short_term_trend_option(&target, |f| f.net_interest_margin, 4, true, 0.5, 2);
    let cor_trend =
        metrics::get_short_term_trend_option(&target, |f| f.cost_of_risk, 4, true, 0.5, 2);
    let gross_margin_trend =
        metrics::get_short_term_trend_option(&target, |f| f.gross_margin, 4, true, 0.5, 2);
    let sga_ratio_trend =
        metrics::get_short_term_trend_option(&target, |f| f.sga_gp_ratio, 4, true, 0.01, 2);
    let rnd_ratio_trend =
        metrics::get_short_term_trend_option(&target, |f| f.rnd_gp_ratio, 4, true, 0.01, 2);
    let operating_margin_trend =
        metrics::get_short_term_trend(&target, |f| f.operating_margin, 4, 0.5, 2);
    // query::update_metrics_table(
    //     comp_id,
    //     (
    //         gross_margin_short_term_trend.eq(gross_margin_trend),
    //         sga_short_term_trend.eq(sga_ratio_trend),
    //         rnd_short_term_trend.eq(rnd_ratio_trend),
    //         operating_margin_short_term_trend.eq(operating_margin_trend),
    //     ),
    //     conn,
    // )?;
    let updated_row = query::update_and_return_table(
        current_metrics.filter(company_id.eq(comp_id)),
        (
            net_interest_margin_short_term_trend.eq(nim_trend),
            cost_of_risk_short_term_trend.eq(cor_trend),
            gross_margin_short_term_trend.eq(gross_margin_trend),
            sga_short_term_trend.eq(sga_ratio_trend),
            rnd_short_term_trend.eq(rnd_ratio_trend),
            operating_margin_short_term_trend.eq(operating_margin_trend),
        ),
        conn,
    )?;
    Ok(updated_row)
}

/// updates all long term trends columns in the metrics table
pub fn update_long_term_trends(
    comp_id: i32,
    conn: &mut PgConnection,
) -> Result<CurrentMetrics, DieselError> {
    use crate::schema::current_metrics::dsl::*;
    let target = query::load_multiple_earnings_annual(comp_id, 6, conn)?;
    let nim_trend =
        metrics::get_long_term_trend_option(&target, |f| f.net_interest_margin, false, 1.);
    let cor_trend = metrics::get_long_term_trend_option(&target, |f| f.cost_of_risk, false, 1.);
    let gross_margin_trend =
        metrics::get_long_term_trend_option(&target, |f| f.gross_margin, false, 1.);
    let sga_ratio_trend =
        metrics::get_long_term_trend_option(&target, |f| f.sga_gp_ratio, false, 0.02);
    let rnd_ratio_trend =
        metrics::get_long_term_trend_option(&target, |f| f.rnd_gp_ratio, false, 0.02);
    let operating_margin_trend = metrics::get_long_term_trend(&target, |f| f.operating_margin, 1.);
    let dilution_trend = metrics::get_long_term_trend(&target, |f| f.shares_change_yoy, 2.);
    let retained_earnings_change_trend =
        metrics::get_long_term_trend(&target, |f| f.retained_earnings, 100.);
    let net_cash_change_trend = metrics::get_long_term_trend(&target, |f| f.net_cash, 100.);
    let ocfm_trend =
        metrics::get_long_term_trend_option(&target, |f| f.operating_cash_flow_margin, false, 1.);
    let ffom_trend = metrics::get_long_term_trend_option(&target, |f| f.ffo_margin, false, 1.);

    let updated_row = query::update_and_return_table(
        current_metrics.filter(company_id.eq(comp_id)),
        (
            net_interest_margin_long_term_trend.eq(nim_trend),
            cost_of_risk_long_term_trend.eq(cor_trend),
            gross_margin_long_term_trend.eq(gross_margin_trend),
            sga_long_term_trend.eq(sga_ratio_trend),
            rnd_long_term_trend.eq(rnd_ratio_trend),
            operating_margin_long_term_trend.eq(operating_margin_trend),
            shares_change_trend.eq(dilution_trend),
            retained_earnings_trend.eq(retained_earnings_change_trend),
            net_cash_trend.eq(net_cash_change_trend),
            operating_cash_flow_margin_trend.eq(ocfm_trend),
            ffo_margin_trend.eq(ffom_trend),
        ),
        conn,
    )?;
    Ok(updated_row)
}

/// copies the latest earnings data from earnings table and put them in the metrics table
pub fn copy_latest_data(comp_id: i32, conn: &mut PgConnection) -> Result<(), DieselError> {
    use crate::schema::current_metrics;
    use crate::schema::current_metrics::dsl::*;
    use crate::schema::earnings_report;
    use crate::schema::earnings_report::dsl::*;
    let latest_earnings_ttm: EarningsReport = query::load_first_row(
        earnings_report
            .filter(earnings_report::company_id.eq(comp_id))
            .filter(duration.eq("T"))
            .order((year_str.desc(), quarter_str.desc())),
        conn,
    )?;
    query::update_metrics_table(
        comp_id,
        (
            current_metrics::currency.eq(latest_earnings_ttm.currency),
            net_interest_income_growth_yoy_ttm.eq(latest_earnings_ttm.net_interest_growth_yoy),
            net_interest_margin_ttm.eq(latest_earnings_ttm.net_interest_margin),
            cost_of_risk_ttm.eq(latest_earnings_ttm.cost_of_risk),
            revenue_ttm.eq(latest_earnings_ttm.revenue),
            revenue_growth_yoy_ttm.eq(latest_earnings_ttm.revenue_growth_yoy),
            gross_profit_growth_yoy_ttm.eq(latest_earnings_ttm.gross_profit_growth_yoy),
            gross_margin_ttm.eq(latest_earnings_ttm.gross_margin),
            sga_ratio_ttm.eq(latest_earnings_ttm.sga_gp_ratio),
            rnd_ratio_ttm.eq(latest_earnings_ttm.rnd_gp_ratio),
            operating_margin_ttm.eq(latest_earnings_ttm.operating_margin),
            interest_expense_ratio_ttm.eq(latest_earnings_ttm.interest_expenses_op_income_ratio),
            net_margin_ttm.eq(latest_earnings_ttm.net_margin),
            shares_outstanding_diluted_ttm.eq(latest_earnings_ttm.shares_outstanding_diluted),
            shares_change_ttm.eq(latest_earnings_ttm.shares_change_yoy),
            retained_earnings_ttm.eq(latest_earnings_ttm.retained_earnings),
            net_cash_ttm.eq(latest_earnings_ttm.net_cash),
            operating_cash_flow_ttm.eq(latest_earnings_ttm.operating_cash_flow),
            operating_cash_flow_margin_ttm.eq(latest_earnings_ttm.operating_cash_flow_margin),
            free_cash_flow_ttm.eq(latest_earnings_ttm.free_cash_flow),
            free_cash_flow_margin_ttm.eq(latest_earnings_ttm.free_cash_flow_margin),
            ffo_margin_ttm.eq(latest_earnings_ttm.ffo_margin),
        ),
        conn,
    )?;
    Ok(())
}

/// updates price target in the metrics table, which is calculated with current-year or multi-year growth rate
pub fn update_price_target(comp_id: i32, conn: &mut PgConnection) -> Result<(), DieselError> {
    use crate::schema::companies;
    use crate::schema::companies::dsl::*;
    use crate::schema::current_metrics::dsl::*;
    use crate::schema::forecasts::dsl::*;
    let latest_earnings = EarningsReport::latest_quarter_data(comp_id, conn)?;
    let company: Company =
        query::load_first_row(companies.filter(companies::id.eq(comp_id)), conn)?;
    let target_metrics = CurrentMetrics::load_by_id(comp_id, conn)?;
    let net_margin_factor = calculate::get_net_margin_factor(&company.industry);
    let (curr_theoretical_net_margin, is_optimized) =
        metrics::is_net_margin_optimized(&latest_earnings, net_margin_factor);
    let curr_theoretical_net_income = match is_optimized {
        true => calculate::calculate_margin_portion(
            target_metrics.revenue_ttm,
            target_metrics.net_margin_ttm,
        ),
        false => target_metrics
            .revenue_ttm
            .map(|val| val * curr_theoretical_net_margin / 100.),
    };
    let curr_theoretical_eps = calculate::calculate_per_share(
        curr_theoretical_net_income,
        target_metrics.shares_outstanding_diluted_ttm,
    );
    let curr_theoretical_price_rev = calculate::calculate_price_target_option(
        curr_theoretical_eps,
        target_metrics.revenue_growth_yoy_ttm,
        target_metrics.shares_change_ttm,
    );
    let curr_theoretical_price_gp = calculate::calculate_price_target_option(
        curr_theoretical_eps,
        target_metrics.gross_profit_growth_yoy_ttm,
        target_metrics.shares_change_ttm,
    );
    let curr_theoretical_price_multi_rev = calculate::calculate_price_target_option(
        curr_theoretical_eps,
        target_metrics.revenue_growth_multi_year,
        target_metrics.shares_change_ttm,
    );
    let curr_theoretical_price_multi_gp = calculate::calculate_price_target_option(
        curr_theoretical_eps,
        target_metrics.gross_profit_growth_multi_year,
        target_metrics.shares_change_ttm,
    );

    query::update_metrics_table(
        comp_id,
        (
            theoretical_net_margin.eq(curr_theoretical_net_margin),
            is_net_margin_optimized.eq(is_optimized),
        ),
        conn,
    )?;
    query::update_forecasts_table(
        comp_id,
        (
            price_current_revenue_growth.eq(curr_theoretical_price_rev),
            price_current_gp_growth.eq(curr_theoretical_price_gp),
            price_multi_year_revenue_growth.eq(curr_theoretical_price_multi_rev),
            price_multi_year_gp_growth.eq(curr_theoretical_price_multi_gp),
        ),
        conn,
    )?;
    Ok(())
}

/// updates price target in the metrics table, which is calculated based on the guidance
pub fn update_guidance(comp_id: i32, conn: &mut PgConnection) -> Result<(), DieselError> {
    use crate::schema::forecasts::dsl::*;
    let latest_earnings = EarningsReport::latest_annual_data(comp_id, conn)?;
    let target_metrics = CurrentMetrics::load_by_id(comp_id, conn)?;
    let target_forecast = Forecasts::load_by_id(comp_id, conn)?;
    let curr_rev = latest_earnings.revenue;
    let next_yr_rev = target_forecast.revenue_next_year;
    let next_yr_rev_growth = next_yr_rev.map(|val| calculate::calculate_yoy_growth(val, curr_rev));
    let next_theoretical_net_income = match target_metrics.is_net_margin_optimized {
        Some(true) => {
            calculate::calculate_margin_portion(next_yr_rev, target_metrics.net_margin_ttm)
        }
        Some(false) => {
            calculate::calculate_margin_portion(next_yr_rev, target_metrics.theoretical_net_margin)
        }
        None => None,
    };
    let next_theoretical_eps = calculate::calculate_per_share(
        next_theoretical_net_income,
        target_metrics.shares_outstanding_diluted_ttm,
    );
    let next_yr_theoretical_price = calculate::calculate_price_target_option(
        next_theoretical_eps,
        next_yr_rev_growth,
        target_metrics.shares_change_ttm,
    );
    query::update_forecasts_table(
        comp_id,
        (
            revenue_growth_next_year.eq(next_yr_rev_growth),
            price_next_year_revenue_growth.eq(next_yr_theoretical_price),
        ),
        conn,
    )?;
    Ok(())
}

// pub fn run_sim<'a>(
//     symbol: &str,
//     exc: &str,
//     sim_net_margin: u8,
//     sim_growth: u8,
//     conn: &mut PgConnection,
// ) -> f64 {
//     use crate::schema::stock_data::dsl::*;
//     let target = stock_data
//         .filter(ticker.eq(symbol))
//         .filter(exchange.eq(exc))
//         .filter(duration.eq("T"))
//         .order((year_str.desc(), quarter_str.desc()))
//         .limit(1)
//         .first::<StockData>(conn)
//         .expect("Cannot load database. Failed to simulate");
//     let sim_net_margin: f64 = sim_net_margin as f64;
//     let sim_growth: f64 = sim_growth as f64;
//     let sim_theoretical_eps =
//         target.revenue() * sim_net_margin / 100. / target.shares_outstanding_diluted();
//     let sim_price = sim_theoretical_eps
//         * calculate::calculate_growth_adjustment_factor(sim_growth - target.shares_change_yoy());
//     sim_price
// }
