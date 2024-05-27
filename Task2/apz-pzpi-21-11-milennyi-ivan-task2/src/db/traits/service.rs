use std::sync::Arc;

pub(crate) trait Service<T>{
    type Model;
    fn new(pool: Arc<T>) -> Self;
}