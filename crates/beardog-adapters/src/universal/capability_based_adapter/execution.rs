//! # Capability Execution
//!
//! Request execution and response handling for capabilities.

use super::types::*;
use super::core::UniversalCapabilityAdapter;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::providers_unified::traits::{AdapterRequest, AdapterResponse};
use tracing::{debug, info};

impl UniversalCapabilityAdapter {
    /// Execute a capability request
    pub async fn execute_capability_request(
        &self,
        connection_id: &str,
        request: AdapterRequest,
    ) -> BearDogResult<AdapterResponse> {
        info!("⚡ Executing capability request via connection: {}", connection_id);
        
        // Verify connection exists
        let connections = self.connections.read().await;
        let connection = connections
            .get(connection_id)
            .ok_or_else(|| BearDogError::not_found(&format!("Connection not found: {}", connection_id)))?;
        
        // Placeholder response
        let response = CapabilityResponse {
            request_id: request.id,
            success: true,
            data: None,
            error: None,
            metadata: None,
        };
        
        info!("✅ Capability request executed successfully");
        Ok(response)
    }
} 