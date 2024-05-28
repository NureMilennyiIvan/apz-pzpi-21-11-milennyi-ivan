use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow, ToSchema)]
pub(crate) struct Storekeeper{
    id: Option<u64>,
    #[validate(length(min = 1))]
    username: String,
    #[validate(length(min = 1))]
    password: String,
    #[validate(length(min = 1))]
    name: String,
    #[validate(length(min = 1))]
    surname: String,
}