use crate::db::StockData;
use crate::schema::{stock_data, stock_health_eval};
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
    target_ticker: &str,
    target_exchange: &str,
    num_row: i64,
    conn: &mut PgConnection,
) -> Result<Vec<StockData>, DieselError> {
    use crate::schema::stock_data::dsl::*;
    load_table(
        stock_data
            .filter(ticker.eq(target_ticker))
            .filter(exchange.eq(target_exchange))
            .filter(duration.eq("T"))
            .order((year_str.desc(), quarter_str.desc())),
        num_row,
        conn,
    )
}

pub fn load_multiple_earnings_annual(
    target_ticker: &str,
    target_exchange: &str,
    num_row: i64,
    conn: &mut PgConnection,
) -> Result<Vec<StockData>, DieselError> {
    use crate::schema::stock_data::dsl::*;
    load_table(
        stock_data
            .filter(ticker.eq(target_ticker))
            .filter(exchange.eq(target_exchange))
            .filter(duration.eq("Y"))
            .order(year_str.desc()),
        num_row,
        conn,
    )
}

pub fn load_multiple_earnings_annual_filter<'a, F>(
    target_ticker: &'a str,
    target_exchange: &'a str,
    additional_filter: F,
    num_row: i64,
    conn: &mut PgConnection,
) -> Result<Vec<StockData>, DieselError>
where
    F: FnOnce(stock_data::BoxedQuery<'a, Pg>) -> stock_data::BoxedQuery<'a, Pg>,
{
    use crate::schema::stock_data::dsl::*;
    let base_query = stock_data
        .filter(ticker.eq(target_ticker))
        .filter(exchange.eq(target_exchange))
        .filter(duration.eq("Y"))
        .order(year_str.desc())
        .into_boxed();
    let query = additional_filter(base_query);
    load_table(query, num_row, conn)
}

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

// fn update_earnings_table<U>(
//     target_ticker: &str,
//     target_exchange: &str,
//     updates: U,
//     conn: &mut PgConnection,
// ) -> Result<usize, DieselError>
// where
//     U: AsChangeset<Target = stock_data::table>,
//     U::Changeset: QueryFragment<Pg>,
// {
//     use crate::schema::stock_data::dsl::*;
//     update_table(
//         stock_data
//             .filter(ticker.eq(target_ticker))
//             .filter(exchange.eq(target_exchange)),
//         updates,
//         conn,
//     )
// }
pub fn update_eval_table<U>(
    target_ticker: &str,
    target_exchange: &str,
    updates: U,
    conn: &mut PgConnection,
) -> Result<usize, DieselError>
where
    U: AsChangeset<Target = stock_health_eval::table>,
    U::Changeset: QueryFragment<Pg>,
{
    use crate::schema::stock_health_eval::dsl::*;
    update_table(
        stock_health_eval
            .filter(ticker.eq(target_ticker))
            .filter(exchange.eq(target_exchange)),
        updates,
        conn,
    )
}
