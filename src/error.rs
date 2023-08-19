use std::fmt;

/// Custom error type for representing different types of errors.
#[derive(Debug)]
pub enum Errors {
    /// Represents an error that occurred during a request.
    RequestError(String)
}

impl fmt::Display for Errors {
    /// Formats the error for display to end users.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::RequestError(err) => write!(f, "[Request Error]:🙀 {}", err)
        }
    }
}

impl std::error::Error for Errors {
    /// Provides information about the source of the error.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}
