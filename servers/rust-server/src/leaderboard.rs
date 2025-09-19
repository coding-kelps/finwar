use crate::error::AppError;

pub async fn leaderboard() -> Result<&'static str, AppError> {
    Ok("leaderboard")
}
