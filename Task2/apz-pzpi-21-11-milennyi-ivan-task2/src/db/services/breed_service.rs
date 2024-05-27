use std::sync::Arc;
use sqlx::{MySql, Pool};
use crate::db::services::traits::Service;

pub(crate) struct BreedService{
    pool: Arc<Pool<MySql>>,
}
impl Service for BreedService {
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        BreedService { pool }
    }
}