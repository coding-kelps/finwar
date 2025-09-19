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

    let wallet = wallet::ActiveModel {
        bot_id: Set(bot.id),
        cash: Set(1000),
        asset: Set(Decimal::new(0, 0)),
        ..Default::default()
    };
    let wallet = wallet.insert(&state.db).await.unwrap();
    Ok(format!(
        "Created wallet {} with {} for bot {}\n id: {}",
        &wallet.id, &wallet.cash, &bot.name, bot.uuid
    ))
}
