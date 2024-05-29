use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool, query_as, query};
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

    async fn create(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            INSERT INTO Breeds (name, feed_id, info)
            VALUES (?, ?, ?)
            RETURNING id, name, feed_id, info
            "#
        )
        .bind(item.name())
        .bind(item.feed_id())
        .bind(item.info())
        .fetch_one(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        query(
            r#"
            DELETE FROM Breeds
            WHERE id = ?
            "#
        )
        .bind(item_id)
        .execute(&*self.pool).await
        .map(|_| ()).map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn update(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            UPDATE Breeds
            SET name = ?, feed_id = ?, info = ?
            WHERE id = ?
            RETURNING id, name, feed_id, info
            "#
        )
        .bind(item.name())
        .bind(item.feed_id())
        .bind(item.info())
        .bind(item.id().ok_or(ServiceError::CustomError("ID is required".to_string()))?)
        .fetch_one(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_all(&self) -> Result<Vec<Self::Model>, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT * FROM Breeds
            "#
        )
        .fetch_all(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT * FROM Breeds
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }
}
#[async_trait]
impl BreedManage<Pool<MySql>> for BreedService<Pool<MySql>>{
    async fn get_all_vms(&self) -> Result<Vec<Self::ViewModel>, Self::Error> {
        todo!()
    }
}