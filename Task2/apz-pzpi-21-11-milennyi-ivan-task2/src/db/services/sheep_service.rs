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
        query_as::<_, SheepVM>(
            r#"
            SELECT
            s.id,
            s.name,
            b.name AS breed,
            s.sex,
            s.birth_date,
            (
                SELECT MAX(fl.timestamp)
                FROM FeedingLogs fl
                WHERE fl.sheep_id = s.id
            ) AS last_feeding_timestamp,
            (
                SELECT MAX(sl.timestamp)
                FROM ShearingLogs sl
                WHERE sl.sheep_id = s.id
            ) AS last_shearing_timestamp
            FROM Sheep s
            LEFT JOIN Breeds b ON s.breed_id = b.id
            WHERE s.shepherd_id = ?
            "#
        )
        .bind(id)
        .fetch_all(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_details_by_id(&self, id: u64) -> Result<Option<Self::SheepDetails>, Self::Error> {
        query_as::<_, SheepDetailsVM>(
            r#"
            SELECT
            s.id,
            s.name,
            b.name AS breed,
            s.sex,
            s.birth_date,
            (
                SELECT MAX(fl.timestamp)
                FROM FeedingLogs fl
                WHERE fl.sheep_id = s.id
            ) AS last_feeding_timestamp,
            (
                SELECT MAX(sl.timestamp)
                FROM ShearingLogs sl
                WHERE sl.sheep_id = s.id
            ) AS last_shearing_timestamp,
            s.weight,
            ts.temperature,
            f.id AS feed_id,
            f.name AS feed_name,
            ROUND(
                CASE
                    WHEN s.sex = true THEN s.weight * (0.05 + 0.0001 * TIMESTAMPDIFF(DAY, FROM_UNIXTIME(s.birth_date), NOW()))
                    ELSE s.weight * (0.04 + 0.0001 * TIMESTAMPDIFF(DAY, FROM_UNIXTIME(s.birth_date), NOW()))
                END
            ) AS feed_amount
            FROM Sheep s
            INNER JOIN Breeds b ON s.breed_id = b.id
            LEFT JOIN TemperatureScanners ts ON s.id = ts.sheep_id
            INNER JOIN Feeds f ON b.feed_id = f.id
            WHERE s.id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn change_shepherd(&self, sheep_id: u64, shepherd_id: u64) -> Result<(), Self::Error> {
        query(
            r#"
            UPDATE Sheep
            SET shepherd_id = ?,
            WHERE id = ?
            "#
        )
       .bind(shepherd_id)
       .bind(sheep_id)
       .execute(&*self.pool).await
       .map(|_| ()).map_err(|error| ServiceError::DatabaseError(error))
    }
}