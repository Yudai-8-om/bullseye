use crate::calculate;
use crate::metrics_v2;
use crate::models::earnings_model::NominalEarnings;
use crate::query;
use chrono::{Duration, Local, NaiveDate};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

/// extracts a specific column from the earnings table
pub fn extract_field<T, F>(data: &[NominalEarnings], f: F) -> Vec<T>
where
    F: Fn(&NominalEarnings) -> T,
{
    data.iter().map(f).collect()
}
/// updates all missing ratios for all earnings data
pub fn update_ratios_batch(conn: &mut PgConnection) -> Result<(), DieselError> {
    use crate::schema::nominal_earnings::dsl::*;
    let target: Vec<NominalEarnings> = nominal_earnings
        .filter(ratio_calculated.eq(false))
        .load::<NominalEarnings>(conn)?;
    for i in target {
        i.update_ratios(conn)?;
    }
    Ok(())
}

/// updates all missing growth rates for all earnings data
pub fn update_growths_batch(conn: &mut PgConnection) -> Result<(), DieselError> {
    use crate::schema::nominal_earnings::dsl::*;
    let target = nominal_earnings
        .filter(growth_calculated.eq(false))
        .load::<NominalEarnings>(conn)?;
    for i in target {
        i.update_gp_yoy_growth(conn)?;
    }
    Ok(())
}

/// updates next earnings date after date is updated for the given ticker
pub fn update_earnings_date(
    target_ticker: &str,
    target_exchange: &str,
    earnings_date: Option<String>,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::nominal_metrics::dsl::*;
    let next_earnings =
        earnings_date.map(|date_str| NaiveDate::parse_from_str(&date_str, "%b %d, %Y").unwrap());
    let valid_next_earnings = next_earnings.filter(|&date| {
        date >= Local::now().date_naive() || Local::now().date_naive() - date <= Duration::days(1)
    });
    query::update_metrics_table(
        target_ticker,
        target_exchange,
        next_earnings_date.eq(valid_next_earnings),
        conn,
    )?;
    Ok(())
}

/// empties next earning date data for the given ticker
pub fn empty_earnings_date(
    target_ticker: &str,
    target_exchange: &str,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::nominal_metrics::dsl::*;
    query::update_metrics_table(
        target_ticker,
        target_exchange,
        next_earnings_date.eq::<Option<NaiveDate>>(None),
        conn,
    )?;
    Ok(())
}

/// updates current stock price in the metrics table
pub fn update_price(
    target_ticker: &str,
    target_exchange: &str,
    price: Option<f64>,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::nominal_metrics::dsl::*;
    query::update_metrics_table(
        target_ticker,
        target_exchange,
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
    target_ticker: &str,
    target_exchange: &str,
    next_yr_rev: Option<f64>,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::nominal_metrics::dsl::*;
    query::update_metrics_table(
        target_ticker,
        target_exchange,
        revenue_next_year.eq(next_yr_rev),
        conn,
    )?;
    Ok(())
}

/// updates all multi-year growth rate columns in the metrics table
pub fn update_multi_yr_growth(
    target_ticker: &str,
    target_exchange: &str,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::nominal_metrics::dsl::*;
    let target = query::load_multiple_earnings_annual_new(target_ticker, target_exchange, 4, conn)?;
    let rev_growth = extract_field(&target, |data| data.revenue_growth_yoy);
    let rev_growth_ave = calculate::calculate_average_growth(rev_growth);
    let gp_growth = extract_field(&target, |data| data.gross_profit_growth_yoy);
    let gp_growth_ave = calculate::average_options(&gp_growth, true);
    let shares_change = extract_field(&target, |data| data.shares_change_yoy);
    let shares_change_ave = calculate::calculate_average_growth(shares_change);
    query::update_metrics_table(
        target_ticker,
        target_exchange,
        (
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
    target_ticker: &str,
    target_exchange: &str,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::nominal_metrics::dsl::*;
    let target = query::load_multiple_earnings_ttm_new(target_ticker, target_exchange, 8, conn)?;
    let gross_margin_trend = metrics_v2::get_gross_margin_trend_short(&target, 8);
    let sga_ratio_trend = metrics_v2::get_sga_ratio_trend_short(&target, 8);
    let rnd_ratio_trend = metrics_v2::get_rnd_ratio_trend_short(&target, 8);
    let operating_margin_trend = metrics_v2::get_operating_margin_trend_short(&target, 8);
    query::update_metrics_table(
        target_ticker,
        target_exchange,
        (
            gross_margin_short_term_trend.eq(gross_margin_trend),
            sga_short_term_trend.eq(sga_ratio_trend),
            rnd_short_term_trend.eq(rnd_ratio_trend),
            operating_margin_short_term_trend.eq(operating_margin_trend),
        ),
        conn,
    )?;
    Ok(())
}

/// updates all long term trends columns in the metrics table
pub fn update_long_term_trends(
    target_ticker: &str,
    target_exchange: &str,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::nominal_metrics::dsl::*;
    let target = query::load_multiple_earnings_annual_new(target_ticker, target_exchange, 6, conn)?;
    let gross_margin_trend = metrics_v2::get_long_term_trend(&target, |f| f.gross_margin, 1.);
    let sga_ratio_trend =
        metrics_v2::get_long_term_trend_option(&target, |f| f.sga_gp_ratio, false, 0.02);
    let rnd_ratio_trend =
        metrics_v2::get_long_term_trend_option(&target, |f| f.rnd_gp_ratio, false, 0.02);
    let operating_margin_trend =
        metrics_v2::get_long_term_trend(&target, |f| f.operating_margin, 1.);
    let dilution_trend = metrics_v2::get_long_term_trend(&target, |f| f.shares_change_yoy, 2.);
    let retained_earnings_change_trend =
        metrics_v2::get_long_term_trend(&target, |f| f.retained_earnings, 100.);
    let net_cash_change_trend = metrics_v2::get_long_term_trend(&target, |f| f.net_cash, 100.);

    let ocfm_trend = metrics_v2::get_long_term_trend_option(
        &target,
        |f| f.operating_cash_flow_margin,
        false,
        1.,
    );

    query::update_metrics_table(
        target_ticker,
        target_exchange,
        (
            gross_margin_long_term_trend.eq(gross_margin_trend),
            sga_long_term_trend.eq(sga_ratio_trend),
            rnd_long_term_trend.eq(rnd_ratio_trend),
            operating_margin_long_term_trend.eq(operating_margin_trend),
            shares_change_trend.eq(dilution_trend),
            retained_earnings_trend.eq(retained_earnings_change_trend),
            net_cash_trend.eq(net_cash_change_trend),
            operating_cash_flow_margin_trend.eq(ocfm_trend),
        ),
        conn,
    )?;
    Ok(())
}

/// copies the latest earnings data from earnings table and put them in the metrics table
pub fn copy_latest_data(
    target_ticker: &str,
    target_exchange: &str,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::nominal_earnings;
    use crate::schema::nominal_earnings::dsl::*;
    use crate::schema::nominal_metrics::dsl::*;
    let latest_earnings_ttm: NominalEarnings = query::load_first_row(
        nominal_earnings
            .filter(nominal_earnings::ticker.eq(target_ticker))
            .filter(nominal_earnings::exchange.eq(target_exchange))
            .filter(duration.eq("T"))
            .order((year_str.desc(), quarter_str.desc())),
        conn,
    )?;
    query::update_metrics_table(
        target_ticker,
        target_exchange,
        (
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
        ),
        conn,
    )?;
    Ok(())
}
