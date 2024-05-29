use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow, ToSchema)]
pub(crate) struct TemperatureScanner{
    id: Option<u64>,
    temperature: Option<u16>,
    sheep_id: u64,
    #[validate(length(min = 1))]
    password: String
}
impl TemperatureScanner {
    pub fn id(&self) -> Option<u64> {
        self.id
    }

    pub fn temperature(&self) -> Option<u16> {
        self.temperature
    }

    pub fn sheep_id(&self) -> u64 {
        self.sheep_id
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
