use async_trait::async_trait;
use crate::db::traits::Service;

#[async_trait]
pub(crate) trait FeedManage<T>: Service<T>{

    async fn get_all_vms(&self) -> Result<Option<Vec<Self::ViewModel>>, Self::Error>;
}