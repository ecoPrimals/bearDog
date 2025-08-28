

use super::types::*;
use beardog_errors::BearDogError;
use std::collections::HashMap;

#[allow(async_fn_in_trait)]
pub trait EcosystemPrimalClient: Send + Sync {

    fn get_primal_id(&self) -> String;

    fn get_primal_name(&self) -> String;

    async fn get_available_genetic_traits(&self) -> Result<Vec<EcosystemGeneticTrait>, BearDogError>;

    async fn supports_capabilities(&self, capabilities: &[EcosystemCapability]) -> Result<bool, BearDogError>;

    async fn get_resource_availability(&self) -> Result<EcosystemResourceAvailability, BearDogError>;

    async fn reserve_resources(&self, allocation: &EcosystemResourceAllocation) -> Result<String, BearDogError>;

    async fn release_resources(&self, reservation_id: &str) -> Result<(), BearDogError>;

    async fn create_hybrid_component(&self, blueprint: &EcosystemGeneticBlueprint) -> Result<serde_json::Value, BearDogError>;

    async fn health_check(&self) -> Result<bool, BearDogError>;

    async fn get_resource_utilization(&self) -> Result<HashMap<String, f64>, BearDogError>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EcosystemGeneticTrait {

    pub trait_id: String,

    pub trait_name: String,

    pub category: TraitCategory,

    pub strength: f64,

    pub dominance: f64,

    pub required_capabilities: Vec<EcosystemCapability>,

    pub trait_config: HashMap<String, serde_json::Value>,

    pub source_primal: String,

    pub compatibility_scores: HashMap<String, f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EcosystemResourceAvailability {

    pub available_cpu_cores: u32,

    pub available_memory_gb: u32,

    pub available_storage_gb: u32,

    pub available_bandwidth_mbps: u32,

    pub available_gpu_units: u32,

    pub cost_per_hour: f64,

    pub available_locations: Vec<String>,

    pub timestamp: chrono::DateTime<chrono::Utc>,
} 