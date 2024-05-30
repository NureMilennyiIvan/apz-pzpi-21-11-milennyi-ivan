use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub(crate) struct BreedVM {
    id: u64,
    name: String,
    info: String,
    feed_name: String,
    sheep_count: u64,
}