use std::sync::Arc;
use sqlx::{MySql, Pool};
use crate::db::services::traits::Service;

pub(crate) struct StorekeeperService{
    pool: Arc<Pool<MySql>>,
}
impl Service for StorekeeperService {
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        StorekeeperService { pool }
    }
}