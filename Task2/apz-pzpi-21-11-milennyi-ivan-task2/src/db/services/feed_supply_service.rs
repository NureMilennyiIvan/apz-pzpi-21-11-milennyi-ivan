use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::service_error::ServiceError;
use crate::db::traits::{FeedSupplyManage, Service};
use crate::models::FeedSupply;

pub(crate) struct FeedSupplyService<T>{
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for FeedSupplyService<Pool<MySql>> {
    type Model = FeedSupply;
    type Error = ServiceError;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        FeedSupplyService { pool }
    }

    async fn create(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        todo!()
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        todo!()
    }

    async fn get_all(&self) -> Result<Vec<Self::Model>, Self::Error> {
        todo!()
    }

    async fn get_by_id(&self, id: u64) -> Result<Self::Model, Self::Error> {
        todo!()
    }
}
#[async_trait]
impl FeedSupplyManage<Pool<MySql>> for FeedSupplyService<Pool<MySql>>{

}