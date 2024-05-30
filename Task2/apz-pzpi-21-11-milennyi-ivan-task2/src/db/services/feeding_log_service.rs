use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool, query, query_as};
use crate::db::service_error::ServiceError;
use crate::db::traits::{FeedingLogManage, Service};
use crate::models::FeedingLog;
use crate::view_models::FeedingLogVM;

pub(crate) struct FeedingLogService<T> {
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for FeedingLogService<Pool<MySql>> {
    type Model = FeedingLog;
    type Error = ServiceError;
    type ViewModel = FeedingLogVM;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        FeedingLogService { pool }
    }

    async fn create(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            INSERT INTO FeedingLogs (sheep_id, shepherd_id, timestamp, feed_id, amount)
            VALUES (?, ?, ?, ?)
            RETURNING id, sheep_id, shepherd_id, timestamp, feed_id, amount
            "#
        )
        .bind(item.sheep_id())
        .bind(item.shepherd_id().ok_or(ServiceError::CustomError("ID is required".to_string()))?)
        .bind(item.timestamp())
        .bind(item.feed_id())
        .bind(item.amount())
        .fetch_one(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        query(
            r#"
            DELETE FROM FeedingLogs
            WHERE id = ?
            "#
        )
        .bind(item_id)
        .execute(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
        .map(|result|
            if result.rows_affected() == 0 {
                Err(ServiceError::CustomError("Zero rows affected".to_string()))
            }
            else{
                Ok(())
            }
        )
        .unwrap_or_else(|error| Err(error))
    }

    async fn update(&self, _item: Self::Model) -> Result<Self::Model, Self::Error> {
        Err(ServiceError::ForbiddenError)
    }

    async fn get_all(&self) -> Result<Vec<Self::Model>, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT * FROM FeedingLogs
            "#
        )
        .fetch_all(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT * FROM FeedingLogs
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }
}
#[async_trait]
impl FeedingLogManage<Pool<MySql>> for FeedingLogService<Pool<MySql>>{
    async fn get_all_vms_by_sheep_id(&self, id: u64) -> Result<Vec<Self::ViewModel>, Self::Error> {
        query_as::<_, FeedingLogVM>(
            r#"
            SELECT
            fl.id,
            fl.timestamp,
            fl.amount,
            sh.name AS shepherd_name,
            sh.surname AS shepherd_surname,
            fl.sheep_id
            FROM FeedingLogs fl
            LEFT JOIN Shepherds sh ON fl.shepherd_id = sh.id
            WHERE fl.sheep_id = ?
            "#
        )
        .bind(id)
        .fetch_all(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_all_vms_by_feed_id(&self, id: u64) -> Result<Vec<Self::ViewModel>, Self::Error> {
        query_as::<_, FeedingLogVM>(
            r#"
            SELECT
            fl.id,
            fl.timestamp,
            fl.amount,
            sh.name AS shepherd_name,
            sh.surname AS shepherd_surname,
            fl.sheep_id
            FROM FeedingLogs fl
            LEFT JOIN Shepherds sh ON fl.shepherd_id = sh.id
            WHERE fl.feed_id = ?
            "#
        )
        .bind(id)
        .fetch_all(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }
}