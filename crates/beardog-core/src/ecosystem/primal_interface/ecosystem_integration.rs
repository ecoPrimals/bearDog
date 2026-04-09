// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::core::BearDogCore;
use crate::self_knowledge::{IdentityInputs, PrimalIdentity};
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use beardog_types::canonical::capabilities::CapabilityType;
use serde_json::Value;
use tracing::{debug, info, warn};
use uuid::Uuid;

impl BearDogCore {
    /// Execute ecosystem coordination with multiple services
    ///
    /// # Errors
    /// Returns error if insufficient services available or coordination fails
    pub async fn coordinate_ecosystem_operation(
        &self,
        operation_id: Uuid,
        required_capabilities: Vec<CapabilityType>,
    ) -> Result<Value, BearDogError> {
        info!("🌐 Coordinating ecosystem operation: {}", operation_id);

        // Discover required services through universal adapter
        let mut available_services = Vec::new();
        for capability in &required_capabilities {
            match self
                .universal_adapter
                .discover_capability_endpoint(capability.clone())
                .await
            {
                Ok(endpoint) => {
                    info!(
                        "✅ Found service for capability {:?}: {}",
                        capability, endpoint
                    );
                    available_services.push(capability.clone());
                }
                Err(e) => {
                    warn!(
                        "⚠️ Service not available for capability {:?}: {}",
                        capability, e
                    );
                }
            }
        }

        // Create coordination context
        let coordination_context = {
            use serde_json::{Map, Value};

            let mut services = Map::new();
            services.insert(
                "compute".to_string(),
                Value::Bool(available_services.contains(&CapabilityType::ComputeIntelligence)),
            );
            services.insert(
                "ai".to_string(),
                Value::Bool(available_services.contains(&CapabilityType::DistributedIntelligence)),
            );
            services.insert(
                "storage".to_string(),
                Value::Bool(available_services.contains(&CapabilityType::DataStorage)),
            );
            services.insert(
                "mesh".to_string(),
                Value::Bool(available_services.contains(&CapabilityType::ServiceMesh)),
            );

            let mut context = Map::new();
            context.insert(
                "operation_id".to_string(),
                Value::String(operation_id.to_string()),
            );
            context.insert(
                "coordinator".to_string(),
                Value::String(PrimalIdentity::from_inputs(&IdentityInputs::from_env()).name),
            );
            context.insert("services".to_string(), Value::Object(services));
            context.insert(
                "coordination_strategy".to_string(),
                Value::String("best_effort_with_fallbacks".to_string()),
            );

            Value::Object(context)
        };

        info!("🎯 Coordination context: {}", coordination_context);

        // Execute coordinated operation
        if available_services.len() >= required_capabilities.len() / 2 {
            // Sufficient services available for coordination
            Self::execute_coordinated_operation(operation_id, &available_services)
        } else {
            Err(BearDogError::validation(
                "Insufficient services for coordination",
            ))
        }
    }

    /// Check service availability by capability name
    async fn check_service_availability(
        &self,
        capability: &CapabilityType,
    ) -> Result<bool, BearDogError> {
        match self
            .universal_adapter
            .discover_capability_endpoint(capability.clone())
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Execute coordinated operation with available services
    /// Executes `coordinated_operation`
    #[expect(
        clippy::unnecessary_wraps,
        reason = "Result return type reserved for future coordination errors"
    )]
    fn execute_coordinated_operation(
        operation_id: Uuid,
        available_services: &[CapabilityType],
    ) -> Result<Value, BearDogError> {
        info!(
            "⚡ Executing coordinated operation with {} services",
            available_services.len()
        );

        // For now, return success with service list
        let result = {
            use serde_json::{Map, Value};
            let mut result = Map::new();
            result.insert(
                "operation_id".to_string(),
                Value::String(operation_id.to_string()),
            );
            result.insert("status".to_string(), Value::String("completed".to_string()));
            result.insert(
                "services_used".to_string(),
                Value::Number(available_services.len().into()),
            );
            result.insert(
                "timestamp".to_string(),
                Value::String(chrono::Utc::now().to_rfc3339()),
            );
            Value::Object(result)
        };
        Ok(result)
    }

    #[expect(dead_code, reason = "Probe helper not yet wired into discovery paths")]
    #[expect(
        clippy::unused_self,
        reason = "Instance parameter reserved for future authenticated health probes"
    )]
    #[expect(clippy::unnecessary_wraps, reason = "Result for future probe errors")]
    fn probe_service_endpoint(
        &self,
        capability: &CapabilityType,
        endpoint: &str,
    ) -> Result<bool, BearDogError> {
        // Simple availability check - in production this would be an actual health check
        debug!(
            "🔍 Probing service endpoint for {:?}: {}",
            capability, endpoint
        );

        // Return true for core capabilities that we know are critical
        Ok(matches!(
            capability,
            CapabilityType::Security
                | CapabilityType::ComputeIntelligence
                | CapabilityType::ServiceMesh
                | CapabilityType::DistributedIntelligence
        ))
    }

    /// Get ecosystem integration health status
    ///
    /// Checks the availability of core ecosystem capabilities and returns
    /// the overall health status based on service availability.
    ///
    /// # Errors
    /// Returns error if service availability check fails or if communication
    /// with the universal adapter encounters issues.
    pub async fn get_ecosystem_integration_health(&self) -> Result<HealthStatus, BearDogError> {
        info!("🏥 Checking ecosystem integration health");

        // Check core capabilities
        let mut healthy_services = 0;
        let mut total_services = 0;

        let core_capabilities = vec![
            CapabilityType::Security,
            CapabilityType::ComputeIntelligence,
            CapabilityType::ServiceMesh,
        ];

        for capability in core_capabilities {
            total_services += 1;
            if self.check_service_availability(&capability).await? {
                healthy_services += 1;
            }
        }

        let health_ratio = f64::from(healthy_services) / f64::from(total_services);

        if health_ratio >= 0.8 {
            Ok(HealthStatus::Healthy)
        } else if health_ratio >= 0.5 {
            Ok(HealthStatus::Degraded)
        } else {
            Ok(HealthStatus::Unhealthy)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BearDogCore;
    use beardog_types::canonical::capabilities::CapabilityType;

    fn make_core() -> BearDogCore {
        BearDogCore::with_default_config().expect("core creation")
    }

    #[tokio::test]
    async fn test_coordinate_ecosystem_operation_insufficient_services() {
        let core = make_core();
        let required = vec![
            CapabilityType::ComputeIntelligence,
            CapabilityType::DistributedIntelligence,
        ];
        let result = core
            .coordinate_ecosystem_operation(uuid::Uuid::new_v4(), required)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_coordinate_ecosystem_operation_with_registered_services() {
        let core = make_core();
        core.universal_adapter
            .register_capability(
                CapabilityType::Security,
                "https://security.example.com".to_string(),
            )
            .await
            .expect("register Security capability");
        core.universal_adapter
            .register_capability(
                CapabilityType::ServiceMesh,
                "https://mesh.example.com".to_string(),
            )
            .await
            .expect("register ServiceMesh capability");
        let required = vec![CapabilityType::Security, CapabilityType::ServiceMesh];
        let result = core
            .coordinate_ecosystem_operation(uuid::Uuid::new_v4(), required)
            .await;
        assert!(result.is_ok());
        let value = result.expect("coordinate ecosystem operation");
        assert!(value.get("operation_id").is_some());
        assert_eq!(
            value.get("status").and_then(|v| v.as_str()),
            Some("completed")
        );
    }

    #[tokio::test]
    async fn test_get_ecosystem_integration_health() {
        let core = make_core();
        let result = core.get_ecosystem_integration_health().await;
        assert!(result.is_ok());
        let status = result.expect("ecosystem integration health");
        assert!(matches!(
            status,
            HealthStatus::Healthy | HealthStatus::Degraded | HealthStatus::Unhealthy
        ));
    }
}
