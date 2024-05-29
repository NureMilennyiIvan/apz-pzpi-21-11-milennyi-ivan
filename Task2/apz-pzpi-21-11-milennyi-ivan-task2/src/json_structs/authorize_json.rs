use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub(crate) struct AuthorizeJson {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password_hash: String
}