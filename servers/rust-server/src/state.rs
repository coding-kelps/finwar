use crate::{
    data::{list_files_in_folder, read_csv},
    render::{PlotlyHtml, plot_pie_files},
};
use polars::frame::DataFrame;
use rand::seq::SliceRandom;
use sea_orm::DatabaseConnection;
use std::fmt;

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
    pub all_files: Vec<String>,
    pub randos: Vec<String>,
    pub dataframes: Vec<DataFrame>,
    pub pie_distrubtion: PlotlyHtml,
    pub db: sea_orm::DatabaseConnection,
    pub uuid_prefix_length: usize,
}

impl AppState {
    pub async fn new(db: DatabaseConnection) -> Result<Self, StateError> {
        let mut all_files = list_files_in_folder("./local/data/Stocks")
            .map_err(|_| StateError::InitState)?;
        let plot_pie = plot_pie_files(&all_files)
            .map_err(|_| StateError::InitState)?
            .to_inline_html(Some("file size distribution"));
        let mut rng = rand::rng();
        all_files.shuffle(&mut rng);
        let randos = all_files.clone().into_iter().take(3).collect::<Vec<_>>();

        let dataframes = randos
            .iter()
            .map(|file| read_csv(&format!("./local/data/Stocks/{}", file)))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| StateError::InitState)?;

        Ok(AppState {
            all_files,
            randos,
            dataframes,
            pie_distrubtion: plot_pie,
            db,
            uuid_prefix_length: 18,
        })
    }
}
