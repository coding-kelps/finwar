use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use entity::{bot, wallet};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use crate::{error::AppError, state::AppState};

#[derive(Template)]
#[template(path = "leaderboard.html")]
struct LeaderboardTemplate {
    total_bots: usize,
    bots: Vec<BotRank>,
}

#[derive(Debug)]
struct BotRank {
    id_short: String,
    name: String,
    cash: String,
    asset: i32,
}

pub async fn leaderboard(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let bots = get_ranked_bots(&state.db, state.uuid_prefix_length).await?;
    let template = LeaderboardTemplate {
        total_bots: bots.len(),
        bots,
    };
    Ok(Html(template.render()?))
}

async fn get_ranked_bots(db: &DatabaseConnection, uuid_prefix_length: usize) -> Result<Vec<BotRank>, sea_orm::DbErr> {
    let bots = bot::Entity::find()
        .find_also_related(wallet::Entity)
        .order_by_desc(wallet::Column::Cash)
        .all(db)
        .await?;

    Ok(bots
        .into_iter()
        .filter_map(|(bot, wallet)| {
            wallet.map(|w| {
                let uuid_str = bot.uuid.to_string();
                let id_short = uuid_str.chars().take(uuid_prefix_length).collect::<String>();
                
                BotRank {
                    id_short,
                    name: bot.name,
                    cash: w.cash.to_string(),
                    asset: w.asset,
                }
            })
        })
        .collect())
}