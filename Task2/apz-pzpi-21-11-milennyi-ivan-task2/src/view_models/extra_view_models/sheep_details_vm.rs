use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub(crate) struct SheepDetailsVM {
    id: u64,
    name: String,
    breed: String,
    sex: bool,
    birth_date: u64,
    last_feeding_timestamp: Option<u64>,
    last_shearing_timestamp: Option<u64>,
    weight: u64,
    temperature: Option<u64>,
    feed_id: u64,
    feed_name: String,
    feed_amount: u64,
}