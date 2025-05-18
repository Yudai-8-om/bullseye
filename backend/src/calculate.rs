pub fn calculate_price_target(eps: f64, growth_pct: f64, share_change: f64) -> f64 {
    eps * calculate_growth_adjustment_factor(growth_pct - share_change)
}

pub fn calculate_growth_adjustment_factor(growth: f64) -> f64 {
    let (punishment, adjusted_growth) = match growth {
        val if val > 50. => (0.6, 1.5),
        val if val > 40. => (0.6 + 0.2 * (50. - val) / 10., 1. + val / 100.),
        val if val > 20. => (0.8 + 0.2 * (40. - val) / 20., 1. + val / 100.),
        val if val > 1. => (1., 1. + val / 100.),
        _ => (1., 1.01),
    };
    let factor = (adjusted_growth.powi(10) / 2.6 * 10. + 5.) * punishment;
    factor
}

pub fn calculate_yoy_growth(curr_val: f64, prev_val: f64) -> f64 {
    let growth = ((curr_val / prev_val * 100. - 100.) * 100.).round() / 100.;
    growth
}

pub fn calculate_average_growth(growth_vec: Vec<f64>) -> f64 {
    growth_vec.iter().sum::<f64>() / growth_vec.len() as f64
}

#[derive(PartialEq)]
pub enum Trend {
    Uptrend,
    Downtrend,
    Flat,
    Irrelevant,
}

// fn calculate_short_term_trend(vals: Vec<f64>) -> Vec<bool> {
//     vals.iter()
//         .take(4)
//         .enumerate()
//         .map(|(i, v)| {
//             let past_four_vals = &vals[i + 1..i + 5];
//             let past_four_sum = past_four_vals.iter().sum::<f64>();
//             let past_four_ave = past_four_sum / past_four_vals.len() as f64;
//             let is_uptrend = v >= &past_four_ave;
//             is_uptrend
//         })
//         .collect()
// }

pub fn calculate_short_term_trend_option(
    vals: &[Option<f64>],
    ignore_none: bool,
    flat_threshold: f64,
) -> Vec<Trend> {
    vals.iter()
        .take(4)
        .enumerate()
        .map(|(i, v)| {
            let past_four_vals = &vals[i + 1..i + 5]; //TODO range error fix needed
            let past_four_ave = average_options(past_four_vals, ignore_none);
            match (v, past_four_ave) {
                (&Some(curr), Some(prev)) => {
                    if curr - prev >= flat_threshold {
                        Trend::Uptrend
                    } else if curr - prev <= -flat_threshold {
                        Trend::Downtrend
                    } else {
                        Trend::Flat
                    }
                }
                _ => Trend::Irrelevant,
            }
        })
        .collect()
}

fn average_options(options: &[Option<f64>], ignore_none: bool) -> Option<f64> {
    if ignore_none {
        let valid_val: Vec<f64> = options.iter().flatten().cloned().collect();
        let valid_len = valid_val.len();
        let valid_sum: f64 = valid_val.iter().sum();
        Some(valid_sum / valid_len as f64)
    } else {
        if options.iter().any(|x| x.is_none()) {
            return None;
        }
        let valid_len = options.len();
        let valid_sum: f64 = options.iter().flatten().cloned().sum();
        Some(valid_sum / valid_len as f64)
    }
}

pub fn analyze_trend(trend_vec: Vec<Trend>, count_threshold: usize) -> (bool, bool) {
    let uptrend = trend_vec
        .iter()
        .filter(|val| **val == Trend::Uptrend)
        .count();
    let downtrend = trend_vec
        .iter()
        .filter(|val| **val == Trend::Downtrend)
        .count();
    (
        uptrend >= count_threshold && uptrend > downtrend,
        downtrend >= count_threshold && downtrend > uptrend,
    )
}

pub fn calculate_ratio(value: Option<f64>, total: f64) -> Option<f64> {
    if total <= 0. {
        return None;
    }
    value.map(|val| (val / total * 100.).round() / 100.)
}
