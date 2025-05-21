use crate::calculate;
use crate::db_operations;
use crate::models::earnings_model::NominalEarnings;
use crate::models::metrics_model::Trend;

/// returns theoretical net margin calculated based on the current gross margin and their industry
pub fn is_net_margin_optimized(stock_data: &NominalEarnings, margin_factor: f64) -> (f64, bool) {
    let curr_theoretical_net_margin = stock_data.gross_margin / margin_factor;
    let curr_net_margin = stock_data.net_margin;
    let curr_operating_margin = stock_data.operating_margin;
    let is_optimized =
        curr_theoretical_net_margin <= curr_net_margin && curr_operating_margin > curr_net_margin;
    (curr_theoretical_net_margin, is_optimized)
}

/// tells if the current net cash is at a healthy level
pub fn has_healthy_cash_position(stock_data: &NominalEarnings) -> bool {
    let curr_net_income = stock_data.net_income;
    let curr_net_cash = stock_data.net_cash;
    curr_net_cash >= 0. || (-curr_net_cash / curr_net_income < 2. && curr_net_income > 0.)
}

pub fn get_gross_margin_trend_short(target: &[NominalEarnings], min_len: usize) -> Option<Trend> {
    if target.len() < min_len {
        return None;
    }
    let gross_margin_vec = db_operations::extract_field(&target, |data| data.gross_margin);
    let trend_vec = calculate::calculate_short_term_trend(&gross_margin_vec, 0.5);
    Some(calculate::concat_trend(trend_vec, 2))
}

pub fn get_sga_ratio_trend_short(target: &[NominalEarnings], min_len: usize) -> Option<Trend> {
    if target.len() < min_len {
        return None;
    }
    let sga_ratio_vec = db_operations::extract_field(&target, |data| data.sga_gp_ratio);
    let trend_vec = calculate::calculate_short_term_trend_option(&sga_ratio_vec, true, 0.01);
    Some(calculate::concat_trend(trend_vec, 2))
}

pub fn get_rnd_ratio_trend_short(target: &[NominalEarnings], min_len: usize) -> Option<Trend> {
    if target.len() < min_len {
        return None;
    }
    let rnd_ratio_vec = db_operations::extract_field(&target, |data| data.rnd_gp_ratio);
    let trend_vec = calculate::calculate_short_term_trend_option(&rnd_ratio_vec, true, 0.01);
    Some(calculate::concat_trend(trend_vec, 2))
}

pub fn get_operating_margin_trend_short(
    target: &[NominalEarnings],
    min_len: usize,
) -> Option<Trend> {
    if target.len() < min_len {
        return None;
    }
    let operating_margin_vec = db_operations::extract_field(&target, |data| data.operating_margin);
    let trend_vec = calculate::calculate_short_term_trend(&operating_margin_vec, 0.5);
    Some(calculate::concat_trend(trend_vec, 2))
}

pub fn get_long_term_trend<F>(target: &[NominalEarnings], field: F, flat_threshold: f64) -> Trend
where
    F: Fn(&NominalEarnings) -> f64,
{
    let values = db_operations::extract_field(&target, field);
    let long_term_trend = calculate::calculate_long_term_trend(&values, flat_threshold);
    long_term_trend
}

/// outputs long-term trend for the given metrics
pub fn get_long_term_trend_option<F>(
    target: &[NominalEarnings],
    field: F,
    ignore_none: bool,
    flat_threshold: f64,
) -> Trend
where
    F: Fn(&NominalEarnings) -> Option<f64>,
{
    let values = db_operations::extract_field(&target, field);
    let long_term_trend =
        calculate::calculate_long_term_trend_option(&values, ignore_none, flat_threshold);
    long_term_trend
}
