use axum::{Json, extract::State};
use sea_orm::{ActiveValue::Set, prelude::Decimal};

use crate::{error::AppError, state::AppState};
use entity::{bot, wallet};
use sea_orm::ActiveModelTrait;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct EnrollPayload {
    pub name: String,
}

#[utoipa::path(
    post,
    path = "/api/enroll",
    description = "Enroll a new bot to the market. Each bot must have a unique name.",
    request_body(content = EnrollPayload, content_type = "application/json"),
    responses(
        (status = 200, description = "New bot enrolled successfully", body = String),
        (status = 406, description = "Bot with same name already exists", body = String),
    ),
)]
pub async fn enroll(
    State(state): State<AppState>, Json(payload): Json<EnrollPayload>,
) -> Result<String, AppError> {
    let bot = bot::ActiveModel { 
        name: Set(payload.name), 
        ..Default::default() 
    };
    let bot = match bot.insert(&state.db).await {
        Ok(b) => b,
        Err(sea_orm::DbErr::Query(e)) if e.to_string().contains("duplicate key") || e.to_string().contains("UNIQUE constraint") => {
            return Err(AppError::BotNameExists);
        }
        Err(e) => return Err(e.into()),
    };

    let starting_cash = (state.starting_cash * 100.0) as i64;
    
    let wallet = wallet::ActiveModel {
        bot_id: Set(bot.id),
        cash: Set(Decimal::new(starting_cash, 2)),
        asset: Set(state.starting_assets),
        ..Default::default()
    };
    wallet.insert(&state.db).await?;

    Ok(format!(
        "Bot {} created with ${:.2} cash and {} assets\nUUID: {}",
        &bot.name, state.starting_cash, state.starting_assets, bot.uuid
    ))
}
