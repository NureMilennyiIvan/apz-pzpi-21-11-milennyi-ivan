use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow, ToSchema)]
pub(crate) struct FeedSupply{
    id: Option<u64>,
    storekeeper_id: Option<u64>,
    #[validate(range(min = 1))]
    amount: u64,
    timestamp: u64,
    feed_id: u64
}

impl FeedSupply {
    pub fn id(&self) -> Option<u64> {
        self.id
    }

    pub fn storekeeper_id(&self) -> Option<u64> {
        self.storekeeper_id
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn feed_id(&self) -> u64 {
        self.feed_id
    }
}