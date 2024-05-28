use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::service_error::ServiceError;
use crate::db::traits::{BreedManage, Service};
use crate::models::Breed;
use crate::view_models::BreedVM;

pub(crate) struct BreedService<T>{
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for BreedService<Pool<MySql>> {
    type Model = Breed;
    type Error = ServiceError;
    type ViewModel = BreedVM;

    fn new(pool: Arc<Pool<MySql>>) -> Self {
        BreedService { pool }
    }

    async fn create(&self, item: Self::Model) -> Result<Option<Self::Model>, Self::Error> {
        todo!()
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        todo!()
    }

    async fn update(&self, item: Self::Model) -> Result<Option<Self::Model>, Self::Error> {
        todo!()
    }

    async fn get_all(&self) -> Result<Option<Vec<Self::Model>>, Self::Error> {
        todo!()
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error> {
        todo!()
    }
}
#[async_trait]
impl BreedManage<Pool<MySql>> for BreedService<Pool<MySql>>{
    async fn get_all_vms(&self, id: u64) -> Result<Option<Vec<Self::ViewModel>>, Self::Error> {
        todo!()
    }
}