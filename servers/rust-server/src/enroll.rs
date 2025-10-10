use axum::{Json, extract::State};
use sea_orm::{ActiveValue::Set, prelude::Decimal};
use rand::Rng;

use crate::{error::AppError, state::AppState};
use entity::{bot, wallet, orderbook};
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
    let wallet = wallet.insert(&state.db).await?;

    
    let orders: Vec<_> = {
        let mut rng = rand::rng();
        let symbols = vec!["AAPL", "GOOGL", "MSFT", "AMZN", "TSLA"];
        let order_types = vec!["buy", "sell"];
        
        (0..3).map(|_| {
            let symbol = symbols[rng.random_range(0..symbols.len())];
            let order_type = order_types[rng.random_range(0..order_types.len())];
            let quantity = rng.random_range(1..20);
            let price = rng.random_range(100.0..500.0);
            (symbol, order_type, quantity, price)
        }).collect()
    };
    
    let mut total_spent = 0.0;
    let mut total_shares = 0;
    
    for (symbol, order_type, quantity, price) in orders {
        let order = orderbook::ActiveModel {
            bot_id: Set(bot.id),
            symbol: Set(symbol.to_string()),
            order_type: Set(order_type.to_string()),
            quantity: Set(quantity),
            price: Set(price),
            status: Set("pending".to_string()),
            ..Default::default()
        };
        order.insert(&state.db).await?;
        
        if order_type == "buy" {
            total_spent += price * quantity as f64;
            total_shares += quantity;
        }
    }
    
    let mut wallet: wallet::ActiveModel = wallet.into();
    wallet.cash = Set(Decimal::new(starting_cash - (total_spent * 100.0) as i64, 2));
    wallet.asset = Set(state.starting_assets + total_shares);
    let wallet = wallet.update(&state.db).await?;

    Ok(format!(
        "Created wallet {} with {} for bot {}\n id: {}",
        &wallet.id, &wallet.cash, &bot.name, bot.uuid
    ))
}
