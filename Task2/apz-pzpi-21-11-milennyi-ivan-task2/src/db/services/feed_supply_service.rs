use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::traits::{FeedSupplyManage, Service};
use crate::models::FeedSupply;

pub(crate) struct FeedSupplyService<T>{
    pool: Arc<T>,
}
impl Service<Pool<MySql>> for FeedSupplyService<Pool<MySql>> {
    type Model = FeedSupply;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        FeedSupplyService { pool }
    }
}
#[async_trait]
impl FeedSupplyManage<Pool<MySql>> for FeedSupplyService<Pool<MySql>>{

}