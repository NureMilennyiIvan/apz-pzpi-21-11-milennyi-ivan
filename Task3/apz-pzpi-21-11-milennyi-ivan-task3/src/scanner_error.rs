use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum ScannerError {
    ArgumentsError(String),
    UpdatingError(String),
    AuthenticationError(String)
}
impl Display for ScannerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ScannerError::ArgumentsError(error) => write!(formatter, "Arguments error: {}.\nPlease restart device and try again", error),
            ScannerError::UpdatingError(error) => write!(formatter, "Updating temperature error: {}", error),
            ScannerError::AuthenticationError(error) => write!(formatter, "Authentication error: {}. \nPlease restart device and try again", error)
        }
    }
}

impl Error for ScannerError{ }