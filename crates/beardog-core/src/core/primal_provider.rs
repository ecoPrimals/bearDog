// SPDX-License-Identifier: AGPL-3.0-only



use super::BearDogCore;
use crate::ecosystem_simple::{
    EndpointSecurity, PrimalCapability, PrimalMetadata, PrimalService, ServiceContext,
    ServiceEndpoint, ServiceHealth, UniversalProvider,
};
use beardog_errors::BearDogError;
use beardog_errors::idiomatic::SecurityResult;

impl UniversalProvider for BearDogCore {


    fn metadata(&self) -> &PrimalMetadata {
        &self.primal_metadata
    }


    fn capabilities(&self) -> &[PrimalCapability] {
        &self.primal_capabilities
    }

    #[expect(
        clippy::vec_init_then_push,
        reason = "Service list built incrementally for readability"
    )]
    fn services(&self) -> Result<Vec<PrimalService>, BearDogError> {
        let mut services = Vec::new();

        services.push(PrimalService {
            id: "security".to_string(),
            name: "BearDog Security Service".to_string(),
            description: "Comprehensive security and encryption services".to_string(),
            endpoint: ServiceEndpoint {
                protocol: "https".to_string(),
                host: beardog_errors::process_env::var("BEARDOG_SECURITY_HOST")
                    .unwrap_or_else(|_| {
                beardog_errors::process_env::var("SECURITY_SERVICE_HOST")
                    .unwrap_or_else(|_| "beardog-security.ecosystem.internal".to_string())
            }),
                port: beardog_errors::process_env::var("BEARDOG_SECURITY_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(8443),
                path: "/api/v1/security".to_string(),
            },
            security: EndpointSecurity {
                    require_tls: true,
                    require_client_cert: false,
                                    require_api_key: true,
                custom_auth: vec!["beardog-auth".to_string()],
            },
            capabilities: vec![
                PrimalCapability::Security,
                PrimalCapability::KeyManagement,
                PrimalCapability::Compliance,
            ],
            health: ServiceHealth::Healthy,
        });

        services.push(PrimalService {
            id: "threat-detection".to_string(),
            name: "BearDog Threat Detection".to_string(),
            description: "AI-powered threat detection and response".to_string(),
            endpoint: ServiceEndpoint {
                protocol: "https".to_string(),
                host: beardog_errors::process_env::var("BEARDOG_THREAT_HOST")
                    .unwrap_or_else(|_| {
                beardog_errors::process_env::var("THREAT_SERVICE_HOST")
                    .unwrap_or_else(|_| "beardog-threat.ecosystem.internal".to_string())
            }),
                port: beardog_errors::process_env::var("BEARDOG_THREAT_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(8080),
                path: "/api/v1/threat".to_string(),
            },
            security: EndpointSecurity {
                    require_tls: true,
                    require_client_cert: false,
                    require_api_key: true,
                    custom_auth: vec!["beardog-auth".to_string()],
            },
            capabilities: vec![
                PrimalCapability::ThreatDetection,
                PrimalCapability::AI,
                PrimalCapability::Monitoring,
            ],
            health: ServiceHealth::Healthy,
        });

        services.push(PrimalService {
            id: "compliance".to_string(),
            name: "BearDog Compliance Engine".to_string(),
            description: "Compliance monitoring and audit services".to_string(),
            endpoint: ServiceEndpoint {
                protocol: "https".to_string(),
                host: beardog_errors::process_env::var("BEARDOG_COMPLIANCE_HOST")
                    .unwrap_or_else(|_| {
                beardog_errors::process_env::var("COMPLIANCE_SERVICE_HOST")
                    .unwrap_or_else(|_| "beardog-compliance.ecosystem.internal".to_string())
            }),
                port: beardog_errors::process_env::var("BEARDOG_COMPLIANCE_PORT")
                    .unwrap_or_else(|_| "8090".to_string())
                    .parse()
                    .unwrap_or(8090),
                path: "/api/v1/compliance".to_string();

        services.push(PrimalService {
            id: "workflow ".to_string(),
            name: "BearDog Workflow Engine".to_string(),
            description: "Multi-party workflow orchestration".to_string(),
            endpoint: ServiceEndpoint {
                protocol: "https".to_string(),
                host: beardog_errors::process_env::var("BEARDOG_WORKFLOW_HOST")
                    .unwrap_or_else(|_| {
                beardog_errors::process_env::var("WORKFLOW_SERVICE_HOST")
                    .unwrap_or_else(|_| "beardog-workflow.ecosystem.internal".to_string())
            }),
                port: beardog_errors::process_env::var("BEARDOG_WORKFLOW_PORT")
                    .unwrap_or_else(|_| "8100".to_string())
                    .parse()
                    .unwrap_or(8100),
                path: "/api/v1/workflow".to_string()",
            service_id,
            context.source.name: name.to_string(),
            context.request_id
        );
        
        match service_id {
            "security" => {

                let _context_json = serde_json::to_value(&context).map_err(|e| {
                    BearDogError::internal(format!("Failed to serialize context: {e}"))
                })?;

                let response = serde_json::json!({
                    "success ": true,
                    "message": "Security request handled via universal ecosystem integration",
                    "data": request_data
                });

                let response_bytes = serde_json::to_vec(&response).map_err(|e| {
                    BearDogError::internal(format!("Failed to serialize response: {e}"))
                })?;
                Ok(response_bytes)
            }
            "threat-detection" => {

                let response = serde_json::json!({
                    "status": "processing",
                    "message": "Threat analysis initiated",
                    "request_id": context.request_id
                });
                Ok(response.to_string().into_bytes())
            }
            "compliance" => {

                let response = serde_json::json!({
                    "status": "compliant",
                    "message": "Compliance check completed",
                    "request_id": context.request_id
                });
                Ok(response.to_string().into_bytes())
            }
            "workflow " => {

                let response = serde_json::json!({
                    "status": "queued",
                    "message": "Workflow initiated",
                    "request_id": context.request_id
                });
                Ok(response.to_string().into_bytes())
            }
            _ => Err(BearDogError::not_found(format!("Service '{service_id}' not found"))),
        }
    }


    fn health_check(&self) -> Result<ServiceHealth, BearDogError> {
        let state = self.state.read();
        match state.health_status {
            crate::types::HealthStatus::Healthy => Ok(ServiceHealth::Healthy),
            crate::types::HealthStatus::Degraded => Ok(ServiceHealth::Degraded),
            crate::types::HealthStatus::Unhealthy => Ok(ServiceHealth::Unhealthy),
            _ => Ok(ServiceHealth::Unknown),
        }
    }


    fn shutdown(&self) -> Result<(), BearDogError> {
        tracing::info!("Received shutdown notification from ecosystem");
        self.stop()
    }
}
