use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::traits::{Service, ShepherdManage};
use crate::models::Shepherd;

pub(crate) struct ShepherdService<T>{
    pool: Arc<T>,
}
impl Service<Pool<MySql>> for ShepherdService<Pool<MySql>> {
    type Model = Shepherd;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        ShepherdService { pool }
    }
}
#[async_trait]
impl ShepherdManage<Pool<MySql>> for ShepherdService<Pool<MySql>>{

}