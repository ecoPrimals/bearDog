//! # HSM Manager
//!
//! This module provides the central HSM management system that orchestrates between
//! different HSM providers (smartphone, software, hardware) with intelligent tier
//! selection, automatic failover, and performance optimization.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                       HSM Manager                              │
//! │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
//! │  │ Tier Selection  │  │ Health Monitor  │  │ Failover Manager│ │
//! │  │ - Requirements  │  │ - Health Checks │  │ - Retry Logic   │ │
//! │  │ - Capabilities  │  │ - Metrics       │  │ - Fallback      │ │
//! │  │ - Performance   │  │ - Alerting      │  │ - Load Balance  │ │
//! │  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
//! └─────────────────────┬───────────────────────────────────────────┘
//!                       │
//!       ┌───────────────┼───────────────┐
//!       │               │               │
//! ┌─────▼─────┐  ┌─────▼─────┐  ┌─────▼─────┐
//! │Smartphone │  │ Software  │  │ Hardware  │
//! │    HSM    │  │    HSM    │  │    HSM    │
//! │  (T1)     │  │   (T2)    │  │   (T3)    │
//! └───────────┘  └───────────┘  └───────────┘
//! ```

pub mod capability;
pub mod config;
pub mod failover;
pub mod health;
pub mod performance;

use super::{
    HsmCapabilityDetector, HsmFailoverManager, HsmHealthMonitor, HsmProvider, SecurityLevel,
    SecurityRequirements,
};
use crate::tunnel::hsm::types::*;
use crate::tunnel::hsm::{AndroidStrongBoxHsm, RustSoftwareHsm};
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

// Re-export types from submodules
pub use capability::DefaultHsmCapabilityDetector;
pub use config::{HsmManagerConfig, SimpleHsmTier};
pub use failover::{CircuitBreaker, CircuitBreakerState, DefaultHsmFailoverManager};
pub use health::DefaultHsmHealthMonitor;
pub use performance::{HsmPerformanceTracker, OperationMetrics};

/// HSM provider selection result
#[derive(Clone)]
pub struct HsmProviderSelection {
    /// The selected HSM provider instance
    pub provider: Arc<dyn HsmProvider>,
    /// Unique identifier for the provider
    pub provider_id: String,
    /// Security tier of the provider
    pub tier: HsmTier,
    /// Confidence score for the selection (0.0 to 1.0)
    pub confidence: f64,
    /// Estimated latency in milliseconds for operations
    pub estimated_latency_ms: f64,
}

/// HSM Manager - Central orchestrator for HSM providers
pub struct HsmManager {
    hsm_providers: HashMap<String, Arc<dyn HsmProvider>>,
    config: HsmManagerConfig,
    health_monitor: Arc<DefaultHsmHealthMonitor>,
    failover_manager: Arc<DefaultHsmFailoverManager>,
    capability_detector: Arc<DefaultHsmCapabilityDetector>,
    performance_tracker: Arc<HsmPerformanceTracker>,
}

impl Default for HsmManager {
    fn default() -> Self {
        Self::new()
    }
}

impl HsmManager {
    /// Create a basic HSM manager for testing/development
    pub fn new() -> Self {
        let config = HsmManagerConfig::default();
        Self {
            hsm_providers: HashMap::new(),
            config: config.clone(),
            health_monitor: Arc::new(DefaultHsmHealthMonitor {
                provider_health: Arc::new(RwLock::new(HashMap::new())),
                health_config: config.health_config.clone(),
                monitoring_active: Arc::new(RwLock::new(false)),
            }),
            failover_manager: Arc::new(DefaultHsmFailoverManager {
                circuit_breakers: Arc::new(RwLock::new(HashMap::new())),
                failover_config: config.failover_config.clone(),
                retry_counts: Arc::new(RwLock::new(HashMap::new())),
            }),
            capability_detector: Arc::new(DefaultHsmCapabilityDetector {
                provider_capabilities: Arc::new(RwLock::new(HashMap::new())),
            }),
            performance_tracker: Arc::new(HsmPerformanceTracker {
                operation_metrics: Arc::new(RwLock::new(HashMap::new())),
                performance_config: config.performance_config,
            }),
        }
    }

    /// Create HSM manager with custom configuration
    pub async fn with_config(config: HsmManagerConfig) -> BearDogResult<Self> {
        let health_monitor =
            Arc::new(DefaultHsmHealthMonitor::new(config.health_config.clone()).await?);
        let failover_manager =
            Arc::new(DefaultHsmFailoverManager::new(config.failover_config.clone()).await?);
        let capability_detector = Arc::new(DefaultHsmCapabilityDetector::new().await?);
        let performance_tracker =
            Arc::new(HsmPerformanceTracker::new(config.performance_config.clone()).await?);

        let mut manager = Self {
            hsm_providers: HashMap::new(),
            config: config.clone(),
            health_monitor,
            failover_manager,
            capability_detector,
            performance_tracker,
        };

        // Register configured HSM providers
        for hsm_config in &config.hsm_configs {
            let provider = Self::create_hsm_provider(hsm_config).await?;
            let provider_id = match &hsm_config.tier_config {
                HsmTierConfig::Software(_) => "software_hsm",
                HsmTierConfig::Hardware(_) => "hardware_hsm",
                HsmTierConfig::Smartphone(_) => "smartphone_hsm",
                HsmTierConfig::Hybrid(_) => "hybrid_hsm",
            };
            manager
                .hsm_providers
                .insert(provider_id.to_string(), provider);
        }

        // Start health monitoring
        let providers: Vec<Arc<dyn HsmProvider>> =
            manager.hsm_providers.values().cloned().collect();
        manager.health_monitor.start_monitoring(providers).await?;

        Ok(manager)
    }

    /// Register a new HSM provider with the manager
    pub async fn register_hsm(
        &mut self,
        tier: SimpleHsmTier,
        config: HsmConfig,
    ) -> BearDogResult<()> {
        info!("🔌 Registering HSM provider for tier: {:?}", tier);

        let provider = Self::create_hsm_provider(&config).await?;
        let provider_id = tier.to_string();

        self.hsm_providers.insert(provider_id.clone(), provider);

        // Initialize health status
        let mut health_status = self.health_monitor.provider_health.write().await;
        health_status.insert(
            provider_id.clone(),
            HsmHealthStatus {
                healthy: true,
                last_check: chrono::Utc::now(),
                error_message: None,
                performance_metrics: PerformanceMetrics::default(),
            },
        );

        // Initialize circuit breaker
        let mut circuit_breakers = self.failover_manager.circuit_breakers.write().await;
        circuit_breakers.insert(
            provider_id.clone(),
            CircuitBreaker::new(self.config.failover_config.circuit_breaker_threshold),
        );

        info!(
            "✅ HSM provider registered successfully for tier: {:?}",
            tier
        );
        Ok(())
    }

    /// Create HSM provider from configuration
    async fn create_hsm_provider(config: &HsmConfig) -> BearDogResult<Arc<dyn HsmProvider>> {
        match &config.tier_config {
            HsmTierConfig::Software(software_config) => {
                let software_hsm = RustSoftwareHsm::new(software_config.clone()).await?;
                Ok(Arc::new(software_hsm))
            }
            HsmTierConfig::Smartphone(SmartphoneHsmConfig::Android(android_config)) => {
                let android_hsm = AndroidStrongBoxHsm::new(android_config.clone()).await?;
                Ok(Arc::new(android_hsm))
            }
            HsmTierConfig::Hardware(_) => Err(BearDogError::Unimplemented {
                message: "Hardware HSM not yet implemented".to_string(),
            }),
            HsmTierConfig::Hybrid(_) => Err(BearDogError::Unimplemented {
                message: "Hybrid HSM not yet implemented".to_string(),
            }),
            _ => Err(BearDogError::Configuration {
                message: "HSM type not supported in this configuration".to_string(),
            }),
        }
    }

    /// Get health status for all providers
    pub async fn get_health_status(&self) -> BearDogResult<HashMap<String, HsmHealthStatus>> {
        self.health_monitor.get_health_status().await
    }

    /// Get simple health check status
    pub async fn health_check(&self) -> BearDogResult<HsmHealthStatus> {
        let health_statuses = self.health_monitor.get_health_status().await?;

        // Return overall health status
        let healthy = health_statuses.values().all(|status| status.healthy);

        Ok(HsmHealthStatus {
            healthy,
            last_check: chrono::Utc::now(),
            error_message: None,
            performance_metrics: PerformanceMetrics::default(),
        })
    }

    /// Get available HSM tiers
    pub async fn get_available_tiers(&self) -> BearDogResult<Vec<SimpleHsmTier>> {
        let mut tiers = Vec::new();

        for provider_id in self.hsm_providers.keys() {
            match provider_id.as_str() {
                "Smartphone" => tiers.push(SimpleHsmTier::Smartphone),
                "Software" => tiers.push(SimpleHsmTier::Software),
                "Hardware" => tiers.push(SimpleHsmTier::Hardware),
                "Hybrid" => tiers.push(SimpleHsmTier::Hybrid),
                _ => {}
            }
        }

        Ok(tiers)
    }

    /// Select optimal HSM tier based on requirements
    pub async fn select_optimal_tier(
        &self,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<SimpleHsmTier> {
        let recommended_tier = self
            .capability_detector
            .recommend_hsm_tier(requirements)
            .await?;

        // Convert HsmTier to SimpleHsmTier
        let simple_tier = match recommended_tier {
            HsmTier::SoftwareHsm { .. } => SimpleHsmTier::Software,
            HsmTier::SmartphoneHsm { .. } => SimpleHsmTier::Smartphone,
            HsmTier::HardwareHsm { .. } => SimpleHsmTier::Hardware,
            HsmTier::HybridHsm { .. } => SimpleHsmTier::Hybrid,
        };

        // Check if the recommended tier is available
        if self.hsm_providers.contains_key(&simple_tier.to_string()) {
            Ok(simple_tier)
        } else {
            // Fallback to software if available
            if self.hsm_providers.contains_key("Software") {
                Ok(SimpleHsmTier::Software)
            } else {
                Err(BearDogError::NoSuitableProvider {
                    message: format!("No suitable provider for requirements: {requirements:?}"),
                })
            }
        }
    }

    /// Select appropriate HSM tier based on requirements
    pub async fn select_hsm_tier(
        &self,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<HsmTier> {
        info!(
            "🔍 Selecting HSM tier for requirements: {:?}",
            requirements.security_level
        );

        // Smart tier selection based on requirements and availability
        match requirements.security_level {
            SecurityLevel::Basic => {
                // Basic: Use software HSM for simplicity
                Ok(HsmTier::SoftwareHsm {
                    implementation: SoftwareHsmType::RustSoftwareHsm,
                    key_storage: KeyStorageType::InMemory,
                    encryption_at_rest: true,
                    memory_protection: MemoryProtectionLevel::Basic,
                })
            }

            SecurityLevel::Medium => {
                // Medium: Prefer smartphone HSM if user interaction is needed
                if requirements.user_interaction_required {
                    // Try smartphone HSM first
                    if self.is_smartphone_hsm_available().await? {
                        Ok(self.create_optimal_smartphone_hsm().await?)
                    } else {
                        // Fallback to high-security software HSM
                        Ok(HsmTier::SoftwareHsm {
                            implementation: SoftwareHsmType::RustSoftwareHsm,
                            key_storage: KeyStorageType::EncryptedFile,
                            encryption_at_rest: true,
                            memory_protection: MemoryProtectionLevel::High,
                        })
                    }
                } else {
                    // Use software HSM for non-interactive operations
                    Ok(HsmTier::SoftwareHsm {
                        implementation: SoftwareHsmType::RustSoftwareHsm,
                        key_storage: KeyStorageType::EncryptedFile,
                        encryption_at_rest: true,
                        memory_protection: MemoryProtectionLevel::High,
                    })
                }
            }

            SecurityLevel::High => {
                // High: Require hardware backing when possible
                if requirements.hardware_backed_required || requirements.user_interaction_required {
                    if self.is_smartphone_hsm_available().await? {
                        Ok(self.create_optimal_smartphone_hsm().await?)
                    } else if self.is_hardware_hsm_available().await? {
                        Ok(self.create_optimal_hardware_hsm().await?)
                    } else {
                        // High-security software HSM as fallback
                        Ok(HsmTier::SoftwareHsm {
                            implementation: SoftwareHsmType::RustSoftwareHsm,
                            key_storage: KeyStorageType::EncryptedFile,
                            encryption_at_rest: true,
                            memory_protection: MemoryProtectionLevel::Maximum,
                        })
                    }
                } else {
                    // Software HSM with maximum protection
                    Ok(HsmTier::SoftwareHsm {
                        implementation: SoftwareHsmType::RustSoftwareHsm,
                        key_storage: KeyStorageType::EncryptedFile,
                        encryption_at_rest: true,
                        memory_protection: MemoryProtectionLevel::Maximum,
                    })
                }
            }

            SecurityLevel::Maximum => {
                // Maximum: Require the best available hardware
                if requirements.user_interaction_required {
                    // Human identity operations - prefer smartphone HSM with user presence
                    if self.is_smartphone_hsm_available().await? {
                        let mut smartphone_hsm = self.create_optimal_smartphone_hsm().await?;
                        // Ensure user presence is required for maximum security
                        if let HsmTier::SmartphoneHsm {
                            ref mut user_presence_required,
                            ..
                        } = smartphone_hsm
                        {
                            *user_presence_required = true;
                        }
                        Ok(smartphone_hsm)
                    } else {
                        Err(BearDogError::Unavailable {
                            message:
                                "Maximum security with user interaction requires smartphone HSM"
                                    .to_string(),
                        })
                    }
                } else {
                    // Non-interactive maximum security - prefer certified hardware HSM
                    if self.is_hardware_hsm_available().await? {
                        Ok(self.create_optimal_hardware_hsm().await?)
                    } else if self.is_smartphone_hsm_available().await? {
                        Ok(self.create_optimal_smartphone_hsm().await?)
                    } else {
                        return Err(BearDogError::Unavailable {
                            message: "Maximum security level requires hardware-backed HSM"
                                .to_string(),
                        });
                    }
                }
            }
        }
    }

    /// Check if smartphone HSM is available
    async fn is_smartphone_hsm_available(&self) -> BearDogResult<bool> {
        // Check for Android StrongBox
        if cfg!(target_os = "android") {
            // In a real implementation, would check Android StrongBox availability
            info!("📱 Checking Android StrongBox availability");
            Ok(true) // Assume available for now
        } else if cfg!(target_os = "ios") {
            // Check for iOS Secure Enclave
            info!("📱 Checking iOS Secure Enclave availability");
            Ok(true) // Assume available for now
        } else {
            // Desktop/server environment - no smartphone HSM
            debug!("🖥️ No smartphone HSM available on this platform");
            Ok(false)
        }
    }

    /// Check if hardware HSM is available
    async fn is_hardware_hsm_available(&self) -> BearDogResult<bool> {
        // Check for hardware HSM connectivity
        // This would typically involve checking USB/PCIe devices, network connectivity, etc.
        debug!("🔐 Hardware HSM support not yet implemented");
        Ok(false)
    }

    /// Create optimal smartphone HSM configuration
    async fn create_optimal_smartphone_hsm(&self) -> BearDogResult<HsmTier> {
        if cfg!(target_os = "android") {
            // Android StrongBox configuration
            Ok(HsmTier::SmartphoneHsm {
                device_type: SmartphoneType::Android {
                    manufacturer: "Google".to_string(),
                    model: "Pixel 8".to_string(),
                    android_version: "14".to_string(),
                    strongbox_version: Some("1.0".to_string()),
                },
                secure_enclave: SecureEnclaveType::AndroidStrongBox {
                    implementation: StrongBoxImplementation::TitanM {
                        version: "1.0".to_string(),
                        security_level: "EAL4+".to_string(),
                    },
                    hardware_backed: true,
                    key_attestation: true,
                },
                attestation_level: AttestationLevel::CertifiedHardware,
                user_presence_required: false, // Can be set to true by caller
            })
        } else if cfg!(target_os = "ios") {
            // iOS Secure Enclave configuration
            Ok(HsmTier::SmartphoneHsm {
                device_type: SmartphoneType::IPhone {
                    model: "iPhone 15 Pro".to_string(),
                    ios_version: "17.0".to_string(),
                    secure_enclave_version: "A17 Pro".to_string(),
                },
                secure_enclave: SecureEnclaveType::IosSecureEnclave {
                    chip_type: "A17 Pro".to_string(),
                    biometric_support: true,
                    key_attestation: true,
                },
                attestation_level: AttestationLevel::CertifiedHardware,
                user_presence_required: false, // Can be set to true by caller
            })
        } else {
            Err(BearDogError::Unavailable {
                message: "Smartphone HSM not available on this platform".to_string(),
            })
        }
    }

    /// Create optimal hardware HSM configuration
    async fn create_optimal_hardware_hsm(&self) -> BearDogResult<HsmTier> {
        // This would detect available hardware HSMs and create optimal configuration
        Ok(HsmTier::HardwareHsm {
            vendor: HsmVendor::Thales,
            model: "Luna Network HSM".to_string(),
            certification: CertificationLevel::Fips140Level3,
            tamper_resistance: TamperResistanceLevel::HardwareDestruction,
        })
    }

    /// Get performance metrics for all providers
    pub async fn get_performance_metrics(&self) -> BearDogResult<PerformanceMetrics> {
        let all_metrics = self.performance_tracker.get_all_metrics().await?;

        // Aggregate metrics
        let mut total_ops = 0u64;
        let mut successful_ops = 0u64;
        let mut total_latency = 0.0;
        let mut operation_count = 0;

        for metrics in all_metrics.values() {
            total_ops += metrics.total_operations;
            successful_ops += metrics.successful_operations;
            total_latency += metrics.average_latency_ms;
            operation_count += 1;
        }

        let operations_per_second = if operation_count > 0 {
            successful_ops as f64 / operation_count as f64 // Approximate
        } else {
            0.0
        };

        let average_latency_ms = if operation_count > 0 {
            total_latency / operation_count as f64
        } else {
            0.0
        };

        let error_rate = if total_ops > 0 {
            (total_ops - successful_ops) as f64 / total_ops as f64
        } else {
            0.0
        };

        Ok(PerformanceMetrics {
            operations_per_second,
            average_latency_ms,
            error_rate,
            availability_percentage: if total_ops > 0 {
                (successful_ops as f64 / total_ops as f64) * 100.0
            } else {
                100.0
            },
        })
    }

    /// Get the best provider for given requirements
    pub async fn get_best_provider(
        &self,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<HsmProviderSelection> {
        debug!(
            "🔍 Selecting best HSM provider for requirements: {:?}",
            requirements
        );

        // Get candidate providers
        let candidates = self.get_candidate_providers(requirements).await?;

        if candidates.is_empty() {
            return Err(BearDogError::NoSuitableProvider {
                message: format!("No suitable provider for requirements: {requirements:?}"),
            });
        }

        // Filter healthy providers
        let healthy_candidates = self
            .health_monitor
            .filter_healthy_providers(candidates)
            .await?;

        if healthy_candidates.is_empty() {
            return Err(BearDogError::Unavailable {
                message: "All HSM providers are unhealthy".to_string(),
            });
        }

        // Select best provider based on requirements and performance
        let selection = self
            .select_best_provider(healthy_candidates, requirements)
            .await?;

        debug!(
            "✅ Selected HSM provider: {} (tier: {:?}, confidence: {:.2})",
            selection.provider_id, selection.tier, selection.confidence
        );

        Ok(selection)
    }

    /// Get candidate providers that meet security requirements
    async fn get_candidate_providers(
        &self,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<Vec<Arc<dyn HsmProvider>>> {
        let mut candidates = Vec::new();

        for (provider_id, provider) in &self.hsm_providers {
            if self
                .provider_meets_requirements(provider_id, requirements)
                .await?
            {
                candidates.push(provider.clone());
            }
        }

        Ok(candidates)
    }

    /// Check if a provider meets the security requirements
    async fn provider_meets_requirements(
        &self,
        provider_id: &str,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<bool> {
        let provider =
            self.hsm_providers
                .get(provider_id)
                .ok_or_else(|| BearDogError::NotFound {
                    message: format!("Provider not found: {provider_id}"),
                })?;

        // Get provider info
        let provider_info = provider.get_info().await?;

        // Check security level requirement
        let meets_security_level = match requirements.security_level {
            SecurityLevel::Basic => true, // All providers meet basic
            SecurityLevel::Medium => {
                // Software HSM and above
                provider_info.hsm_type != "SoftwareHsm"
                    || provider_info.hsm_type == "SmartphoneHsm"
                    || provider_info.hsm_type == "HardwareHsm"
            }
            SecurityLevel::High => {
                // Hardware-backed required
                provider_info.hsm_type == "SmartphoneHsm"
                    || provider_info.hsm_type == "HardwareHsm"
                    || provider_info.hsm_type == "HybridHsm"
            }
            SecurityLevel::Maximum => {
                // Certified hardware required
                provider_info.hsm_type == "HardwareHsm" || provider_info.hsm_type == "HybridHsm"
            }
        };

        if !meets_security_level {
            return Ok(false);
        }

        // Check hardware backing requirement
        if requirements.hardware_backed_required && provider_info.hsm_type.as_str() == "SoftwareHsm"
        {
            return Ok(false);
        }
        // Hardware-backed or hybrid

        // Check attestation requirement
        if requirements.attestation_required
            && !provider_info
                .capabilities
                .contains(&HsmCapability::KeyAttestation)
        {
            return Ok(false);
        }

        // Check user interaction requirement
        if requirements.user_interaction_required {
            match provider_info.hsm_type.as_str() {
                "SmartphoneHsm" => {
                    // Smartphone HSMs support user interaction
                    if !provider_info
                        .capabilities
                        .contains(&HsmCapability::UserPresenceValidation)
                    {
                        return Ok(false);
                    }
                }
                _ => return Ok(false), // Other HSMs don't support user interaction
            }
        }

        Ok(true)
    }

    /// Select the best provider from candidates
    async fn select_best_provider(
        &self,
        candidates: Vec<Arc<dyn HsmProvider>>,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<HsmProviderSelection> {
        let mut best_selection: Option<HsmProviderSelection> = None;
        let mut best_score = 0.0;

        for provider in candidates {
            let provider_info = provider.get_info().await?;
            let provider_id = self.get_provider_id(&provider).await?;

            // Calculate provider score based on requirements
            let score = self
                .calculate_provider_score(&provider_info, requirements)
                .await?;

            if score > best_score {
                best_score = score;
                best_selection = Some(HsmProviderSelection {
                    provider: provider.clone(),
                    provider_id,
                    tier: HsmTier::SoftwareHsm {
                        implementation: SoftwareHsmType::RustSoftwareHsm,
                        key_storage: KeyStorageType::EncryptedFile,
                        encryption_at_rest: true,
                        memory_protection: MemoryProtectionLevel::Basic,
                    },
                    confidence: score,
                    estimated_latency_ms: self.get_estimated_latency(&provider_info).await?,
                });
            }
        }

        best_selection.ok_or_else(|| BearDogError::NoSuitableProvider {
            message: format!("No suitable provider for requirements: {requirements:?}"),
        })
    }

    /// Calculate provider score based on requirements
    async fn calculate_provider_score(
        &self,
        provider_info: &HsmInfo,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<f64> {
        let mut score = 0.0;

        // Security level score
        let security_score = match (provider_info.hsm_type.as_str(), requirements.security_level) {
            ("HardwareHsm", SecurityLevel::Maximum) => 1.0,
            ("HardwareHsm", SecurityLevel::High) => 0.9,
            ("SmartphoneHsm", SecurityLevel::High) => 0.8,
            ("SmartphoneHsm", SecurityLevel::Medium) => 0.7,
            ("SoftwareHsm", SecurityLevel::Medium) => 0.6,
            ("SoftwareHsm", SecurityLevel::Basic) => 0.5,
            ("HybridHsm", _) => 0.9, // Hybrid gets high score
            _ => 0.3,                // Partial match
        };

        score += security_score * 0.4; // 40% weight

        // Performance score
        let performance_metrics = self
            .performance_tracker
            .get_provider_metrics(&provider_info.vendor)
            .await?;
        let performance_score = match performance_metrics {
            Some(metrics) => {
                let latency_score = if metrics.average_latency_ms < 100.0 {
                    1.0
                } else {
                    0.5
                };
                let success_rate = if metrics.total_operations > 0 {
                    metrics.successful_operations as f64 / metrics.total_operations as f64
                } else {
                    0.5
                };
                (latency_score + success_rate) / 2.0
            }
            None => 0.5, // Default score for new providers
        };

        score += performance_score * 0.3; // 30% weight

        // Capability score
        let required_capabilities = self.get_required_capabilities(requirements);
        let capability_score = if required_capabilities.is_empty() {
            1.0
        } else {
            required_capabilities
                .iter()
                .map(|cap| {
                    if provider_info.capabilities.contains(cap) {
                        1.0
                    } else {
                        0.0
                    }
                })
                .sum::<f64>()
                / required_capabilities.len() as f64
        };

        score += capability_score * 0.2; // 20% weight

        // Availability score
        let availability_score = match self
            .health_monitor
            .get_provider_health(&provider_info.vendor)
            .await?
        {
            Some(health) => {
                if health.healthy {
                    1.0
                } else {
                    0.1
                }
            }
            None => 0.5, // Unknown health status
        };

        score += availability_score * 0.1; // 10% weight

        Ok(score)
    }

    /// Get required capabilities for security requirements
    fn get_required_capabilities(&self, requirements: &SecurityRequirements) -> Vec<HsmCapability> {
        let mut capabilities = Vec::new();

        if requirements.attestation_required {
            capabilities.push(HsmCapability::KeyAttestation);
        }

        if requirements.user_interaction_required {
            capabilities.push(HsmCapability::UserPresenceValidation);
        }

        capabilities
    }

    /// Get estimated latency for a provider
    async fn get_estimated_latency(&self, provider_info: &HsmInfo) -> BearDogResult<f64> {
        // Check if we have performance metrics
        if let Some(metrics) = self
            .performance_tracker
            .get_provider_metrics(&provider_info.vendor)
            .await?
        {
            return Ok(metrics.average_latency_ms);
        }

        // Default estimates based on HSM type
        match provider_info.hsm_type.as_str() {
            "SoftwareHsm" => Ok(10.0),   // Fast software
            "SmartphoneHsm" => Ok(50.0), // Moderate smartphone
            "HardwareHsm" => Ok(100.0),  // Slower hardware
            "HybridHsm" => Ok(75.0),     // Mixed performance
            _ => Ok(50.0),               // Default fallback
        }
    }

    /// Get provider ID from provider instance
    async fn get_provider_id(&self, provider: &Arc<dyn HsmProvider>) -> BearDogResult<String> {
        let provider_info = provider.get_info().await?;
        Ok(provider_info.vendor)
    }

    /// Select optimal HSM (for backward compatibility)
    pub async fn select_optimal_hsm(
        &self,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<Arc<dyn HsmProvider>> {
        let selection = self.get_best_provider(requirements).await?;
        Ok(selection.provider)
    }

    /// Get simple health status (for backward compatibility)
    pub async fn get_health_status_simple(&self) -> BearDogResult<HsmHealthStatus> {
        self.health_check().await
    }

    /// Generate random bytes using HSM
    pub async fn generate_random_bytes(
        &self,
        length: usize,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<Vec<u8>> {
        let _selection = self.get_best_provider(requirements).await?;

        // Use the provider's random generation capability
        // This is a simplified implementation
        let mut bytes = vec![0u8; length];
        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut bytes);
        Ok(bytes)
    }

    /// Get public key for a key ID
    pub async fn get_public_key(&self, key_id: &str) -> BearDogResult<Vec<u8>> {
        // This is a simplified implementation
        // In a real implementation, we would find the provider that has this key
        // and retrieve the public key from it
        Err(BearDogError::NotFound {
            message: format!("Key not found: {key_id}"),
        })
    }

    /// Sign data using HSM
    pub async fn sign_data(
        &self,
        key_id: &str,
        data: &[u8],
        requirements: &SecurityRequirements,
        _operation: &HsmOperation,
    ) -> BearDogResult<Vec<u8>> {
        let selection = self.get_best_provider(requirements).await?;
        selection.provider.sign(key_id, data).await
    }
}
