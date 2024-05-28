use async_trait::async_trait;
use crate::db::traits::Service;

#[async_trait]
pub(crate) trait BreedManage<T>: Service<T>{
    async fn get_all_vms(&self, id: u64) -> Result<Option<Vec<Self::ViewModel>>, Self::Error>;
}