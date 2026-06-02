// SPDX-License-Identifier: AGPL-3.0-or-later

// Metrics Types and Configurations
//
// This module provides metrics-related types and configurations for the BearDog ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Host-level CPU, memory, disk, network, process, and custom gauges.
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

/// Processor utilization, per-core breakdown, load averages, and sensor data.
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

/// Physical memory and swap usage snapshot.
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

/// Block device space and IOPS-style activity counters.
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

/// Interface throughput, packet rates, errors, and connection counts.
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

/// Single-process resource usage: CPU, memory, threads, descriptors, uptime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMetrics {
    /// Operating-system process ID
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

/// Per-service request, latency, error, and saturation metrics at a point in time.
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

#[cfg(test)]
mod tests {
    // SPDX-License-Identifier: AGPL-3.0-or-later
    #![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

    use super::*;
    use chrono::Utc;

    #[test]
    fn system_metrics_default_hostname_and_nested_defaults() {
        let m = SystemMetrics::default();
        assert_eq!(m.hostname, "unknown");
        assert_eq!(m.cpu.usage_percent, 0.0);
        assert_eq!(m.memory.total_bytes, 0);
        assert_eq!(m.disk.total_bytes, 1_073_741_824);
        assert_eq!(m.network.bytes_sent_per_sec, 0);
        assert_eq!(m.process.pid, 0);
        assert!(m.custom.is_empty());
    }

    #[test]
    fn cpu_disk_process_load_defaults_are_consistent() {
        let cpu = CpuMetrics::default();
        assert_eq!(cpu.per_core_usage, vec![0.0]);
        assert!(cpu.temperature_celsius.is_none());
        assert!(cpu.frequency_mhz.is_none());

        let disk = DiskMetrics::default();
        assert_eq!(disk.free_bytes, disk.total_bytes);
        assert_eq!(disk.avg_response_time_ms, 0.0);

        let proc = ProcessMetrics::default();
        assert_eq!(proc.thread_count, 1);

        let la = LoadAverage::default();
        assert_eq!(la.one_minute, 0.0);
    }

    #[test]
    fn application_and_response_time_defaults() {
        let app = ApplicationMetrics::default();
        assert_eq!(app.service_name, "unknown");
        let rt = ResponseTimeMetrics::default();
        assert_eq!(rt.p99, 0.0);
        assert_eq!(app.response_time_ms.p50, rt.p50);
    }

    #[test]
    fn serde_roundtrip_system_metrics_minimal() {
        let m = SystemMetrics {
            hostname: "h1".to_string(),
            cpu: CpuMetrics::default(),
            memory: MemoryMetrics::default(),
            disk: DiskMetrics::default(),
            network: NetworkMetrics::default(),
            process: ProcessMetrics::default(),
            custom: std::iter::once(("q".to_string(), 1.5)).collect(),
        };
        let json = serde_json::to_string(&m).expect("serialize");
        let back: SystemMetrics = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.hostname, "h1");
        assert_eq!(back.custom.get("q"), Some(&1.5));
    }

    #[test]
    fn serde_roundtrip_security_and_resource_metrics() {
        let sec = SecurityMetrics {
            timestamp: Utc::now(),
            authentication_attempts: 1,
            authentication_failures: 0,
            authorization_failures: 0,
            suspicious_activities: 0,
            blocked_requests: 0,
            threat_detections: 0,
            security_events: 2,
        };
        let json = serde_json::to_string(&sec).expect("serialize sec");
        let sec2: SecurityMetrics = serde_json::from_str(&json).expect("deserialize sec");
        assert_eq!(sec2.security_events, 2);

        let cap = CapacityMetrics {
            projected_growth_percent: 10.0,
            time_to_capacity_days: Some(30),
            recommended_scaling_factor: 1.2,
            resource_pressure_score: 0.4,
        };
        let res = ResourceMetrics {
            timestamp: Utc::now(),
            cpu_utilization: 0.5,
            memory_utilization: 0.6,
            disk_utilization: 0.7,
            network_utilization: 0.2,
            resource_efficiency: 0.85,
            capacity_planning: cap,
        };
        let json = serde_json::to_string(&res).expect("serialize res");
        let res2: ResourceMetrics = serde_json::from_str(&json).expect("deserialize res");
        assert_eq!(res2.capacity_planning.time_to_capacity_days, Some(30));
    }

    #[test]
    fn service_metrics_roundtrip_with_response_percentiles() {
        let sm = ServiceMetrics {
            timestamp: Utc::now(),
            service_name: "api".to_string(),
            requests_per_second: 10.0,
            response_time_ms: ResponseTimeMetrics {
                average: 1.0,
                p50: 1.0,
                p95: 2.0,
                p99: 3.0,
                max: 4.0,
                min: 0.5,
            },
            error_rate_percent: 0.01,
            active_connections: 3,
            queue_length: 0,
            cache_hit_ratio: 0.9,
        };
        let json = serde_json::to_string(&sm).expect("serialize");
        let sm2: ServiceMetrics = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(sm2.service_name, "api");
        assert_eq!(sm2.response_time_ms.p95, 2.0);
    }
}
