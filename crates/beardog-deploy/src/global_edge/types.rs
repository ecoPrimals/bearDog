

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalDeploymentConfig {
    pub regions: Vec<RegionConfig>,
    pub load_balancing: LoadBalancingConfig,
    pub monitoring: MonitoringConfig,
    pub cdn: CDNConfig,
    pub auto_scaling: AutoScalingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionConfig {
    pub region_id: String,
    pub cloud_provider: CloudProvider,
    pub geographical_location: GeographicalLocation,
    pub capacity: RegionCapacity,
    pub node_configs: Vec<NodeConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    pub algorithm: RoutingAlgorithm,
    pub health_check_interval: Duration,
    pub failover_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub check_interval: Duration,
    pub alert_thresholds: AlertThresholds,
    pub metrics_retention: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CDNConfig {
    pub providers: Vec<CDNProvider>,
    pub cache_policies: HashMap<String, CachePolicy>,
    pub invalidation_strategy: InvalidationStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingConfig {
    pub enabled: bool,
    pub min_nodes_per_region: u32,
    pub max_nodes_per_region: u32,
    pub scale_up_threshold: f64,
    pub scale_down_threshold: f64,
    pub cooldown_period: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeRegion {
    pub region_id: String,
    pub region_name: String,
    pub cloud_provider: CloudProvider,
    pub geographical_location: GeographicalLocation,
    pub edge_nodes: Vec<EdgeNode>,
    pub capacity: RegionCapacity,
    pub status: RegionStatus,
    pub performance_metrics: RegionMetrics,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeNode {
    pub node_id: String,
    pub node_type: NodeType,
    pub endpoint_url: String,
    pub capacity: NodeCapacity,
    pub current_load: f64,
    pub health_status: HealthStatus,
    pub supported_protocols: Vec<String>,
    pub deployment_timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub node_type: NodeType,
    pub capacity: NodeCapacity,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloudProvider {
    AWS,
    GoogleCloud,
    Azure,
    DigitalOcean,
    Cloudflare,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicalLocation {
    pub continent: String,
    pub country: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {

    Compute,

    Storage,

    Network,

    Hybrid,

    Security,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapacity {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub storage_gb: u32,
    pub network_gbps: f64,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionCapacity {
    pub total_nodes: u32,
    pub total_cpu_cores: u32,
    pub total_memory_gb: u32,
    pub total_storage_gb: u32,
    pub total_network_gbps: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegionStatus {
    Active,
    Degraded,
    Maintenance,
    Offline,
}

// UNIFIED: Use canonical HealthStatus from beardog-types
pub use beardog_types::canonical::HealthStatus;
    Warning,
    Critical,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionMetrics {
    pub average_latency: f64,
    pub throughput_mbps: f64,
    pub error_rate: f64,
    pub uptime_percentage: f64,
    pub active_connections: u32,
    pub cache_hit_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoutingAlgorithm {

    Geographic,

    LoadBased,

    LatencyOptimized,

    AIOptimized,

    Hybrid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CDNProvider {
    Cloudflare,
    AWS,
    Azure,
    GoogleCloud,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePolicy {
    pub ttl: Duration,
    pub cache_control: String,
    pub vary_headers: Vec<String>,
    pub compression: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvalidationStrategy {
    Immediate,
    Scheduled,
    TagBased,
    PathBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub latency_ms: f64,
    pub error_rate_percent: f64,
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalHealthStatus {
    pub overall_status: HealthStatus,
    pub region_statuses: HashMap<String, HealthStatus>,
    pub total_nodes: u32,
    pub healthy_nodes: u32,
    pub average_latency: f64,
    pub total_throughput: f64,
}

impl Default for GlobalDeploymentConfig {
    fn default() -> Self {
        Self {
            regions: Vec::new(),
            load_balancing: LoadBalancingConfig::default(),
            monitoring: MonitoringConfig::default(),
            cdn: CDNConfig::default(),
            auto_scaling: AutoScalingConfig::default(),
        }
    }
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            algorithm: RoutingAlgorithm::Hybrid,
            health_check_interval: Duration::from_secs(30),
            failover_threshold: 0.8,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(60),
            alert_thresholds: AlertThresholds::default(),
            metrics_retention: Duration::from_secs(86400 * 7), // 7 days
        }
    }
}

impl Default for CDNConfig {
    fn default() -> Self {
        Self {
            providers: vec![CDNProvider::Cloudflare],
            cache_policies: HashMap::with_capacity(16),
            invalidation_strategy: InvalidationStrategy::TagBased,
        }
    }
}

impl Default for AutoScalingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_nodes_per_region: 2,
            max_nodes_per_region: 10,
            scale_up_threshold: 0.8,
            scale_down_threshold: 0.3,
            cooldown_period: Duration::from_secs(300),
        }
    }
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            latency_ms: 1000.0,
            error_rate_percent: 5.0,
            cpu_usage_percent: 80.0,
            memory_usage_percent: 85.0,
            disk_usage_percent: 90.0,
        }
    }
} 