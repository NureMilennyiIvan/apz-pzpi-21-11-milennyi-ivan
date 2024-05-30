use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub(crate) struct StorekeeperVM{
    id: u64,
    name: String,
    surname: String,
}