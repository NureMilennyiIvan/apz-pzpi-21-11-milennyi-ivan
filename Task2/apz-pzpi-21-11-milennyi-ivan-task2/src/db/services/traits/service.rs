use std::sync::Arc;
use sqlx::{MySql, Pool};

pub(crate) trait Service{
    fn new(pool: Arc<Pool<MySql>>) -> Self;
}