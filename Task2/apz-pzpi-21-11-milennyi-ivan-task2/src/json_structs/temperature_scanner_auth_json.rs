use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub(crate) struct TemperatureScannerAuthJson {
    #[validate(length(min = 1))]
    pub password: String
}