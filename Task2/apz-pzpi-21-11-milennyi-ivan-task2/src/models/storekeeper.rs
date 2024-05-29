use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow, ToSchema)]
pub(crate) struct Storekeeper{
    id: Option<u64>,
    #[validate(length(min = 1))]
    username: String,
    #[validate(length(min = 1))]
    password: String,
    #[validate(length(min = 1))]
    name: String,
    #[validate(length(min = 1))]
    surname: String,
}
impl Storekeeper {
    pub fn id(&self) -> Option<u64> {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn surname(&self) -> &str {
        &self.surname
    }
}