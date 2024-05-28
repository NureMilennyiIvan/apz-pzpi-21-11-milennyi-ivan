use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::service_error::ServiceError;
use crate::db::traits::{FeedingLogManage, Service};
use crate::models::FeedingLog;
use crate::view_models::FeedingLogVM;

pub(crate) struct FeedingLogService<T> {
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for FeedingLogService<Pool<MySql>> {
    type Model = FeedingLog;
    type Error = ServiceError;
    type ViewModel = FeedingLogVM;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        FeedingLogService { pool }
    }

    async fn create(&self, item: Self::Model) -> Result<Option<Self::Model>, Self::Error> {
        todo!()
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        todo!()
    }

    async fn update(&self, item: Self::Model) -> Result<Option<Self::Model>, Self::Error> {
        todo!()
    }

    async fn get_all(&self) -> Result<Option<Vec<Self::Model>>, Self::Error> {
        todo!()
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error> {
        todo!()
    }
}
#[async_trait]
impl FeedingLogManage<Pool<MySql>> for FeedingLogService<Pool<MySql>>{
    async fn get_all_vms_by_sheep_id(&self, id: u64) -> Result<Option<Vec<Self::ViewModel>>, Self::Error> {
        todo!()
    }

    async fn get_all_vms_by_feed_id(&self, id: u64) -> Result<Option<Vec<Self::ViewModel>>, Self::Error> {
        todo!()
    }
}