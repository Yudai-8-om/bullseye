use crate::helper;
use crate::query;
use crate::schema::forecasts;
use chrono::format::ParseError;
use chrono::{Duration, Local, NaiveDate};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = forecasts)]
#[serde(rename_all = "camelCase")]
pub struct Forecasts {
    id: i32,
    company_id: i32,
    next_earnings_date: Option<NaiveDate>,
    latest_price: Option<f64>,
    last_updated: Option<NaiveDate>,
    pub revenue_next_year: Option<f64>,
    revenue_growth_next_year: Option<f64>,
    price_current_revenue_growth: Option<f64>,
    price_current_gp_growth: Option<f64>,
    price_next_year_revenue_growth: Option<f64>,
    price_multi_year_revenue_growth: Option<f64>,
    price_multi_year_gp_growth: Option<f64>,
}

impl Forecasts {
    pub fn load_by_id(comp_id: i32, conn: &mut PgConnection) -> Result<Self, DieselError> {
        use crate::schema::forecasts::dsl::*;
        let target = query::load_first_row(forecasts.filter(company_id.eq(comp_id)), conn)?;
        Ok(target)
    }
    ///checks if the earnings data needs to be updated
    pub fn is_earnings_update_needed(&self) -> bool {
        self.next_earnings_date
            .map(|date| Local::now().date_naive() - date >= Duration::days(1))
            .unwrap_or(true)
    }

    ///checks if the price data needs to be updated
    pub fn is_regular_update_needed(&self) -> bool {
        self.last_updated
            .map(|last_updated| last_updated < Local::now().date_naive())
            .unwrap_or(true)
    }
}

#[derive(Insertable)]
#[diesel(table_name = forecasts)]
pub struct NewForecasts {
    company_id: i32,
    next_earnings_date: Option<NaiveDate>,
    latest_price: Option<f64>,
    last_updated: Option<NaiveDate>,
    pub revenue_next_year: Option<f64>,
    revenue_growth_next_year: Option<f64>,
    price_current_revenue_growth: Option<f64>,
    price_current_gp_growth: Option<f64>,
    price_next_year_revenue_growth: Option<f64>,
    price_multi_year_revenue_growth: Option<f64>,
    price_multi_year_gp_growth: Option<f64>,
}
impl NewForecasts {
    pub fn create_empty(company_id: i32) -> Self {
        NewForecasts {
            company_id: company_id,
            next_earnings_date: None,
            latest_price: None,
            last_updated: None,
            revenue_next_year: None,
            revenue_growth_next_year: None,
            price_current_revenue_growth: None,
            price_current_gp_growth: None,
            price_next_year_revenue_growth: None,
            price_multi_year_revenue_growth: None,
            price_multi_year_gp_growth: None,
        }
    }
    pub fn create_new_entry(
        company_id: i32,
        earnings_date: Option<String>,
        price: Option<f64>,
        rev_next_yr: Option<f64>,
    ) -> Result<Self, ParseError> {
        let valid_date = get_validate_next_earnings_date(earnings_date)?;
        Ok(NewForecasts {
            company_id: company_id,
            next_earnings_date: valid_date,
            latest_price: price,
            last_updated: Some(Local::now().date_naive()),
            revenue_next_year: rev_next_yr,
            revenue_growth_next_year: None,
            price_current_revenue_growth: None,
            price_current_gp_growth: None,
            price_next_year_revenue_growth: None,
            price_multi_year_revenue_growth: None,
            price_multi_year_gp_growth: None,
        })
    }
    pub fn insert_new_forecast(&self, conn: &mut PgConnection) -> Result<bool, DieselError> {
        use crate::schema::forecasts::dsl::*;
        let updated = diesel::insert_into(forecasts)
            .values(self)
            .on_conflict(company_id)
            .do_nothing()
            .execute(conn)?;
        Ok(updated > 0)
    }
}

/// converts date string to NaiveDate type
fn get_validate_next_earnings_date(date: Option<String>) -> Result<Option<NaiveDate>, ParseError> {
    let next_earnings = date
        .map(|date_str| helper::convert_date_from_string(&date_str))
        .transpose()?;
    let valid_next_earnings = next_earnings.filter(|&date| {
        date >= Local::now().date_naive() || Local::now().date_naive() - date <= Duration::days(1)
    });
    Ok(valid_next_earnings)
}
