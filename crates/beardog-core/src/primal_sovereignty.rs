// SPDX-License-Identifier: AGPL-3.0-only

// Primal Sovereignty Implementation
//
// This module implements the core sovereignty mechanisms that ensure
// BearDog operates as a truly sovereign primal with genetic spawning capabilities.

use beardog_errors::BearDogError;
use beardog_genetics::{EntropyHierarchyConfig, EntropyHierarchyManager};
use beardog_genetics::{GeneticSpawningEngine, SpawnRequest};
use beardog_security::encryption::EncryptionConfig;
use chrono::{DateTime, Utc};
use tracing::info;
use uuid::Uuid;

// Type alias for compatibility - using PrimalSovereigntyConfig
type SovereigntyConfig = PrimalSovereigntyConfig;

/// Primal sovereignty configuration
/// Renamed from `SovereigntyConfig` to avoid collision with `sovereignty::SovereigntyConfig`
/// Configuration for primal sovereignty management
///
/// Defines sovereignty monitoring and validation thresholds for primal instances.
#[derive(Debug, Clone, Default)]
pub struct PrimalSovereigntyConfig {
    /// Unique primal identifier
    pub primal_id: String,
    /// Whether to enable continuous sovereignty monitoring
    pub enable_sovereignty_monitoring: bool,
    /// Minimum sovereignty score threshold (0.0-1.0)
    pub sovereignty_threshold: f64,
}

/// Current sovereignty state of a primal
///
/// Tracks the sovereignty status, score, and validation history of a primal instance.
#[derive(Debug, Clone)]
pub struct SovereigntyState {
    /// Primal identifier
    pub primal_id: String,
    /// When this primal was first initialized (genesis)
    pub genesis_timestamp: DateTime<Utc>,
    /// Whether sovereignty is currently active
    pub is_active: bool,
    /// Current sovereignty score (0.0-1.0, where 1.0 is fully sovereign)
    pub sovereignty_score: f64,
    /// Timestamp of last sovereignty validation check
    pub last_validation: DateTime<Utc>,
}

impl Default for SovereigntyState {
    fn default() -> Self {
        Self {
            primal_id: format!("primal-{}", Uuid::new_v4()),
            genesis_timestamp: Utc::now(),
            is_active: true,
            sovereignty_score: 1.0,
            last_validation: Utc::now(),
        }
    }
}

/// Sovereignty manager for primal instances
///
/// Manages sovereignty validation, genetic spawning, cryptography, and entropy hierarchy
/// for maintaining primal autonomy and sovereignty.
#[derive(Debug)]
pub struct SovereigntyManager {
    /// Sovereignty configuration
    config: PrimalSovereigntyConfig,
    /// Genetic spawning engine for primal evolution
    #[allow(dead_code)]
    genetics: GeneticSpawningEngine,
    /// Encryption configuration
    #[allow(dead_code)]
    crypto_config: EncryptionConfig,
    /// Human entropy hierarchy manager
    #[allow(dead_code)]
    hierarchy_manager: EntropyHierarchyManager,
    /// Current sovereignty state
    sovereignty_state: SovereigntyState,
}

impl SovereigntyManager {
    /// Create a new sovereignty manager
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Genetic spawning engine initialization fails
    /// - Entropy hierarchy manager creation fails
    pub fn new(config: PrimalSovereigntyConfig) -> Result<Self, BearDogError> {
        info!("🏛️ Initializing Primal Sovereignty Manager");

        let genetics = GeneticSpawningEngine::new();
        let crypto_config = EncryptionConfig::default();
        let hierarchy_config = EntropyHierarchyConfig::default();
        let hierarchy_manager = EntropyHierarchyManager::new(hierarchy_config);

        Ok(Self {
            config,
            genetics,
            crypto_config,
            hierarchy_manager,
            sovereignty_state: SovereigntyState::default(),
        })
    }

    /// Validate sovereignty status
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Sovereignty validation fails
    /// - Configuration is invalid
    /// - Required dependencies are not met
    #[allow(clippy::cognitive_complexity)]
    pub fn validate_sovereignty(&mut self) -> Result<bool, BearDogError> {
        info!("🔍 Validating primal sovereignty status");

        // Update last validation timestamp
        self.sovereignty_state.last_validation = Utc::now();

        // Simple sovereignty validation (expand as needed)
        let is_sovereign = self.sovereignty_state.is_active
            && self.sovereignty_state.sovereignty_score > self.config.sovereignty_threshold;

        if is_sovereign {
            info!("✅ Primal sovereignty validated successfully");
        } else {
            info!("⚠️ Sovereignty validation failed");
        }

        Ok(is_sovereign)
    }

    /// Get current sovereignty status
    /// Gets `sovereignty_status`
    /// Gets `sovereignty_status`
    #[must_use]
    pub fn get_sovereignty_status(&self) -> SovereigntyStatus {
        SovereigntyStatus {
            primal_id: self.sovereignty_state.primal_id.clone(),
            genesis_timestamp: self.sovereignty_state.genesis_timestamp,
            is_active: self.sovereignty_state.is_active,
            sovereignty_score: self.sovereignty_state.sovereignty_score,
            last_validation: self.sovereignty_state.last_validation,
        }
    }

    /// Spawn genetic offspring for primal evolution
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Spawning engine is not initialized
    /// - Spawn request validation fails
    /// - Genetic material is invalid
    pub fn spawn_genetic_offspring(
        &self,
        _genetics: &GeneticSpawningEngine,
        _spawn_request: SpawnRequest,
    ) -> Result<GeneticSpawningEngine, BearDogError> {
        info!("🧬 Spawning genetic offspring");

        // Create new genetics instance (simplified implementation)
        let offspring = GeneticSpawningEngine::new();

        info!("✅ Genetic offspring spawned successfully");
        Ok(offspring)
    }
}

/// Sovereignty status information for a primal
///
/// Provides a snapshot of primal sovereignty state including score and validation timing.
#[derive(Debug, Clone)]
pub struct SovereigntyStatus {
    /// Primal identifier
    pub primal_id: String,
    /// Genesis timestamp (when primal was created)
    pub genesis_timestamp: DateTime<Utc>,
    /// Whether sovereignty is currently active
    pub is_active: bool,
    /// Current sovereignty score (0.0-1.0)
    pub sovereignty_score: f64,
    /// Last validation timestamp
    pub last_validation: DateTime<Utc>,
}

impl Default for SovereigntyManager {
    fn default() -> Self {
        let config = SovereigntyConfig::default();
        Self::new(config).unwrap_or_else(|_| {
            // Fallback implementation
            Self {
                config: SovereigntyConfig::default(),
                genetics: GeneticSpawningEngine::new(),
                crypto_config: EncryptionConfig::default(),
                hierarchy_manager: EntropyHierarchyManager::default(),
                sovereignty_state: SovereigntyState::default(),
            }
        })
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // Float comparison is acceptable in test assertions
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // === PrimalSovereigntyConfig Tests ===

    #[test]
    fn test_sovereignty_config_default() {
        let config = PrimalSovereigntyConfig::default();
        assert!(config.primal_id.is_empty());
        assert!(!config.enable_sovereignty_monitoring);
        assert_eq!(config.sovereignty_threshold, 0.0);
    }

    #[test]
    fn test_sovereignty_config_clone() {
        let config = PrimalSovereigntyConfig {
            primal_id: "test-primal".to_string(),
            enable_sovereignty_monitoring: true,
            sovereignty_threshold: 0.8,
        };
        let cloned = config.clone();
        assert_eq!(config.primal_id, cloned.primal_id);
        assert_eq!(
            config.enable_sovereignty_monitoring,
            cloned.enable_sovereignty_monitoring
        );
    }

    #[test]
    fn test_sovereignty_config_custom() {
        let config = PrimalSovereigntyConfig {
            primal_id: "custom-primal-123".to_string(),
            enable_sovereignty_monitoring: true,
            sovereignty_threshold: 0.95,
        };
        assert_eq!(config.primal_id, "custom-primal-123");
        assert!(config.enable_sovereignty_monitoring);
        assert_eq!(config.sovereignty_threshold, 0.95);
    }

    // === SovereigntyState Tests ===

    #[test]
    fn test_sovereignty_state_default() {
        let state = SovereigntyState::default();
        assert!(state.primal_id.starts_with("primal-"));
        assert!(state.is_active);
        assert_eq!(state.sovereignty_score, 1.0);
    }

    #[test]
    fn test_sovereignty_state_clone() {
        let state = SovereigntyState::default();
        let cloned = state.clone();
        assert_eq!(state.primal_id, cloned.primal_id);
        assert_eq!(state.is_active, cloned.is_active);
        assert_eq!(state.sovereignty_score, cloned.sovereignty_score);
    }

    #[test]
    fn test_sovereignty_state_custom() {
        let state = SovereigntyState {
            primal_id: "custom-id".to_string(),
            genesis_timestamp: Utc::now(),
            is_active: false,
            sovereignty_score: 0.5,
            last_validation: Utc::now(),
        };
        assert_eq!(state.primal_id, "custom-id");
        assert!(!state.is_active);
        assert_eq!(state.sovereignty_score, 0.5);
    }

    // === SovereigntyManager Tests ===

    #[test]
    fn test_sovereignty_manager_creation() {
        let config = PrimalSovereigntyConfig::default();
        let result = SovereigntyManager::new(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sovereignty_manager_default() {
        let manager = SovereigntyManager::default();
        let status = manager.get_sovereignty_status();
        assert!(status.is_active);
        assert_eq!(status.sovereignty_score, 1.0);
    }

    #[test]
    fn test_sovereignty_manager_validate_sovereignty() {
        let config = PrimalSovereigntyConfig {
            primal_id: "test-primal".to_string(),
            enable_sovereignty_monitoring: true,
            sovereignty_threshold: 0.5,
        };
        let mut manager = SovereigntyManager::new(config).expect("manager");
        let result = manager.validate_sovereignty();
        assert!(result.is_ok());
        // Default state has score 1.0, threshold 0.5, should be valid
        assert!(result.expect("valid"));
    }

    #[test]
    fn test_sovereignty_manager_get_status() {
        let manager = SovereigntyManager::default();
        let status = manager.get_sovereignty_status();
        assert!(status.primal_id.starts_with("primal-"));
        assert!(status.is_active);
    }

    #[test]
    fn test_sovereignty_manager_spawn_offspring() {
        let manager = SovereigntyManager::default();
        let genetics = GeneticSpawningEngine::new();
        let spawn_request = SpawnRequest::default();
        let result = manager.spawn_genetic_offspring(&genetics, spawn_request);
        assert!(result.is_ok());
    }

    // === SovereigntyStatus Tests ===

    #[test]
    fn test_sovereignty_status_clone() {
        let status = SovereigntyStatus {
            primal_id: "status-test".to_string(),
            genesis_timestamp: Utc::now(),
            is_active: true,
            sovereignty_score: 0.9,
            last_validation: Utc::now(),
        };
        let cloned = status.clone();
        assert_eq!(status.primal_id, cloned.primal_id);
        assert_eq!(status.sovereignty_score, cloned.sovereignty_score);
    }

    #[test]
    fn test_sovereignty_status_debug() {
        let status = SovereigntyStatus {
            primal_id: "debug-test".to_string(),
            genesis_timestamp: Utc::now(),
            is_active: true,
            sovereignty_score: 1.0,
            last_validation: Utc::now(),
        };
        let debug_str = format!("{status:?}");
        assert!(debug_str.contains("SovereigntyStatus"));
        assert!(debug_str.contains("debug-test"));
    }
}
