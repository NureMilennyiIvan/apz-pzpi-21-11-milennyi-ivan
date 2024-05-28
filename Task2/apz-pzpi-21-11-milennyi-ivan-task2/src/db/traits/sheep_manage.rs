use async_trait::async_trait;
use crate::db::traits::Service;

#[async_trait]
pub(crate) trait SheepManage<T>: Service<T>{
    type SheepDetails;
    async fn get_all_vms_by_shepherd_id(&self, id: u64) -> Result<Option<Vec<Self::ViewModel>>, Self::Error>;
    async fn get_details_by_id(&self, id: u64) -> Result<Option<Self::SheepDetails>, Self::Error>;
    async fn change_shepherd(&self, sheep_id: u64, shepherd_id: u64) -> Result<(), Self::Error>;
}