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
impl Feed {
    pub fn new(id: Option<u64>, amount: u32, name: String, calories: u32, fat: u32, protein: u32, carbohydrates: u32) -> Self{
        Self{
            id,
            amount,
            name,
            calories,
            fat,
            protein,
            carbohydrates
        }
    }
    pub fn id(&self) -> Option<u64> {
        self.id
    }
    pub fn set_id(&mut self, id: u64) -> (){
        self.id = Some(id);
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
}