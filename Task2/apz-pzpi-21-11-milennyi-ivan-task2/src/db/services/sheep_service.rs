use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool, query, query_as};
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

    async fn create(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            INSERT INTO Sheep (birth_date, breed_id, sex, shepherd_id)
            VALUES (?, ?, ?, ?)
            RETURNING id, birth_date, breed_id, sex, shepherd_id
            "#
        )
        .bind(item.birth_date())
        .bind(item.breed_id())
        .bind(item.sex())
        .bind(item.shepherd_id())
        .fetch_one(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        query(
            r#"
            DELETE FROM Sheep
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
            UPDATE Sheep
            SET birth_date = ?, breed_id = ?, sex = ?, shepherd_id = ?
            WHERE id = ?
            RETURNING id, birth_date, breed_id, sex, shepherd_id
            "#
        )
        .bind(item.birth_date())
        .bind(item.breed_id())
        .bind(item.sex())
        .bind(item.shepherd_id())
        .bind(item.id().ok_or(ServiceError::CustomError("ID is required".to_string()))?)
        .fetch_one(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_all(&self) -> Result<Vec<Self::Model>, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT * FROM Sheep
            "#
        )
        .fetch_all(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT * FROM Sheep
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }
}
#[async_trait]
impl SheepManage<Pool<MySql>> for SheepService<Pool<MySql>>{
    type SheepDetails = SheepDetailsVM;

    async fn get_all_vms_by_shepherd_id(&self, id: u64) -> Result<Vec<Self::ViewModel>, Self::Error> {
        todo!()
    }

    async fn get_details_by_id(&self, id: u64) -> Result<Option<Self::SheepDetails>, Self::Error> {
        todo!()
    }

    async fn change_shepherd(&self, sheep_id: u64, shepherd_id: u64) -> Result<(), Self::Error> {
        todo!()
    }
}