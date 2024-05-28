use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow, ToSchema)]
pub(crate) struct FeedSupply{
    id: Option<u64>,
    storekeeper_id: u64,
    #[validate(range(min = 1))]
    amount: u64,
    feed_id: u64
}