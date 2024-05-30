use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub(crate) struct TemperatureScannerAuthJson {
    pub id: u64,
    #[validate(length(min = 1))]
    pub hash_password: String
}