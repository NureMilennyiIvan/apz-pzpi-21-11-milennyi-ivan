use async_trait::async_trait;
use crate::db::traits::Service;

#[async_trait]
pub(crate) trait FeedingLogManage<T>: Service<T>{
    async fn get_all_vms_by_sheep_id(&self, id: u64) -> Result<Vec<Self::ViewModel>, Self::Error>;
    async fn get_all_vms_by_feed_id(&self, id: u64) -> Result<Vec<Self::ViewModel>, Self::Error>;
}