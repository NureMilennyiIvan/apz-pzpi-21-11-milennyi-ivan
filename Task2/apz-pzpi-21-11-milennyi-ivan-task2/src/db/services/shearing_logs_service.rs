use std::sync::Arc;
use sqlx::{MySql, Pool};
use crate::db::services::traits::Service;

pub(crate) struct ShearingLogsService{
    pool: Arc<Pool<MySql>>,
}
impl Service for ShearingLogsService {
    fn new(pool: Arc<Pool<MySql>>) -> Self {
        ShearingLogsService { pool }
    }
}