use axum::{Json, extract::State};
use sea_orm::{ActiveValue::Set, prelude::Decimal};

use crate::{error::AppError, state::AppState};
use entity::{bot, wallet};
use sea_orm::ActiveModelTrait;

#[derive(serde::Deserialize)]
pub struct EnrollPayload {
    pub name: String,
}

pub async fn enroll(
    State(state): State<AppState>, Json(payload): Json<EnrollPayload>,
) -> Result<String, AppError> {
    let bot =
        bot::ActiveModel { name: Set(payload.name), ..Default::default() };
    let bot = bot.insert(&state.db).await.unwrap();

    let starting_cash = (state.starting_cash * 100.0) as i64;
    
    let wallet = wallet::ActiveModel {
        bot_id: Set(bot.id),
        cash: Set(Decimal::new(starting_cash, 2)),
        asset: Set(state.starting_assets),
        ..Default::default()
    };
    let wallet = wallet.insert(&state.db).await.unwrap();
    Ok(format!(
        "Created wallet {} with {} for bot {}\n id: {}",
        &wallet.id, &wallet.cash, &bot.name, bot.uuid
    ))
}
