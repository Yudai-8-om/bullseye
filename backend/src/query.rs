use crate::models::earnings_model::EarningsReport;
use crate::schema::{current_metrics, earnings_report, forecasts};
use diesel::associations::HasTable;
use diesel::helper_types::Limit;
use diesel::pg::{Pg, PgConnection};
use diesel::prelude::*;
use diesel::query_builder::{
    AsChangeset, AsQuery, IntoUpdateTarget, QueryFragment, UpdateStatement,
};
use diesel::query_dsl::methods::{LimitDsl, LoadQuery};
use diesel::result::Error as DieselError;

pub fn load_table<'query, T, U>(
    table: T,
    num_row: i64,
    conn: &mut PgConnection,
) -> Result<Vec<U>, DieselError>
where
    T: LimitDsl + RunQueryDsl<PgConnection>,
    T::Output: LoadQuery<'query, PgConnection, U>,
{
    table.limit(num_row).load::<U>(conn)
}

pub fn load_first_row<'query, T, U>(table: T, conn: &mut PgConnection) -> Result<U, DieselError>
where
    T: LimitDsl + RunQueryDsl<PgConnection>,
    Limit<T>: LoadQuery<'query, PgConnection, U>,
{
    table.first::<U>(conn)
}

pub fn load_multiple_earnings_ttm(
    target_id: i32,
    num_row: i64,
    conn: &mut PgConnection,
) -> Result<Vec<EarningsReport>, DieselError> {
    use crate::schema::earnings_report::dsl::*;
    load_table(
        earnings_report
            .filter(company_id.eq(target_id))
            .filter(duration.eq("T"))
            .order((year_str.desc(), quarter_str.desc())),
        num_row,
        conn,
    )
}

pub fn load_multiple_earnings_annual(
    target_id: i32,
    num_row: i64,
    conn: &mut PgConnection,
) -> Result<Vec<EarningsReport>, DieselError> {
    use crate::schema::earnings_report::dsl::*;
    load_table(
        earnings_report
            .filter(company_id.eq(target_id))
            .filter(duration.eq("Y"))
            .order(year_str.desc()),
        num_row,
        conn,
    )
}

// pub fn load_multiple_earnings_annual_filter<'a, F>(
//     target_ticker: &'a str,
//     target_exchange: &'a str,
//     additional_filter: F,
//     num_row: i64,
//     conn: &mut PgConnection,
// ) -> Result<Vec<StockData>, DieselError>
// where
//     F: FnOnce(stock_data::BoxedQuery<'a, Pg>) -> stock_data::BoxedQuery<'a, Pg>,
// {
//     use crate::schema::stock_data::dsl::*;
//     let base_query = stock_data
//         .filter(ticker.eq(target_ticker))
//         .filter(exchange.eq(target_exchange))
//         .filter(duration.eq("Y"))
//         .order(year_str.desc())
//         .into_boxed();
//     let query = additional_filter(base_query);
//     load_table(query, num_row, conn)
// }

fn update_table<T, U>(table: T, updates: U, conn: &mut PgConnection) -> Result<usize, DieselError>
where
    T: IntoUpdateTarget + HasTable,
    U: AsChangeset<Target = T::Table>,
    T::Table: QueryFragment<Pg>,
    T::WhereClause: QueryFragment<Pg>,
    UpdateStatement<T::Table, T::WhereClause, U::Changeset>: AsQuery + QueryFragment<Pg>,
    U::Changeset: QueryFragment<Pg>,
{
    diesel::update(table).set(updates).execute(conn)
}

pub fn update_and_return_table<'query, T, U, V>(
    table: T,
    updates: U,
    conn: &mut PgConnection,
) -> Result<V, DieselError>
where
    T: IntoUpdateTarget + HasTable,
    U: AsChangeset<Target = T::Table>,
    T::Table: QueryFragment<Pg>,
    T::WhereClause: QueryFragment<Pg>,
    UpdateStatement<T::Table, T::WhereClause, U::Changeset>:
        AsQuery + QueryFragment<Pg> + LoadQuery<'query, PgConnection, V>,
    U::Changeset: QueryFragment<Pg>,
{
    diesel::update(table).set(updates).get_result(conn)
}

/// updates specific earnings data for the given earnings
pub fn update_earnings_table<U>(
    curr_id: i32,
    updates: U,
    conn: &mut PgConnection,
) -> Result<usize, DieselError>
where
    U: AsChangeset<Target = earnings_report::table>,
    U::Changeset: QueryFragment<Pg>,
{
    use crate::schema::earnings_report::dsl::*;
    update_table(earnings_report.filter(id.eq(curr_id)), updates, conn)
}

pub fn update_metrics_table<U>(
    comp_id: i32,
    updates: U,
    conn: &mut PgConnection,
) -> Result<usize, DieselError>
where
    U: AsChangeset<Target = current_metrics::table>,
    U::Changeset: QueryFragment<Pg>,
{
    use crate::schema::current_metrics::dsl::*;
    update_table(
        current_metrics.filter(company_id.eq(comp_id)),
        updates,
        conn,
    )
}

pub fn update_forecasts_table<U>(
    comp_id: i32,
    updates: U,
    conn: &mut PgConnection,
) -> Result<usize, DieselError>
where
    U: AsChangeset<Target = forecasts::table>,
    U::Changeset: QueryFragment<Pg>,
{
    use crate::schema::forecasts::dsl::*;
    update_table(forecasts.filter(company_id.eq(comp_id)), updates, conn)
}
