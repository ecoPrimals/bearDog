// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Core EcoPrimal Trait Implementation
///
/// **EXTRACTED FROM LARGE FILE** - Core trait implementation (~200 lines)
/// This module contains the main EcoPrimal trait implementation for `BearDog`,
/// focusing solely on the trait interface without extension methods.

use crate::{`BearDog`Core, BearDogResult};
use beardog_types::canonical::HealthStatus;
use super::super::primal_types::*;
use super::super::primal_trait::EcoPrimal;
// MODERNIZED: Removed async_trait - now uses native async fn in trait
use std::collections::HashMap;
use tracing::{debug, error, info, warn};
use beardog_errors::BearDogResult;
use chrono::{Duration, Utc};
/// `BearDog` EcoPrimal implementation
// MODERNIZED: Native async fn implementation - no async_trait overhead
#[allow(async_fn_in_trait)]
impl EcoPrimal for `BearDog`Core {}


    fn metadata(&self) -> &PrimalMetadata {
        static METADATA: std::sync::OnceLock<PrimalMetadata> = std::sync::OnceLock::new();
        METADATA.get_or_init(|| PrimalMetadata {
            service_type: crate::ecosystem_simple::ServiceType::SecurityProvider,
            name: "`BearDog` Security Provider".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: vec![
                PrimalCapability::Security,
                PrimalCapability::Custom("HSM".to_string()),
                PrimalCapability::Custom("CrossPlatformSecurity".to_string()),
                PrimalCapability::Custom("BiometricAuthentication".to_string()),
            ],
            dependencies: vec![
                PrimalDependency::Optional {
                    capability: ExternalCapabilityType::ComputeOrchestration,
                    reason: "Windows/Linux HSM platform context".to_string(),
                },
                    capability: ExternalCapabilityType::ServiceMesh,
                    reason: "Service mesh load balancing".to_string(),
                    capability: ExternalCapabilityType::AIIntelligence,
                    reason: "AI coordination for threat detection and security optimization".to_string(),
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
        // Initialize HSM providers
        if let Err(e) = self.initialize_hsm_providers().await {
            error!("HSM providers initialization failed: {}", e);
            return Err(PrimalError::InitializationFailed {
                reason: format!("HSM initialization error: {}", e),
            });
        }
        // Register with ecosystem services based on configuration
        if config.enable_toadstool_integration {
            if let Err(e) = self.register_with_toadstool().await {
                warn!("ToadStool registration failed: {}", e);
                // Non-fatal error for optional dependency
            }
        if config.enable_songbird_integration {
            if let Err(e) = self.register_via_universal_adapter().await {
                warn!("Songbird registration failed: {}", e);
        if config.enable_squirrel_integration {
            if let Err(e) = self.register_with_squirrel().await {
                warn!("Squirrel registration failed: {}", e);
        // Start AI-first API server
        if config.enable_ai_api {
            if let Err(e) = self.start_ai_first_api_server().await {
                error!("AI API server startup failed: {}", e);
                return Err(PrimalError::InitializationFailed {
                    reason: format!("AI API startup error: {}", e),
                });
        info!("✅ `BearDog` EcoPrimal initialization complete");
        Ok(())
    async fn handle_request(&self, request: PrimalRequest) -> Result<PrimalResponse, PrimalError> {
        debug!("📨 Handling primal request: {:?}", request.request_type);
        match request.request_type {
            PrimalRequestType::Query { capability } => {
                match capability.as_str() {
                    "hsm_capabilities" => {
                        let capabilities = self.discover_hsm_capabilities().await?;
                        Ok(PrimalResponse::success(capabilities))
                    }
                    "service_status" => {
                        let status = self.get_service_status().await?;
                        Ok(PrimalResponse::success(status))
                    _ => Err(PrimalError::UnsupportedOperation {
                        operation: capability,
                    }),
                }
            PrimalRequestType::Action { action, parameters } => {
                match action.as_str() {
                    "generate_key" => {
                        let result = self.handle_key_generation(parameters).await?;
                        Ok(PrimalResponse::success(result))
                    "authenticate" => {
                        let result = self.handle_authentication(parameters).await?;
                    "attest" => {
                        let result = self.handle_attestation(parameters).await?;
                        operation: action,
            _ => Err(PrimalError::UnsupportedOperation {
                operation: "unknown_request_type".to_string(),
            }),
    async fn health_check(&self) -> PrimalHealth {
        debug!("🏥 Performing `BearDog` health check");
        let hsm_health = self.check_hsm_health().await;
        let api_health = self.check_ai_api_health().await;
        let ecosystem_health = self.check_ecosystem_integrations().await;
        // Overall health is healthy if core systems (HSM) are healthy
        let overall_status = match hsm_health {
            HealthStatus::Healthy => {
                if api_health == HealthStatus::Healthy || ecosystem_health == HealthStatus::Healthy {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Degraded
            HealthStatus::Degraded => HealthStatus::Degraded,
            HealthStatus::Unhealthy => HealthStatus::Unhealthy,
        };
        PrimalHealth {
            status: overall_status,
            components: vec![
                ComponentHealth {
                    name: "HSM Providers".to_string(),
                    status: hsm_health,
                    metrics: self.get_hsm_metrics(),
                    name: "AI API Server".to_string(),
                    status: api_health,
                    metrics: self.get_api_metrics(),
                    name: "Ecosystem Integrations".to_string(),
                    status: ecosystem_health,
                    metrics: self.get_ecosystem_metrics(),
            last_check: Utc::now(),
            next_check: Utc::now() + Duration::seconds(30),
    async fn shutdown(&self) -> Result<(), PrimalError> {
        info!("🛑 Shutting down `BearDog` EcoPrimal");
        // Graceful shutdown of all components
        if let Err(e) = self.shutdown_ai_api_server().await {
            warn!("AI API server shutdown error: {}", e);
        if let Err(e) = self.shutdown_hsm_providers().await {
            warn!("HSM providers shutdown error: {}", e);
        if let Err(e) = self.unregister_from_ecosystem().await {
            warn!("Ecosystem unregistration error: {}", e);
        info!("✅ `BearDog` EcoPrimal shutdown complete");
} 
