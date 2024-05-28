use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::service_error::ServiceError;
use crate::db::traits::{AuthService, Service, ShepherdManage};
use crate::models::Shepherd;
use crate::view_models::ShepherdVM;

pub(crate) struct ShepherdService<T>{
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for ShepherdService<Pool<MySql>> {
    type Model = Shepherd;
    type Error = ServiceError;
    type ViewModel = ShepherdVM;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        ShepherdService { pool }
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

    async fn get_by_id(&self, id: u64) -> Result<Self::Model, Self::Error> {
        todo!()
    }
}
#[async_trait]
impl AuthService<Pool<MySql>> for ShepherdService<Pool<MySql>> {
    async fn check_username(&self, username: String) -> Result<bool, Self::Error> {
        todo!()
    }

    async fn authorize(&self, username: String, password_hash: String) -> Result<Self::ViewModel, Self::Error> {
        todo!()
    }
}

#[async_trait]
impl ShepherdManage<Pool<MySql>> for ShepherdService<Pool<MySql>>{

}