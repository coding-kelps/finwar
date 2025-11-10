pub mod bot;
pub mod cli;
pub mod clock;
pub mod data;
pub mod enroll;
pub mod error;
pub mod home;
pub mod leaderboard;
pub mod render;
pub mod state;
pub mod trade;

use axum::{Router, response::Redirect, routing::get, routing::post};
use dotenvy::dotenv;
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder, QueryFilter, ColumnTrait};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{Level, event};

use crate::bot::bot_detail;
use crate::clock::time;
use crate::clock::{MarketClock, start_clock};
use crate::enroll::enroll;
use crate::error::AppError;
use crate::error::Error;
use crate::home::home;
use crate::leaderboard::{leaderboard, orderbook_page, ranking_bot_page};
use crate::state::AppState;
use crate::trade::{buy, price, sell};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_test_writer()
        .init();

    // Delegate to cli module to choose between server or migration commands.
    cli::run().await
}

/// Start the HTTP server. Separated out so `main` can dispatch to either
/// the server or other management subcommands (like `migrate`).
pub async fn run_server(db: DatabaseConnection) -> Result<(), Error> {
    let addr = std::env::var("FINWAR_MARKET_ADDR")
        .unwrap_or_else(|_| "0.0.0.0".to_string());

    let port = std::env::var("FINWAR_MARKET_PORT")
        .unwrap_or_else(|_| "4444".to_string())
        .parse::<u64>()
        .expect("FINWAR_MARKET_PORT must be a valid integer");

    let tick_interval_seconds = std::env::var("FINWAR_MARKET_INTERVAL_SECONDS")
        .unwrap_or_else(|_| "60".to_string())
        .parse::<u64>()
        .expect("FINWAR_MARKET_INTERVAL_SECONDS must be a valid integer");    

    let target_symbol = std::env::var("FINWAR_MARKET_TARGET_SYMBOL")
        .unwrap_or_else(|_| "AAPL".to_string());

    let start_time = entity::stocks_history::Entity::find()
        .order_by_asc(entity::stocks_history::Column::Time)
        .filter(entity::stocks_history::Column::Symbol.eq(&target_symbol))
        .one(&db)
        .await
        .map_err(Error::InitDb)?
        .map(|s| s.time)
        .expect("No stock data found to initialize clock");

    event!(Level::INFO, "Initializing market clock starting at {}", start_time);

    let clock = Arc::new(RwLock::new(MarketClock::new(
        start_time,
        tick_interval_seconds,
    )));
    start_clock(Arc::clone(&clock));

    let state = AppState::new(db, clock, target_symbol).await.map_err(Error::State)?;

    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/home") }))
        .route("/home", get(home))
        .route("/leaderboard", get(leaderboard))
        .route("/api/bot/{id}", get(bot_detail))
        .route("/api/ranking/", get(ranking_bot_page))
        .route("/api/orderbook/", get(orderbook_page))
        .route("/api/enroll", post(enroll))
        .route("/api/time", get(time))
        .route("/api/buy", post(buy))
        .route("/api/sell", post(sell))
        .route("/api/price", get(price))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(|| async { AppError::NotFound })
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("{addr}:{port}"))
        .await
        .map_err(Error::Bind)?;
    event!(Level::INFO, "server started and listening on http://{addr}:{port}");

    axum::serve(listener, app).await.map_err(Error::Run)?;
    Ok(())
}
