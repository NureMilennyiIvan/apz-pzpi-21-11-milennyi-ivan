use async_trait::async_trait;
use crate::db::traits::Service;

// Асинхронний трейт для управління сканерами температури
#[async_trait]
pub(crate) trait TemperatureScannerManage<T>: Service<T> {
    // Функція для автентифікації сканера температури за ідентифікатором і хешем пароля
    async fn authenticate(&self, id: u64, hash_password: String) -> Result<bool, Self::Error>;

    // Функція для оновлення температури сканера за ідентифікатором
    async fn update_temperature(&self, id: u64, temperature: u64) -> Result<(), Self::Error>;
}
