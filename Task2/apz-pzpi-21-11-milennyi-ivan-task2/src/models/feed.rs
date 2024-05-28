use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow, ToSchema)]
pub(crate) struct Feed{
    id: Option<u64>,
    #[validate(range(min = 1))]
    amount: u32,
    #[validate(length(min = 1))]
    name: String,
    #[validate(range(min = 1))]
    calories: u32,
    #[validate(range(min = 1))]
    fat: u32,
    #[validate(range(min = 1))]
    protein: u32,
    #[validate(range(min = 1))]
    carbohydrates: u32
}