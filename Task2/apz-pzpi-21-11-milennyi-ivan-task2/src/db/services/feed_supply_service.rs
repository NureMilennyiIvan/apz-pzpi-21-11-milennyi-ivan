use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::service_error::ServiceError;
use crate::db::traits::{FeedSupplyManage, Service};
use crate::models::FeedSupply;
use crate::view_models::FeedSupplyVM;

pub(crate) struct FeedSupplyService<T>{
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for FeedSupplyService<Pool<MySql>> {
    type Model = FeedSupply;
    type Error = ServiceError;
    type ViewModel = FeedSupplyVM;

    fn new(pool: Arc<Pool<MySql>>) -> Self {
        FeedSupplyService { pool }
    }

    async fn create(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        // update feed amount
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

    async fn get_all_vms(&self) -> Result<Vec<Self::ViewModel>, Self::Error> {
        todo!()
    }
}