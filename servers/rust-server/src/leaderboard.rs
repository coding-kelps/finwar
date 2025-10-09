use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use entity::{bot, wallet, orderbook};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use crate::{error::AppError, state::AppState};

#[derive(Template)]
#[template(path = "leaderboard.html")]
struct LeaderboardTemplate {
    starting_cash: f64,
    starting_assets: i32,
    total_bots: usize,
    bots: Vec<BotRank>,
    all_orders: Vec<OrderWithBot>,
}

#[derive(Debug)]
struct BotRank {
    id_short: String,
    name: String,
    cash: String,
    asset: i32,
    profit_score: f64,
    profit_percentage: f64,
}

#[derive(Debug)]
struct OrderWithBot {
    bot_name: String,
    bot_id_short: String,
    symbol: String,
    order_type: String,
    quantity: i32,
    price: f64,
    status: String,
}

pub async fn leaderboard(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let bots = get_ranked_bots(&state.db, state.uuid_prefix_length, state.starting_cash).await?;
    let all_orders = get_all_orders(&state.db, state.uuid_prefix_length).await?;
    let template = LeaderboardTemplate {
        starting_cash: state.starting_cash,
        starting_assets: state.starting_assets,
        total_bots: bots.len(),
        bots,
        all_orders,
    };
    Ok(Html(template.render()?))
}

async fn get_all_orders(db: &DatabaseConnection, uuid_prefix_length: usize) -> Result<Vec<OrderWithBot>, sea_orm::DbErr> {
    let orders = orderbook::Entity::find().all(db).await?;
    let bots = bot::Entity::find().all(db).await?;
    
    let bot_map: std::collections::HashMap<i32, bot::Model> = bots
        .into_iter()
        .map(|b| (b.id, b))
        .collect();
    
    Ok(orders
        .into_iter()
        .filter_map(|order| {
            bot_map.get(&order.bot_id).map(|b| {
                let uuid_str = b.uuid.to_string();
                let bot_id_short = uuid_str.chars().take(uuid_prefix_length).collect::<String>();
                
                OrderWithBot {
                    bot_name: b.name.clone(),
                    bot_id_short,
                    symbol: order.symbol,
                    order_type: order.order_type,
                    quantity: order.quantity,
                    price: order.price,
                    status: order.status,
                }
            })
        })
        .collect())
}

async fn get_ranked_bots(db: &DatabaseConnection, uuid_prefix_length: usize, starting_cash: f64) -> Result<Vec<BotRank>, sea_orm::DbErr> {
    let bots = bot::Entity::find()
        .find_also_related(wallet::Entity)
        .order_by_desc(wallet::Column::Cash)
        .all(db)
        .await?;

    let mut bot_ranks = Vec::new();
    
    for (bot, wallet) in bots {
        if let Some(w) = wallet {
            let uuid_str = bot.uuid.to_string();
            let id_short = uuid_str.chars().take(uuid_prefix_length).collect::<String>();
            
            let current_cash = w.cash.to_string().parse::<f64>().unwrap_or(0.0);
            let profit_score = current_cash - starting_cash;
            let profit_percentage = if starting_cash > 0.0 {
                (profit_score / starting_cash) * 100.0
            } else {
                0.0
            };
            
            bot_ranks.push(BotRank {
                id_short,
                name: bot.name,
                cash: w.cash.to_string(),
                asset: w.asset,
                profit_score,
                profit_percentage,
            });
        }
    }
    
    Ok(bot_ranks)
}