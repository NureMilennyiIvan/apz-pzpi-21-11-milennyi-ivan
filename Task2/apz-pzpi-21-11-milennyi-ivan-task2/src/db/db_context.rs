use std::sync::Arc;
use sqlx::{MySql, MySqlPool, Pool};

#[derive(Clone)]
pub(crate) struct DbContextMySql{
    pool: Arc<Pool<MySql>>
}
impl DbContextMySql{
    pub(crate) async fn new(db_string: String) -> DbContextMySql{
        let db_pool = MySqlPool::connect(&db_string).await.expect("Failed to connect to the database");
        DbContextMySql{
            pool: Arc::new(db_pool)
        }
    }
    pub(crate) fn get_pool(&self) -> Arc<Pool<MySql>> {
        Arc::clone(&self.pool)
    }
}