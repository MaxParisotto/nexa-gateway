//! Common utilities and shared code for Nexa Gateway

use std::fmt;

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
pub mod errors;
pub mod database;
pub mod logging;
pub mod middleware;
pub mod models;

// Re-export commonly used items
pub use config::Settings;
pub use error::CommonError;
pub use errors::{AppError, Result};
