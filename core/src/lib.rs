//! Core functionality for the Nexa Gateway

use common::errors::Result;

/// Data provider trait for abstracting data access
pub trait DataProvider: Send + Sync {
    fn get_data(&self, id: &str) -> Result<String>;
}

/// Real implementation of the data provider
pub struct RealDataProvider;

impl RealDataProvider {
    pub fn new() -> Self {
        Self
    }
}

impl DataProvider for RealDataProvider {
    fn get_data(&self, id: &str) -> Result<String> {
        Ok(format!("Data for {}", id))
    }
}

pub fn get_data_provider() -> Box<dyn DataProvider> {
    Box::new(RealDataProvider::new())
}

/// Initialize the core module
pub async fn initialize() -> Result<()> {
    tracing::info!("Initializing core module");
    Ok(())
}

#[cfg(test)]
mod tests {
    // Removed unused import: super::*

    #[test]
    fn test_version() {
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.1.0");
    }
}
