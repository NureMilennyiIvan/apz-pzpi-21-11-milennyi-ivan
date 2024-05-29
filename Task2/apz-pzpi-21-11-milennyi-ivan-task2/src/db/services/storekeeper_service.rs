use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool, query, query_as};
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
        query_as::<_, Self::Model>(
            r#"
            INSERT INTO Storekeepers (username, password, name, surname)
            VALUES (?, ?, ?, ?)
            RETURNING id, username, password, name, surname
            "#
        )
        .bind(item.username())
        .bind(item.password())
        .bind(item.name())
        .bind(item.surname())
        .fetch_one(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        query(
            r#"
            DELETE FROM Storekeepers
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
            UPDATE Storekeepers
            SET username = ?, password = ?, name = ?, surname = ?
            WHERE id = ?
            RETURNING id, username, password, name, surname
            "#
        )
        .bind(item.username())
        .bind(item.password())
        .bind(item.name())
        .bind(item.surname())
        .bind(item.id().ok_or(ServiceError::CustomError("ID is required".to_string()))?)
        .fetch_one(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_all(&self) -> Result<Vec<Self::Model>, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT * FROM Storekeepers
            "#
        )
        .fetch_all(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT * FROM Storekeepers
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
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