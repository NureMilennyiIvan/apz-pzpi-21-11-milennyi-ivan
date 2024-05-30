use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub(crate) struct TemperatureScannerTempJson {
    pub id: u64,
    pub temperature: u64
}