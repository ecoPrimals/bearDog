

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
    /// The load balancing value
    pub load_balancing: LoadBalancingConfig,
    /// The monitoring value
    pub monitoring: MonitoringConfig,
    /// The cdn value
    pub cdn: CDNConfig,
    /// The auto scaling value
    pub auto_scaling: AutoScalingConfig,
}

#[derive(Debug, Clone)]
    pub cloud_provider: CloudProvider,
    /// The geographical location value
    pub geographical_location: GeographicalLocation,
    /// The capacity value
    pub capacity: RegionCapacity,
    pub node_configs: Vec<NodeConfig>,
}

#[derive(Debug, Clone)]
    /// The health check interval value
    pub health_check_interval: Duration,
    /// The failover threshold value
    pub failover_threshold: f64,
}

#[derive(Debug, Clone)]
    /// The alert thresholds value
    pub alert_thresholds: AlertThresholds,
    /// The metrics retention value
    pub metrics_retention: Duration,
}

#[derive(Debug, Clone)]
    /// Mapping of cache policies
    pub cache_policies: HashMap<String, CachePolicy>,
    pub invalidation_strategy: InvalidationStrategy,
}

#[derive(Debug, Clone)]
    /// Number of min_nodes_per_region
    pub min_nodes_per_region: u32,
    /// Number of max_nodes_per_region
    pub max_nodes_per_region: u32,
    /// The scale up threshold value
    pub scale_up_threshold: f64,
    /// The scale down threshold value
    pub scale_down_threshold: f64,
    /// The cooldown period value
    pub cooldown_period: Duration,
}

#[derive(Debug, Clone)]
    /// Name of the region
    pub region_name: String,
    pub cloud_provider: CloudProvider,
    /// The geographical location value
    pub geographical_location: GeographicalLocation,
    /// Collection of edge nodes
    pub edge_nodes: Vec<EdgeNode>,
    /// The capacity value
    pub capacity: RegionCapacity,
    /// Current status of the component
    pub status: RegionStatus,
    pub performance_metrics: RegionMetrics,
    /// Number of last_updated
    pub last_updated: u64,
}

#[derive(Debug, Clone)]
    /// The node type value
    pub node_type: NodeType,
    /// The endpoint url value
    pub endpoint_url: String,
    /// The capacity value
    pub capacity: NodeCapacity,
    /// The current load value
    pub current_load: f64,
    /// Current status of the health
    pub health_status: HealthStatus,
    /// Collection of supported protocols
    pub supported_protocols: Vec<String>,
    pub deployment_timestamp: u64,
}

#[derive(Debug, Clone)]
    /// The capacity value
    pub capacity: NodeCapacity,
    /// Number of items
    pub count: u32,
}

#[derive(Debug, Clone)]
    /// Number of itemsry
    pub country: String,
    /// The city value
    pub city: String,
    /// The latitude value
    pub latitude: f64,
    /// The longitude value
    pub longitude: f64,
    pub timezone: String,
}

#[derive(Debug, Clone)]
    /// Number of memory_gb
    pub memory_gb: u32,
    /// Number of storage_gb
    pub storage_gb: u32,
    /// The network gbps value
    pub network_gbps: f64,
    /// Number of max_connections
    pub max_connections: u32,
}

#[derive(Debug, Clone)]
    /// Number of total_cpu_cores
    pub total_cpu_cores: u32,
    /// Number of total_memory_gb
    pub total_memory_gb: u32,
    /// Number of total_storage_gb
    pub total_storage_gb: u32,
    /// The total network gbps value
    pub total_network_gbps: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegionStatus {
    /// Active or enabled state
    Active,
    /// State indicating degraded
    Degraded,
    /// Represents maintenance variant
    Maintenance,
    /// Represents offline variant
    Offline,
}

pub use beardog_types::canonical::HealthStatus;
    Warning,
    Critical,
    Offline,
}

#[derive(Debug, Clone)]
    /// The throughput mbps value
    pub throughput_mbps: f64,
    /// The error rate value
    pub error_rate: f64,
    pub uptime_percentage: f64,
    /// Number of active_connections
    pub active_connections: u32,
    /// The cache hit rate value
    pub cache_hit_rate: f64,
}

#[derive(Debug, Clone)]
    /// The cache control value
    pub cache_control: String,
    /// Collection of vary headers
    pub vary_headers: Vec<String>,
    /// Whether compression is enabled
    pub compression: bool,
}

#[derive(Debug, Clone)]
    /// The error rate percent value
    pub error_rate_percent: f64,
    /// The cpu usage percent value
    pub cpu_usage_percent: f64,
    /// The memory usage percent value
    pub memory_usage_percent: f64,
    /// The disk usage percent value
    pub disk_usage_percent: f64,
}

#[derive(Debug, Clone)]
    /// Current status of the regiones
    pub region_statuses: HashMap<String, HealthStatus>,
    /// Number of total_nodes
    pub total_nodes: u32,
    /// Number of healthy_nodes
    pub healthy_nodes: u32,
    /// The average latency value
    pub average_latency: f64,
    /// The total throughput value
    pub total_throughput: f64,
}

impl Default for GlobalDeploymentConfig {
    fn default() -> Self {
        Self {
            regions: Vec::new(),
            load_balancing: LoadBalancingConfig::default(),
            monitoring: MonitoringConfig::default(),
            cdn: CDNConfig::default(),
            auto_scaling: AutoScalingConfig::default(RoutingAlgorithm::Hybrid,
            health_check_interval: Duration::from_secs(0.8,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(
                std::env::var("BEARDOG_EDGE_MONITORING_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60)
            ),
            alert_thresholds: AlertThresholds::default(),
            metrics_retention: Duration::from_secs(vec![CDNProvider::Cloudflare],
            cache_policies: HashMap::with_capacity(InvalidationStrategy::TagBased,
        }
    }
}

impl Default for AutoScalingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_nodes_per_region: std::env::var("BEARDOG_DEPLOY_MIN_NODES_PER_REGION")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(2),
            max_nodes_per_region: std::env::var("BEARDOG_DEPLOY_MAX_NODES_PER_REGION")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            scale_up_threshold: std::env::var("BEARDOG_DEPLOY_SCALE_UP_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.8),
            scale_down_threshold: std::env::var("BEARDOG_DEPLOY_SCALE_DOWN_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.3),
            cooldown_period: Duration::from_secs(
                std::env::var("BEARDOG_DEPLOY_COOLDOWN_PERIOD_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(300)
            ),
            error_rate_percent: std::env::var("BEARDOG_DEPLOY_ERROR_RATE_THRESHOLD_PERCENT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5.0),
            cpu_usage_percent: std::env::var("BEARDOG_DEPLOY_CPU_USAGE_THRESHOLD_PERCENT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(80.0),
            memory_usage_percent: std::env::var("BEARDOG_DEPLOY_MEMORY_USAGE_THRESHOLD_PERCENT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(85.0),
            disk_usage_percent: std::env::var("BEARDOG_DEPLOY_DISK_USAGE_THRESHOLD_PERCENT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(90.0),
        }
    }
} 
