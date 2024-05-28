use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait Service<T>{
    type Model;
    type Error;
    type ViewModel;
    fn new(pool: Arc<T>) -> Self;
    async fn create(&self, item: Self::Model) -> Result<Option<Self::Model>, Self::Error>;
    async fn delete(&self, item_id: u64) -> Result<(), Self::Error>;
    async fn update(&self, item: Self::Model) -> Result<Option<Self::Model>, Self::Error>;
    async fn get_all(&self) -> Result<Option<Vec<Self::Model>>, Self::Error>;
    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Self::Error>;
}