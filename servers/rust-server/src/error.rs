use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use sea_orm::DbErr;

#[derive(displaydoc::Display, pretty_error_debug::Debug, thiserror::Error)]
pub enum Error {
    /// Error binding the port to the server
    Bind(#[source] std::io::Error),
    /// Error running the server
    Run(#[source] std::io::Error),
    /// Error with app state
    State(#[from] crate::state::StateError),
    /// Error initializing the database
    InitDb(#[from] DbErr),
}

#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum TradeError {
    /// Bot not found
    BotNotFound,
    /// Wallet not found
    WalletNotFound,
    /// Insufficient funds
    InsufficientFunds,
    /// Invalid quantity
    InvalidQuantity,
    /// Database error
    DatabaseError,
    /// Database error
    DbError(#[from] sea_orm::DbErr),
}

#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum AppError {
    /// Resource not found
    NotFound,
    /// Internal server error
    Render(#[from] askama::Error),
    /// Data error
    Data(#[from] polars::prelude::PolarsError),
    /// IO error
    Io(#[from] std::io::Error),
    /// App state
    State(#[from] crate::state::StateError),
    /// Trade error
    Trade(#[from] TradeError),
    /// Database error
    DatabaseError(#[from] sea_orm::DbErr),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        #[derive(Debug, Template)]
        #[template(path = "error.html")]
        struct Tmpl {
            error: String,
        }

        let status = match &self {
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Data(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::State(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Trade(e) => match e {
                TradeError::BotNotFound => StatusCode::NOT_FOUND,
                TradeError::WalletNotFound => StatusCode::NOT_FOUND,
                TradeError::InsufficientFunds => StatusCode::BAD_REQUEST,
                TradeError::InvalidQuantity => StatusCode::BAD_REQUEST,
                TradeError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
                TradeError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let tmpl = Tmpl { error: self.to_string() };
        if let Ok(body) = tmpl.render() {
            (status, Html(body)).into_response()
        } else {
            (status, "Something went wrong").into_response()
        }
    }
}
