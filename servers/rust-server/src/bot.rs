use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use entity::{bot, wallet, orderbook};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use crate::{error::AppError, state::AppState};

#[derive(Template)]
#[template(path = "bot.html")]
struct BotTemplate {
    bot: BotDetail,
    orders: Vec<OrderDetail>,
}

#[derive(Debug)]
struct BotDetail {
    id_short: String,
    name: String,
    cash: String,
    asset: i32,
}

#[derive(Debug)]
struct OrderDetail {
    id: i32,
    symbol: String,
    order_type: String,
    quantity: i32,
    price: String,
    status: String,
}

pub async fn bot_detail(
    State(state): State<AppState>,
    Path(bot_id_short): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let (bot_detail, bot_id) = get_bot_detail(&state.db, &bot_id_short, state.uuid_prefix_length).await?;
    let orders = get_bot_orders(&state.db, bot_id).await?;
    let template = BotTemplate { bot: bot_detail, orders };
    Ok(Html(template.render()?))
}

async fn get_bot_detail(
    db: &DatabaseConnection,
    bot_id_short: &str,
    uuid_prefix_length: usize,
) -> Result<(BotDetail, i32), AppError> {
    let all_bots = bot::Entity::find()
        .find_also_related(wallet::Entity)
        .all(db)
        .await?;

    for (bot, wallet_opt) in all_bots {
        let uuid_str = bot.uuid.to_string();
        let id_short = uuid_str.chars().take(uuid_prefix_length).collect::<String>();
        
        if id_short == bot_id_short {
            if let Some(wallet) = wallet_opt {
                return Ok((BotDetail {
                    id_short,
                    name: bot.name,
                    cash: wallet.cash.to_string(),
                    asset: wallet.asset,
                }, bot.id));
            }
        }
    }
    
    Err(AppError::NotFound)
}

async fn get_bot_orders(
    db: &DatabaseConnection,
    bot_id: i32,
) -> Result<Vec<OrderDetail>, AppError> {
    let orders = orderbook::Entity::find()
        .filter(orderbook::Column::BotId.eq(bot_id))
        .all(db)
        .await?;

    Ok(orders
        .into_iter()
        .map(|order| OrderDetail {
            id: order.id,
            symbol: order.symbol,
            order_type: order.order_type,
            quantity: order.quantity,
            price: format!("{:.2}", order.price),
            status: order.status,
        })
        .collect())
}
