use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(displaydoc::Display, pretty_error_debug::Debug, thiserror::Error)]
pub enum Error {
    /// Error binding the port to the server
    Bind(#[source] std::io::Error),
    /// Error running the server
    Run(#[source] std::io::Error),
    /// Error with app state
    State(#[from] crate::state::StateError),
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
        };
        let tmpl = Tmpl { error: self.to_string() };
        if let Ok(body) = tmpl.render() {
            (status, Html(body)).into_response()
        } else {
            (status, "Something went wrong").into_response()
        }
    }
}
