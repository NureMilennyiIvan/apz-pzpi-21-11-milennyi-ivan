use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow, ToSchema)]
pub(crate) struct FeedingLog{
    id: Option<u64>,
    sheep_id: u64,
    shepherd_id: Option<u64>,
    timestamp: u64,
    feed_id: u64,
    #[validate(range(min = 1))]
    amount: u64
}

impl FeedingLog {
    pub fn id(&self) -> Option<u64> {
        self.id
    }

    pub fn sheep_id(&self) -> u64 {
        self.sheep_id
    }
    pub fn shepherd_id(&self) -> Option<u64> {
        self.shepherd_id
    }
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn feed_id(&self) -> u64 {
        self.feed_id
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }
}