use std::sync::Arc;
use sqlx::{MySql, Pool};
use crate::db::services::traits::Service;

pub(crate) struct TemperatureScannerService{
    pool: Arc<Pool<MySql>>,
}
impl Service for TemperatureScannerService {
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        TemperatureScannerService { pool }
    }
}