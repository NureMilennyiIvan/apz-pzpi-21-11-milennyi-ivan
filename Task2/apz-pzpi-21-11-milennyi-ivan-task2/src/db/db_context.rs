use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{MySql, MySqlPool, Pool};
use crate::db::traits::Context;

pub(crate) struct DbContextMySql<T>{
    pool: Arc<T>
}
#[async_trait]
impl Context<Pool<MySql>, String> for DbContextMySql<Pool<MySql>>{
    async fn new(connection: String) -> Self{
        let db_pool = MySqlPool::connect(&connection).await.expect("Failed to connect to the database");
        println!("Database connection established");
        DbContextMySql{
            pool: Arc::new(db_pool)
        }
    }
    fn get_pool(&self) -> Arc<Pool<MySql>> {
        Arc::clone(&self.pool)
    }
}