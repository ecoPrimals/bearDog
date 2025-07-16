//! PrimalProvider trait implementation for BearDog
//!
//! This module contains the main PrimalProvider trait implementation
//! and the primary request handling logic.

use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::json;
use tracing::{info, warn};
use uuid::Uuid;

use super::super::traits::*;
use super::super::{ecosystem_ids, request_types};
use super::core::BearDogPrimalProvider;
use crate::BearDogResult;

#[async_trait]
impl PrimalProvider for BearDogPrimalProvider {
    fn ecosystem_id(&self) -> &str {
        ecosystem_ids::BEARDOG
    }

    fn instance_id(&self) -> &str {
        &self.instance_id
    }

    fn service_name(&self) -> &str {
        "BearDog Security Provider"
    }

    fn service_version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    fn capabilities(&self) -> Vec<Capability> {
        self.get_security_capabilities()
    }

    fn dependencies(&self) -> Vec<Dependency> {
        self.get_dependencies()
    }

    fn endpoints(&self) -> ServiceEndpoints {
        self.endpoints.clone()
    }

    async fn health_check(&self) -> HealthStatus {
        // Use actual BearDog health check
        let health_result = self.core.health_check().await.unwrap_or_else(|_| {
            crate::core::HealthCheck {
                status: crate::core::HealthStatus::Unhealthy,
                uptime: None,
                components: vec![],
                metrics: crate::core::SystemMetrics {
                    cpu_usage_percent: 0.0,
                    memory_usage_bytes: 0,
                    active_connections: 0,
                    requests_per_second: 0.0,
                    avg_response_time_ms: 0.0,
                    error_rate_percent: 0.0,
                },
                timestamp: chrono::Utc::now(),
            }
        });
        
        // Convert core::HealthStatus to traits::HealthStatus
        match health_result.status {
            crate::core::HealthStatus::Healthy => HealthStatus::Healthy,
            crate::core::HealthStatus::Degraded => HealthStatus::Degraded {
                issues: vec!["System degraded".to_string()],
                impact: crate::adapters::universal::traits::HealthImpact::Medium,
            },
            crate::core::HealthStatus::Unhealthy => HealthStatus::Unhealthy {
                reason: "System unhealthy".to_string(),
                recovery_time: None,
            },
            crate::core::HealthStatus::Starting => HealthStatus::Starting,
            crate::core::HealthStatus::Stopping => HealthStatus::Stopping,
        }
    }

    async fn handle_request(&self, request: ServiceRequest) -> BearDogResult<ServiceResponse> {
        info!(
            "Handling request: {} ({})",
            request.request_type, request.request_id
        );

        match request.request_type.as_str() {
            request_types::HEALTH_CHECK => Ok(ServiceResponse {
                request_id: request.request_id,
                success: true,
                payload: json!({
                    "status": "healthy",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }),
                timestamp: chrono::Utc::now(),
                metadata: HashMap::new(),
                error: None,
            }),
            request_types::STATUS_GET => Ok(ServiceResponse {
                request_id: request.request_id,
                success: true,
                payload: json!({
                    "ecosystem_id": self.ecosystem_id(),
                    "instance_id": self.instance_id(),
                    "service_name": self.service_name(),
                    "version": self.service_version(),
                    "status": "running",
                    "capabilities": self.capabilities().len(),
                    "dependencies": self.dependencies().len()
                }),
                timestamp: chrono::Utc::now(),
                metadata: HashMap::new(),
                error: None,
            }),
            _ if request.request_type.starts_with("security.") => {
                self.handle_security_request(&request).await
            }
            _ => {
                warn!("Unsupported request type: {}", request.request_type);
                Ok(ServiceResponse {
                    request_id: request.request_id,
                    success: false,
                    payload: json!({}),
                    timestamp: chrono::Utc::now(),
                    metadata: HashMap::new(),
                    error: Some(ServiceError {
                        code: "UNSUPPORTED_REQUEST".to_string(),
                        message: format!("Unsupported request type: {}", request.request_type),
                        details: None,
                        retryable: false,
                    }),
                })
            }
        }
    }

    async fn register_with_ecosystem(&self) -> BearDogResult<EcosystemRegistration> {
        info!("Registering BearDog with ecosystem");

        // TODO: Implement actual registration with SongBird
        // For now, return a mock registration
        Ok(EcosystemRegistration {
            registration_id: Uuid::new_v4(),
            ecosystem_id: self.ecosystem_id().to_string(),
            instance_id: self.instance_id().to_string(),
            endpoints: self.endpoints(),
            capabilities: self.capabilities(),
            registration_time: chrono::Utc::now(),
            status: RegistrationStatus::Active,
        })
    }

    async fn initialize(&mut self, _config: ProviderConfig) -> BearDogResult<()> {
        info!("Initializing BearDog PrimalProvider");

        // TODO: Implement initialization logic
        // - Initialize BearDog core
        // - Set up security configurations
        // - Initialize capability handlers

        Ok(())
    }

    async fn shutdown(&mut self) -> BearDogResult<()> {
        info!("Shutting down BearDog PrimalProvider");

        // TODO: Implement shutdown logic
        // - Graceful shutdown of BearDog core
        // - Clean up resources
        // - Save state if needed

        Ok(())
    }

    fn can_handle_request(&self, request: &ServiceRequest) -> bool {
        // BearDog can handle all security-related requests and basic service requests
        matches!(
            request.request_type.as_str(),
            request_types::HEALTH_CHECK
                | request_types::STATUS_GET
                | request_types::SECURITY_ENCRYPT
                | request_types::SECURITY_DECRYPT
                | request_types::SECURITY_AUTHENTICATE
                | request_types::SECURITY_AUTHORIZE
        )
    }

    fn metadata(&self) -> ProviderMetadata {
        self.metadata.clone()
    }
} 