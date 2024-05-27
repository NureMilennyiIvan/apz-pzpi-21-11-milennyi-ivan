use async_trait::async_trait;
use crate::db::traits::Service;

#[async_trait]
pub(crate) trait StorekeeperManage<T>: Service<T>{

}