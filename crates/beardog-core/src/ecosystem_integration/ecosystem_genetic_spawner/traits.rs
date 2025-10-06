// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::types::{
    EcosystemCapability, EcosystemGeneticBlueprint, EcosystemResourceAllocation, TraitCategory,
};
use beardog_errors::BearDogError;
use std::collections::HashMap;

#[allow(async_fn_in_trait)]
pub trait EcosystemPrimalClient: Send + Sync {
    /// Gets `primal_id`
    fn get_primal_id(&self, capabilities: &[EcosystemCapability]) -> Result<bool, BearDogError>;

    /// Gets `resource_availability`
    fn get_resource_availability(
        &self,
        allocation: &EcosystemResourceAllocation,
    ) -> Result<String, BearDogError>;

    fn release_resources(&self, reservation_id: &str) -> Result<(), BearDogError>;

    /// Creates `hybrid_component`
    fn create_hybrid_component(
        &self,
        blueprint: &EcosystemGeneticBlueprint,
    ) -> Result<serde_json::Value, BearDogError>;

    fn health_check(&self) -> Result<bool, BearDogError>;

    fn get_resource_utilization(&self) -> Result<HashMap<String, f64>, BearDogError>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EcosystemGeneticTrait {
    pub trait_id: String,
    /// Name of the trait
    pub trait_name: String,
    /// The category value
    pub category: TraitCategory,
    /// The strength value
    pub strength: f64,
    /// The dominance value
    pub dominance: f64,
    /// Collection of required capabilities
    pub required_capabilities: Vec<EcosystemCapability>,
    pub trait_config: HashMap<String, serde_json::Value>,
    /// The source primal value
    pub source_primal: String,
    /// Mapping of compatibility scores
    pub compatibility_scores: HashMap<String, f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EcosystemResourceAvailability {
    /// Number of `available_cpu_cores`
    pub available_cpu_cores: u32,
    /// Number of `available_memory_gb`
    pub available_memory_gb: u32,
    /// Number of `available_storage_gb`
    pub available_storage_gb: u32,
    pub available_bandwidth_mbps: u32,
    /// Number of `available_gpu_units`
    pub available_gpu_units: u32,
    /// The cost per hour value
    pub cost_per_hour: f64,
    /// Collection of available locations
    pub available_locations: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
