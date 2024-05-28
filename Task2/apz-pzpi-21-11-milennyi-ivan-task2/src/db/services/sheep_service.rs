use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::service_error::ServiceError;
use crate::db::traits::{Service, SheepManage};
use crate::models::Sheep;
use crate::view_models::{extra_view_models::SheepDetailsVM, SheepVM};

pub(crate) struct SheepService<T>{
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for SheepService<Pool<MySql>> {
    type Model = Sheep;
    type Error = ServiceError;
    type ViewModel = SheepVM;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        SheepService { pool }
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
impl SheepManage<Pool<MySql>> for SheepService<Pool<MySql>>{
    type SheepDetails = SheepDetailsVM;

    async fn get_all_vms_by_shepherd_id(&self, id: u64) -> Result<Option<Vec<Self::ViewModel>>, Self::Error> {
        todo!()
    }

    async fn get_details_by_id(&self, id: u64) -> Result<Option<Self::SheepDetails>, Self::Error> {
        todo!()
    }

    async fn change_shepherd(&self, sheep_id: u64, shepherd_id: u64) -> Result<(), Self::Error> {
        todo!()
    }
}