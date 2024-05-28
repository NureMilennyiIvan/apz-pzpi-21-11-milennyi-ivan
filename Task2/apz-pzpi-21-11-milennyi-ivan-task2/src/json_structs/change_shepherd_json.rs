use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, ToSchema, Debug)]
pub(crate) struct ChangeShepherdJson {
    pub sheep_id: u64,
    pub shepherd_id: u64
}