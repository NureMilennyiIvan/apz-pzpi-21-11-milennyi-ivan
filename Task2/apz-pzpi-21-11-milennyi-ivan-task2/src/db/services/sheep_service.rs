use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::traits::{Service, SheepManage};
use crate::models::Sheep;

pub(crate) struct SheepService<T>{
    pool: Arc<T>,
}
impl Service<Pool<MySql>> for SheepService<Pool<MySql>> {
    type Model = Sheep;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        SheepService { pool }
    }
}
#[async_trait]
impl SheepManage<Pool<MySql>> for SheepService<Pool<MySql>>{

}