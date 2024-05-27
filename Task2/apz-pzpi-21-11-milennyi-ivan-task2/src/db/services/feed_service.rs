use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::services::BreedService;
use crate::db::traits::{BreedManage, FeedManage, Service};
use crate::models::Feed;

pub(crate) struct FeedService<T>{
    pool: Arc<T>,
}
impl Service<Pool<MySql>> for FeedService<Pool<MySql>> {
    type Model = Feed;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        FeedService { pool }
    }
}
#[async_trait]
impl FeedManage<Pool<MySql>> for FeedService<Pool<MySql>>{

}