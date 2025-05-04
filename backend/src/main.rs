// use axum::http::StatusCode;
use axum::{extract::Path, routing::get, Json, Router};
use bullseye_api::table;
use bullseye_api::table::FinancialStatement;
use bullseye_api::{self, client::ScraperError};
use chrono::{Duration, Local};
use db::{StockData, StockHealthEval};
// use http::Method;
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};
mod db;
mod schema;
mod services;

async fn search(Path(ticker): Path<String>) -> Result<Json<StockHealthEval>, ScraperError> {
    let exchange = match ticker.parse::<u64>() {
        Ok(_) => table::Exchange::TSE,
        Err(_) => table::Exchange::NASDAQ,
    };
    let conn = &mut db::establish_connection();
    if !StockHealthEval::is_ticker_existed(&ticker, table::get_exchange_string(&exchange), conn) {
        services::handle_new_ticker(&ticker, &exchange, conn).await?;
    } else {
        let target =
            db::StockHealthEval::search(&ticker, table::get_exchange_string(&exchange), conn);
        let earnings_update_needed = target
            .next_earnings_date()
            .map(|earnings_date| Local::now().date_naive() - earnings_date > Duration::days(3));
        if let Some(true) | None = earnings_update_needed {
            let latest_earnings = db::StockData::latest_quarter_data(
                &ticker,
                table::get_exchange_string(&exchange),
                conn,
            );
            if latest_earnings.quarter() == 3 {
                services::update_earnings_all(&ticker, &exchange, conn).await?;
            } else {
                services::update_earnings_ttm(&ticker, &exchange, conn).await?;
            }
        } else {
            let price_update_needed = target
                .last_updated()
                .map(|last_updated| last_updated < Local::now().date_naive());
            if let Some(true) | None = price_update_needed {
                services::update_price(&ticker, &exchange, conn).await?;
            }
        }
    }
    db::run_eval_prep(&ticker, table::get_exchange_string(&exchange), conn);
    db::run_eval(&ticker, table::get_exchange_string(&exchange), conn);
    let healtheval =
        db::StockHealthEval::search(&ticker, table::get_exchange_string(&exchange), conn);
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

//Temporary-----------------------------------------
#[derive(Deserialize)]
struct Params {
    ticker: String,
    net_margin: u8,
    growth: u8,
}

async fn simulate(Path(params): Path<Params>) {
    let exchange = match params.ticker.parse::<u64>() {
        Ok(_) => table::Exchange::TSE,
        Err(_) => table::Exchange::NASDAQ,
    };
    let conn = &mut db::establish_connection();
    let sim_price = db::run_sim(
        &params.ticker,
        table::get_exchange_string(&exchange),
        params.net_margin,
        params.growth,
        conn,
    );
    println!("Simulated Price: {sim_price}");
}

// temporary-------------------------------------

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
        .route("/search/{ticker}", get(search))
        .route("/print", get(print_stock_data))
        .route("/printeval", get(print_eval_data))
        .route("/evaluate/{ticker}", get(evaluate))
        .route("/simulate/{ticker}/{net_margin}/{growth}", get(simulate))
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
