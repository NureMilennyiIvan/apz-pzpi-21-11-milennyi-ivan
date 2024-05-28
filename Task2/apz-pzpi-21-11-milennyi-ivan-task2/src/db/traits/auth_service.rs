use async_trait::async_trait;
use crate::db::traits::Service;

#[async_trait]
pub(crate) trait AuthService<T>: Service<T> {
    async fn check_username(&self, username: String) -> Result<bool, Self::Error>;
    async fn authorize(&self, username: String, password_hash: String) -> Result<Self::ViewModel, Self::Error>;
}