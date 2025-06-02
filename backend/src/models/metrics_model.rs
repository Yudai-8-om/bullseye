use crate::query;
use crate::schema::current_metrics;
use chrono::format::ParseError;
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgConnection, PgValue};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, AsExpression)]
#[diesel(sql_type = Text)]
pub enum Trend {
    Uptrend,
    Downtrend,
    Flat,
    Irrelevant,
}

impl ToSql<Text, Pg> for Trend
where
    str: ToSql<Text, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match self {
            Trend::Uptrend => <str as ToSql<Text, Pg>>::to_sql("up", out),
            Trend::Downtrend => <str as ToSql<Text, Pg>>::to_sql("down", out),
            Trend::Flat => <str as ToSql<Text, Pg>>::to_sql("flat", out),
            Trend::Irrelevant => <str as ToSql<Text, Pg>>::to_sql("irrelevant", out),
        }
    }
}

impl FromSql<Text, Pg> for Trend {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes())?;
        match s {
            "up" => Ok(Trend::Uptrend),
            "down" => Ok(Trend::Downtrend),
            "flat" => Ok(Trend::Flat),
            "irrelevant" => Ok(Trend::Irrelevant),
            x => Err(format!("Invalid trend value detected: {}", x).into()),
        }
    }
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = current_metrics)]
#[serde(rename_all = "camelCase")]
pub struct CurrentMetrics {
    id: i32,
    pub company_id: i32,
    pub currency: String,
    pub net_interest_income_growth_yoy_ttm: Option<f64>,
    pub net_interest_income_growth_multi_year: Option<f64>,
    pub net_interest_margin_ttm: Option<f64>,
    pub net_interest_margin_short_term_trend: Option<Trend>,
    pub net_interest_margin_long_term_trend: Option<Trend>,
    pub cost_of_risk_ttm: Option<f64>,
    pub cost_of_risk_short_term_trend: Option<Trend>,
    pub cost_of_risk_long_term_trend: Option<Trend>,
    pub revenue_ttm: Option<f64>,
    pub revenue_growth_yoy_ttm: Option<f64>,
    pub revenue_growth_multi_year: Option<f64>,
    pub gross_profit_growth_yoy_ttm: Option<f64>,
    pub gross_profit_growth_multi_year: Option<f64>,
    pub gross_margin_ttm: Option<f64>,
    pub gross_margin_short_term_trend: Option<Trend>,
    pub gross_margin_long_term_trend: Option<Trend>,
    pub sga_ratio_ttm: Option<f64>,
    pub sga_short_term_trend: Option<Trend>,
    pub sga_long_term_trend: Option<Trend>,
    pub rnd_ratio_ttm: Option<f64>,
    pub rnd_short_term_trend: Option<Trend>,
    pub rnd_long_term_trend: Option<Trend>,
    pub operating_margin_ttm: Option<f64>,
    pub operating_margin_short_term_trend: Option<Trend>,
    pub operating_margin_long_term_trend: Option<Trend>,
    pub interest_expense_ratio_ttm: Option<f64>,
    pub net_margin_ttm: Option<f64>,
    pub theoretical_net_margin: Option<f64>,
    pub is_net_margin_optimized: Option<bool>,
    pub shares_outstanding_diluted_ttm: Option<f64>,
    pub shares_change_ttm: Option<f64>,
    pub shares_change_multi_year: Option<f64>,
    pub shares_change_trend: Option<Trend>,
    pub retained_earnings_ttm: Option<f64>,
    pub retained_earnings_trend: Option<Trend>,
    pub net_cash_ttm: Option<f64>,
    pub has_healthy_net_cash: Option<bool>,
    pub net_cash_trend: Option<Trend>,
    pub operating_cash_flow_ttm: Option<f64>,
    pub operating_cash_flow_margin_ttm: Option<f64>,
    pub operating_cash_flow_margin_trend: Option<Trend>,
    pub free_cash_flow_ttm: Option<f64>,
    pub free_cash_flow_margin_ttm: Option<f64>,
    pub ffo_margin_ttm: Option<f64>,
    pub ffo_margin_trend: Option<Trend>,
}
impl CurrentMetrics {
    /// retrieve metric data for the given company id
    pub fn load_by_id(comp_id: i32, conn: &mut PgConnection) -> Result<Self, DieselError> {
        use crate::schema::current_metrics::dsl::*;
        let target = query::load_first_row(current_metrics.filter(company_id.eq(comp_id)), conn)?;
        Ok(target)
    }
}

#[derive(Insertable)]
#[diesel(table_name = current_metrics)]
pub struct NewCurrentMetrics<'a> {
    company_id: i32,
    currency: &'a str,
    net_interest_income_growth_yoy_ttm: Option<f64>,
    net_interest_income_growth_multi_year: Option<f64>,
    net_interest_margin_ttm: Option<f64>,
    net_interest_margin_short_term_trend: Option<Trend>,
    net_interest_margin_long_term_trend: Option<Trend>,
    cost_of_risk_ttm: Option<f64>,
    cost_of_risk_short_term_trend: Option<Trend>,
    cost_of_risk_long_term_trend: Option<Trend>,
    revenue_ttm: Option<f64>,
    revenue_growth_yoy_ttm: Option<f64>,
    revenue_growth_multi_year: Option<f64>,
    gross_profit_growth_yoy_ttm: Option<f64>,
    gross_profit_growth_multi_year: Option<f64>,
    gross_margin_ttm: Option<f64>,
    gross_margin_short_term_trend: Option<Trend>,
    gross_margin_long_term_trend: Option<Trend>,
    sga_ratio_ttm: Option<f64>,
    sga_short_term_trend: Option<Trend>,
    sga_long_term_trend: Option<Trend>,
    rnd_ratio_ttm: Option<f64>,
    rnd_short_term_trend: Option<Trend>,
    rnd_long_term_trend: Option<Trend>,
    operating_margin_ttm: Option<f64>,
    operating_margin_short_term_trend: Option<Trend>,
    operating_margin_long_term_trend: Option<Trend>,
    interest_expense_ratio_ttm: Option<f64>,
    net_margin_ttm: Option<f64>,
    theoretical_net_margin: Option<f64>,
    is_net_margin_optimized: Option<bool>,
    shares_outstanding_diluted_ttm: Option<f64>,
    shares_change_ttm: Option<f64>,
    shares_change_multi_year: Option<f64>,
    shares_change_trend: Option<Trend>,
    retained_earnings_ttm: Option<f64>,
    retained_earnings_trend: Option<Trend>,
    net_cash_ttm: Option<f64>,
    has_healthy_net_cash: Option<bool>, //yet
    net_cash_trend: Option<Trend>,
    operating_cash_flow_ttm: Option<f64>,
    operating_cash_flow_margin_ttm: Option<f64>,
    operating_cash_flow_margin_trend: Option<Trend>,
    free_cash_flow_ttm: Option<f64>,
    free_cash_flow_margin_ttm: Option<f64>,
    ffo_margin_ttm: Option<f64>,
    ffo_margin_trend: Option<Trend>,
}

impl<'a> NewCurrentMetrics<'a> {
    ///creates a new entry for the metrics table
    pub fn create_new_entry(company_id: i32, currency_str: &'a str) -> Result<Self, ParseError> {
        Ok(NewCurrentMetrics {
            company_id: company_id,
            currency: currency_str,
            net_interest_income_growth_yoy_ttm: None,
            net_interest_income_growth_multi_year: None,
            net_interest_margin_ttm: None,
            net_interest_margin_short_term_trend: None,
            net_interest_margin_long_term_trend: None,
            cost_of_risk_ttm: None,
            cost_of_risk_short_term_trend: None,
            cost_of_risk_long_term_trend: None,
            revenue_ttm: None,
            revenue_growth_yoy_ttm: None,
            revenue_growth_multi_year: None,
            gross_profit_growth_yoy_ttm: None,
            gross_profit_growth_multi_year: None,
            gross_margin_ttm: None,
            gross_margin_short_term_trend: None,
            gross_margin_long_term_trend: None,
            sga_ratio_ttm: None,
            sga_short_term_trend: None,
            sga_long_term_trend: None,
            rnd_ratio_ttm: None,
            rnd_short_term_trend: None,
            rnd_long_term_trend: None,
            operating_margin_ttm: None,
            operating_margin_short_term_trend: None,
            operating_margin_long_term_trend: None,
            interest_expense_ratio_ttm: None,
            net_margin_ttm: None,
            theoretical_net_margin: None,
            is_net_margin_optimized: None,
            shares_outstanding_diluted_ttm: None,
            shares_change_ttm: None,
            shares_change_multi_year: None,
            shares_change_trend: None,
            retained_earnings_ttm: None,
            retained_earnings_trend: None,
            net_cash_ttm: None,
            has_healthy_net_cash: None,
            net_cash_trend: None,
            operating_cash_flow_ttm: None,
            operating_cash_flow_margin_ttm: None,
            operating_cash_flow_margin_trend: None,
            free_cash_flow_ttm: None,
            free_cash_flow_margin_ttm: None,
            ffo_margin_ttm: None,
            ffo_margin_trend: None,
        })
    }
    /// inserts new ticker data to the metrics database
    pub fn insert_new_metrics(&self, conn: &mut PgConnection) -> Result<bool, DieselError> {
        use crate::schema::current_metrics::dsl::*;
        let updated = diesel::insert_into(current_metrics)
            .values(self)
            .on_conflict(company_id)
            .do_nothing()
            .execute(conn)?;
        Ok(updated > 0)
    }
}
