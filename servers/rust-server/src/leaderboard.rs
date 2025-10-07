use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use entity::bot;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait};
use crate::{error::AppError, state::AppState};

#[derive(Template)]
#[template(path = "leaderboard.html")]
struct LeaderboardTemplate {
    total_bots: usize,
}

pub async fn leaderboard(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    
    let template = LeaderboardTemplate {
        total_bots: total_bots(&state.db).await?,
    };
    Ok(Html(template.render()?))
}   

async fn total_bots(
    db: &DatabaseConnection,
) -> Result<usize, sea_orm::DbErr> {
    let count = bot::Entity::find()
        .count(db)
        .await
        .map_err(|_| sea_orm::DbErr::Query(sea_orm::RuntimeErr::Internal("Failed to count bots".to_string())))?;
    Ok(count as usize)
}