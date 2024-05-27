use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::traits::{BreedManage, Service};
use crate::models::Breed;

pub(crate) struct BreedService<T>{
    pool: Arc<T>,
}
impl Service<Pool<MySql>> for BreedService<Pool<MySql>> {
    type Model = Breed;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        BreedService { pool }
    }
}
#[async_trait]
impl BreedManage<Pool<MySql>> for BreedService<Pool<MySql>>{

}