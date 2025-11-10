use axum::{extract::State, response::IntoResponse};
use sea_orm::prelude::DateTimeWithTimeZone;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Duration;
use crate::{error::AppError, state::AppState};

#[derive(Debug, Clone)]
pub struct MarketClock {
    current_time: DateTimeWithTimeZone,
    tick_interval_seconds: u64,
}

impl MarketClock {
    pub fn new(start_time: DateTimeWithTimeZone, tick_interval_seconds: u64) -> Self {
        Self {
            current_time: start_time,
            tick_interval_seconds,
        }
    }

    pub fn current_time(&self) -> DateTimeWithTimeZone {
        self.current_time
    }

    pub fn tick_interval_seconds(&self) -> u64 {
        self.tick_interval_seconds
    }

    pub fn advance(&mut self, seconds: u64) {
        self.current_time = self.current_time + Duration::from_secs(seconds);
    }
}

pub type SharedClock = Arc<RwLock<MarketClock>>;

pub fn start_clock(clock: SharedClock) {
    tokio::spawn(async move {
        let tick_interval_seconds = clock.read().await.tick_interval_seconds();
        let mut interval = tokio::time::interval(Duration::from_secs(tick_interval_seconds));
        loop {
            interval.tick().await;
            let mut clock = clock.write().await;
            clock.advance(tick_interval_seconds);
        }
    });
}

pub async fn time(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let clock = state.clock.read().await;
    let time_str = clock.current_time().format("%Y-%m-%d %H:%M:%S").to_string();
    Ok(time_str)
}
