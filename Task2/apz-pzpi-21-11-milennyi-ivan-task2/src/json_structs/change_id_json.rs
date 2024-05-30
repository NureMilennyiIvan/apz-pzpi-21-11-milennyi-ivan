use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug)]
pub(crate) struct ChangeIdJson {
    pub change_id: Option<u64>
}