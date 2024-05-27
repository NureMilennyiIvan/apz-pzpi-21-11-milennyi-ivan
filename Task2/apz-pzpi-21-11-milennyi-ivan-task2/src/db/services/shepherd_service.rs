use std::sync::Arc;
use sqlx::{MySql, Pool};
use crate::db::services::traits::Service;

pub(crate) struct ShepherdService{
    pool: Arc<Pool<MySql>>,
}
impl Service for ShepherdService {
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        ShepherdService { pool }
    }
}