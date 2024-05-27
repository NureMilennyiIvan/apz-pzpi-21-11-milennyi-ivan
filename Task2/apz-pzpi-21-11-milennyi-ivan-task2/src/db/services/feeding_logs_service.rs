use std::sync::Arc;
use sqlx::{MySql, Pool};
use crate::db::services::traits::Service;

pub(crate) struct FeedingLogsService{
    pool: Arc<Pool<MySql>>,
}
impl Service for FeedingLogsService {
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        FeedingLogsService { pool }
    }
}