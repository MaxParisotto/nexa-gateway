pub fn get_data_provider() -> Box<dyn DataProvider> {
    Box::new(RealDataProvider::new())
}

//! Core functionality for the Nexa Gateway

use common::Result;

/// Initialize the core module
pub async fn initialize() -> Result<()> {
    tracing::info!("Initializing core module");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.1.0");
    }
}
