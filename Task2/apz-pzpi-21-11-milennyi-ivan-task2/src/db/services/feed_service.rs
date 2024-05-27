use std::sync::Arc;
use sqlx::{MySql, Pool};
use crate::db::services::traits::Service;

pub(crate) struct FeedService{
    pool: Arc<Pool<MySql>>,
}
impl Service for FeedService {
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        FeedService { pool }
    }
}