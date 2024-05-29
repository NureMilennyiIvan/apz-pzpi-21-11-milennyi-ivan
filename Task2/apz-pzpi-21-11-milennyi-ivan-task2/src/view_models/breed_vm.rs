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

impl BreedVM {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn info(&self) -> &str {
        &self.info
    }

    pub fn feed_name(&self) -> &str {
        &self.feed_name
    }

    pub fn sheep_count(&self) -> u64 {
        self.sheep_count
    }
}