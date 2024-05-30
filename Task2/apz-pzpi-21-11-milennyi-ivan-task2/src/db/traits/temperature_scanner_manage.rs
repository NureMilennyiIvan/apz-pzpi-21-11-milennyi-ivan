use async_trait::async_trait;
use crate::db::traits::Service;

#[async_trait]
pub(crate) trait TemperatureScannerManage<T>: Service<T>{
    async fn authenticate(&self, id: u64, hash_password: String) -> Result<bool, Self::Error>;
    async fn update_temperature(&self, id: u64, temperature: u64) -> Result<(), Self::Error>;

}