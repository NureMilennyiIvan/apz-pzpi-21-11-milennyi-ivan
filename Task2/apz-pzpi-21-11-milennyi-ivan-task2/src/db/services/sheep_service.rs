use std::sync::Arc;
use sqlx::{MySql, Pool};
use crate::db::services::traits::Service;

pub(crate) struct SheepService{
    pool: Arc<Pool<MySql>>,
}
impl Service for SheepService {
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        SheepService { pool }
    }
}