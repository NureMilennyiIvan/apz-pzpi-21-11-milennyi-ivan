use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::service_error::ServiceError;
use crate::db::traits::{BreedManage, Service};
use crate::models::Breed;

pub(crate) struct BreedService<T>{
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for BreedService<Pool<MySql>> {
    type Model = Breed;
    type Error = ServiceError;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        BreedService { pool }
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
impl BreedManage<Pool<MySql>> for BreedService<Pool<MySql>>{

}