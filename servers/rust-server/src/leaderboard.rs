use std::collections::HashMap;

use askama::Template;
use axum::{
    extract::{Query, State},
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
    page: usize,
}

#[derive(Debug)]
struct BotRank {
    rank: usize,
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
    let bots = get_ranked_bots_paginated(&state.db, state.uuid_prefix_length, state.starting_cash, 5).await?;
    let all_orders = get_all_orders_paginated(&state.db, state.uuid_prefix_length, 10).await?;
    let template = LeaderboardTemplate {
        starting_cash: state.starting_cash,
        starting_assets: state.starting_assets,
        total_bots: bots.len(),
        bots,
        all_orders,
        page: 1,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "orderbook-row.html")]
struct OrderbookRowTemplate {
    all_orders: Vec<OrderWithBot>,
    page: usize,
}

pub async fn orderbook_page(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>) -> Result<impl IntoResponse, AppError> {
    let page = params.get("page")
        .and_then(|p| p.parse::<u64>().ok())
        .unwrap_or(1);
    
    let per_page = page * 10;
    
    let all_orders = get_all_orders_paginated(&state.db, state.uuid_prefix_length, per_page).await?;
    
    let template = OrderbookRowTemplate { 
        all_orders,
        page: (page + 1) as usize,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "ranking-bot-row.html")]
struct RankingBotRowTemplate {
    bots: Vec<BotRank>,
    page: usize,
}

pub async fn ranking_bot_page(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>) -> Result<impl IntoResponse, AppError> {
    let page = params.get("page")
        .and_then(|p| p.parse::<u64>().ok())
        .unwrap_or(1);
    
    let per_page = page * 5;
    
    let bots = get_ranked_bots_paginated(&state.db, state.uuid_prefix_length, state.starting_cash, per_page).await?;
    
    let template = RankingBotRowTemplate { 
        bots,
        page: (page + 1) as usize,
    };
    Ok(Html(template.render()?))
}

async fn get_all_orders_paginated(db: &DatabaseConnection, uuid_prefix_length: usize, limit: u64) -> Result<Vec<OrderWithBot>, sea_orm::DbErr> {
    use sea_orm::QuerySelect;
    
    let orders = orderbook::Entity::find()
        .limit(limit)
        .all(db)
        .await?;
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

async fn get_ranked_bots_paginated(db: &DatabaseConnection, uuid_prefix_length: usize, starting_cash: f64, limit: u64) -> Result<Vec<BotRank>, sea_orm::DbErr> {
    use sea_orm::QuerySelect;
    
    let bots = bot::Entity::find()
        .find_also_related(wallet::Entity)
        .order_by_desc(wallet::Column::Cash)
        .limit(limit)
        .all(db)
        .await?;

    let start_rank = 1;
    Ok(build_bot_ranks_with_offset(bots, uuid_prefix_length, starting_cash, start_rank))
}

fn build_bot_ranks_with_offset(bots: Vec<(bot::Model, Option<wallet::Model>)>, uuid_prefix_length: usize, starting_cash: f64, start_rank: usize) -> Vec<BotRank> {
    let mut bot_ranks = Vec::new();
    
    for (index, (bot, wallet)) in bots.into_iter().enumerate() {
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
                rank: start_rank + index,
                id_short,
                name: bot.name,
                cash: w.cash.to_string(),
                asset: w.asset,
                profit_score,
                profit_percentage,
            });
        }
    }
    
    bot_ranks
}
