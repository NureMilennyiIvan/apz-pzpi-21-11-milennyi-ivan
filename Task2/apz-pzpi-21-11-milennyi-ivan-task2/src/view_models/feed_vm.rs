use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub(crate) struct FeedVM {
    id: u64,
    amount: u32,
    name: String,
    calories: u32,
    fat: u32,
    protein: u32,
    carbohydrates: u32,
    breed_name: String,
    sheep_count: u64,
}