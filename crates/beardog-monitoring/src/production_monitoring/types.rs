

use beardog_genetics::{GeneticSignature, BiomeIdentity, TrustLevel};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize};

#[derive(Debug, Clone)]
    /// The alert thresholds value
    pub alert_thresholds: AlertThresholds,
    /// Number of health_check_interval_seconds
    pub health_check_interval_seconds: u32,
    pub performance_sampling_rate: f64,
    /// The security log level value
    pub security_log_level: SecurityLogLevel,
    pub enable_real_time_analytics: bool,
    /// Number of dashboard_refresh_seconds
    pub dashboard_refresh_seconds: u32,
}

#[derive(Debug, Clone)]
    /// The genetic quality minimum value
    pub genetic_quality_minimum: f64,
    pub response_time_max_ms: u64,
    /// The memory usage max percent value
    pub memory_usage_max_percent: f64,
    /// The cpu usage max percent value
    pub cpu_usage_max_percent: f64,
    /// The biome connectivity min percent value
    pub biome_connectivity_min_percent: f64,
    /// The error rate max percent value
    pub error_rate_max_percent: f64,
}

#[derive(Debug, Clone)]
    /// The biome type value
    pub biome_type: String,
    /// The trust level value
    pub trust_level: TrustLevel,
    /// The total operations value
    pub total_operations: AtomicU64,
    /// The successful operations value
    pub successful_operations: AtomicU64,
    /// The failed operations value
    pub failed_operations: AtomicU64,
    pub average_response_time_ms: AtomicU64,
    /// The genetic quality score value
    pub genetic_quality_score: AtomicU64, // Fixed-point representation
    /// The last activity value
    pub last_activity: parking_lot::Mutex<DateTime<Utc>>,
    /// Number of collaboration
    pub collaboration_count: AtomicU64,
    pub security_incidents: AtomicU64,
}

#[derive(Debug, Clone)]
    /// The active biomes value
    pub active_biomes: AtomicUsize,
    /// The total authorizations value
    pub total_authorizations: AtomicU64,
    /// The memory usage bytes value
    pub memory_usage_bytes: AtomicU64,
    /// The cpu usage percent value
    pub cpu_usage_percent: AtomicU64,
    /// The network throughput bps value
    pub network_throughput_bps: AtomicU64,
    pub uptime_seconds: AtomicU64,
    /// The system health score value
    pub system_health_score: AtomicU64,
}

#[derive(Debug, Clone)]
    /// The spawning events value
    pub spawning_events: AtomicU64,
    /// The evolution cycles value
    pub evolution_cycles: AtomicU64,
    /// The average genetic quality value
    pub average_genetic_quality: AtomicU64,
    /// The genetic diversity index value
    pub genetic_diversity_index: AtomicU64,
    /// The consensus participation value
    pub consensus_participation: AtomicU64,
}

#[derive(Debug, Clone)]
    /// The successful authorizations value
    pub successful_authorizations: AtomicU64,
    /// The failed authorizations value
    pub failed_authorizations: AtomicU64,
    pub average_authorization_time_ms: AtomicU64,
    /// The trust score distribution value
    pub trust_score_distribution: parking_lot::Mutex<HashMap<String, u64>>,
}

#[derive(Debug, Clone)]
    /// The memory pool utilization value
    pub memory_pool_utilization: AtomicU64,
    /// The simd operations value
    pub simd_operations: AtomicU64,
    /// The lock contention events value
    pub lock_contention_events: AtomicU64,
    /// The gc events value
    pub gc_events: AtomicU64,
    /// The average operation latency ns value
    pub average_operation_latency_ns: AtomicU64,
}

#[derive(Debug, Clone)]
    /// The system health value
    pub system_health: f64,
    /// Number of active_biomes
    pub active_biomes: usize,
    /// Number of total_operations
    pub total_operations: u64,
    pub average_response_time: u64,
    /// The genetic quality avg value
    pub genetic_quality_avg: f64,
    /// The authorization success rate value
    pub authorization_success_rate: f64,
}

#[derive(Debug, Clone)]
    /// The alert type value
    pub alert_type: AlertType,
    /// The severity value
    pub severity: AlertSeverity,
    /// The source component value
    pub source_component: String,
    /// The message value
    pub message: String,
    pub timestamp: DateTime<Utc>,
    /// Whether resolved is enabled
    pub resolved: bool,
    pub resolution_timestamp: Option<DateTime<Utc>>,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    /// Current status of the component
    pub status: HealthStatus,
    pub timestamp: DateTime<Utc>,
    pub response_time_ms: u64,
    /// Mapping of details
    pub details: HashMap<String, String>,
    /// The score value
    pub score: f64, // 0.0 to 1.0
}

#[derive(Debug, Clone)]
    /// The component value
    pub component: String,
    /// The severity value
    pub severity: f64,
    /// The impact description value
    pub impact_description: String,
    /// Collection of suggested solutions
    pub suggested_solutions: Vec<String>,
    /// The detected at value
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
    /// The component value
    pub component: String,
    /// The description value
    pub description: String,
    /// The expected improvement value
    pub expected_improvement: f64,
    pub implementation_effort: EffortLevel,
    /// The priority value
    pub priority: Priority,
    /// The generated at value
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
    /// The event type value
    pub event_type: SecurityEventType,
    /// The severity value
    pub severity: AlertSeverity,
    /// Optional source biome
    pub source_biome: Option<String>,
    /// Optional target biome
    pub target_biome: Option<String>,
    /// The description value
    pub description: String,
    pub timestamp: DateTime<Utc>,
    /// Whether resolved is enabled
    pub resolved: bool,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    pub biome_identity: BiomeIdentity,
    pub registration_time: DateTime<Utc>,
    /// The last seen value
    pub last_seen: DateTime<Utc>,
    /// Current status of the connection
    pub connection_status: ConnectionStatus,
    pub performance_history: Vec<PerformanceDataPoint>,
    /// The security score value
    pub security_score: f64,
    /// Collection of collaboration partners
    pub collaboration_partners: Vec<String>,
}

#[derive(Debug, Clone)]
    pub response_time_ms: u64,
    /// The success rate value
    pub success_rate: f64,
    /// The genetic quality value
    pub genetic_quality: f64,
    /// The resource usage value
    pub resource_usage: f64,
}

#[derive(Debug, Clone)]
    /// The system metrics value
    pub system_metrics: SystemMetricsSnapshot,
    /// Mapping of biome metrics
    pub biome_metrics: HashMap<String, BiomeMetricsSnapshot>,
    pub performance_metrics: PerformanceMetricsSnapshot,
}

#[derive(Debug, Clone)]
    /// Number of active_biomes
    pub active_biomes: usize,
    /// Number of memory_usage_bytes
    pub memory_usage_bytes: u64,
    /// Number of cpu_usage_percent
    pub cpu_usage_percent: u64,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone)]
    /// The operations per second value
    pub operations_per_second: f64,
    /// The success rate value
    pub success_rate: f64,
    pub response_time_ms: u64,
    /// The genetic quality value
    pub genetic_quality: f64,
}

#[derive(Debug, Clone)]
    /// Number of memory_pool_utilization
    pub memory_pool_utilization: u64,
    /// Number of average_latency_ns
    pub average_latency_ns: u64,
    /// Number of lock_contention_events
    pub lock_contention_events: u64,
}

impl Default for MonitoringConfig {
    fn default(24,
            alert_thresholds: AlertThresholds::default(30,
            performance_sampling_rate: 0.1,
            security_log_level: SecurityLogLevel::Medium,
            enable_real_time_analytics: true,
            dashboard_refresh_seconds: 5,
        }
    }
}

impl Default for AlertThresholds {
    fn default(0.05,
            genetic_quality_minimum: 0.7,
            response_time_max_ms: 1000,
            memory_usage_max_percent: 80.0,
            cpu_usage_max_percent: 85.0,
            biome_connectivity_min_percent: 90.0,
            error_rate_max_percent: 5.0,
        }
    }
} 
