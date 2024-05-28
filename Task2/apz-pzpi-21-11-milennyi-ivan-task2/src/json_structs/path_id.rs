use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, ToSchema, Debug, IntoParams)]
pub(crate) struct PathId{
   pub id: u64
}