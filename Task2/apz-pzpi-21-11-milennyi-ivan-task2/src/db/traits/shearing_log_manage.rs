use async_trait::async_trait;
use crate::db::traits::Service;

#[async_trait]
pub(crate) trait ShearingLogManage<T>: Service<T>{
    async fn get_all_vms_by_sheep_id(&self, id: u64) -> Result<Option<Vec<Self::ViewModel>>, Self::Error>;
}