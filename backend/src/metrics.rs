use crate::calculate;
use crate::db;
use crate::db::StockData;

pub fn is_share_diluted(stock_data: &StockData) -> bool {
    stock_data.shares_change_yoy() > 3.
}
pub fn is_active_share_buyback(stock_data: &StockData) -> bool {
    stock_data.shares_change_yoy() < 0.
}

pub fn is_room_for_buyback(stock_data: &StockData) -> bool {
    stock_data.retained_earnings() > 0.
}

pub fn is_net_margin_optimized(stock_data: &StockData, margin_factor: f64) -> (f64, bool) {
    let curr_gross_margin = stock_data.gross_margin();
    let curr_theoretical_net_margin = match curr_gross_margin {
        Some(val) => val / margin_factor,
        None => 100. / margin_factor,
    };
    let curr_net_margin = stock_data.net_margin();
    let curr_operating_margin = stock_data.operating_margin();
    let is_optimized =
        curr_theoretical_net_margin <= curr_net_margin && curr_operating_margin > curr_net_margin;
    (curr_theoretical_net_margin, is_optimized)
}

pub fn is_ocf_positive(stock_data: &StockData) -> bool {
    stock_data.operating_cash_flow() > 0.
}
pub fn is_rnd_light(stock_data: &StockData) -> Option<bool> {
    let curr_rnd_gp_ratio = stock_data.rnd_gp_ratio();
    curr_rnd_gp_ratio.map(|val| val <= 0.3)
}

pub fn is_sga_light(stock_data: &StockData) -> Option<bool> {
    let curr_sga_gp_ratio = stock_data.sga_gp_ratio();
    curr_sga_gp_ratio.map(|val| val <= 0.3)
}
pub fn has_low_interest_expense(stock_data: &StockData) -> Option<bool> {
    let curr_interst_expense_ratio = stock_data.interest_expenses_op_income_ratio();
    curr_interst_expense_ratio.map(|val| val >= -0.15)
}
pub fn has_healthy_cash_position(stock_data: &StockData) -> bool {
    let curr_net_income = stock_data.net_income();
    let curr_net_cash = stock_data.net_cash();
    curr_net_cash >= 0. || (-curr_net_cash / curr_net_income < 2. && curr_net_income > 0.)
}

pub fn get_gross_margin_trend_short(target: Vec<StockData>, min_len: usize) -> (bool, bool) {
    if target.len() < min_len {
        return (false, false);
    }
    let gross_margin_vec = db::extract_field(&target, |data| data.gross_margin());
    let trend_vec = calculate::calculate_short_term_trend_option(&gross_margin_vec, true, 0.5);
    let (is_uptrend, is_downtrend) = calculate::analyze_trend(trend_vec, 2);
    (is_uptrend, is_downtrend)
}
