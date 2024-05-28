use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow, ToSchema)]
pub(crate) struct Breed{
    id: Option<u64>,
    #[validate(length(min = 1))]
    name: String,
    feed_id: u64,
    #[validate(length(min = 1))]
    info: String
}