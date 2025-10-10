use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use crate::{
    error::AppError,
    state::AppState,
};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {

}

pub async fn home(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let template = HomeTemplate {  };
    Ok(Html(template.render()?))
}
