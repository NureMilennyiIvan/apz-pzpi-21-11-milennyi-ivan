use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::traits::{FeedingLogManage, Service};
use crate::models::FeedingLog;

pub(crate) struct FeedingLogService<T> {
    pool: Arc<T>,
}
impl Service<Pool<MySql>> for FeedingLogService<Pool<MySql>> {
    type Model = FeedingLog;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        FeedingLogService { pool }
    }
}
#[async_trait]
impl FeedingLogManage<Pool<MySql>> for FeedingLogService<Pool<MySql>>{

}