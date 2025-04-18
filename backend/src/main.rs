// use axum::http::StatusCode;
use axum::{extract::Path, routing::get, Json, Router};
use bullseye_api::table;
use bullseye_api::table::{ConcatStatement, FinancialStatement};
use bullseye_api::{self, client::ScraperError};
use chrono::Local;
use db::{NewStockEntry, StockData, StockHealthEval};
use http::Method;
use tower_http::cors::{Any, CorsLayer};
mod db;
mod schema;

async fn search(Path(ticker): Path<String>) -> Result<Json<StockHealthEval>, ScraperError> {
    let exchange = match ticker.parse::<u64>() {
        Ok(_) => table::Exchange::TSE,
        Err(_) => table::Exchange::NASDAQ,
    };
    let conn = &mut db::establish_connection();
    if !StockHealthEval::is_ticker_existed(&ticker, table::get_exchange_string(&exchange), conn) {
        println!("Ticker doesn't exist in database.");
        let (
            concat_statement_ttm,
            concat_statement_annual,
            currency,
            industry,
            earnings_date,
            price,
        ) = bullseye_api::scrape_init(&ticker, &exchange).await?;
        let ttm_entries: Vec<NewStockEntry> = concat_statement_ttm
            .into_iter()
            .filter_map(|x| db::NewStockEntry::add_new_entry(x))
            .collect();
        let annual_entries: Vec<NewStockEntry> = concat_statement_annual
            .into_iter()
            .filter_map(|x| db::NewStockEntry::add_new_entry(x))
            .collect();
        db::insert_stock_data_batch(ttm_entries, conn);
        db::insert_stock_data_batch(annual_entries, conn);
        db::update_growths(conn);
        db::update_ratios(conn);
        db::add_new_eval(
            &ticker,
            &exchange,
            &currency,
            &industry,
            earnings_date,
            price,
            conn,
        )
        .await;
    } else {
        let target =
            db::StockHealthEval::search(&ticker, table::get_exchange_string(&exchange), conn);
        let update_needed = target
            .last_updated()
            .map(|last_updated| last_updated < Local::now().date_naive());
        let (earnings_date, price) = bullseye_api::scrape_update(&ticker, &exchange).await?;
        if let Some(true) | None = update_needed {
            println!("Updating Earnings Date and Price");
            db::update_earnings_date(
                &ticker,
                table::get_exchange_string(&exchange),
                earnings_date,
                conn,
            );
            db::update_price(&ticker, table::get_exchange_string(&exchange), price, conn);
        } else {
            println!("No Database updates needed");
        }
    }
    db::run_eval_prep(&ticker, table::get_exchange_string(&exchange), conn);
    db::run_eval(&ticker, table::get_exchange_string(&exchange), conn);
    let healtheval =
        db::StockHealthEval::search(&ticker, table::get_exchange_string(&exchange), conn);
    println!("Completed!");
    Ok(Json(healtheval))
}

async fn evaluate(Path(ticker): Path<String>) {
    let exchange = match ticker.parse::<u64>() {
        Ok(_) => table::Exchange::TSE,
        Err(_) => table::Exchange::NASDAQ,
    };
    let conn = &mut db::establish_connection();
    db::run_eval_prep(&ticker, table::get_exchange_string(&exchange), conn);
    db::run_eval(&ticker, table::get_exchange_string(&exchange), conn);
    println!("Completed!");
}

// async fn write_data(Path(ticker): Path<String>) -> Result<(), ScraperError> {
//     let exchange = match ticker.parse::<u64>() {
//         Ok(_) => table::Exchange::TSE,
//         Err(_) => table::Exchange::NASDAQ,
//     };
//     let concat_statement_ttm =
//         bullseye_api::scrape_init(&ticker, &exchange, table::Term::TTM).await?;
//     // if let Err(_) = concat_statement_ttm {
//     //     return Err("TTM data scrape error".to_string());
//     // }
//     let concat_statement_annual =
//         bullseye_api::scrape_init(&ticker, &exchange, table::Term::Annual).await?;
//     // if let Err(_) = concat_statement_annual {
//     //     return Err("Annual data scrape error".to_string());
//     // }
//     let ttm_entries: Vec<NewStockEntry> = concat_statement_ttm
//         .into_iter()
//         .filter_map(|x| db::NewStockEntry::add_new_entry(x))
//         .collect();
//     let annual_entries: Vec<NewStockEntry> = concat_statement_annual
//         .into_iter()
//         .filter_map(|x| db::NewStockEntry::add_new_entry(x))
//         .collect();
//     let conn = &mut db::establish_connection();
//     db::insert_stock_data_batch(ttm_entries, conn);
//     db::insert_stock_data_batch(annual_entries, conn);
//     db::add_new_eval(&ticker, &exchange, conn).await;
//     db::update_growths(conn);
//     db::update_ratios(conn);
//     println!("Completed!");
//     Ok(())
// }

async fn print_stock_data() -> Json<Vec<StockData>> {
    let conn = &mut db::establish_connection();
    let table = db::load_stock_data(conn);
    Json(table)
}

async fn print_eval_data() -> Json<Vec<StockHealthEval>> {
    let conn = &mut db::establish_connection();
    let table = db::load_health_data(conn);
    Json(table)
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    // let allowed_origins = vec![
    //     "http://localhost:5173".parse().unwrap(),
    //     "http://localhost:3000".parse().unwrap(),
    // ];
    let cors = CorsLayer::new().allow_origin(Any);
    // .allow_methods([Method::GET, Method::POST]);
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        // .route("/write/{ticker}", get(write_data))
        .route("/search/{ticker}", get(search))
        .route("/print", get(print_stock_data))
        .route("/printeval", get(print_eval_data))
        .route("/evaluate/{ticker}", get(evaluate))
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
