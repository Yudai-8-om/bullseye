use crate::query;
use crate::schema::companies;
use bullseye_api::model::get_exchange_string;
use bullseye_api::model::Exchange;
use bullseye_api::profile::CompanyProfile;
use chrono::{Duration, Local, NaiveDate};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = companies)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub id: i32,
    pub company_name: String,
    pub industry: String,
    pub isin: String,
    pub exchange: String,
    pub ticker: String,
    last_updated: NaiveDate,
}
impl Company {
    /// loads campany data if existed
    pub fn load_if_existed(
        company_profile: &CompanyProfile,
        conn: &mut PgConnection,
    ) -> Result<Option<Self>, DieselError> {
        use crate::schema::companies::dsl::*;
        let curr_isin = &company_profile.isin_number;
        let target =
            query::load_first_row(companies.filter(isin.eq(curr_isin)), conn).optional()?;
        Ok(target)
    }
    /// loads campany data by ticker symbol if existed
    pub fn load_by_ticker_if_existed(
        curr_ticker: &str,
        curr_exchange: &str,
        conn: &mut PgConnection,
    ) -> Result<Option<Self>, DieselError> {
        use crate::schema::companies::dsl::*;
        let target = query::load_first_row(
            companies
                .filter(ticker.eq(curr_ticker))
                .filter(exchange.eq(curr_exchange)),
            conn,
        )
        .optional()?;
        Ok(target)
    }
    pub fn ticker_check_needed(&self) -> bool {
        Local::now().date_naive() - self.last_updated >= Duration::days(90)
    }
}

#[derive(Insertable)]
#[diesel(table_name = companies)]
pub struct NewCompany<'a> {
    company_name: &'a str,
    industry: &'a str,
    isin: &'a str,
    exchange: &'a str,
    ticker: &'a str,
    last_updated: NaiveDate,
}
impl<'a> NewCompany<'a> {
    pub fn create_new_entry(
        name: &'a str,
        industry: &'a str,
        isin: &'a str,
        exchange: &Exchange,
        ticker: &'a str,
    ) -> Self {
        NewCompany {
            company_name: name,
            industry: industry,
            isin: isin,
            exchange: get_exchange_string(exchange),
            ticker: ticker,
            last_updated: Local::now().date_naive(),
        }
    }
    pub fn add_new_company(&self, conn: &mut PgConnection) -> Result<Company, DieselError> {
        use crate::schema::companies::dsl::*;

        let new_company = diesel::insert_into(companies)
            .values(self)
            .on_conflict(isin)
            .do_nothing()
            .get_result::<Company>(conn)?;
        Ok(new_company)
    }
}
