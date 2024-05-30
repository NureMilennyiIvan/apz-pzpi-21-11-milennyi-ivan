use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug)]
pub(crate) struct ChangeForSheepJson {
    pub sheep_id: u64,
    pub change_id: Option<u64>
}