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