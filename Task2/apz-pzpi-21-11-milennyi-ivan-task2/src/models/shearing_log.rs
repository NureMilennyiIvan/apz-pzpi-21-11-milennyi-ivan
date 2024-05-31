use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow, ToSchema)]
pub(crate) struct ShearingLog{
    id: Option<u64>,
    sheep_id: u64,
    shepherd_id: Option<u64>,
    timestamp: u64,
    #[validate(range(min = 1))]
    wool_amount: u32
}

impl ShearingLog {
    pub fn id(&self) -> Option<u64> {
        self.id
    }
    pub fn set_id(&mut self, id: u64) -> (){
        self.id = Some(id);
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

    pub fn wool_amount(&self) -> u32 {
        self.wool_amount
    }
}