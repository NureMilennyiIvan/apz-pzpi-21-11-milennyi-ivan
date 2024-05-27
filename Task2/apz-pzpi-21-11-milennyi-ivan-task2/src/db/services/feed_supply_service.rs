use std::sync::Arc;
use sqlx::{MySql, Pool};
use crate::db::services::traits::Service;

pub(crate) struct FeedSupplyService{
    pool: Arc<Pool<MySql>>,
}
impl Service for FeedSupplyService {
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        FeedSupplyService { pool }
    }
}