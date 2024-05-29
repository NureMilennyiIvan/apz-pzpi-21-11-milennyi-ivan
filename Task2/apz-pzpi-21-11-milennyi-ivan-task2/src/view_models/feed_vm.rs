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

impl FeedVM {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn calories(&self) -> u32 {
        self.calories
    }

    pub fn fat(&self) -> u32 {
        self.fat
    }

    pub fn protein(&self) -> u32 {
        self.protein
    }

    pub fn carbohydrates(&self) -> u32 {
        self.carbohydrates
    }

    pub fn breed_name(&self) -> &str {
        &self.breed_name
    }

    pub fn sheep_count(&self) -> u64 {
        self.sheep_count
    }
}