

#![allow(async_fn_in_trait)]

use crate::BearDogCore;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use super::super::primal_types::*;
use super::super::primal_trait::EcoPrimal;

use std::collections::HashMap;
use tracing::{debug, error, info, warn};
use chrono::{Duration, Utc};

impl EcoPrimal for BearDogCore {

    fn metadata(&self) -> &PrimalMetadata {
        static METADATA: std::sync::OnceLock<PrimalMetadata> = std::sync::OnceLock::new();
        METADATA.get_or_init(|| PrimalMetadata {
            primal_type: PrimalType::BearDog,
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: vec![
                PrimalCapability::Security,
                PrimalCapability::Custom("HSM".to_string()),
                PrimalCapability::Custom("CrossPlatformSecurity".to_string()),
                PrimalCapability::Custom("BiometricAuthentication".to_string()),
            ],
            dependencies: vec![
                PrimalDependency::Optional {
                    primal: PrimalType::Custom("ComputeOrchestration".to_string()),
                    min_version: "1.0.0".to_string(),
                    reason: "Windows/Linux HSM platform context".to_string(),
                },
                PrimalDependency::Optional {
                    primal: PrimalType::Custom("ServiceMesh".to_string()),
                    min_version: "1.0.0".to_string(),
                    reason: "Service mesh load balancing".to_string(),
                },
                PrimalDependency::Optional {
                    primal: PrimalType::Custom("AIIntelligence".to_string()),
                    min_version: "1.0.0".to_string(),
                    reason: "AI coordination for threat detection and security optimization".to_string(),
                },
            ],
        })
    }
    fn capabilities(&self) -> Vec<PrimalCapability> {
        vec![
            PrimalCapability::Security,
            PrimalCapability::Custom("HSM".to_string()),
            PrimalCapability::Custom("CrossPlatformSecurity".to_string()),
        ]}

    async fn initialize(&self, config: &PrimalIntegrationConfig) -> Result<(), PrimalError> {
        info!("🚀 Initializing `BearDog` EcoPrimal with integration config");
        debug!("Integration config: {:?}", config);

        if let Err(e) = self.initialize_hsm_providers().await {
            error!("HSM providers initialization failed: {}", e);
            return Err(PrimalError::InitializationFailed {
                reason: format_args!("HSM initialization error: {}", e).to_string(),
            });
        }

        if config.enable_toadstool_integration {
            if let Err(e) = self.register_with_toadstool().await {
                warn!("ToadStool registration failed: {}", e);
            }
        }
        
        if config.enable_songbird_integration {
            if let Err(e) = self.register_via_universal_adapter().await {
                warn!("Songbird registration failed: {}", e);
            }
        }
        
        if config.enable_squirrel_integration {
            if let Err(e) = self.register_with_squirrel().await {
                warn!("Squirrel registration failed: {}", e);
            }
        }

        if config.enable_ai_api {
            if let Err(e) = self.start_ai_first_api_server().await {
                error!("AI API server startup failed: {}", e);
                return Err(PrimalError::InitializationFailed {
                    reason: format_args!("AI API startup error: {}", e).to_string(),
                });
            }
        }

        info!("✅ `BearDog` EcoPrimal initialization complete");
        Ok(())
    }

    async fn handle_request(&self, request: PrimalRequest) -> Result<PrimalResponse, PrimalError> {
        debug!("📨 Handling primal request: {:?}", request.id);
        
        // For now, return a simple success response
        Ok(PrimalResponse {
            id: request.id.clone(),
            success: true,
            data: Some(serde_json::json!({
                "message": "Request handled successfully",
                "timestamp": chrono::Utc::now()
            })),
            error: None,
            metadata: ahash::HashMap::default(),
            timestamp: chrono::Utc::now(),
        })
    }

    async fn health_check(&self) -> Result<PrimalHealth, PrimalError> {
        Ok(PrimalHealth {
            status: HealthStatus::Healthy,
            components: ahash::HashMap::default(),
            last_check: chrono::Utc::now(),
            next_check: chrono::Utc::now() + chrono::Duration::seconds(30),
        })
    }
    async fn shutdown(&self) -> Result<(), PrimalError> {
        info!("🛑 Shutting down `BearDog` EcoPrimal");

        if let Err(e) = self.shutdown_ai_api_server().await {
            warn!("AI API server shutdown error: {}", e);
        }
        
        if let Err(e) = self.shutdown_hsm_providers().await {
            warn!("HSM providers shutdown error: {}", e);
        }
        
        if let Err(e) = self.unregister_from_ecosystem().await {
            warn!("Ecosystem unregistration error: {}", e);
        }
        
        info!("✅ `BearDog` EcoPrimal shutdown complete");
        Ok(())
    }
} 
