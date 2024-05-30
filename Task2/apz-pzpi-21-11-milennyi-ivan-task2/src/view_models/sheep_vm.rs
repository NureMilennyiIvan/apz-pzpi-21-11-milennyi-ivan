use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub(crate) struct SheepVM{
    id: u64,
    breed: String,
    sex: bool,
    birth_date: u64,
    last_feeding_timestamp: Option<u64>,
    last_shearing_timestamp: Option<u64>,
}