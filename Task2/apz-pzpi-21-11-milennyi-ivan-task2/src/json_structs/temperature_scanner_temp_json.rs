use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug)]
pub(crate) struct TemperatureScannerTempJson {
    pub temperature: u64
}