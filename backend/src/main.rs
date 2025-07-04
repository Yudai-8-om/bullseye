// use axum::http::StatusCode;
use axum::{extract::Path, extract::State, routing::get, Json, Router};
use db::{establish_connection_pool, lookup_exchange};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use errors::BullsEyeError;
use models::earnings_model::EarningsReport;
use models::forecast_models::Forecasts;
use models::metrics_model::CurrentMetrics;
use models::returning_model::ReturningModel;
use tower_http::cors::{Any, CorsLayer};
// use serde::Deserialize;
// use http::Method;

mod calculate;
mod db;
mod errors;
mod helper;
mod metrics;
mod models;
mod query;
mod schema;
mod services;

async fn search(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(ticker): Path<String>,
) -> Result<Json<ReturningModel>, BullsEyeError> {
    let exchange = lookup_exchange(&ticker);
    let conn = &mut pool.get().unwrap();
    let company = services::get_company(&ticker, &exchange, conn).await?;
    let forecast = Forecasts::load_by_id(company.id, conn)?;
    let earnings_update_needed = forecast.is_earnings_update_needed();
    if earnings_update_needed {
        let latest_earnings = EarningsReport::latest_quarter_data_if_existed(company.id, conn)?;
        let all_earnings = match latest_earnings {
            Some(earnings) => earnings.quarter_str == 3,
            None => true,
        };
        if all_earnings {
            services::update_earnings_all(company.id, &ticker, &exchange, conn).await?;
            services::update_metrics_annual(company.id, conn)?;
        } else {
            services::update_earnings_ttm(company.id, &ticker, &exchange, conn).await?;
            services::update_metrics_ttm(company.id, conn)?;
        }
    } else {
        let regular_update_needed = forecast.is_regular_update_needed();
        if regular_update_needed {
            services::update_regular(company.id, &ticker, &exchange, conn).await?;
        }
        services::update_metrics_annual(company.id, conn)?;
    }
    let all_metrics = CurrentMetrics::load_by_id(company.id, conn)?;
    let all_forecasts = Forecasts::load_by_id(company.id, conn)?;
    Ok(Json(ReturningModel::new(
        company,
        all_metrics,
        all_forecasts,
    )))
}
async fn list_all(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
) -> Result<Json<Vec<ReturningModel>>, BullsEyeError> {
    let conn = &mut pool.get().unwrap();
    let all_companies: Vec<ReturningModel> = services::get_all_companies(conn)?;
    Ok(Json(all_companies))
}

// #[derive(Deserialize)]
// struct Params {
//     ticker: String,
//     net_margin: u8,
//     growth: u8,
// }

// async fn simulate(Path(params): Path<Params>) {
//     let exchange = match params.ticker.parse::<u64>() {
//         Ok(_) => model::Exchange::TSE,
//         Err(_) => model::Exchange::NASDAQ,
//     };
//     let conn = &mut db::establish_connection();
//     let sim_price = db::run_sim(
//         &params.ticker,
//         model::get_exchange_string(&exchange),
//         params.net_margin,
//         params.growth,
//         conn,
//     );
//     println!("Simulated Price: {sim_price}");
// }

#[tokio::main]
async fn main() {
    // build our application with a single route
    // let allowed_origins = vec![
    //     "http://localhost:5173".parse().unwrap(),
    //     "http://localhost:3000".parse().unwrap(),
    // ];
    let cors = CorsLayer::new().allow_origin(Any);
    // .allow_methods([Method::GET, Method::POST]);
    let pool = establish_connection_pool().unwrap();
    let app = Router::new()
        .route("/screener", get(list_all))
        .route("/companies/{ticker}", get(search))
        .with_state(pool)
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
