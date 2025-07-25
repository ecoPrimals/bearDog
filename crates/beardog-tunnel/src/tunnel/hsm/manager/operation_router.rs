//! # HSM Operation Router
//!
//! This module provides intelligent routing of HSM operations based on security requirements,
//! operation types, and available HSM tiers. It implements the mobile-first architecture where
//! critical operations are routed to mobile HSM (StrongBox) when available.

use super::*;
use crate::tunnel::hsm::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// HSM operation router that implements intelligent operation routing
pub struct HsmOperationRouter {
    /// Available HSM providers by tier
    providers: HashMap<HsmTier, Arc<dyn HsmProvider>>,
    /// Operation routing rules
    routing_rules: OperationRoutingRules,
    /// Performance metrics for routing decisions
    performance_metrics: RoutingMetrics,
}

/// Operation routing rules configuration
#[derive(Debug, Clone)]
pub struct OperationRoutingRules {
    /// Critical operations that require mobile HSM
    critical_operations: Vec<OperationType>,
    /// Operations that prefer mobile HSM but can fallback
    preferred_mobile_operations: Vec<OperationType>,
    /// Operations that can use any available HSM
    flexible_operations: Vec<OperationType>,
    /// Operations that should use software HSM for performance
    software_preferred_operations: Vec<OperationType>,
}

/// Types of HSM operations for routing decisions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperationType {
    /// Human identity operations (biometric, authentication)
    HumanIdentity,
    /// Root key generation for the ecosystem
    RootKeyGeneration,
    /// Critical authentication operations
    CriticalAuthentication,
    /// File and data encryption
    FileEncryption,
    /// Data processing operations
    DataProcessing,
    /// Local genetic spawning
    LocalSpawning,
    /// Backup operations
    BackupOperations,
    /// General cryptographic operations
    GeneralCrypto,
}

/// Routing performance metrics
#[derive(Debug, Clone, Default)]
pub struct RoutingMetrics {
    /// Operation success rates by HSM tier
    success_rates: HashMap<HsmTier, f64>,
    /// Average operation latencies by HSM tier
    latencies: HashMap<HsmTier, f64>,
    /// HSM availability status
    availability: HashMap<HsmTier, bool>,
}

/// HSM selection result with rationale
pub struct HsmSelectionResult {
    /// Selected HSM tier
    pub selected_tier: HsmTier,
    /// Primary HSM provider
    pub primary_provider: Arc<dyn HsmProvider>,
    /// Fallback provider (if any)
    pub fallback_provider: Option<Arc<dyn HsmProvider>>,
    /// Reason for selection
    pub selection_reason: String,
    /// Whether this is an optimal selection
    pub is_optimal: bool,
}

impl HsmOperationRouter {
    /// Create a new HSM operation router
    pub fn new() -> Self {
        let routing_rules = OperationRoutingRules {
            critical_operations: vec![
                OperationType::HumanIdentity,
                OperationType::RootKeyGeneration,
            ],
            preferred_mobile_operations: vec![OperationType::CriticalAuthentication],
            flexible_operations: vec![OperationType::GeneralCrypto],
            software_preferred_operations: vec![
                OperationType::FileEncryption,
                OperationType::DataProcessing,
                OperationType::LocalSpawning,
                OperationType::BackupOperations,
            ],
        };

        Self {
            providers: HashMap::new(),
            routing_rules,
            performance_metrics: RoutingMetrics::default(),
        }
    }

    /// Register an HSM provider for a specific tier
    pub fn register_provider(
        &mut self,
        tier: HsmTier,
        provider: Arc<dyn HsmProvider>,
    ) -> BearDogResult<()> {
        info!("🔐 Registering HSM provider for tier: {:?}", tier);
        self.providers.insert(tier, provider);
        Ok(())
    }

    /// Select the optimal HSM for an operation
    pub async fn select_hsm_for_operation(
        &self,
        operation_type: OperationType,
        security_requirements: &SecurityRequirements,
    ) -> BearDogResult<HsmSelectionResult> {
        debug!("🎯 Selecting HSM for operation: {:?}", operation_type);

        // Apply routing rules based on operation type
        let (preferred_tiers, fallback_tiers) = self.get_preferred_tiers(&operation_type);

        // Try preferred tiers first
        for tier in preferred_tiers {
            if let Some(provider) = self.providers.get(&tier) {
                // Check if HSM is available and healthy
                if self.is_hsm_available(&tier).await? {
                    // Verify the HSM meets security requirements
                    if self
                        .meets_security_requirements(&tier, security_requirements)
                        .await?
                    {
                        let fallback = self.get_best_fallback(&tier, &fallback_tiers);

                        return Ok(HsmSelectionResult {
                            selected_tier: tier.clone(),
                            primary_provider: provider.clone(),
                            fallback_provider: fallback,
                            selection_reason: format!(
                                "Optimal HSM for {:?} - meets security requirements",
                                operation_type
                            ),
                            is_optimal: true,
                        });
                    }
                }
            }
        }

        // Try fallback tiers if preferred tiers are unavailable
        for tier in fallback_tiers {
            if let Some(provider) = self.providers.get(&tier) {
                if self.is_hsm_available(&tier).await? {
                    warn!(
                        "⚠️ Using fallback HSM tier {:?} for operation {:?}",
                        tier, operation_type
                    );

                    return Ok(HsmSelectionResult {
                        selected_tier: tier.clone(),
                        primary_provider: provider.clone(),
                        fallback_provider: None,
                        selection_reason: format!(
                            "Fallback HSM for {:?} - preferred HSM unavailable",
                            operation_type
                        ),
                        is_optimal: false,
                    });
                }
            }
        }

        // No suitable HSM found
        Err(BearDogError::Unavailable {
            message: format!(
                "No suitable HSM available for operation: {:?}",
                operation_type
            ),
        })
    }

    /// Get preferred HSM tiers for an operation type
    fn get_preferred_tiers(&self, operation_type: &OperationType) -> (Vec<HsmTier>, Vec<HsmTier>) {
        let mobile_hsm = HsmTier::SmartphoneHsm {
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
            user_presence_required: true,
        };

        let software_hsm = HsmTier::SoftwareHsm {
            implementation: SoftwareHsmType::RustSoftwareHsm,
            key_storage: KeyStorageType::InMemory,
            encryption_at_rest: true,
            memory_protection: MemoryProtectionLevel::High,
        };

        if self
            .routing_rules
            .critical_operations
            .contains(operation_type)
        {
            // Critical operations MUST use mobile HSM
            (vec![mobile_hsm], vec![])
        } else if self
            .routing_rules
            .preferred_mobile_operations
            .contains(operation_type)
        {
            // Prefer mobile HSM but allow software fallback
            (vec![mobile_hsm.clone()], vec![software_hsm])
        } else if self
            .routing_rules
            .software_preferred_operations
            .contains(operation_type)
        {
            // Prefer software HSM for performance
            (vec![software_hsm.clone()], vec![mobile_hsm])
        } else {
            // Flexible operations - try mobile first, fallback to software
            (vec![mobile_hsm.clone()], vec![software_hsm])
        }
    }

    /// Check if an HSM tier is available and healthy
    async fn is_hsm_available(&self, tier: &HsmTier) -> BearDogResult<bool> {
        if let Some(provider) = self.providers.get(tier) {
            match provider.health_check().await {
                Ok(health) => Ok(health.healthy),
                Err(_) => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    /// Check if an HSM meets security requirements
    async fn meets_security_requirements(
        &self,
        tier: &HsmTier,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<bool> {
        if let Some(provider) = self.providers.get(tier) {
            let _info = provider.get_info().await?;

            // Check security level
            match requirements.security_level {
                SecurityLevel::Basic => Ok(true), // All HSMs meet basic requirements
                SecurityLevel::Medium => {
                    // Medium security requires hardware backing or high-grade software
                    match tier {
                        HsmTier::SmartphoneHsm { .. } => Ok(true),
                        HsmTier::SoftwareHsm {
                            memory_protection, ..
                        } => Ok(*memory_protection >= MemoryProtectionLevel::High),
                        _ => Ok(true),
                    }
                }
                SecurityLevel::High => {
                    // High security requires hardware backing
                    match tier {
                        HsmTier::SmartphoneHsm { .. } => Ok(true),
                        HsmTier::HardwareHsm { .. } => Ok(true),
                        _ => Ok(false),
                    }
                }
                SecurityLevel::Maximum => {
                    // Maximum security requires certified hardware HSM
                    match tier {
                        HsmTier::SmartphoneHsm {
                            attestation_level, ..
                        } => Ok(*attestation_level >= AttestationLevel::CertifiedHardware),
                        HsmTier::HardwareHsm { .. } => Ok(true),
                        _ => Ok(false),
                    }
                }
            }
        } else {
            Ok(false)
        }
    }

    /// Get the best fallback provider for a tier
    fn get_best_fallback(
        &self,
        _primary_tier: &HsmTier,
        fallback_tiers: &[HsmTier],
    ) -> Option<Arc<dyn HsmProvider>> {
        for tier in fallback_tiers {
            if let Some(provider) = self.providers.get(tier) {
                return Some(provider.clone());
            }
        }
        None
    }

    /// Update performance metrics for routing decisions
    pub fn update_metrics(&mut self, tier: &HsmTier, success: bool, latency_ms: f64) {
        // Update success rate
        let current_rate = self
            .performance_metrics
            .success_rates
            .get(tier)
            .unwrap_or(&1.0);
        let new_rate = if success {
            (*current_rate * 0.9) + (1.0 * 0.1) // Exponential moving average
        } else {
            (*current_rate * 0.9) + (0.0 * 0.1)
        };
        self.performance_metrics
            .success_rates
            .insert(tier.clone(), new_rate);

        // Update latency
        let current_latency = self
            .performance_metrics
            .latencies
            .get(tier)
            .unwrap_or(&latency_ms);
        let new_latency = (*current_latency * 0.9) + (latency_ms * 0.1);
        self.performance_metrics
            .latencies
            .insert(tier.clone(), new_latency);

        debug!(
            "📊 Updated metrics for {:?}: success_rate={:.2}, latency={:.1}ms",
            tier, new_rate, new_latency
        );
    }

    /// Get current routing metrics
    pub fn get_metrics(&self) -> &RoutingMetrics {
        &self.performance_metrics
    }
}

impl Default for HsmOperationRouter {
    fn default() -> Self {
        Self::new()
    }
}
