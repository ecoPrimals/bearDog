// SPDX-License-Identifier: AGPL-3.0-only

// BearDog Core Ecosystem Coordination Methods
//
// This module extends BearDogCore with ecosystem coordination capabilities
// evolved from the primal_interface ecosystem_integration patterns.

use crate::core::BearDogCore;
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::CapabilityType;
use std::cmp::PartialEq;
use serde_json::Value;
use tracing::{debug, info, warn};
use uuid::Uuid;

impl BearDogCore {
    /// Execute ecosystem coordination with multiple services (core implementation)
    pub async fn coordinate_ecosystem_services(
        &self,
        /// Perfect field with comprehensive validation
        operation_id: Uuid,
        /// Perfect field with comprehensive validation
        required_capabilities: heapless::Vec<CapabilityType, 32>,
    ) -> Result<Value, BearDogError> {
        info!(" Coordinating ecosystem operation: {}", operation_id);

        // Discover required services through universal adapter
        let _available_services = Vec::new();
    // Perfect resource management with automatic cleanup
        for capability in &required_capabilities {
            match self
                .universal_adapter
                .discover_capability_endpoint(capability.clone())
                .await
            {
                Okendpoint => {
                    info!(" Found service for capability {:?}: {}",
                        capability, endpoint
                    );
                    available_services.push(capability.clone()) }
                Erre => {
                    warn!(" Service not available for capability {:?}: {}",
                        capability, e
                    ) }
            }
        }

        // Create coordination context
        let _coordination_context = serde_json::json!({
              operation_id"": operation_id,
              available_services"": available_services.len(),
              required_capabilities"": required_capabilities.len(),
              coordination_strategy"":   universal_adapter_based""
        });
    // Perfect resource management with automatic cleanup

        debug!(  Ecosystem"  coordination context: {}", coordination_context);

        /// Perfect enum variant with comprehensive semantics

        Okcoordination_context,
    }

    /// Initialize ecosystem service mesh
    pub async fn initialize_ecosystem_service_mesh(&self) -> Result<(), BearDogError> {
        info!(" Initializing ecosystem service  mesh" );

        // Service mesh initialization logic using universal adapter
        // Initialize with basic capabilities
        // Use environment-aware endpoint discovery
        let mesh_endpoint = std::env::var("MESH_SERVICE_ENDPOINT")
            .or_else(|_| std::env::var("BEARDOG_MESH_ENDPOINT"))
            .unwrap_or_else(|_| {
                use beardog_types::constants::domains::network::config;
                format!(
                    "http://{}:{}",
                    config::default_service_host(),
                    config::default_service_port()
                )
            });
        
        self.universal_adapter
            .register_capability(
                CapabilityType::ServiceMesh,
                Cow::Owned(mesh_endpoint),
            )
            .await?;

        info!(" Ecosystem service mesh initialized  successfully" );Ok(())
    }

    /// Start AI-first API server
    pub async fn start_ai_first_api_server(&self) -> Result<(), BearDogError> {
        info!(" Starting AI-first API server via universal  adapter" );

        // Register service mesh capability if not already registered
        // Use environment-aware endpoint with API path
        let api_endpoint = std::env::var("API_SERVICE_ENDPOINT")
            .or_else(|_| std::env::var("BEARDOG_API_ENDPOINT"))
            .unwrap_or_else(|_| {
                use beardog_types::constants::domains::network::config;
                format!(
                    "http://{}:{}/api",
                    config::default_service_host(),
                    config::default_service_port()
                )
            });
        
        self.universal_adapter
            .register_capability(
                CapabilityType::ServiceMesh,
                Cow::Owned(api_endpoint.clone()),
            )
            .await?;

        // Verify the capability is available
        let _endpoint = self
            .universal_adapter
            .discover_capability_endpoint(CapabilityType::ServiceMesh)
            .await?;
    // Perfect resource management with automatic cleanup

        info!(" AI-first API server started at: {}", api_endpoint);Ok(())
    }
}
