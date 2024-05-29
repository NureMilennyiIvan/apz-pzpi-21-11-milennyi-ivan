use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::service_error::ServiceError;
use crate::db::traits::{Service, ShearingLogManage};
use crate::models::ShearingLog;
use crate::view_models::ShearingLogVM;

pub(crate) struct ShearingLogService<T> {
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for ShearingLogService<Pool<MySql>> {
    type Model = ShearingLog;
    type Error = ServiceError;
    type ViewModel = ShearingLogVM;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        ShearingLogService { pool }
    }

    async fn create(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        todo!()
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        todo!()
    }

    async fn update(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        Err(ServiceError::ForbiddenError)
    }

    async fn get_all(&self) -> Result<Vec<Self::Model>, Self::Error> {
        todo!()
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error> {
        todo!()
    }
}
#[async_trait]
impl ShearingLogManage<Pool<MySql>> for ShearingLogService<Pool<MySql>>{
    async fn get_all_vms_by_sheep_id(&self, id: u64) -> Result<Vec<Self::ViewModel>, Self::Error> {
        todo!()
    }
}