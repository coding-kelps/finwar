use crate::error::AppError;

pub async fn buy() -> Result<&'static str, AppError> {
    Ok("buy")
}
pub async fn sell() -> Result<&'static str, AppError> {
    Ok("sell")
}
pub async fn price() -> Result<&'static str, AppError> {
    Err(AppError::NotFound)
}
