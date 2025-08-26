

use super::core_types::*;
use beardog_errors::{BearDogError, BearDogResult};

#[derive(Debug)]
pub struct UniversalAdapter {

    connected_hsms: std::collections::HashMap<String, super::super::DiscoveredHsm>,
}
impl UniversalAdapter {

    pub fn new() -> BearDogResult<Self> {
        Ok(Self {
            connected_hsms: std::collections::HashMap::with_capacity(16),
        })
    }

    pub async fn connect_to_hsm(&mut self, hsm_id: &str) -> BearDogResult<()> {

        tracing::info!("Connecting to HSM: {}", hsm_id);
        Ok(())

    pub async fn test_connection(&self, hsm_id: &str) -> BearDogResult<bool> {
        tracing::info!("Testing connection to HSM: {}", hsm_id);
        Ok(true)
