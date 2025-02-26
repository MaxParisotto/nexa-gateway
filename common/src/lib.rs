//! Common utilities and shared code for Nexa Gateway

use std::fmt;

/// Result type used throughout the Nexa Gateway codebase
pub type Result<T> = std::result::Result<T, Error>;

/// Common error type for the Nexa Gateway
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Version information for the Nexa Gateway
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Returns the current version of the Nexa Gateway
pub fn version() -> Version {
    Version {
        major: 0,
        minor: 1,
        patch: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_display() {
        let version = version();
        assert_eq!(format!("{}", version), "0.1.0");
    }
}

pub mod config;
pub mod error;

// Re-export commonly used items
pub use config::Settings;
pub use error::CommonError;
