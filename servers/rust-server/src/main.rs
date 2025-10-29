pub mod bot;
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
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing::event;
use std::sync::Arc;
use tokio::sync::RwLock;
use sea_orm::EntityTrait;
use sea_orm::QueryOrder;

use crate::bot::bot_detail;
use crate::clock::{MarketClock, start_clock};
use crate::clock::time;
use crate::enroll::enroll;
use crate::error::AppError;
use crate::error::Error;
use crate::home::home;
use crate::leaderboard::{leaderboard, ranking_bot_page, orderbook_page};
use crate::state::AppState;
use crate::trade::{buy, price, sell};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_test_writer()
        .init();

    dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let tick_interval_seconds = std::env::var("TICK_INTERVAL_SECONDS")
        .unwrap_or_else(|_| "1".to_string())
        .parse::<u64>()
        .expect("TICK_INTERVAL_SECONDS must be a valid integer");

    let addr = "0.0.0.0";
    let port = 4444;

    let db_connection = sea_orm::Database::connect(&database_url)
        .await
        .map_err(Error::InitDb)?;

    let start_time = entity::stocks_history::Entity::find()
        .order_by_asc(entity::stocks_history::Column::Time)
        .one(&db_connection)
        .await
        .map_err(Error::InitDb)?
        .map(|s| s.time)
        .expect("No stock data found to initialize clock");

    event!(Level::INFO, "Initializing market clock starting at {}", start_time);

    let clock = Arc::new(RwLock::new(MarketClock::new(start_time, tick_interval_seconds)));
    start_clock(Arc::clone(&clock));

    let state = AppState::new(db_connection, clock).await.map_err(Error::State)?;

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
