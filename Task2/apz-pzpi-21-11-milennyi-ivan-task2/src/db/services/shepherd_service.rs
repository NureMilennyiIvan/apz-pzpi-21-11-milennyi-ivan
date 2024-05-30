use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool, query, query_as};
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
        query_as::<_, Self::Model>(
            r#"
            INSERT INTO Shepherds (username, password, name, surname)
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
            DELETE FROM Shepherds
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
            UPDATE Shepherds
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
            SELECT * FROM Shepherds
            "#
        )
        .fetch_all(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT * FROM Shepherds
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }
}
#[async_trait]
impl AuthService<Pool<MySql>> for ShepherdService<Pool<MySql>> {
    async fn check_username(&self, user: &Self::Model) -> Result<bool, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT * FROM Shepherds
            WHERE username = ?
            "#
        )
        .bind(user.username())
        .fetch_optional(&*self.pool).await.map(|result| result.is_some())
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn authorize(&self, username: String, password_hash: String) -> Result<Option<Self::ViewModel>, Self::Error> {
        query_as::<_, Self::ViewModel>(
            r#"
            SELECT id, name, surname FROM Shepherds
            WHERE username = ? AND password = ?
            "#
        )
        .bind(username)
        .bind(password_hash)
        .fetch_optional(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }
}

#[async_trait]
impl ShepherdManage<Pool<MySql>> for ShepherdService<Pool<MySql>>{

}