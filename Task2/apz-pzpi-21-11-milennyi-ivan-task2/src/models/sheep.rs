use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow, ToSchema)]
pub(crate) struct Sheep{
    id: Option<u64>,
    birth_date: u64,
    breed_id: u64,
    weight: u64,
    sex: bool,
    temperature_scanner_id: Option<u64>,
    shepherd_id: Option<u64>
}
impl Sheep {
    pub fn id(&self) -> Option<u64> {
        self.id
    }
    pub fn set_id(&mut self, id: u64) -> (){
        self.id = Some(id);
    }
    pub fn birth_date(&self) -> u64 {
        self.birth_date
    }

    pub fn breed_id(&self) -> u64 {
        self.breed_id
    }
    pub fn weight(&self) -> u64{
        self.weight
    }
    pub fn sex(&self) -> bool {
        self.sex
    }

    pub fn temperature_scanner_id(&self) -> Option<u64> {
        self.temperature_scanner_id
    }

    pub fn shepherd_id(&self) -> Option<u64> {
        self.shepherd_id
    }
}