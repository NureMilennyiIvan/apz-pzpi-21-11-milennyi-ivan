use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool, query, query_as};
use crate::db::service_error::ServiceError;
use crate::db::traits::{FeedSupplyManage, Service};
use crate::models::FeedSupply;
use crate::view_models::FeedSupplyVM;

pub(crate) struct FeedSupplyService<T>{
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for FeedSupplyService<Pool<MySql>> {
    type Model = FeedSupply;
    type Error = ServiceError;
    type ViewModel = FeedSupplyVM;

    fn new(pool: Arc<Pool<MySql>>) -> Self {
        FeedSupplyService { pool }
    }

    async fn create(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        let mut tx = self.pool.begin().await.map_err(|error| ServiceError::DatabaseError(error))?;

        query(
            r#"
            UPDATE Feeds
            SET amount = amount + ?
            WHERE id = ?
            "#
        )
        .bind(item.amount())
        .bind(item.feed_id())
        .execute(&mut *tx).await
        .map_err(|error| ServiceError::DatabaseError(error))?;

        let result = query_as::<_, Self::Model>(
            r#"
            INSERT INTO FeedSupplies (storekeeper_id, amount, timestamp, feed_id)
            VALUES (?, ?, ?)
            RETURNING id, storekeeper_id, amount, timestamp, feed_id
            "#
        )
        .bind(item.storekeeper_id())
        .bind(item.amount())
        .bind(item.timestamp())
        .bind(item.feed_id())
        .fetch_one(&mut *tx).await
        .map_err(|error| ServiceError::DatabaseError(error));

        tx.commit().await.map_err(|error| ServiceError::DatabaseError(error))?;

        result
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        query(
            r#"
            DELETE FROM FeedSupplies
            WHERE id = ?
            "#
        )
        .bind(item_id)
        .execute(&*self.pool).await
        .map(|_| ()).map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn update(&self, _item: Self::Model) -> Result<Self::Model, Self::Error> {
        Err(ServiceError::ForbiddenError)
    }

    async fn get_all(&self) -> Result<Vec<Self::Model>, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT id, storekeeper_id, amount, timestamp, feed_id
            FROM FeedSupplies
            "#
        )
        .fetch_all(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT id, storekeeper_id, amount, timestamp, feed_id
            FROM FeedSupplies
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }
}
#[async_trait]
impl FeedSupplyManage<Pool<MySql>> for FeedSupplyService<Pool<MySql>>{
    async fn get_all_vms(&self) -> Result<Vec<Self::ViewModel>, Self::Error> {
        query_as::<_, FeedSupplyVM>(
            r#"
            SELECT
            fs.id,
            fs.amount,
            fs.timestamp,
            s.name AS storekeeper_name,
            s.surname AS storekeeper_surname
            FROM FeedSupplies fs
            LEFT JOIN Storekeepers s ON fs.storekeeper_id = s.id
            "#
        )
        .fetch_all(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }
}