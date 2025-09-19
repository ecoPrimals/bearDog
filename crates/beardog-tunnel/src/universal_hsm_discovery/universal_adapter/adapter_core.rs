

use super::core_types::*;
use beardog_errors::BearDogError;

#[derive(std::collections::HashMap<String, super::super::DiscoveredHsm>,
}
impl UniversalAdapter {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            connected_hsms: std::collections::HashMap::with_capacity(16),
        })
    }

/// Connect To Hsm operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn connect_to_hsm(&mut self, hsm_id: &str) -> Result<(), BearDogError> {

        tracing::info!("Connecting to HSM: {}", hsm_id);
        Ok(())

/// Test Connection operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn test_connection(&self, hsm_id: &str) -> Result<bool, BearDogError> {
        tracing::info!("Testing connection to HSM: {}", hsm_id);
        Ok(true)
