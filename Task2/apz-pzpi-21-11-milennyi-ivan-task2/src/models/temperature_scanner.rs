use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow, ToSchema)]
pub(crate) struct TemperatureScanner{
    id: Option<u64>,
    temperature: u16,
    #[validate(length(min = 1))]
    password: String
}
impl TemperatureScanner {
    pub fn id(&self) -> Option<u64> {
        self.id
    }
    pub fn set_id(&mut self, id: u64) -> (){
        self.id = Some(id);
    }
    pub fn temperature(&self) -> u16 {
        self.temperature
    }


    pub fn password(&self) -> &str {
        &self.password
    }
}
