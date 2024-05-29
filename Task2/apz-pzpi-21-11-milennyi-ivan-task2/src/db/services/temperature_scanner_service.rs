use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::service_error::ServiceError;
use crate::db::traits::{Service, TemperatureScannerManage};
use crate::models::TemperatureScanner;
use crate::view_models::TemperatureScannerVM;

pub(crate) struct TemperatureScannerService<T>{
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for TemperatureScannerService<Pool<MySql>> {
    type Model = TemperatureScanner;
    type Error = ServiceError;
    type ViewModel = TemperatureScannerVM;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        TemperatureScannerService { pool }
    }

    async fn create(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        todo!()
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        todo!()
    }

    async fn update(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        todo!()
    }

    async fn get_all(&self) -> Result<Vec<Self::Model>, Self::Error> {
        todo!()
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error> {
        todo!()
    }
}
#[async_trait]
impl TemperatureScannerManage<Pool<MySql>> for TemperatureScannerService<Pool<MySql>>{

}