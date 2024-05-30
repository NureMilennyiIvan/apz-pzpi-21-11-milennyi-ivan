use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub(crate) struct FeedingLogVM{
    id: u64,
    timestamp: u64,
    amount: u64,
    shepherd_name: Option<String>,
    shepherd_surname: Option<String>,
    sheep_id: u64
}