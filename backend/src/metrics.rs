use crate::db;
use crate::models::metrics_model::Trend;
use crate::{calculate, models::earnings_model::EarningsReport};

/// returns the oretical net margin calculated based on the current gross margin and their industry
pub fn is_net_margin_optimized(stock_data: &EarningsReport, margin_factor: f64) -> (f64, bool) {
    let curr_gross_margin = stock_data.gross_margin.unwrap_or(100.);
    let curr_theoretical_net_margin = curr_gross_margin / margin_factor;
    let curr_net_margin = stock_data.net_margin;
    let curr_operating_margin = stock_data.operating_margin;
    let is_optimized =
        curr_theoretical_net_margin <= curr_net_margin && curr_operating_margin > curr_net_margin;
    (curr_theoretical_net_margin, is_optimized)
}

/// tells if the current net cash is at a healthy level
pub fn has_healthy_cash_position(stock_data: &EarningsReport) -> bool {
    let curr_net_income = stock_data.net_income;
    let curr_net_cash = stock_data.net_cash;
    curr_net_cash >= 0. || (-curr_net_cash / curr_net_income < 2. && curr_net_income > 0.)
}

pub fn get_short_term_trend<F>(
    target: &[EarningsReport],
    field: F,
    length: usize,
    flat_threshold: f64,
    count_threshold: usize,
) -> Trend
where
    F: Fn(&EarningsReport) -> f64,
{
    let values = db::extract_field(&target, field);
    let trend_vec = calculate::calculate_short_term_trend(&values, length, flat_threshold);
    let short_term_trend = calculate::concat_trend(trend_vec, count_threshold);
    short_term_trend
}

pub fn get_short_term_trend_option<F>(
    target: &[EarningsReport],
    field: F,
    length: usize,
    ignore_none: bool,
    flat_threshold: f64,
    count_threshold: usize,
) -> Trend
where
    F: Fn(&EarningsReport) -> Option<f64>,
{
    let values = db::extract_field(&target, field);
    let trend_vec =
        calculate::calculate_short_term_trend_option(&values, length, ignore_none, flat_threshold);
    let short_term_trend = calculate::concat_trend(trend_vec, count_threshold);
    short_term_trend
}

pub fn get_long_term_trend<F>(target: &[EarningsReport], field: F, flat_threshold: f64) -> Trend
where
    F: Fn(&EarningsReport) -> f64,
{
    let values = db::extract_field(&target, field);
    let long_term_trend = calculate::calculate_long_term_trend(&values, flat_threshold);
    long_term_trend
}

/// outputs long-term trend for the given metrics
pub fn get_long_term_trend_option<F>(
    target: &[EarningsReport],
    field: F,
    ignore_none: bool,
    flat_threshold: f64,
) -> Trend
where
    F: Fn(&EarningsReport) -> Option<f64>,
{
    let values = db::extract_field(&target, field);
    let long_term_trend =
        calculate::calculate_long_term_trend_option(&values, ignore_none, flat_threshold);
    long_term_trend
}
