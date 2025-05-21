use crate::calculate;
use crate::metrics_v2;
use crate::models::earnings_model::NominalEarnings;
use crate::query;
use crate::schema::nominal_metrics;
use chrono::format::ParseError;
use chrono::{Duration, Local, NaiveDate};
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
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
#[diesel(table_name = nominal_metrics)]
#[serde(rename_all = "camelCase")]
pub struct NominalMetrics {
    id: i32,
    exchange: String,
    ticker: String,
    currency: String,
    favorite: bool,
    industry: String,
    next_earnings_date: Option<NaiveDate>,
    revenue_ttm: Option<f64>,
    revenue_growth_yoy_ttm: Option<f64>,
    revenue_growth_multi_year: Option<f64>,
    gross_profit_growth_yoy_ttm: Option<f64>,
    gross_profit_growth_multi_year: Option<f64>,
    gross_margin_ttm: Option<f64>,
    gross_margin_short_term_trend: Option<Trend>,
    gross_margin_long_term_trend: Option<Trend>,
    sga_ratio_ttm: Option<f64>, //low customer acquisition
    sga_short_term_trend: Option<Trend>,
    sga_long_term_trend: Option<Trend>,
    rnd_ratio_ttm: Option<f64>, //low innovation
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
    has_healthy_net_cash: Option<bool>,
    net_cash_trend: Option<Trend>,
    operating_cash_flow_ttm: Option<f64>,
    operating_cash_flow_margin_ttm: Option<f64>,
    operating_cash_flow_margin_trend: Option<Trend>,
    free_cash_flow_ttm: Option<f64>,
    free_cash_flow_margin_ttm: Option<f64>,
    revenue_next_year: Option<f64>,
    revenue_growth_next_year: Option<f64>,
    price_current_revenue_growth: Option<f64>,
    price_current_gp_growth: Option<f64>,
    price_next_year_revenue_growth: Option<f64>,
    price_multi_year_revenue_growth: Option<f64>,
    price_multi_year_gp_growth: Option<f64>,
    latest_price: Option<f64>,
    last_updated: Option<NaiveDate>,
}
impl NominalMetrics {
    /// retrieve metric data for the given ticker
    pub fn find(
        target_ticker: &str,
        target_exchange: &str,
        conn: &mut PgConnection,
    ) -> Result<Self, DieselError> {
        use crate::schema::nominal_metrics::dsl::*;
        let target = query::load_first_row(
            nominal_metrics
                .filter(ticker.eq(target_ticker))
                .filter(exchange.eq(target_exchange)),
            conn,
        )?;
        Ok(target)
    }
    /// returns if the metric data is already existed in db for the given ticker
    pub fn is_ticker_existed(
        target_ticker: &str,
        target_exchange: &str,
        conn: &mut PgConnection,
    ) -> bool {
        use crate::schema::nominal_metrics::dsl::*;
        let ticker_exists = diesel::dsl::select(diesel::dsl::exists(
            nominal_metrics
                .filter(ticker.eq(target_ticker))
                .filter(exchange.eq(target_exchange)),
        ))
        .get_result::<bool>(conn);
        match ticker_exists {
            Ok(exist_bool) => exist_bool,
            Err(_) => false,
        }
    }

    ///checks if the earnings data needs to be updated
    pub fn is_earnings_update_needed(&self) -> Option<bool> {
        self.next_earnings_date
            .map(|date| Local::now().date_naive() - date > Duration::days(1))
    }

    ///checks if the price data needs to be updated
    pub fn is_price_update_needed(&self) -> Option<bool> {
        self.last_updated
            .map(|last_updated| last_updated < Local::now().date_naive())
    }

    /// updates price target in the metrics table, which is calculated with current-year or multi-year growth rate
    pub fn update_price_target(
        &self,
        target_ticker: &str,
        target_exchange: &str,
        conn: &mut PgConnection,
    ) -> Result<(), DieselError> {
        use crate::schema::nominal_earnings;
        use crate::schema::nominal_earnings::dsl::*;
        use crate::schema::nominal_metrics::dsl::*;
        let target: NominalEarnings = query::load_first_row(
            nominal_earnings
                .filter(nominal_earnings::ticker.eq(target_ticker))
                .filter(nominal_earnings::exchange.eq(target_exchange))
                .filter(duration.eq("T"))
                .filter(gross_margin.ne(0.))
                .order((year_str.desc(), quarter_str.desc())),
            conn,
        )?;
        let net_margin_factor = calculate::get_net_margin_factor(&self.industry);

        let (curr_theoretical_net_margin, is_optimized) =
            metrics_v2::is_net_margin_optimized(&target, net_margin_factor);

        let curr_theoretical_net_income = match is_optimized {
            true => calculate::calculate_margin_portion(self.revenue_ttm, self.net_margin_ttm),
            false => self
                .revenue_ttm
                .map(|val| val * curr_theoretical_net_margin / 100.),
        };
        let curr_theoretical_eps = calculate::calculate_per_share(
            curr_theoretical_net_income,
            self.shares_outstanding_diluted_ttm,
        );

        let curr_theoretical_price_rev = calculate::calculate_price_target_option(
            curr_theoretical_eps,
            self.revenue_growth_yoy_ttm,
            self.shares_change_ttm,
        );
        let curr_theoretical_price_gp = calculate::calculate_price_target_option(
            curr_theoretical_eps,
            self.gross_profit_growth_yoy_ttm,
            self.shares_change_ttm,
        );
        let curr_theoretical_price_multi_rev = calculate::calculate_price_target_option(
            curr_theoretical_eps,
            self.revenue_growth_multi_year,
            self.shares_change_ttm,
        );
        let curr_theoretical_price_multi_gp = calculate::calculate_price_target_option(
            curr_theoretical_eps,
            self.gross_profit_growth_multi_year,
            self.shares_change_ttm,
        );

        query::update_metrics_table(
            target_ticker,
            target_exchange,
            (
                theoretical_net_margin.eq(curr_theoretical_net_margin),
                is_net_margin_optimized.eq(is_optimized),
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
    pub fn update_guidance(
        &self,
        target_ticker: &str,
        target_exchange: &str,
        conn: &mut PgConnection,
    ) -> Result<(), DieselError> {
        use crate::schema::nominal_metrics::dsl::*;
        let target = NominalEarnings::latest_annual_data(target_ticker, target_exchange, conn)?;
        let curr_rev = target.revenue;
        let next_yr_rev = self.revenue_next_year;
        let next_yr_rev_growth =
            next_yr_rev.map(|val| calculate::calculate_yoy_growth(val, curr_rev));
        let next_theoretical_net_income = match self.is_net_margin_optimized {
            Some(true) => calculate::calculate_margin_portion(next_yr_rev, self.net_margin_ttm),
            Some(false) => {
                calculate::calculate_margin_portion(next_yr_rev, self.theoretical_net_margin)
            }
            None => None,
        };
        let next_theoretical_eps = calculate::calculate_per_share(
            next_theoretical_net_income,
            self.shares_outstanding_diluted_ttm,
        );
        let next_yr_theoretical_price = calculate::calculate_price_target_option(
            next_theoretical_eps,
            next_yr_rev_growth,
            self.shares_change_ttm,
        );
        query::update_metrics_table(
            target_ticker,
            target_exchange,
            (
                revenue_growth_next_year.eq(next_yr_rev_growth),
                price_next_year_revenue_growth.eq(next_yr_theoretical_price),
            ),
            conn,
        )?;
        Ok(())
    }
}

#[derive(Insertable)]
#[diesel(table_name = nominal_metrics)]
pub struct NewNominalMetrics<'a> {
    exchange: &'a str,
    ticker: &'a str,
    currency: &'a str,
    favorite: bool,
    industry: String,
    next_earnings_date: Option<NaiveDate>,
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
    revenue_next_year: Option<f64>,
    revenue_growth_next_year: Option<f64>,
    price_current_revenue_growth: Option<f64>,
    price_current_gp_growth: Option<f64>,
    price_next_year_revenue_growth: Option<f64>,
    price_multi_year_revenue_growth: Option<f64>,
    price_multi_year_gp_growth: Option<f64>,
    latest_price: Option<f64>,
    last_updated: Option<NaiveDate>,
}

impl<'a> NewNominalMetrics<'a> {
    ///creates a new entry for the metrics table
    pub fn create_new_entry(
        exchange: &bullseye_api::table::Exchange,
        symbol: &'a str,
        currency_str: &'a str,
        industry_str: &'a str,
        next_earnings: Option<String>,
        price: Option<f64>,
        next_yr_rev: Option<f64>,
    ) -> Result<Self, ParseError> {
        let converted_earnings = convert_date(next_earnings)?;
        Ok(NewNominalMetrics {
            exchange: bullseye_api::table::get_exchange_string(exchange),
            ticker: symbol,
            currency: currency_str,
            favorite: false,
            industry: industry_str.to_string(),
            next_earnings_date: converted_earnings,
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
            revenue_next_year: next_yr_rev,
            revenue_growth_next_year: None,
            price_current_revenue_growth: None,
            price_current_gp_growth: None,
            price_next_year_revenue_growth: None,
            price_multi_year_revenue_growth: None,
            price_multi_year_gp_growth: None,
            latest_price: price,
            last_updated: Some(Local::now().date_naive()),
        })
    }
}
/// converts date string to NaiveDate type
pub fn convert_date(date: Option<String>) -> Result<Option<NaiveDate>, ParseError> {
    let next_earnings = date
        .map(|date_str: String| NaiveDate::parse_from_str(&date_str, "%b %d, %Y"))
        .transpose()?;
    let valid_next_earnings = next_earnings.filter(|&date| {
        date >= Local::now().date_naive() || Local::now().date_naive() - date <= Duration::days(1)
    });
    Ok(valid_next_earnings)
}

/// inserts new ticker data to the metrics database
pub fn insert_new_ticker(
    new_entry: NewNominalMetrics,
    conn: &mut PgConnection,
) -> Result<bool, DieselError> {
    use crate::schema::nominal_metrics::dsl::*;

    let updated = diesel::insert_into(nominal_metrics)
        .values(&new_entry)
        .on_conflict((exchange, ticker))
        .do_nothing()
        .execute(conn)?;
    Ok(updated > 0)
}
