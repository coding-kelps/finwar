use sea_orm::DatabaseConnection;
use std::fmt;
use crate::clock::SharedClock;

#[derive(Debug)]
pub enum StateError {
    InvalidState(String),
    MissingState,
    InitState,
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateError::InvalidState(msg) => {
                write!(f, "Invalid state: {}", msg)
            },
            StateError::MissingState => write!(f, "Missing state"),
            StateError::InitState => write!(f, "Error initializing state"),
        }
    }
}

impl std::error::Error for StateError {}

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub uuid_prefix_length: usize,
    pub starting_cash: f64,
    pub starting_assets: i32,
    pub clock: SharedClock,
    pub tracked_symbol: String,
}

impl AppState {
    pub async fn new(db: DatabaseConnection, clock: SharedClock, tracked_symbol: String) -> Result<Self, StateError> {
        Ok(AppState {
            db,
            uuid_prefix_length: 18,
            starting_cash: 10000.0,
            starting_assets: 0,
            clock,
            tracked_symbol,
        })
    }
}
