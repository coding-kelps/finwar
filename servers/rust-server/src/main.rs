use axum::{Router, response::Redirect, routing::get, routing::post};

use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing::event;

pub mod data;
pub mod error;
pub mod home;
pub mod leaderboard;
pub mod render;
pub mod state;

use crate::error::AppError;
use crate::error::Error;
use crate::home::home;
use crate::leaderboard::leaderboard;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();

    let addr = "0.0.0.0";
    let port = 4444;

    let db = sea_orm::Database::connect(
        "postgres://root:toto@localhost:5432/database",
    )
    .await
    .map_err(Error::InitDb)?;

    let state = AppState::new(db).await.map_err(Error::State)?;

    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/home") }))
        .route("/home", get(home))
        .route("/leaderboard", get(leaderboard))
        .route("/buy", post(buy))
        .route("/sell", post(sell))
        .route("/price", get(price))
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

async fn buy() -> Result<&'static str, AppError> {
    Ok("buy")
}
async fn sell() -> Result<&'static str, AppError> {
    Ok("sell")
}
async fn price() -> Result<&'static str, AppError> {
    Err(AppError::NotFound)
}
