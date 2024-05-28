use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow, ToSchema)]
pub(crate) struct Sheep{
    id: Option<u64>,
    birth_date: u64,
    breed_id: u64,
    sex: bool,
    shepherd_id: Option<u64>
}