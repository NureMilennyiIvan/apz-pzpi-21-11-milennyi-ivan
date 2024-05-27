use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait Context<T, C>{
    async fn new(connection: C) -> Self;
    fn get_pool(&self) -> Arc<T>;
}