use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, Pool, query, query_as};
use crate::db::service_error::ServiceError;
use crate::db::traits::{FeedManage, Service};
use crate::models::Feed;
use crate::view_models::FeedVM;

pub(crate) struct FeedService<T>{
    pool: Arc<T>,
}
#[async_trait]
impl Service<Pool<MySql>> for FeedService<Pool<MySql>> {
    type Model = Feed;
    type Error = ServiceError;
    type ViewModel = FeedVM;
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        FeedService { pool }
    }

    async fn create(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            INSERT INTO Feeds (amount, name, calories, fat, protein, carbohydrates)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING id, amount, name, calories, fat, protein, carbohydrates
            "#
        )
        .bind(item.amount())
        .bind(item.name())
        .bind(item.calories())
        .bind(item.fat())
        .bind(item.protein())
        .bind(item.carbohydrates())
        .fetch_one(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn delete(&self, item_id: u64) -> Result<(), Self::Error> {
        query(
            r#"
            DELETE FROM Feeds
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

    async fn update(&self, item: Self::Model) -> Result<Self::Model, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            UPDATE Feeds
            SET amount = ?, name = ?, calories = ?, fat = ?, protein = ?, carbohydrates = ?
            WHERE id = ?
            RETURNING id, amount, name, calories, fat, protein, carbohydrates
            "#
        )
        .bind(item.amount())
        .bind(item.name())
        .bind(item.calories())
        .bind(item.fat())
        .bind(item.protein())
        .bind(item.carbohydrates())
        .bind(item.id().ok_or(ServiceError::CustomError("ID is required".to_string()))?)
        .fetch_one(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_all(&self) -> Result<Vec<Self::Model>, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT * FROM Feeds
            "#
        )
        .fetch_all(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error> {
        query_as::<_, Self::Model>(
            r#"
            SELECT * FROM Feeds
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }
}
#[async_trait]
impl FeedManage<Pool<MySql>> for FeedService<Pool<MySql>>{
    async fn get_all_vms(&self) -> Result<Vec<Self::ViewModel>, Self::Error> {
        query_as::<_, FeedVM>(
            r#"
            SELECT
            f.id,
            f.amount,
            f.name,
            f.calories,
            f.fat,
            f.protein,
            f.carbohydrates,
            b.name AS breed_name,
            CAST((SELECT COUNT(*) FROM Sheep s WHERE s.breed_id = b.id) AS UNSIGNED) AS sheep_count
            FROM Feeds f
            INNER JOIN Breeds b ON f.id = b.feed_id
            "#
        )
        .fetch_all(&*self.pool).await
        .map_err(|error| ServiceError::DatabaseError(error))
    }
}