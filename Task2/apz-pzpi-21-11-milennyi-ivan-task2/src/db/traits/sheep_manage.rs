use async_trait::async_trait;
use crate::db::traits::Service;

#[async_trait]
pub(crate) trait SheepManage<T>: Service<T>{
    type SheepDetails;
    async fn get_all_vms_by_shepherd_id(&self, id: u64) -> Result<Vec<Self::ViewModel>, Self::Error>;
    async fn get_details_by_id(&self, id: u64) -> Result<Option<Self::SheepDetails>, Self::Error>;
    async fn change_shepherd(&self, sheep_id: u64, shepherd_id: Option<u64>) -> Result<(), Self::Error>;
    async fn change_temperature_scanner(&self, sheep_id: u64, temperature_scanner_id: Option<u64>) -> Result<(), Self::Error>;
}