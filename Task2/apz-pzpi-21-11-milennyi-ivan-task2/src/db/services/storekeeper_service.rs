use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::traits::{Service, StorekeeperManage};
use crate::models::Storekeeper;

pub(crate) struct StorekeeperService<T>{
    pool: Arc<T>,
}
impl Service<Pool<MySql>> for StorekeeperService<Pool<MySql>> {
    type Model = Storekeeper;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        StorekeeperService { pool }
    }
}
#[async_trait]
impl StorekeeperManage<Pool<MySql>> for StorekeeperService<Pool<MySql>>{

}