use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::service_error::ServiceError;
use crate::db::traits::{Service, SheepManage};
use crate::models::Sheep;

pub(crate) struct SheepService<T>{
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for SheepService<Pool<MySql>> {
    type Model = Sheep;
    type Error = ServiceError;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        SheepService { pool }
    }

    async fn create(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        todo!()
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        todo!()
    }

    async fn get_all(&self) -> Result<Vec<Self::Model>, Self::Error> {
        todo!()
    }

    async fn get_by_id(&self, id: u64) -> Result<Self::Model, Self::Error> {
        todo!()
    }
}
#[async_trait]
impl SheepManage<Pool<MySql>> for SheepService<Pool<MySql>>{

}