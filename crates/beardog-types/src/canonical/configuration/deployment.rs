use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Canonical deployment configuration - consolidates all deployment-related configs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct DeploymentConfig {
    pub global: GlobalDeploymentConfig,
    pub optimization: DeploymentOptimizationConfig,
    pub regional: RegionalDeploymentConfig,
    pub monitoring: DeploymentMonitoringConfig,
    pub security: DeploymentSecurityConfig,
}


/// Global deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalDeploymentConfig {
    pub regions: Vec<RegionConfig>,
    pub load_balancing: LoadBalancingConfig,
    pub cdn: CDNConfig,
    pub auto_scaling: AutoScalingConfig,
    pub environment: String,
    pub instance_count: u32,
}

impl Default for GlobalDeploymentConfig {
    fn default() -> Self {
        Self {
            regions: vec![],
            load_balancing: LoadBalancingConfig::default(),
            cdn: CDNConfig::default(),
            auto_scaling: AutoScalingConfig::default(),
            environment: "production".to_string(),
            instance_count: 3,
        }
    }
}

/// Deployment optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentOptimizationConfig {
    pub enable_ai_optimization: bool,
    pub performance_weight: f64,
    pub cost_weight: f64,
    pub reliability_weight: f64,
    pub max_deployment_time_minutes: u32,
    pub enable_auto_rollback: bool,
    pub rollback_threshold: f64,
}

impl Default for DeploymentOptimizationConfig {
    fn default() -> Self {
        Self {
            enable_ai_optimization: true,
            performance_weight: 0.4,
            cost_weight: 0.3,
            reliability_weight: 0.3,
            max_deployment_time_minutes: 30,
            enable_auto_rollback: true,
            rollback_threshold: 0.95,
        }
    }
}

/// Regional deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalDeploymentConfig {
    pub primary_region: String,
    pub failover_regions: Vec<String>,
    pub region_capacity: RegionCapacityConfig,
    pub inter_region_replication: bool,
    pub data_locality_requirements: HashMap<String, String>,
}

impl Default for RegionalDeploymentConfig {
    fn default() -> Self {
        Self {
            primary_region: "us-east-1".to_string(),
            failover_regions: vec!["us-west-2".to_string(), "eu-west-1".to_string()],
            region_capacity: RegionCapacityConfig::default(),
            inter_region_replication: true,
            data_locality_requirements: HashMap::new(),
        }
    }
}

/// Region configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionConfig {
    pub region_id: String,
    pub cloud_provider: CloudProvider,
    pub geographical_location: GeographicalLocation,
    pub capacity: RegionCapacityConfig,
    pub node_configs: Vec<NodeConfig>,
}

/// Cloud provider options
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloudProvider {
    AWS,
    GoogleCloud,
    Azure,
    DigitalOcean,
    Cloudflare,
    Custom(String),
}

/// Geographical location for deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicalLocation {
    pub continent: String,
    pub country: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    pub algorithm: RoutingAlgorithm,
    pub health_check_interval: Duration,
    pub failover_threshold: f64,
    pub sticky_sessions: bool,
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            algorithm: RoutingAlgorithm::WeightedRoundRobin,
            health_check_interval: Duration::from_secs(30),
            failover_threshold: 0.95,
            sticky_sessions: false,
        }
    }
}

/// Routing algorithms for load balancing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingAlgorithm {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    PerformanceBased,
    GeographicallyAware,
}

/// CDN configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CDNConfig {
    pub providers: Vec<CDNProvider>,
    pub cache_policies: HashMap<String, CachePolicy>,
    pub invalidation_strategy: InvalidationStrategy,
    pub global_cache_ttl: Duration,
}

impl Default for CDNConfig {
    fn default() -> Self {
        Self {
            providers: vec![CDNProvider::Cloudflare],
            cache_policies: HashMap::new(),
            invalidation_strategy: InvalidationStrategy::Immediate,
            global_cache_ttl: Duration::from_secs(3600),
        }
    }
}

/// CDN provider options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CDNProvider {
    Cloudflare,
    AWS,
    GoogleCloud,
    Azure,
    Custom(String),
}

/// Cache policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePolicy {
    pub ttl: Duration,
    pub max_size_mb: u64,
    pub compression: bool,
}

/// Cache invalidation strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvalidationStrategy {
    Immediate,
    Scheduled,
    LazyExpiration,
    TagBased,
}

/// Auto-scaling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingConfig {
    pub enabled: bool,
    pub min_nodes_per_region: u32,
    pub max_nodes_per_region: u32,
    pub scale_up_threshold: f64,
    pub scale_down_threshold: f64,
    pub cooldown_period: Duration,
    pub target_cpu_utilization: f64,
}

impl Default for AutoScalingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_nodes_per_region: 2,
            max_nodes_per_region: 100,
            scale_up_threshold: 0.7,
            scale_down_threshold: 0.3,
            cooldown_period: Duration::from_secs(300),
            target_cpu_utilization: 0.6,
        }
    }
}

/// Region capacity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionCapacityConfig {
    pub total_nodes: u32,
    pub total_cpu_cores: u32,
    pub total_memory_gb: u32,
    pub total_storage_gb: u32,
    pub total_network_gbps: f64,
}

impl Default for RegionCapacityConfig {
    fn default() -> Self {
        Self {
            total_nodes: 10,
            total_cpu_cores: 80,
            total_memory_gb: 640,
            total_storage_gb: 10000,
            total_network_gbps: 100.0,
        }
    }
}

/// Node configuration for deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub node_type: NodeType,
    pub capacity: NodeCapacityConfig,
    pub count: u32,
    pub enable_monitoring: bool,
}

/// Node types for deployment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Compute,
    Storage,
    Network,
    Hybrid,
    Security,
}

/// Node capacity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapacityConfig {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub storage_gb: u32,
    pub network_gbps: f64,
    pub max_connections: u32,
}

impl Default for NodeCapacityConfig {
    fn default() -> Self {
        Self {
            cpu_cores: 8,
            memory_gb: 64,
            storage_gb: 1000,
            network_gbps: 10.0,
            max_connections: 1000,
        }
    }
}

/// Deployment monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentMonitoringConfig {
    pub enable_health_checks: bool,
    pub health_check_interval: Duration,
    pub alert_thresholds: DeploymentAlertThresholds,
    pub metrics_retention: Duration,
    pub enable_distributed_tracing: bool,
}

impl Default for DeploymentMonitoringConfig {
    fn default() -> Self {
        Self {
            enable_health_checks: true,
            health_check_interval: Duration::from_secs(30),
            alert_thresholds: DeploymentAlertThresholds::default(),
            metrics_retention: Duration::from_secs(86400 * 30), // 30 days
            enable_distributed_tracing: true,
        }
    }
}

/// Alert thresholds for deployment monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentAlertThresholds {
    pub cpu_usage_threshold: f64,
    pub memory_usage_threshold: f64,
    pub disk_usage_threshold: f64,
    pub network_latency_threshold_ms: u32,
    pub error_rate_threshold: f64,
    pub availability_threshold: f64,
}

impl Default for DeploymentAlertThresholds {
    fn default() -> Self {
        Self {
            cpu_usage_threshold: 0.8,
            memory_usage_threshold: 0.85,
            disk_usage_threshold: 0.9,
            network_latency_threshold_ms: 100,
            error_rate_threshold: 0.01,
            availability_threshold: 0.99,
        }
    }
}

/// Security configuration for deployments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSecurityConfig {
    pub enable_encryption_at_rest: bool,
    pub enable_encryption_in_transit: bool,
    pub certificate_management: CertificateConfig,
    pub access_control: AccessControlConfig,
    pub audit_logging: bool,
}

impl Default for DeploymentSecurityConfig {
    fn default() -> Self {
        Self {
            enable_encryption_at_rest: true,
            enable_encryption_in_transit: true,
            certificate_management: CertificateConfig::default(),
            access_control: AccessControlConfig::default(),
            audit_logging: true,
        }
    }
}

/// Certificate management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateConfig {
    pub auto_renewal: bool,
    pub renewal_threshold_days: u32,
    pub certificate_authority: String,
    pub key_algorithm: String,
    pub key_size: u32,
}

impl Default for CertificateConfig {
    fn default() -> Self {
        Self {
            auto_renewal: true,
            renewal_threshold_days: 30,
            certificate_authority: "letsencrypt".to_string(),
            key_algorithm: "ecdsa".to_string(),
            key_size: 256,
        }
    }
}

/// Access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    pub enable_rbac: bool,
    pub default_permissions: Vec<String>,
    pub admin_roles: Vec<String>,
    pub session_timeout: Duration,
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            enable_rbac: true,
            default_permissions: vec!["read".to_string()],
            admin_roles: vec!["admin".to_string(), "operator".to_string()],
            session_timeout: Duration::from_secs(3600),
        }
    }
} 