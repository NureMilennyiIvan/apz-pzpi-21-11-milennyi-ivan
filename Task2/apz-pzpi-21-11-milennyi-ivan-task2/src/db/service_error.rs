use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum ServiceError {
    DatabaseError(sqlx::Error),
    ValidationError(validator::ValidationErrors),
    UniqueError,
    CustomError(String),
    ForbiddenError

}
impl Display for ServiceError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ServiceError::DatabaseError(error) => write!(formatter, "Database error: {}", error.to_string()),
            ServiceError::ValidationError(error) => write!(formatter, "Validation error: {}", error.to_string()),
            ServiceError::UniqueError => write!(formatter, "This user already exist:"),
            ServiceError::CustomError(error) => write!(formatter, "Custom error: {}", error),
            ServiceError::ForbiddenError => write!(formatter, "This action is forbidden!")
        }
    }
}

impl Error for ServiceError{ }