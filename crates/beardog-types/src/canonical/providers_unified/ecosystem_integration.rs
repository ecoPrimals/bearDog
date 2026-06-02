// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Ecosystem Integration Module
//!
//! This module provides **ecosystem integration adapters** that enable seamless migration
//! from scattered provider implementations to the unified provider registry system.
//!
//! ## 🎯 **Integration Strategy**
//!
//! This module provides:
//! - **Migration adapters** for existing provider instantiations
//! - **Compatibility layers** for legacy provider interfaces
//! - **Automated discovery** of existing provider implementations
//! - **Gradual migration** support with fallback mechanisms
//!
//! ## 🚀 **Usage Examples**
//!
//! ```rust
//! use beardog_types::canonical::providers_unified::ecosystem_integration::{
//!     EcosystemIntegrator, MigrationAdapter, ProviderMigrationPlan
//! };
//!
//! // Automatic discovery and migration of existing providers
//! let integrator = EcosystemIntegrator::new().await?;
//! let migration_plan = integrator.discover_existing_providers().await?;
//! integrator.execute_migration_plan(migration_plan).await?;
//! ```

use beardog_config::env_keys;
use beardog_errors::BearDogError;
use super::consolidated_registry::{ConsolidatedProviderRegistry, ProviderRegistryConfig};
use super::hsm_unified::{HsmUnifiedProvider, AndroidHsmConfig, IosHsmConfig, SoftwareHsmConfig};
use super::traits::consolidated::{ConsolidatedProvider, ProviderInfo, ProviderType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};

/// **Ecosystem Integrator**
///
/// Main integration coordinator that manages the migration from scattered providers
/// to the unified provider registry system.
pub struct EcosystemIntegrator {
    /// Unified provider registry
    registry: Arc<ConsolidatedProviderRegistry>,
    /// Migration adapters for different provider types
    migration_adapters: HashMap<String, Box<dyn MigrationAdapter>>,
    /// Integration configuration
    config: IntegrationConfig,
    /// Migration state tracking
    migration_state: Arc<RwLock<MigrationState>>,
}

/// Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    /// Enable automatic provider discovery
    pub auto_discovery: bool,
    /// Enable gradual migration mode
    pub gradual_migration: bool,
    /// Migration timeout in seconds
    pub migration_timeout_secs: u64,
    /// Enable fallback to legacy providers
    pub enable_fallback: bool,
    /// Maximum concurrent migrations
    pub max_concurrent_migrations: u32,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            auto_discovery: true,
            gradual_migration: true,
            migration_timeout_secs: std::env::var(env_keys::ENV_PROVIDER_MIGRATION_TIMEOUT_SECS)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(300), // 5 minutes default
            enable_fallback: true,
            max_concurrent_migrations: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
        }
    }
}

/// Migration state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationState {
    /// Total providers discovered
    pub total_discovered: u32,
    /// Providers successfully migrated
    pub migrated: u32,
    /// Providers failed to migrate
    pub failed: u32,
    /// Providers skipped
    pub skipped: u32,
    /// Migration start time
    pub started_at: Option<std::time::SystemTime>,
    /// Migration completion time
    pub completed_at: Option<std::time::SystemTime>,
    /// Current migration phase
    pub phase: MigrationPhase,
}

/// Migration phase enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MigrationPhase {
    /// Discovery phase
    Discovery,
    /// Planning phase
    Planning,
    /// Execution phase
    Execution,
    /// Validation phase
    Validation,
    /// Completed
    Completed,
    /// Failed
    Failed,
}

impl Default for MigrationState {
    fn default() -> Self {
        Self {
            total_discovered: 0,
            migrated: 0,
            failed: 0,
            skipped: 0,
            started_at: None,
            completed_at: None,
            phase: MigrationPhase::Discovery,
        }
    }
}

/// Migration adapter trait for different provider types
pub trait MigrationAdapter: Send + Sync {
    /// Discover existing providers of this type
    fn discover_providers(&self) -> Result<Vec<DiscoveredProvider>>;
    
    /// Create unified provider from legacy provider
    fn migrate_provider(&self, discovered: &DiscoveredProvider) -> Result<Arc<dyn ConsolidatedProvider>>;
    
    /// Get adapter name
    fn adapter_name(&self) -> &str;
    
    /// Get supported provider types
    fn supported_types(&self) -> Vec<String>;
}

/// Information about a discovered legacy provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredProvider {
    /// Provider name/identifier
    pub name: String,
    /// Provider type
    pub provider_type: String,
    /// Location information
    pub location: ProviderLocation,
    /// Configuration data
    pub config_data: HashMap<String, serde_json::Value>,
    /// Priority for migration
    pub priority: i32,
    /// Migration complexity
    pub complexity: MigrationComplexity,
}

/// Provider location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderLocation {
    /// Crate name
    pub crate_name: String,
    /// Module path
    pub module_path: String,
    /// Struct name
    pub struct_name: String,
    /// File path
    pub file_path: String,
}

/// Migration complexity level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MigrationComplexity {
    /// Simple migration - direct mapping
    Simple,
    /// Moderate migration - some adaptation required
    Moderate,
    /// Complex migration - significant changes needed
    Complex,
    /// Manual migration - requires human intervention
    Manual,
}

/// Provider migration plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMigrationPlan {
    /// Discovered providers to migrate
    pub providers: Vec<DiscoveredProvider>,
    /// Migration order (by priority)
    pub migration_order: Vec<usize>,
    /// Estimated migration time
    pub estimated_duration: std::time::Duration,
    /// Risk assessment
    pub risk_level: RiskLevel,
    /// Required manual interventions
    pub manual_interventions: Vec<ManualIntervention>,
}

/// Risk level enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    /// Low risk migration
    Low,
    /// Medium risk migration
    Medium,
    /// High risk migration
    High,
    /// Critical risk migration
    Critical,
}

/// Manual intervention required
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualIntervention {
    /// Provider requiring intervention
    pub provider_name: String,
    /// Type of intervention needed
    pub intervention_type: InterventionType,
    /// Description of what needs to be done
    pub description: String,
    /// Estimated time required
    pub estimated_time: std::time::Duration,
}

/// Intervention type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InterventionType {
    /// Configuration update required
    ConfigurationUpdate,
    /// Code refactoring required
    CodeRefactoring,
    /// Dependency update required
    DependencyUpdate,
    /// Testing required
    Testing,
    /// Documentation update required
    Documentation,
}

impl EcosystemIntegrator {
    /// Create new ecosystem integrator
    pub async fn new() -> Result<Self> {
        Self::with_config(IntegrationConfig::default()).await
    }

    /// Create ecosystem integrator with custom configuration
    pub async fn with_config(config: IntegrationConfig) -> Result<Self> {
        let registry = Arc::new(ConsolidatedProviderRegistry::new(ProviderRegistryConfig::default()));
        
        let mut migration_adapters: HashMap<String, Box<dyn MigrationAdapter>> = HashMap::new();
        
        // Register built-in migration adapters (BearDog-owned domains only;
        // security/storage/network migration belongs to the primals that own those domains).
        migration_adapters.insert("hsm".to_string(), Box::new(HsmMigrationAdapter::new()));
        
        Ok(Self {
            registry,
            migration_adapters,
            config,
            migration_state: Arc::new(RwLock::new(MigrationState::default())),
        })
    }

    /// Discover existing providers across the ecosystem
    pub async fn discover_existing_providers(&self) -> Result<Vec<DiscoveredProvider>> {
        info!("Starting provider discovery across ecosystem");
        
        {
            let mut state = self.migration_state.write().await;
            state.phase = MigrationPhase::Discovery;
            state.started_at = Some(std::time::SystemTime::now());
        }

        let mut all_discovered = Vec::new();
        
        for (adapter_name, adapter) in &self.migration_adapters {
            debug!("Running discovery with adapter: {}", adapter_name);
            
            match adapter.discover_providers() {
                Ok(discovered) => {
                    info!("Adapter {} discovered {} providers", adapter_name, discovered.len());
                    all_discovered.extend(discovered);
                }
                Err(e) => {
                    warn!("Adapter {} discovery failed: {}", adapter_name, e);
                    // Continue with other adapters
                }
            }
        }

        {
            let mut state = self.migration_state.write().await;
            state.total_discovered = all_discovered.len() as u32;
            state.phase = MigrationPhase::Planning;
        }

        info!("Discovery complete: {} providers found", all_discovered.len());
        Ok(all_discovered)
    }

    /// Create migration plan from discovered providers
    pub async fn create_migration_plan(&self, discovered: Vec<DiscoveredProvider>) -> Result<ProviderMigrationPlan> {
        info!("Creating migration plan for {} providers", discovered.len());

        // Sort by priority and complexity
        let mut migration_order: Vec<usize> = (0..discovered.len()).collect();
        migration_order.sort_by(|&a, &b| {
            // Higher priority first, then lower complexity first
            discovered[b].priority.cmp(&discovered[a].priority)
                .then(discovered[a].complexity.cmp(&discovered[b].complexity))
        });

        // Estimate duration based on complexity
        let estimated_duration = discovered.iter()
            .map(|p| match p.complexity {
                MigrationComplexity::Simple => std::time::Duration::from_secs(60),
                MigrationComplexity::Moderate => std::time::Duration::from_secs(300),
                MigrationComplexity::Complex => std::time::Duration::from_secs(900),
                MigrationComplexity::Manual => {
                    std::time::Duration::from_secs(crate::constants::time::SECONDS_PER_HOUR)
                }
            })
            .sum();

        // Assess risk level
        let complex_count = discovered.iter()
            .filter(|p| p.complexity >= MigrationComplexity::Complex)
            .count();
        
        let risk_level = match complex_count {
            0 => RiskLevel::Low,
            1..=3 => RiskLevel::Medium,
            4..=beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE => RiskLevel::High,
            _ => RiskLevel::Critical,
        };

        // Identify manual interventions
        let manual_interventions = discovered.iter()
            .filter(|p| p.complexity == MigrationComplexity::Manual)
            .map(|p| ManualIntervention {
                provider_name: p.name.clone(),
                intervention_type: InterventionType::CodeRefactoring,
                description: format!("Manual migration required for complex provider: {}", p.name),
                estimated_time: std::time::Duration::from_secs(
                    crate::constants::time::SECONDS_PER_HOUR,
                ),
            })
            .collect();

        let plan = ProviderMigrationPlan {
            providers: discovered,
            migration_order,
            estimated_duration,
            risk_level,
            manual_interventions,
        };

        info!("Migration plan created: {} providers, estimated duration: {:?}, risk level: {:?}",
              plan.providers.len(), plan.estimated_duration, plan.risk_level);

        Ok(plan)
    }

    /// Execute migration plan
    pub async fn execute_migration_plan(&self, plan: ProviderMigrationPlan) -> Result<MigrationResults> {
        info!("Executing migration plan for {} providers", plan.providers.len());

        {
            let mut state = self.migration_state.write().await;
            state.phase = MigrationPhase::Execution;
        }

        let mut results = MigrationResults {
            total_attempted: plan.providers.len() as u32,
            successful: 0,
            failed: 0,
            skipped: 0,
            errors: Vec::new(),
            duration: std::time::Duration::from_secs(0),
        };

        let start_time = std::time::Instant::now();

        for &index in &plan.migration_order {
            let provider = &plan.providers[index];
            
            debug!("Migrating provider: {} ({})", provider.name, provider.provider_type);

            match self.migrate_single_provider(provider).await {
                Ok(_) => {
                    results.successful += 1;
                    info!("Successfully migrated provider: {}", provider.name);
                }
                Err(e) => {
                    results.failed += 1;
                    results.errors.push(format!("Failed to migrate {}: {}", provider.name, e));
                    warn!("Failed to migrate provider {}: {}", provider.name, e);
                }
            }
        }

        results.duration = start_time.elapsed();

        {
            let mut state = self.migration_state.write().await;
            state.migrated = results.successful;
            state.failed = results.failed;
            state.skipped = results.skipped;
            state.phase = if results.failed == 0 {
                MigrationPhase::Completed
            } else {
                MigrationPhase::Failed
            };
            state.completed_at = Some(std::time::SystemTime::now());
        }

        info!("Migration execution completed: {}/{} successful, {} failed",
              results.successful, results.total_attempted, results.failed);

        Ok(results)
    }

    /// Migrate a single provider
    async fn migrate_single_provider(&self, discovered: &DiscoveredProvider) -> Result<String> {
        // Find appropriate migration adapter
        let adapter = self.migration_adapters.get(&discovered.provider_type)
            .ok_or_else(|| BearDogError::system(format!(
                "No migration adapter found for provider type: {}", discovered.provider_type
            )))?;

        // Create unified provider
        let unified_provider = adapter.migrate_provider(discovered)?;

        // Register in unified registry
        let provider_id = self.registry.register_provider(
            unified_provider,
            discovered.priority,
            vec![discovered.provider_type.clone()],
        ).await?;

        Ok(provider_id)
    }

    /// Get migration state
    pub async fn get_migration_state(&self) -> MigrationState {
        self.migration_state.read().await.clone()
    }

    /// Get unified provider registry
    pub fn get_registry(&self) -> Arc<ConsolidatedProviderRegistry> {
        self.registry.clone()
    }
}

/// Migration results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationResults {
    /// Total providers attempted
    pub total_attempted: u32,
    /// Successfully migrated
    pub successful: u32,
    /// Failed migrations
    pub failed: u32,
    /// Skipped providers
    pub skipped: u32,
    /// Error messages
    pub errors: Vec<String>,
    /// Total migration duration
    pub duration: std::time::Duration,
}

// Built-in migration adapters

/// HSM migration adapter
pub struct HsmMigrationAdapter;

impl HsmMigrationAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl MigrationAdapter for HsmMigrationAdapter {
    fn discover_providers(&self) -> Result<Vec<DiscoveredProvider>> {
        // In a real implementation, this would scan the codebase for HSM provider implementations
        let discovered = vec![
            DiscoveredProvider {
                name: "AndroidUniversalProvider".to_string(),
                provider_type: "hsm".to_string(),
                location: ProviderLocation {
                    crate_name: "beardog-tunnel".to_string(),
                    module_path: "tunnel::hsm::providers::android".to_string(),
                    struct_name: "AndroidUniversalProvider".to_string(),
                    file_path: "crates/beardog-tunnel/src/tunnel/hsm/providers/android.rs".to_string(),
                },
                config_data: HashMap::from([
                    ("device_id".to_string(), serde_json::Value::String("android-default".to_string())),
                    ("use_strongbox".to_string(), serde_json::Value::Bool(true)),
                ]),
                priority: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
                complexity: MigrationComplexity::Simple,
            },
            DiscoveredProvider {
                name: "IosUniversalProvider".to_string(),
                provider_type: "hsm".to_string(),
                location: ProviderLocation {
                    crate_name: "beardog-tunnel".to_string(),
                    module_path: "tunnel::hsm::providers::ios".to_string(),
                    struct_name: "IosUniversalProvider".to_string(),
                    file_path: "crates/beardog-tunnel/src/tunnel/hsm/providers/ios.rs".to_string(),
                },
                config_data: HashMap::from([
                    ("device_id".to_string(), serde_json::Value::String("ios-default".to_string())),
                    ("use_secure_enclave".to_string(), serde_json::Value::Bool(true)),
                ]),
                priority: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
                complexity: MigrationComplexity::Simple,
            },
        ];

        Ok(discovered)
    }

    fn migrate_provider(&self, discovered: &DiscoveredProvider) -> Result<Arc<dyn ConsolidatedProvider>> {
        match discovered.name.as_str() {
            "AndroidUniversalProvider" => {
                let config = AndroidHsmConfig {
                    device_id: discovered.config_data.get("device_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("android-default")
                        .to_string(),
                    use_strongbox: discovered.config_data.get("use_strongbox")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(true),
                    ..Default::default()
                };
                Ok(Arc::new(HsmUnifiedProvider::Android(config)))
            }
            "IosUniversalProvider" => {
                let config = IosHsmConfig {
                    device_id: discovered.config_data.get("device_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("ios-default")
                        .to_string(),
                    use_secure_enclave: discovered.config_data.get("use_secure_enclave")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(true),
                    ..Default::default()
                };
                Ok(Arc::new(HsmUnifiedProvider::Ios(config)))
            }
            _ => Err(BearDogError::system(format!(
                "Unknown HSM provider: {}", discovered.name
            ))),
        }
    }

    fn adapter_name(&self) -> &str {
        "HSM Migration Adapter"
    }

    fn supported_types(&self) -> Vec<String> {
        vec!["hsm".to_string()]
    }
}

// Security, storage, and network migration adapters removed — those domains belong
// to other primals per PRIMAL_RESPONSIBILITY_MATRIX.md v3.0.  BearDog only migrates
// HSM providers (its own domain).  Cross-primal migration is handled by the primals
// that own those capabilities, discovered at runtime via capability-based routing.

// All legacy providers have been migrated to the ConsolidatedProvider system
// Use ConsolidatedProviderRegistry directly for all provider operations
//
// Async/unit coverage for this module lives in `src/tests/ecosystem_integration_tokio_tests.rs`
// (Tokio is a dev-dependency). This file is not yet `mod`’d under `providers_unified::mod`;
// wire it there before calling `EcosystemIntegrator` from crate tests.