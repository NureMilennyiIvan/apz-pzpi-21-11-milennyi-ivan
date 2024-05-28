use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum ServiceError {
    DatabaseError(sqlx::Error),
    ValidationError(validator::ValidationErrors),
    UniqueError
}
impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ServiceError::DatabaseError(error) => write!(f, "Database error: {}", error.to_string()),
            ServiceError::ValidationError(error) => write!(f, "Validation error {}", error.to_string()),
            ServiceError::UniqueError => write!(f, "This user already exist:"),
        }
    }
}

impl Error for ServiceError{ }