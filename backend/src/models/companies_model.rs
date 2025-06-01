use crate::query;
use crate::schema::companies;
use bullseye_api::profile::CompanyProfile;
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
    company_name: String,
    pub industry: String,
    isin: String,
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
}

#[derive(Insertable)]
#[diesel(table_name = companies)]
pub struct NewCompany<'a> {
    company_name: &'a str,
    industry: &'a str,
    isin: &'a str,
}
impl<'a> NewCompany<'a> {
    pub fn create_new_entry(name: &'a str, industry: &'a str, isin: &'a str) -> Self {
        NewCompany {
            company_name: name,
            industry: industry,
            isin: isin,
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
