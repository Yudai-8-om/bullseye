use crate::models::metrics_model::Trend;

pub fn calculate_price_target(eps: f64, growth_pct: f64, share_change: f64) -> f64 {
    eps * calculate_growth_adjustment_factor(growth_pct - share_change)
}

pub fn calculate_price_target_option(
    eps: Option<f64>,
    growth_pct: Option<f64>,
    share_change: Option<f64>,
) -> Option<f64> {
    eps.zip(growth_pct)
        .zip(share_change)
        .map(|((e, g), s)| e * calculate_growth_adjustment_factor(g - s))
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
pub fn calculate_yoy_growth_option(curr_val: Option<f64>, prev_val: Option<f64>) -> Option<f64> {
    if curr_val <= Some(0.) || prev_val <= Some(0.) {
        return None;
    }
    curr_val
        .zip(prev_val)
        .map(|(curr, prev)| ((curr / prev * 100. - 100.) * 100.).round() / 100.)
}

pub fn calculate_short_term_trend(vals: &[f64], length: usize, flat_threshold: f64) -> Vec<Trend> {
    if vals.len() < length + 4 {
        return vec![Trend::Irrelevant];
    }
    vals.iter()
        .take(length)
        .enumerate()
        .map(|(i, v)| {
            let past_four_vals = &vals[i + 1..i + 5];
            let past_four_ave = past_four_vals.iter().sum::<f64>() / past_four_vals.len() as f64;
            // average_options(past_four_vals, ignore_none);
            if v - past_four_ave >= flat_threshold {
                Trend::Uptrend
            } else if v - past_four_ave <= -flat_threshold {
                Trend::Downtrend
            } else {
                Trend::Flat
            }
        })
        .collect()
}

pub fn calculate_short_term_trend_option(
    vals: &[Option<f64>],
    length: usize,
    ignore_none: bool,
    flat_threshold: f64,
) -> Vec<Trend> {
    if vals.len() < length + 4 {
        return vec![Trend::Irrelevant];
    }
    vals.iter()
        .take(length)
        .enumerate()
        .map(|(i, v)| {
            let past_four_vals = &vals[i + 1..i + 5];
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

pub fn calculate_long_term_trend(vals: &[f64], flat_threshold: f64) -> Trend {
    if vals.len() < 2 {
        return Trend::Irrelevant;
    }
    let split_point = vals.len() / 2;
    let new_interval = &vals[..split_point];
    let old_interval = &vals[split_point..];
    let new_ave = new_interval.iter().sum::<f64>() / new_interval.len() as f64;
    let old_ave = old_interval.iter().sum::<f64>() / old_interval.len() as f64;
    if new_ave - old_ave >= flat_threshold {
        Trend::Uptrend
    } else if new_ave - old_ave <= -flat_threshold {
        Trend::Downtrend
    } else {
        Trend::Flat
    }
}

pub fn calculate_long_term_trend_option(
    vals: &[Option<f64>],
    ignore_none: bool,
    flat_threshold: f64,
) -> Trend {
    if vals.len() < 2 {
        return Trend::Irrelevant;
    }
    let split_point = vals.len() / 2;
    let new_interval = &vals[..split_point];
    let old_interval = &vals[split_point..];
    let new_ave = average_options(new_interval, ignore_none);
    let old_ave = average_options(old_interval, ignore_none);
    match (new_ave, old_ave) {
        (Some(new_val), Some(old_val)) => {
            if new_val - old_val >= flat_threshold {
                Trend::Uptrend
            } else if new_val - old_val <= -flat_threshold {
                Trend::Downtrend
            } else {
                Trend::Flat
            }
        }
        _ => Trend::Irrelevant,
    }
}

pub fn calculate_average_growth(growth_vec: Vec<f64>) -> f64 {
    growth_vec.iter().sum::<f64>() / growth_vec.len() as f64
}

pub fn average_options(options: &[Option<f64>], ignore_none: bool) -> Option<f64> {
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

pub fn concat_trend(trend_vec: Vec<Trend>, count_threshold: usize) -> Trend {
    let uptrend = trend_vec
        .iter()
        .filter(|val| **val == Trend::Uptrend)
        .count();
    let downtrend = trend_vec
        .iter()
        .filter(|val| **val == Trend::Downtrend)
        .count();
    let flat = trend_vec.iter().filter(|val| **val == Trend::Flat).count();
    if uptrend >= count_threshold && uptrend > downtrend {
        Trend::Uptrend
    } else if downtrend >= count_threshold && downtrend > uptrend {
        Trend::Downtrend
    } else if flat >= count_threshold || uptrend == downtrend {
        Trend::Flat
    } else {
        Trend::Irrelevant
    }
}

pub fn calculate_ratio(value: Option<f64>, total: f64) -> Option<f64> {
    if total <= 0. {
        return None;
    }
    value.map(|val| (val / total * 100.).round() / 100.)
}

pub fn calculate_ratio_option(value: Option<f64>, total: Option<f64>) -> Option<f64> {
    if total <= Some(0.) {
        return None;
    }
    value
        .zip(total)
        .map(|(top, bottom)| (top / bottom * 100.).round() / 100.)
}

pub fn calculate_ratio_as_pct(value: Option<f64>, total: f64) -> Option<f64> {
    if total <= 0. {
        return None;
    }
    value.map(|val| (val / total * 10000.).round() / 100.)
}

pub fn get_net_margin_factor(industry: &str) -> f64 {
    match industry {
        "Airlines" => 20.,
        "Grocery Stores" | "Department Stores" => 9.,
        "Discount Stores" => 6.,
        "Apparel Retail" | "Apparel Manufacturing" | "Footwear & Accessories" => 5.,
        "Banks - Diversified" | "Banks - Regional" => 5.,
        "Internet Retail" | "Specialty Retail" => 3.5,
        "Specialty Industrial Machinery" => 2.5,
        "Semiconductors"
        | "Semiconductor Equipment & Materials"
        | "Auto Manufacturers"
        | "Auto Parts"
        | "Consumer Electronics"
        | "Electrical Equipment & Parts" => 2.,
        _ => 3.,
    }
}

pub fn calculate_margin_portion(total: Option<f64>, margin: Option<f64>) -> Option<f64> {
    total.zip(margin).map(|(x, y)| x * y / 100.)
}

pub fn calculate_per_share(total: Option<f64>, share: Option<f64>) -> Option<f64> {
    total.zip(share).map(|(x, y)| x / y)
}
