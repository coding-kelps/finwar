use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use entity::{bot, wallet};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::{error::AppError, state::AppState};

#[derive(Template)]
#[template(path = "bot.html")]
struct BotTemplate {
    bot: BotDetail,
}

#[derive(Debug)]
struct BotDetail {
    id_short: String,
    name: String,
    cash: String,
    asset: i32,
}

pub async fn bot_detail(
    State(state): State<AppState>,
    Path(bot_id_short): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let bot_detail = get_bot_detail(&state.db, &bot_id_short, state.uuid_prefix_length).await?;
    let template = BotTemplate { bot: bot_detail };
    Ok(Html(template.render()?))
}

async fn get_bot_detail(
    db: &DatabaseConnection,
    bot_id_short: &str,
    uuid_prefix_length: usize,
) -> Result<BotDetail, AppError> {
    let all_bots = bot::Entity::find()
        .find_also_related(wallet::Entity)
        .all(db)
        .await?;

    for (bot, wallet_opt) in all_bots {
        let uuid_str = bot.uuid.to_string();
        let id_short = uuid_str.chars().take(uuid_prefix_length).collect::<String>();
        
        if id_short == bot_id_short {
            if let Some(wallet) = wallet_opt {
                return Ok(BotDetail {
                    id_short,
                    name: bot.name,
                    cash: wallet.cash.to_string(),
                    asset: wallet.asset,
                });
            }
        }
    }
    
    Err(AppError::NotFound)
}
