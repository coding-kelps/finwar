pub mod bot;
pub mod cli;
pub mod config;
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
use sea_orm::{ColumnTrait, EntityTrait, QueryOrder, QueryFilter};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing::event;
use clap::Parser;
use migration::{Migrator, MigratorTrait};

use crate::bot::bot_detail;
use crate::cli::{Args, Commands};
use crate::config::Config;
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
    let args = Args::parse();
    let cfg = Config::from_args(args.clone());

    match cfg.log.level.parse::<Level>() {
        Ok(level) => {
            tracing_subscriber::fmt()
                .with_max_level(level)
                .with_test_writer()
                .init();
        },
        Err(_) => {
            tracing_subscriber::fmt()
                .with_max_level(Level::INFO)
                .with_test_writer()
                .init();

            event!(Level::WARN, "log level wrongly set at \"{}\" continuing server at default level \"info\"", cfg.log.level);
        }
    }

    match args.command {
        Commands::Serve { .. } => serve(cfg).await?,
        Commands::Migrate { .. } => migrate(cfg).await?,
    }

    Ok(())
}

/// Start the HTTP server.
async fn serve(cfg: Config) -> Result<(), Error> {
    let db = sea_orm::Database::connect(
    format!(
            "postgresql://{}:{}@{}:{}/{}",
            cfg.database.user.expose(),
            cfg.database.password.expose(),
            cfg.database.host,
            cfg.database.port,
            cfg.database.name,
        ))
        .await
        .map_err(crate::error::Error::InitDb)?;

    let start_time = entity::stocks_history::Entity::find()
        .order_by_asc(entity::stocks_history::Column::Time)
        .filter(entity::stocks_history::Column::Symbol.eq(&cfg.tracked_symbol))
        .one(&db)
        .await
        .map_err(Error::InitDb)?
        .map(|s| s.time)
        .expect("No stock data found to initialize clock");

    event!(Level::INFO, "Initializing market clock starting at {}", start_time);

    let clock = Arc::new(RwLock::new(MarketClock::new(
        start_time,
        cfg.interval_seconds,
    )));
    start_clock(Arc::clone(&clock));

    let state = AppState::new(db, clock, cfg.tracked_symbol.clone()).await.map_err(Error::State)?;

    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/home") }))
        .route("/health", get(|| async { "OK" }))
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

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", cfg.server.host, cfg.server.port))
        .await
        .map_err(Error::Bind)?;
    event!(Level::INFO, "server started and listening on http://{}:{}", cfg.server.host, cfg.server.port);

    axum::serve(listener, app).await.map_err(Error::Run)?;
    Ok(())
}

// Migrate the entity to the database
async fn migrate(cfg: Config) -> Result<(), Error> {
    let db = sea_orm::Database::connect(
    format!(
            "postgresql://{}:{}@{}:{}/{}",
            cfg.database.user.expose(),
            cfg.database.password.expose(),
            cfg.database.host,
            cfg.database.port,
            cfg.database.name,
        ))
        .await
        .map_err(crate::error::Error::InitDb)?;

    Migrator::up(&db, None).await?;

    Ok(())
}
