use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use crate::db::service_error::ServiceError;
use crate::db::traits::{Service, TemperatureScannerManage};
use crate::models::TemperatureScanner;
use crate::view_models::TemperatureScannerVM;

pub(crate) struct TemperatureScannerService<T>{
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for TemperatureScannerService<Pool<MySql>> {
    type Model = TemperatureScanner;
    type Error = ServiceError;
    type ViewModel = TemperatureScannerVM;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        TemperatureScannerService { pool }
    }

    async fn create(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        sqlx::query_as::<_, Self::Model>(
            r#"
            INSERT INTO TemperatureScanners (temperature, sheep_id, password)
            VALUES (?, ?, ?)
            RETURNING id, temperature, sheep_id, password
            "#
        )
        .bind(item.temperature())
        .bind(item.sheep_id())
        .bind(item.password())
        .fetch_one(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        sqlx::query(
            r#"
            DELETE FROM TemperatureScanners
            WHERE id = ?
            "#
        )
        .bind(item_id)
        .execute(&*self.pool).await
        .map(|_| ()).map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn update(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        sqlx::query_as::<_, Self::Model>(
            r#"
            UPDATE TemperatureScanners
            SET temperature = ?, sheep_id = ?, password = ?
            WHERE id = ?
            RETURNING id, temperature, sheep_id, password
            "#
        )
        .bind(item.temperature())
        .bind(item.sheep_id())
        .bind(item.password())
        .bind(item.id().ok_or(ServiceError::CustomError("ID is required".to_string()))?)
        .fetch_one(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_all(&self) -> Result<Vec<Self::Model>, Self::Error> {
        sqlx::query_as::<_, Self::Model>(
            r#"
            SELECT * FROM TemperatureScanners
            "#
        )
        .fetch_all(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error> {
        sqlx::query_as::<_, Self::Model>(
            r#"
            SELECT * FROM TemperatureScanners
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }
}
#[async_trait]
impl TemperatureScannerManage<Pool<MySql>> for TemperatureScannerService<Pool<MySql>>{

}