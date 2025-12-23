// Metrics Types and Configurations
//
// This module provides metrics-related types and configurations for the BearDog ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Name of the hostitem
    pub hostname: String,
    /// The cpu value
    pub cpu: CpuMetrics,
    /// Memory usage statistics and availability
    /// The memory value
    pub memory: MemoryMetrics,
    /// The disk value
    pub disk: DiskMetrics,
    /// Network throughput and connectivity metrics
    /// The network value
    pub network: NetworkMetrics,
    /// Process-specific resource consumption
    /// The process value
    pub process: ProcessMetrics,
    /// Custom application-specific metrics
    /// Mapping of custom
    pub custom: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    /// Overall CPU utilization as a percentage (0.0-100.0)
    /// The usage percent value
    pub usage_percent: f64,
    /// Collection of per core usage
    pub per_core_usage: Vec<f64>,
    /// The load average value
    pub load_average: LoadAverage,
    /// CPU temperature in Celsius (if available from sensors)
    /// Optional temperature celsius
    pub temperature_celsius: Option<f64>,
    /// Current CPU frequency in MHz (if available)
    /// Optional frequency mhz
    pub frequency_mhz: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryMetrics {
    /// Total system memory in bytes
    /// Number of `total_bytes`
    pub total_bytes: u64,
    /// Currently used memory in bytes
    /// Number of `used_bytes`
    pub used_bytes: u64,
    /// Available free memory in bytes
    /// Number of `free_bytes`
    pub free_bytes: u64,
    /// Number of `available_bytes`
    pub available_bytes: u64,
    /// Number of `cached_bytes`
    pub cached_bytes: u64,
    /// Number of `buffer_bytes`
    pub buffer_bytes: u64,
    /// Total swap space available in bytes
    /// Number of `swap_total_bytes`
    pub swap_total_bytes: u64,
    /// Currently used swap space in bytes
    /// Number of `swap_used_bytes`
    pub swap_used_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    /// Total disk storage capacity in bytes
    /// Number of `total_bytes`
    pub total_bytes: u64,
    /// Currently used disk space in bytes
    /// Number of `used_bytes`
    pub used_bytes: u64,
    /// Available free disk space in bytes
    /// Number of `free_bytes`
    pub free_bytes: u64,
    /// Disk read throughput in bytes per second
    /// Number of `read_bytes_per_sec`
    pub read_bytes_per_sec: u64,
    /// Disk write throughput in bytes per second
    /// Number of `write_bytes_per_sec`
    pub write_bytes_per_sec: u64,
    /// Read operations per second (IOPS)
    /// Number of `read_ops_per_sec`
    pub read_ops_per_sec: u64,
    /// Write operations per second (IOPS)
    /// Number of `write_ops_per_sec`
    pub write_ops_per_sec: u64,
    /// Average disk response time in milliseconds
    pub avg_response_time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkMetrics {
    /// Network data transmission rate in bytes per second
    /// Number of `bytes_sent_per_sec`
    pub bytes_sent_per_sec: u64,
    /// Network data reception rate in bytes per second
    /// Number of `bytes_received_per_sec`
    pub bytes_received_per_sec: u64,
    /// Outbound network packets per second
    /// Number of `packets_sent_per_sec`
    pub packets_sent_per_sec: u64,
    /// Inbound network packets per second
    /// Number of `packets_received_per_sec`
    pub packets_received_per_sec: u64,
    /// Network errors encountered per second
    /// Number of `errors_per_sec`
    pub errors_per_sec: u64,
    /// Dropped packets per second due to congestion or errors
    /// Number of `dropped_packets_per_sec`
    pub dropped_packets_per_sec: u64,
    /// Number of active network connections
    /// Number of `active_connections`
    pub active_connections: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMetrics {
    pub pid: u32,
    /// The cpu usage percent value
    pub cpu_usage_percent: f64,
    /// Number of `memory_usage_bytes`
    pub memory_usage_bytes: u64,
    /// Number of threads spawned by this process
    /// Number of thread
    pub thread_count: u32,
    /// Number of open file descriptors (Unix systems)
    /// Number of `file_descriptor`
    pub file_descriptor_count: u32,
    /// Process uptime in seconds since start
    pub uptime_seconds: u64,
}

/// Load average metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadAverage {
    /// System load average over the past 1 minute
    /// The one minute value
    pub one_minute: f64,
    /// System load average over the past 5 minutes
    /// The five minute value
    pub five_minute: f64,
    /// System load average over the past 15 minutes
    /// The fifteen minute value
    pub fifteen_minute: f64,
}

/// Application-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMetrics {
    /// Timestamp when these application metrics were collected
    pub timestamp: DateTime<Utc>,
    /// Name of the application service being monitored
    /// Name of the service
    pub service_name: String,
    /// Application request throughput in requests per second
    /// The requests per second value
    pub requests_per_second: f64,
    /// Application response time distribution and percentiles
    pub response_time_ms: ResponseTimeMetrics,
    /// Application error rate as a percentage (0.0-100.0)
    /// The error rate percent value
    pub error_rate_percent: f64,
    /// Number of active application connections
    /// Number of `active_connections`
    pub active_connections: u64,
    /// Length of application request queue
    /// Number of `queue_length`
    pub queue_length: u64,
    /// Application-level cache hit ratio (0.0-1.0)
    /// The cache hit ratio value
    pub cache_hit_ratio: f64,
}

/// Response time metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimeMetrics {
    /// Average response time in milliseconds
    /// The average value
    pub average: f64,
    /// 50th percentile (median) response time in milliseconds
    /// The p50 value
    pub p50: f64,
    /// 95th percentile response time in milliseconds
    /// The p95 value
    pub p95: f64,
    /// 99th percentile response time in milliseconds
    /// The p99 value
    pub p99: f64,
    /// Maximum response time observed in milliseconds
    /// The max value
    pub max: f64,
    /// Minimum response time observed in milliseconds
    /// The min value
    pub min: f64,
}

/// Security metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Authentication Attempts
    /// Number of `authentication_attempts`
    pub authentication_attempts: u64,
    /// Authentication Failures
    /// Number of `authentication_failures`
    pub authentication_failures: u64,
    /// Authorization Failures
    /// Number of `authorization_failures`
    pub authorization_failures: u64,
    /// Suspicious Activities
    /// Number of `suspicious_activities`
    pub suspicious_activities: u64,
    /// Blocked Requests
    /// Number of `blocked_requests`
    pub blocked_requests: u64,
    /// Threat Detections
    /// Number of `threat_detections`
    pub threat_detections: u64,
    /// Security Events
    /// Number of `security_events`
    pub security_events: u64,
}

/// Resource utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Cpu Utilization
    /// The cpu utilization value
    pub cpu_utilization: f64,
    /// Memory Utilization
    /// The memory utilization value
    pub memory_utilization: f64,
    /// Disk Utilization
    /// The disk utilization value
    pub disk_utilization: f64,
    /// Network Utilization
    /// The network utilization value
    pub network_utilization: f64,
    /// Resource Efficiency
    /// The resource efficiency value
    pub resource_efficiency: f64,
    /// Capacity Planning
    /// The capacity planning value
    pub capacity_planning: CapacityMetrics,
}

/// Capacity planning metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityMetrics {
    /// Projected Growth Percent
    /// The projected growth percent value
    pub projected_growth_percent: f64,
    /// Time To Capacity Days
    pub time_to_capacity_days: Option<u32>,
    /// Recommended Scaling Factor
    /// The recommended scaling factor value
    pub recommended_scaling_factor: f64,
    /// Resource Pressure Score
    /// The resource pressure score value
    pub resource_pressure_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetrics {
    /// Timestamp when these metrics were collected
    pub timestamp: DateTime<Utc>,
    /// Name of the service being monitored
    /// Name of the service
    pub service_name: String,
    /// Request throughput measured in requests per second
    /// The requests per second value
    pub requests_per_second: f64,
    /// Response time statistics and percentiles
    pub response_time_ms: ResponseTimeMetrics,
    /// Error rate as a percentage (0.0-100.0)
    /// The error rate percent value
    pub error_rate_percent: f64,
    /// Number of active concurrent connections
    /// Number of `active_connections`
    pub active_connections: u64,
    /// Number of `queue_length`
    pub queue_length: u64,
    /// Cache hit ratio as a percentage (0.0-1.0)
    /// The cache hit ratio value
    pub cache_hit_ratio: f64,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            hostname: "unknown".to_string(),
            cpu: CpuMetrics::default(),
            memory: MemoryMetrics::default(),
            disk: DiskMetrics::default(),
            network: NetworkMetrics::default(),
            process: ProcessMetrics::default(),
            custom: HashMap::new(),
        }
    }
}

impl Default for CpuMetrics {
    fn default() -> Self {
        Self {
            usage_percent: 0.0,
            per_core_usage: vec![0.0],
            load_average: LoadAverage::default(),
            temperature_celsius: None,
            frequency_mhz: None,
        }
    }
}

impl Default for DiskMetrics {
    fn default() -> Self {
        Self {
            total_bytes: 1_073_741_824, // 1GB
            used_bytes: 0,
            free_bytes: 1_073_741_824,
            read_bytes_per_sec: 0,
            write_bytes_per_sec: 0,
            read_ops_per_sec: 0,
            write_ops_per_sec: 0,
            avg_response_time_ms: 0.0,
        }
    }
}

impl Default for ProcessMetrics {
    fn default() -> Self {
        Self {
            pid: 0,
            cpu_usage_percent: 0.0,
            memory_usage_bytes: 0,
            thread_count: 1,
            file_descriptor_count: 0,
            uptime_seconds: 0,
        }
    }
}

impl Default for LoadAverage {
    fn default() -> Self {
        Self {
            one_minute: 0.0,
            five_minute: 0.0,
            fifteen_minute: 0.0,
        }
    }
}

impl Default for ApplicationMetrics {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            service_name: "unknown".to_string(),
            requests_per_second: 0.0,
            response_time_ms: ResponseTimeMetrics::default(),
            error_rate_percent: 0.0,
            active_connections: 0,
            queue_length: 0,
            cache_hit_ratio: 0.0,
        }
    }
}

impl Default for ResponseTimeMetrics {
    fn default() -> Self {
        Self {
            average: 0.0,
            p50: 0.0,
            p95: 0.0,
            p99: 0.0,
            max: 0.0,
            min: 0.0,
        }
    }
}

// Re-export canonical metrics types when available
// Note: Using unified canonical monitoring for metrics functionality
pub use crate::canonical::monitoring_unified::{AlertingConfig, MetricsConfig};
