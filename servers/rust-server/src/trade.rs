use crate::{
    error::{AppError, TradeError},
    state::AppState,
};
use axum::{Json, extract::State, response::IntoResponse};
use entity::{bot, wallet, stocks_history, orderbook};
use num_traits::cast::ToPrimitive;
use sea_orm::ColumnTrait;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, ActiveModelTrait, Set};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct BuyOrderInput {
    pub bot_uuid: String,
    pub investment: f64,
}

#[derive(serde::Deserialize)]
pub struct SellOrderInput {
    pub bot_uuid: String,
    pub quantity: i32,
}

pub async fn buy(
    State(state): State<AppState>, Json(payload): Json<BuyOrderInput>,
) -> Result<impl IntoResponse, AppError> {
    let bot = get_bot(&state.db, Uuid::parse_str(&payload.bot_uuid).unwrap())
        .await
        .map_err(|_| TradeError::BotNotFound)?;

    let wallet = get_wallet(&state.db, bot.id)
        .await
        .map_err(|_| TradeError::WalletNotFound)?;

    if payload.investment <= 0.0 {
        return Err(TradeError::InvalidQuantity.into());
    }
    
    let wallet_cash = wallet.cash.to_f64().unwrap();
    let wallet_asset = wallet.asset;
    
    if payload.investment > wallet_cash {
        return Err(TradeError::InsufficientFunds.into());
    }

    let current_price = get_current_price(&state).await?;
    let shares_to_buy = (payload.investment / current_price).floor() as i32;
    
    if shares_to_buy <= 0 {
        return Err(TradeError::InsufficientFunds.into());
    }
    
    let actual_cost = shares_to_buy as f64 * current_price;
    
    let mut wallet_active: wallet::ActiveModel = wallet.into();
    wallet_active.cash = Set(Decimal::from_f64_retain(wallet_cash - actual_cost).unwrap());
    wallet_active.asset = Set(wallet_asset + shares_to_buy);
    wallet_active.update(&state.db).await.map_err(|_| TradeError::DatabaseError)?;

    record_trade(&state.db, bot.id, "buy", shares_to_buy, current_price).await?;

    Ok(format!("Bought {} shares for ${:.2}", shares_to_buy, actual_cost))
}

pub async fn sell(
    State(state): State<AppState>, Json(payload): Json<SellOrderInput>,
) -> Result<impl IntoResponse, AppError> {
    let bot = get_bot(&state.db, Uuid::parse_str(&payload.bot_uuid).unwrap())
        .await
        .map_err(|_| TradeError::BotNotFound)?;

    let wallet = get_wallet(&state.db, bot.id)
        .await
        .map_err(|_| TradeError::WalletNotFound)?;

    if payload.quantity <= 0 {
        return Err(TradeError::InvalidQuantity.into());
    }
    
    let wallet_cash = wallet.cash.to_f64().unwrap();
    let wallet_asset = wallet.asset;
    
    if payload.quantity > wallet_asset {
        return Err(TradeError::InsufficientFunds.into());
    }

    let current_price = get_current_price(&state).await?;
    let proceeds = payload.quantity as f64 * current_price;
    
    let mut wallet_active: wallet::ActiveModel = wallet.into();
    wallet_active.cash = Set(Decimal::from_f64_retain(wallet_cash + proceeds).unwrap());
    wallet_active.asset = Set(wallet_asset - payload.quantity);
    wallet_active.update(&state.db).await.map_err(|_| TradeError::DatabaseError)?;

    record_trade(&state.db, bot.id, "sell", payload.quantity, current_price).await?;

    Ok(format!("Sold {} shares for ${:.2}", payload.quantity, proceeds))
}

async fn get_current_price(state: &AppState) -> Result<f64, AppError> {
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

    Ok(price_mean)
}

pub async fn price(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let price = get_current_price(&state).await?;
    Ok(format!("{:.2}", price))
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

async fn record_trade(
    db: &DatabaseConnection,
    bot_id: i32,
    order_type: &str,
    quantity: i32,
    price: f64,
) -> Result<(), AppError> {
    let order = orderbook::ActiveModel {
        bot_id: Set(bot_id),
        symbol: Set("STOCK".to_string()),
        order_type: Set(order_type.to_string()),
        quantity: Set(quantity),
        price: Set(price),
        status: Set("completed".to_string()),
        ..Default::default()
    };
    order.insert(db).await.map_err(|_| TradeError::DatabaseError)?;
    Ok(())
}
