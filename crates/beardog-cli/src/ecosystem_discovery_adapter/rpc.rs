// SPDX-License-Identifier: AGPL-3.0-only

//! JSON-RPC helpers and capability matching for discovery / IPC.

use super::EcosystemDiscoveryAdapter;
use beardog_core::ecosystem::primal_types::DiscoveredPrimal;
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::CapabilityType;
use beardog_types::canonical::discovery::UniversalCapabilityType;

impl EcosystemDiscoveryAdapter {
    pub(crate) fn tower_atomic_error(err: beardog_tower_atomic::Error) -> BearDogError {
        BearDogError::network(err.to_string())
    }

    /// Derive JSON-RPC method and params from the payload (explicit `method`/`params` or default).
    pub(crate) fn jsonrpc_method_and_params(
        payload: &serde_json::Value,
    ) -> (String, serde_json::Value) {
        if let Some(obj) = payload.as_object() {
            if let Some(m) = obj.get("method").and_then(|v| v.as_str()) {
                let params = obj
                    .get("params")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null);
                return (m.to_string(), params);
            }
            if let Some(m) = obj.get("jsonrpc_method").and_then(|v| v.as_str()) {
                let params = obj
                    .get("params")
                    .cloned()
                    .unwrap_or_else(|| payload.clone());
                return (m.to_string(), params);
            }
        }
        ("ecosystem.rpc".to_string(), payload.clone())
    }

    /// Check if primal has the requested capability
    ///
    /// Implements agnostic, capability-based matching between Universal and Service capability types.
    /// This function enables runtime capability discovery without hardcoding primal names or types.
    ///
    /// # Arguments
    /// * `primal` - The discovered primal to check
    /// * `requested` - The universal capability type requested
    ///
    /// # Returns
    /// `true` if the primal provides the requested capability, `false` otherwise
    ///
    /// # Design
    /// Maps between `UniversalCapabilityType` (higher-level, domain-focused) and
    /// `ServiceCapabilityType` (lower-level, operation-focused) to enable flexible
    /// capability matching across different abstraction levels.
    pub(crate) fn primal_has_capability(
        primal: &DiscoveredPrimal,
        requested: &UniversalCapabilityType,
    ) -> bool {
        // Map UniversalCapabilityType to ServiceCapabilityType for comparison
        // This enables agnostic capability matching without hardcoding
        let required_service_capabilities: Vec<CapabilityType> = match requested {
            UniversalCapabilityType::Compute { .. } => vec![
                CapabilityType::ComputeIntelligence,
                CapabilityType::DistributedIntelligence,
            ],
            UniversalCapabilityType::Storage { .. } => vec![CapabilityType::DataStorage],
            UniversalCapabilityType::Network { .. } => {
                vec![CapabilityType::ServiceMesh, CapabilityType::Networking]
            }
            UniversalCapabilityType::Security { .. } => vec![
                CapabilityType::Security,
                CapabilityType::Authentication,
                CapabilityType::KeyManagement,
            ],
            UniversalCapabilityType::Orchestration { .. } => vec![
                CapabilityType::ContainerOrchestration,
                CapabilityType::WorkflowOrchestration,
            ],
            UniversalCapabilityType::Collaboration { .. } => vec![
                CapabilityType::DataStorage,    // Template storage
                CapabilityType::Authentication, // User auth
            ],
        };

        // Check if primal has ANY of the required capabilities
        // This OR-based matching allows flexible capability discovery
        required_service_capabilities.iter().any(|required| {
            primal
                .capabilities
                .iter()
                .any(|provided| provided == required)
        })
    }
}
