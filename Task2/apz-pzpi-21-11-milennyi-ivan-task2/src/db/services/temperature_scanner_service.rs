use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::traits::{Service, TemperatureScannerManage};
use crate::models::TemperatureScanner;

pub(crate) struct TemperatureScannerService<T>{
    pool: Arc<T>,
}
impl Service<Pool<MySql>> for TemperatureScannerService<Pool<MySql>> {
    type Model = TemperatureScanner;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        TemperatureScannerService { pool }
    }
}
#[async_trait]
impl TemperatureScannerManage<Pool<MySql>> for TemperatureScannerService<Pool<MySql>>{

}