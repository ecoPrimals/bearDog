//! HSM Status and Monitoring Types
//!
//! Type definitions for HSM health status, performance metrics, and monitoring.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// HSM health status
#[derive(Debug, Clone)]
pub struct HsmHealthStatus {
    /// Whether the HSM is healthy
    pub is_healthy: bool,
    /// Last health check timestamp
    pub last_check: DateTime<Utc>,
    /// Optional error message
    pub error_message: Option<String>,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
}

/// Performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Operations per second
    pub operations_per_second: f64,
    /// Average latency in milliseconds
    pub average_latency_ms: f64,
    /// Success rate (0.0-100.0)
    pub success_rate: f64,
    /// Memory usage in MB
    pub memory_usage_mb: f64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Network throughput in bits per second
    pub network_throughput_bps: f64,
    /// Latency in milliseconds
    pub latency_ms: f64,
    /// Throughput in MB/s
    pub throughput_mbps: f64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            success_rate: 100.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            network_throughput_bps: 0.0,
            latency_ms: 0.0,
            throughput_mbps: 0.0,
            uptime_seconds: 0,
        }
    }
}

/// Status category enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusCategory {
    /// Operational status
    Operational,
    /// Performance status
    Performance,
    /// Security status
    Security,
    /// Connectivity status
    Connectivity,
    /// Resource status
    Resource,
}

/// Status level enumeration
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum StatusLevel {
    /// Normal operation
    Normal,
    /// Warning condition
    Warning,
    /// Error condition
    Error,
    /// Critical condition
    Critical,
}

/// Detailed status entry
#[derive(Debug, Clone)]
pub struct StatusEntry {
    /// Status category
    pub category: StatusCategory,
    /// Status level
    pub level: StatusLevel,
    /// Status message
    pub message: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Optional metadata
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

/// Connection status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    /// Connected and operational
    Connected,
    /// Connecting
    Connecting,
    /// Disconnected
    Disconnected,
    /// Connection error
    Error(String),
}

/// Resource utilization metrics
#[derive(Debug, Clone)]
pub struct ResourceMetrics {
    /// CPU usage percentage
    pub cpu_percent: f64,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// Disk usage in bytes
    pub disk_bytes: u64,
    /// Network bandwidth in bytes per second
    pub network_bps: u64,
}

/// HSM operational status
#[derive(Debug, Clone)]
pub struct OperationalStatus {
    /// Whether HSM is online
    pub is_online: bool,
    /// Whether HSM is ready for operations
    pub is_ready: bool,
    /// Connection status
    pub connection_status: ConnectionStatus,
    /// Number of active operations
    pub active_operations: u64,
    /// Number of queued operations
    pub queued_operations: u64,
}

/// Security status information
#[derive(Debug, Clone)]
pub struct SecurityStatus {
    /// Whether security is intact
    pub is_secure: bool,
    /// Tamper detection status
    pub tamper_detected: bool,
    /// Last security audit timestamp
    pub last_audit: Option<DateTime<Utc>>,
    /// Number of failed authentication attempts
    pub failed_auth_attempts: u64,
}

/// Availability metrics
#[derive(Debug, Clone)]
pub struct AvailabilityMetrics {
    /// Uptime percentage (0.0-100.0)
    pub uptime_percent: f64,
    /// Total uptime duration in seconds
    pub uptime_seconds: u64,
    /// Downtime duration in seconds
    pub downtime_seconds: u64,
    /// Last downtime event
    pub last_downtime: Option<DateTime<Utc>>,
}

/// Error statistics
#[derive(Debug, Clone)]
pub struct ErrorStatistics {
    /// Total errors
    pub total_errors: u64,
    /// Errors in last hour
    pub errors_last_hour: u64,
    /// Errors in last day
    pub errors_last_day: u64,
    /// Error rate (errors per operation)
    pub error_rate: f64,
    /// Most recent error
    pub last_error: Option<String>,
    /// Last error timestamp
    pub last_error_time: Option<DateTime<Utc>>,
}

/// Comprehensive HSM status
#[derive(Debug, Clone)]
pub struct ComprehensiveHsmStatus {
    /// Health status
    pub health: HsmHealthStatus,
    /// Operational status
    pub operational: OperationalStatus,
    /// Security status
    pub security: SecurityStatus,
    /// Resource metrics
    pub resources: ResourceMetrics,
    /// Availability metrics
    pub availability: AvailabilityMetrics,
    /// Error statistics
    pub errors: ErrorStatistics,
    /// Status entries
    pub status_entries: Vec<StatusEntry>,
}

impl ComprehensiveHsmStatus {
    /// Create new comprehensive status with defaults
    pub fn new() -> Self {
        Self {
            health: HsmHealthStatus {
                is_healthy: true,
                last_check: Utc::now(),
                error_message: None,
                performance_metrics: PerformanceMetrics::default(),
            },
            operational: OperationalStatus {
                is_online: true,
                is_ready: true,
                connection_status: ConnectionStatus::Connected,
                active_operations: 0,
                queued_operations: 0,
            },
            security: SecurityStatus {
                is_secure: true,
                tamper_detected: false,
                last_audit: None,
                failed_auth_attempts: 0,
            },
            resources: ResourceMetrics {
                cpu_percent: 0.0,
                memory_bytes: 0,
                disk_bytes: 0,
                network_bps: 0,
            },
            availability: AvailabilityMetrics {
                uptime_percent: 100.0,
                uptime_seconds: 0,
                downtime_seconds: 0,
                last_downtime: None,
            },
            errors: ErrorStatistics {
                total_errors: 0,
                errors_last_hour: 0,
                errors_last_day: 0,
                error_rate: 0.0,
                last_error: None,
                last_error_time: None,
            },
            status_entries: Vec::new(),
        }
    }

    /// Check if status is overall healthy
    pub fn is_healthy(&self) -> bool {
        self.health.is_healthy
            && self.operational.is_online
            && self.operational.is_ready
            && self.security.is_secure
            && !self.security.tamper_detected
    }

    /// Get current status level
    pub fn get_status_level(&self) -> StatusLevel {
        if !self.is_healthy() {
            StatusLevel::Critical
        } else if self.errors.error_rate > 0.1 {
            StatusLevel::Warning
        } else {
            StatusLevel::Normal
        }
    }

    /// Add status entry
    pub fn add_status_entry(&mut self, entry: StatusEntry) {
        self.status_entries.push(entry);
    }
}

impl Default for ComprehensiveHsmStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_metrics_default() -> Result<(), Box<dyn std::error::Error>> {
        let metrics = PerformanceMetrics::default();
        assert_eq!(metrics.operations_per_second, 0.0);
        assert_eq!(metrics.success_rate, 100.0);
        assert_eq!(metrics.uptime_seconds, 0);
        Ok(())
    }

    #[test]
    fn test_status_level_ordering() -> Result<(), Box<dyn std::error::Error>> {
        assert!(StatusLevel::Normal < StatusLevel::Warning);
        assert!(StatusLevel::Warning < StatusLevel::Error);
        assert!(StatusLevel::Error < StatusLevel::Critical);
        Ok(())
    }

    #[test]
    fn test_connection_status_variants() -> Result<(), Box<dyn std::error::Error>> {
        let connected = ConnectionStatus::Connected;
        let disconnected = ConnectionStatus::Disconnected;
        let error = ConnectionStatus::Error("timeout".to_string());

        assert_eq!(connected, ConnectionStatus::Connected);
        assert_eq!(disconnected, ConnectionStatus::Disconnected);

        match error {
            ConnectionStatus::Error(msg) => assert_eq!(msg, "timeout"),
            _ => panic!("Expected Error variant"),
        }
        Ok(())
    }

    #[test]
    fn test_comprehensive_status_creation() -> Result<(), Box<dyn std::error::Error>> {
        let status = ComprehensiveHsmStatus::new();

        assert!(status.is_healthy());
        assert_eq!(status.get_status_level(), StatusLevel::Normal);
        assert!(status.operational.is_online);
        assert!(status.security.is_secure);
        Ok(())
    }

    #[test]
    fn test_comprehensive_status_unhealthy() -> Result<(), Box<dyn std::error::Error>> {
        let mut status = ComprehensiveHsmStatus::new();
        status.health.is_healthy = false;

        assert!(!status.is_healthy());
        assert_eq!(status.get_status_level(), StatusLevel::Critical);
        Ok(())
    }

    #[test]
    fn test_status_entry_creation() -> Result<(), Box<dyn std::error::Error>> {
        let entry = StatusEntry {
            category: StatusCategory::Performance,
            level: StatusLevel::Warning,
            message: "High latency detected".to_string(),
            timestamp: Utc::now(),
            metadata: None,
        };

        assert_eq!(entry.category, StatusCategory::Performance);
        assert_eq!(entry.level, StatusLevel::Warning);
        Ok(())
    }

    #[test]
    fn test_add_status_entry() -> Result<(), Box<dyn std::error::Error>> {
        let mut status = ComprehensiveHsmStatus::new();

        let entry = StatusEntry {
            category: StatusCategory::Security,
            level: StatusLevel::Normal,
            message: "Security check passed".to_string(),
            timestamp: Utc::now(),
            metadata: None,
        };

        status.add_status_entry(entry);
        assert_eq!(status.status_entries.len(), 1);
        Ok(())
    }

    #[test]
    fn test_resource_metrics() -> Result<(), Box<dyn std::error::Error>> {
        let metrics = ResourceMetrics {
            cpu_percent: 45.5,
            memory_bytes: 1024 * 1024 * 512, // 512 MB
            disk_bytes: 1024 * 1024 * 1024,  // 1 GB
            network_bps: 1_000_000,          // 1 Mbps
        };

        assert_eq!(metrics.cpu_percent, 45.5);
        assert_eq!(metrics.memory_bytes, 536_870_912);
        Ok(())
    }

    #[test]
    fn test_error_statistics() -> Result<(), Box<dyn std::error::Error>> {
        let mut errors = ErrorStatistics {
            total_errors: 100,
            errors_last_hour: 5,
            errors_last_day: 20,
            error_rate: 0.01,
            last_error: Some("Connection timeout".to_string()),
            last_error_time: Some(Utc::now()),
        };

        assert_eq!(errors.total_errors, 100);
        assert_eq!(errors.error_rate, 0.01);
        assert!(errors.last_error.is_some());
        Ok(())
    }

    #[test]
    fn test_availability_metrics() -> Result<(), Box<dyn std::error::Error>> {
        let availability = AvailabilityMetrics {
            uptime_percent: 99.9,
            uptime_seconds: 86400, // 1 day
            downtime_seconds: 86,  // ~1.4 minutes
            last_downtime: Some(Utc::now()),
        };

        assert_eq!(availability.uptime_percent, 99.9);
        assert_eq!(availability.uptime_seconds, 86400);
        Ok(())
    }
}
