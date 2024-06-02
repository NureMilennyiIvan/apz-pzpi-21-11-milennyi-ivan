use async_trait::async_trait;
use crate::db::traits::Service;

// Асинхронний трейт для управління постачанням кормів
#[async_trait]
pub(crate) trait FeedSupplyManage<T>: Service<T> {
    // Функція для отримання всіх ViewModel постачань кормів
    async fn get_all_vms(&self) -> Result<Vec<Self::ViewModel>, Self::Error>;
}
