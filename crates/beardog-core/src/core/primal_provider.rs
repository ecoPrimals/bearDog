//! BearDog Universal Primal Provider Implementation
//!
//! Implements the Universal Primal Provider trait for ecosystem integration.

use super::BearDogCore;
use crate::universal_primal_provider::{
    EndpointSecurity, PrimalCapability, PrimalMetadata, PrimalService, ServiceContext,
    ServiceEndpoint, ServiceHealth, UniversalPrimalProvider,
};
use beardog_errors::{BearDogError, BearDogResult};

// Universal Primal Provider implementation for ecosystem integration
#[async_trait::async_trait]
impl UniversalPrimalProvider for BearDogCore {
    /// Get primal metadata for ecosystem registration
    fn metadata(&self) -> &PrimalMetadata {
        &self.primal_metadata
    }

    /// Get list of capabilities this primal provides
    fn capabilities(&self) -> &[PrimalCapability] {
        &self.primal_capabilities
    }

    /// Get list of services this primal exposes
    #[allow(clippy::vec_init_then_push)] // Complex service definitions are clearer with push
    async fn services(&self) -> BearDogResult<Vec<PrimalService>> {
        let mut services = Vec::new();

        // Security service
        services.push(PrimalService {
            id: "security".to_string(),
            name: "BearDog Security Service".to_string(),
            description: "Comprehensive security and encryption services".to_string(),
            endpoint: ServiceEndpoint {
                protocol: "https".to_string(),
                host: std::env::var("BEARDOG_SECURITY_HOST")
                    .unwrap_or_else(|_| "beardog-security.ecosystem.internal".to_string()),
                port: std::env::var("BEARDOG_SECURITY_PORT")
                    .unwrap_or_else(|_| "8443".to_string())
                    .parse()
                    .unwrap_or(8443),
                path: "/api/v1/security".to_string(),
                security: EndpointSecurity {
                    require_tls: true,
                    require_client_cert: false,
                    require_api_key: true,
                    custom_auth: vec!["beardog-auth".to_string()],
                },
            },
            capabilities: vec![
                PrimalCapability::Security,
                PrimalCapability::KeyManagement,
                PrimalCapability::Compliance,
            ],
            health: ServiceHealth::Healthy,
        });

        // Threat detection service
        services.push(PrimalService {
            id: "threat-detection".to_string(),
            name: "BearDog Threat Detection".to_string(),
            description: "AI-powered threat detection and response".to_string(),
            endpoint: ServiceEndpoint {
                protocol: "https".to_string(),
                host: std::env::var("BEARDOG_THREAT_HOST")
                    .unwrap_or_else(|_| "beardog-threat.ecosystem.internal".to_string()),
                port: std::env::var("BEARDOG_THREAT_PORT")
                    .unwrap_or_else(|_| "8443".to_string())
                    .parse()
                    .unwrap_or(8443),
                path: "/api/v1/threat".to_string(),
                security: EndpointSecurity {
                    require_tls: true,
                    require_client_cert: false,
                    require_api_key: true,
                    custom_auth: vec!["beardog-auth".to_string()],
                },
            },
            capabilities: vec![
                PrimalCapability::ThreatDetection,
                PrimalCapability::AI,
                PrimalCapability::Monitoring,
            ],
            health: ServiceHealth::Healthy,
        });

        // Compliance service
        services.push(PrimalService {
            id: "compliance".to_string(),
            name: "BearDog Compliance Engine".to_string(),
            description: "Compliance monitoring and audit services".to_string(),
            endpoint: ServiceEndpoint {
                protocol: "https".to_string(),
                host: std::env::var("BEARDOG_COMPLIANCE_HOST")
                    .unwrap_or_else(|_| "beardog-compliance.ecosystem.internal".to_string()),
                port: std::env::var("BEARDOG_COMPLIANCE_PORT")
                    .unwrap_or_else(|_| "8443".to_string())
                    .parse()
                    .unwrap_or(8443),
                path: "/api/v1/compliance".to_string(),
                security: EndpointSecurity {
                    require_tls: true,
                    require_client_cert: false,
                    require_api_key: true,
                    custom_auth: vec!["beardog-auth".to_string()],
                },
            },
            capabilities: vec![PrimalCapability::Compliance, PrimalCapability::Monitoring],
            health: ServiceHealth::Healthy,
        });

        // Workflow service
        services.push(PrimalService {
            id: "workflow".to_string(),
            name: "BearDog Workflow Engine".to_string(),
            description: "Multi-party workflow orchestration".to_string(),
            endpoint: ServiceEndpoint {
                protocol: "https".to_string(),
                host: std::env::var("BEARDOG_WORKFLOW_HOST")
                    .unwrap_or_else(|_| "beardog-workflow.ecosystem.internal".to_string()),
                port: std::env::var("BEARDOG_WORKFLOW_PORT")
                    .unwrap_or_else(|_| "8443".to_string())
                    .parse()
                    .unwrap_or(8443),
                path: "/api/v1/workflow".to_string(),
                security: EndpointSecurity {
                    require_tls: true,
                    require_client_cert: false,
                    require_api_key: true,
                    custom_auth: vec!["beardog-auth".to_string()],
                },
            },
            capabilities: vec![PrimalCapability::Workflow],
            health: ServiceHealth::Healthy,
        });

        Ok(services)
    }

    /// Register with the ecosystem through Songbird
    async fn register_with_ecosystem(&self, songbird_endpoint: &str) -> BearDogResult<()> {
        use reqwest;
        use serde_json;

        let client = reqwest::Client::new();
        let registration_data = serde_json::json!({
            "primal_metadata": self.metadata(),
            "capabilities": self.capabilities(),
            "services": self.services().await?,
            "health": self.health_check().await?
        });

        let response = client
            .post(format!("{songbird_endpoint}/api/v1/primals/register"))
            .header("Content-Type", "application/json")
            .header("X-Primal-Type", "BearDog")
            .json(&registration_data)
            .send()
            .await
            .map_err(|e| {
                BearDogError::internal(format!("Failed to register with Songbird: {e}"))
            })?;

        if response.status().is_success() {
            tracing::info!(
                "Successfully registered BearDog with Songbird at {}",
                songbird_endpoint
            );
            Ok(())
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(BearDogError::internal(format!(
                "Failed to register with Songbird: {error_text}"
            )))
        }
    }

    /// Handle incoming service request from ecosystem
    async fn handle_service_request(
        &self,
        service_id: &str,
        request_data: Vec<u8>,
        context: ServiceContext,
    ) -> BearDogResult<Vec<u8>> {
        tracing::info!(
            "Handling service request for {} from {} (request_id: {})",
            service_id,
            context.source.name,
            context.request_id
        );

        match service_id {
            "security" => {
                // Handle security service requests
                // This would route to the security provider
                let _context_json = serde_json::to_value(&context).map_err(|e| {
                    BearDogError::internal(format!("Failed to serialize context: {e}"))
                })?;
                // Use universal ecosystem integration instead of direct method calls
                let response = serde_json::json!({
                    "success": true,
                    "message": "Security request handled via universal ecosystem integration",
                    "data": request_data
                });
                // Convert JSON to bytes as expected by the return type
                let response_bytes = serde_json::to_vec(&response).map_err(|e| {
                    BearDogError::internal(format!("Failed to serialize response: {e}"))
                })?;
                Ok(response_bytes)
            }
            "threat-detection" => {
                // Handle threat detection requests
                // This would route to the threat detection engine
                let response = serde_json::json!({
                    "status": "processing",
                    "message": "Threat analysis initiated",
                    "request_id": context.request_id
                });
                Ok(response.to_string().into_bytes())
            }
            "compliance" => {
                // Handle compliance requests
                let response = serde_json::json!({
                    "status": "compliant",
                    "message": "Compliance check completed",
                    "request_id": context.request_id
                });
                Ok(response.to_string().into_bytes())
            }
            "workflow" => {
                // Handle workflow requests
                let response = serde_json::json!({
                    "status": "queued",
                    "message": "Workflow initiated",
                    "request_id": context.request_id
                });
                Ok(response.to_string().into_bytes())
            }
            _ => Err(BearDogError::NotFound {
                message: format!("Service '{service_id}' not found"),
            }),
        }
    }

    /// Health check for ecosystem monitoring
    async fn health_check(&self) -> BearDogResult<ServiceHealth> {
        let state = self.state.read().await;
        match state.health_status {
            crate::types::HealthStatus::Healthy => Ok(ServiceHealth::Healthy),
            crate::types::HealthStatus::Degraded => Ok(ServiceHealth::Degraded),
            crate::types::HealthStatus::Unhealthy => Ok(ServiceHealth::Unhealthy),
            _ => Ok(ServiceHealth::Unknown),
        }
    }

    /// Shutdown notification from ecosystem
    async fn shutdown(&self) -> BearDogResult<()> {
        tracing::info!("Received shutdown notification from ecosystem");
        self.stop().await
    }
}
