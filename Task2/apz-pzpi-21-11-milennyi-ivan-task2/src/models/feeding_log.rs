use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow, ToSchema)]
pub(crate) struct FeedingLog{
    id: Option<u64>,
    sheep_id: u64,
    timestamp: u64,
    feed_id: u64,
    #[validate(range(min = 1))]
    amount: u64
}