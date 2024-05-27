use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::traits::{Service, ShearingLogManage};
use crate::models::ShearingLog;

pub(crate) struct ShearingLogService<T> {
    pool: Arc<T>,
}
impl Service<Pool<MySql>> for ShearingLogService<Pool<MySql>> {
    type Model = ShearingLog;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        ShearingLogService { pool }
    }
}
#[async_trait]
impl ShearingLogManage<Pool<MySql>> for ShearingLogService<Pool<MySql>>{

}