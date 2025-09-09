use axum::{Router, response::Redirect, routing::get, routing::post};

use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing::event;

pub mod data;
pub mod error;
pub mod home;
pub mod render;

use crate::error::AppError;
use crate::error::Error;
use crate::home::home;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();

    let addr = "0.0.0.0";
    let port = 4444;
    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/home") }))
        .route("/home", get(home))
        .route("/ranking", get(ranking))
        .route("/buy", post(buy))
        .route("/sell", post(sell))
        .route("/price", get(price))
        .fallback(|| async { AppError::NotFound })
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("{addr}:{port}"))
        .await
        .map_err(Error::Bind)?;
    event!(Level::INFO, "server started and listening on http://{addr}:{port}");
    axum::serve(listener, app).await.map_err(Error::Run)?;
    Ok(())
}

async fn ranking() -> Result<&'static str, AppError> {
    Ok("ranking")
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
