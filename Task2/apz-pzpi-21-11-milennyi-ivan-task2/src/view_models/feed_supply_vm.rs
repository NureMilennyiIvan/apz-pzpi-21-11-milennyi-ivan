use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub(crate) struct FeedSupplyVM {
    id: u64,
    amount: u64,
    timestamp: u64,
    storekeeper_name: Option<String>,
    storekeeper_surname: Option<String>,
}
