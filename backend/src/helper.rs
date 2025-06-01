use chrono::format::ParseError;
use chrono::NaiveDate;

pub fn convert_date_from_string(date_str: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(date_str, "%b %d, %Y")
}

pub fn convert_period_ending_str(ending_date: &str) -> Result<NaiveDate, ParseError> {
    let date_vec: Vec<&str> = ending_date.trim().split(" ").collect();
    let date_str = format!(
        "{} {}, {}",
        date_vec[date_vec.len() - 3],
        date_vec[date_vec.len() - 2],
        date_vec[date_vec.len() - 1]
    );
    convert_date_from_string(&date_str)
}

/// converts fiscal string ("Qn 20xx") into year and quarter integer
pub fn process_fiscal_string(fiscal_str: &str) -> Option<(i16, i16)> {
    let fiscal_vec: Vec<&str> = fiscal_str.split(' ').collect();
    if fiscal_vec.len() == 1 {
        return None;
    }
    let fiscal_q = match fiscal_vec[0][1..].parse() {
        Ok(q) => q,
        Err(_) => 0, // annual data will be displayed as 0
    };
    let fiscal_y = match fiscal_vec[1].parse() {
        Ok(y) => y,
        Err(_) => 0,
    };
    Some((fiscal_y, fiscal_q))
}
