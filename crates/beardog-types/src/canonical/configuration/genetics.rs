use serde::{Deserialize, Serialize};

/// Canonical genetics configuration - consolidates all genetics-related configs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct GeneticsConfig {
    pub genesis: GenesisConfig,
    pub network: GeneticsNetworkConfig,
    pub system: GeneticsSystemConfig,
    pub spawning: GeneticsSpawningConfig,
    pub entropy: GeneticsEntropyConfig,
}


/// Genesis configuration for initial genetics creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisConfig {
    pub initial_population_size: usize,
    pub min_genetic_diversity: f64,
    pub max_generations: u32,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub elite_preservation: f64,
    pub fitness_threshold: f64,
}

impl Default for GenesisConfig {
    fn default() -> Self {
        Self {
            initial_population_size: 10,
            min_genetic_diversity: 0.3,
            max_generations: 100,
            mutation_rate: 0.1,
            crossover_rate: 0.8,
            elite_preservation: 0.2,
            fitness_threshold: 0.7,
        }
    }
}

/// Network configuration for P2P genetics operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsNetworkConfig {
    pub max_nodes: usize,
    pub heartbeat_interval_seconds: u64,
    pub node_timeout_minutes: u64,
    pub enable_discovery: bool,
    pub discovery_port: u16,
    pub max_connections_per_node: u32,
}

impl Default for GeneticsNetworkConfig {
    fn default() -> Self {
        Self {
            max_nodes: 100,
            heartbeat_interval_seconds: 30,
            node_timeout_minutes: 5,
            enable_discovery: true,
            discovery_port: 8333,
            max_connections_per_node: 10,
        }
    }
}

/// System configuration for genetics management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsSystemConfig {
    pub max_stored_genetics: usize,
    pub cleanup_interval_seconds: u64,
    pub enable_metrics_collection: bool,
    pub fitness_threshold: f64,
    pub storage_path: String,
    pub enable_backup: bool,
    pub backup_interval_hours: u64,
}

impl Default for GeneticsSystemConfig {
    fn default() -> Self {
        Self {
            max_stored_genetics: 1000,
            cleanup_interval_seconds: 3600, // 1 hour
            enable_metrics_collection: true,
            fitness_threshold: 0.5,
            storage_path: "/var/lib/beardog/genetics".to_string(),
            enable_backup: true,
            backup_interval_hours: 24,
        }
    }
}

/// Spawning configuration for genetics operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsSpawningConfig {
    pub max_concurrent_spawns: u32,
    pub spawn_timeout_seconds: u64,
    pub enable_advanced_recombination: bool,
    pub recombination_complexity: f64,
    pub enable_mutation_tracking: bool,
    pub quality_threshold: f64,
}

impl Default for GeneticsSpawningConfig {
    fn default() -> Self {
        Self {
            max_concurrent_spawns: 10,
            spawn_timeout_seconds: 300, // 5 minutes
            enable_advanced_recombination: true,
            recombination_complexity: 0.8,
            enable_mutation_tracking: true,
            quality_threshold: 0.6,
        }
    }
}

/// Entropy collection configuration for genetics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsEntropyConfig {
    pub collection_method: EntropyCollectionMethod,
    pub min_entropy_bits: u32,
    pub collection_timeout_seconds: u64,
    pub enable_human_entropy: bool,
    pub privacy_level: EntropyPrivacyLevel,
    pub buffer_size: usize,
}

/// Methods for collecting entropy for genetics operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntropyCollectionMethod {
    Hardware,
    Software,
    Hybrid,
    HumanInteraction,
}

/// Privacy levels for entropy collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntropyPrivacyLevel {
    Minimal,
    Standard,
    Enhanced,
    Maximum,
}

impl Default for GeneticsEntropyConfig {
    fn default() -> Self {
        Self {
            collection_method: EntropyCollectionMethod::Hybrid,
            min_entropy_bits: 256,
            collection_timeout_seconds: 30,
            enable_human_entropy: true,
            privacy_level: EntropyPrivacyLevel::Standard,
            buffer_size: 4096,
        }
    }
} 