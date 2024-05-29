use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::service_error::ServiceError;
use crate::db::traits::{AuthService, Service, StorekeeperManage};
use crate::models::Storekeeper;
use crate::view_models::StorekeeperVM;

pub(crate) struct StorekeeperService<T>{
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for StorekeeperService<Pool<MySql>> {
    type Model = Storekeeper;
    type Error = ServiceError;
    type ViewModel = StorekeeperVM;

    fn new(pool: Arc<Pool<MySql>>) -> Self {
        StorekeeperService { pool }
    }

    async fn create(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        todo!()
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        todo!()
    }

    async fn update(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        todo!()
    }

    async fn get_all(&self) -> Result<Vec<Self::Model>, Self::Error> {
        todo!()
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error> {
        todo!()
    }
}
#[async_trait]
impl AuthService<Pool<MySql>> for StorekeeperService<Pool<MySql>> {
    async fn check_username(&self, user: &Self::Model) -> Result<bool, Self::Error> {
        todo!()
    }

    async fn authorize(&self, username: String, password_hash: String) -> Result<Option<Self::ViewModel>, Self::Error> {
        todo!()
    }
}

#[async_trait]
impl StorekeeperManage<Pool<MySql>> for StorekeeperService<Pool<MySql>>{

}