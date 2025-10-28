use crate::{
    error::{AppError, TradeError},
    state::AppState,
};
use axum::{Json, extract::State, response::IntoResponse};
use entity::{bot, wallet, stocks_history};
use num_traits::cast::ToPrimitive;
use sea_orm::ColumnTrait;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct OrderInput {
    pub bot_uuid: String,
    pub cash: Option<f64>,
    pub quantity: Option<i32>,
}

pub async fn buy(
    State(state): State<AppState>, Json(payload): Json<OrderInput>,
) -> Result<&'static str, AppError> {
    let bot = get_bot(&state.db, Uuid::parse_str(&payload.bot_uuid).unwrap())
        .await
        .map_err(|_| TradeError::BotNotFound)?;

    let wallet = get_wallet(&state.db, bot.id)
        .await
        .map_err(|_| TradeError::WalletNotFound)?;

    if let Some(qty) = payload.quantity {
        if qty <= 0 {
            return Err(TradeError::InvalidQuantity.into());
        }
    }
    if let Some(cash) = payload.cash {
        if cash <= 0.0 {
            return Err(TradeError::InvalidQuantity.into());
        }
        if cash > wallet.cash.to_f64().unwrap() {
            return Err(TradeError::InsufficientFunds.into());
        }
    }

    Ok("buy")
}
pub async fn sell(
    State(_state): State<AppState>, Json(_payload): Json<OrderInput>,
) -> Result<&'static str, AppError> {
    Ok("sell")
}


pub async fn price(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let current_time = state.clock.read().await.current_time();
    
    let stock = stocks_history::Entity::find()
        .filter(stocks_history::Column::Time.lte(current_time))
        .order_by_desc(stocks_history::Column::Time)
        .one(&state.db)
        .await
        .map_err(|_| TradeError::DatabaseError)?
        .ok_or(TradeError::DatabaseError)?;

    let price_mean = (
        stock.open.unwrap_or_default().to_f64().unwrap_or(0.0) +
        stock.close.unwrap_or_default().to_f64().unwrap_or(0.0) +
        stock.high.unwrap_or_default().to_f64().unwrap_or(0.0) +
        stock.low.unwrap_or_default().to_f64().unwrap_or(0.0)
    ) / 4.0;

    Ok(format!("{:.2}", price_mean))
}

async fn get_bot(
    db: &DatabaseConnection, uuid: Uuid,
) -> Result<bot::Model, TradeError> {
    bot::Entity::find()
        .filter(bot::Column::Uuid.eq(uuid))
        .one(db)
        .await
        .unwrap()
        .ok_or(TradeError::BotNotFound)
}

async fn get_wallet(
    db: &DatabaseConnection, bot_id: i32,
) -> Result<wallet::Model, TradeError> {
    wallet::Entity::find()
        .filter(wallet::Column::BotId.eq(bot_id))
        .one(db)
        .await
        .unwrap()
        .ok_or(TradeError::InsufficientFunds)
}
