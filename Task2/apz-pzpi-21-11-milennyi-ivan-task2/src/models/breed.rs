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

impl Breed {
    pub fn id(&self) -> Option<u64> {
        self.id
    }
    pub fn set_id(&mut self, id: u64) -> (){
        self.id = Some(id);
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn feed_id(&self) -> u64 {
        self.feed_id
    }

    pub fn info(&self) -> &str {
        &self.info
    }
}