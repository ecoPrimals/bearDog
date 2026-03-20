// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::types::{
    EcosystemCapability, EcosystemGeneticBlueprint, EcosystemResourceAllocation, TraitCategory,
};
use beardog_errors::BearDogError;
use std::collections::HashMap;

/// Client interface for interacting with ecosystem primals
///
/// Provides methods for querying primal capabilities, resource availability,
/// and requesting primal spawning operations.
#[allow(
    async_fn_in_trait,
    reason = "Async ecosystem hooks for async trait ecosystem integration"
)]
pub trait EcosystemPrimalClient: Send + Sync {
    /// Checks if a primal with given capabilities exists
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    fn get_primal_id(&self, capabilities: &[EcosystemCapability]) -> Result<bool, BearDogError>;

    /// Gets `resource_availability`
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    fn get_resource_availability(
        &self,
        allocation: &EcosystemResourceAllocation,
    ) -> Result<String, BearDogError>;

    /// Release resources
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    fn release_resources(&self, reservation_id: &str) -> Result<(), BearDogError>;

    /// Creates `hybrid_component`
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    fn create_hybrid_component(
        &self,
        blueprint: &EcosystemGeneticBlueprint,
    ) -> Result<serde_json::Value, BearDogError>;

    /// Health check
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    fn health_check(&self) -> Result<bool, BearDogError>;

    /// Get resource utilization
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    fn get_resource_utilization(&self) -> Result<HashMap<String, f64>, BearDogError>;
}

/// A genetic trait for ecosystem primal evolution
///
/// Represents an inheritable characteristic used in genetic spawning algorithms
/// to create new primal instances with evolved capabilities.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EcosystemGeneticTrait {
    /// Unique trait identifier
    pub trait_id: String,
    /// Human-readable trait name
    pub trait_name: String,
    /// Trait category classification
    pub category: TraitCategory,
    /// Trait strength (0.0-1.0)
    pub strength: f64,
    /// Dominance in genetic combination (0.0-1.0)
    pub dominance: f64,
    /// Capabilities required for this trait
    pub required_capabilities: Vec<EcosystemCapability>,
    /// Trait-specific configuration
    pub trait_config: HashMap<String, serde_json::Value>,
    /// Source primal that contributed this trait
    pub source_primal: String,
    /// Compatibility scores with other traits
    pub compatibility_scores: HashMap<String, f64>,
}

/// Available resources for ecosystem operations
///
/// Tracks current resource availability across compute, memory, storage,
/// network, and GPU resources with associated costs.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EcosystemResourceAvailability {
    /// Available CPU cores
    pub available_cpu_cores: u32,
    /// Available memory in GB
    pub available_memory_gb: u32,
    /// Available storage in GB
    pub available_storage_gb: u32,
    /// Available network bandwidth in Mbps
    pub available_bandwidth_mbps: u32,
    /// Available GPU units
    pub available_gpu_units: u32,
    /// Cost per hour for these resources
    pub cost_per_hour: f64,
    /// Geographic locations where resources are available
    pub available_locations: Vec<String>,
    /// When this availability snapshot was taken
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
