use crate::{
    error::{AppError, TradeError},
    state::AppState,
};
use axum::{Json, extract::State};
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
    State(state): State<AppState>, Json(payload): Json<OrderInput>,
) -> Result<&'static str, AppError> {
    Ok("sell")
}

#[derive(serde::Serialize)]
pub struct PriceResponse {
    pub time: DateTimeWithTimeZone,
    pub symbol: String,
    pub quotation: Option<f64>,
}

pub async fn price(
    State(state): State<AppState>,
) -> Result<Json<PriceResponse>, AppError> {
    let stock = stocks_history::Entity::find()
        .order_by_desc(stocks_history::Column::Time)
        .one(&state.db)
        .await
        .map_err(|_| TradeError::DatabaseError)?
        .ok_or(TradeError::DatabaseError)?;

    Ok(Json(PriceResponse {
        time: stock.time,
        symbol: stock.symbol,
        quotation: stock.quotation,
    }))
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
