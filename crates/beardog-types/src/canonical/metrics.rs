// SPDX-License-Identifier: AGPL-3.0-only

// Metrics collection and reporting types
// Provides structured definitions for system metrics and performance monitoring

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// CPU usage percentage
    /// The cpu usage percent value
    pub cpu_usage_percent: f64,
    /// Memory usage in MB
    /// The memory usage mb value
    pub memory_usage_mb: f64,
    /// Disk usage percentage
    /// The disk usage percent value
    pub disk_usage_percent: f64,
    /// Network throughput in MB/s
    /// The network throughput mbps value
    pub network_throughput_mbps: f64,
    /// Operations per second
    /// The operations per second value
    pub operations_per_second: f64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0.0,
            disk_usage_percent: 0.0,
            network_throughput_mbps: 0.0,
            operations_per_second: 0.0,
        }
    }
}

/// Security metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityMetrics {
    /// Number of failed authentication attempts
    /// Number of failed_auth_attempts
    pub failed_auth_attempts: u64,
    /// Number of successful authentications
    /// Number of successful_auths
    pub successful_auths: u64,
    /// Number of security violations
    /// Number of security_violations
    pub security_violations: u64,
    /// Number of blocked requests
    /// Number of blocked_requests
    pub blocked_requests: u64,
    /// Last security event timestamp
    /// Optional last security event
    pub last_security_event: Option<SystemTime>,
}

/// Error metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    /// Total error count
    /// Number of total_errors
    pub total_errors: u64,
    /// Critical errors
    /// Number of critical_errors
    pub critical_errors: u64,
    /// Warning count
    /// Number of warnings
    pub warnings: u64,
    /// Error rate per minute
    /// The error rate per minute value
    pub error_rate_per_minute: f64,
    /// Recent error messages
    /// Collection of recent errors
    pub recent_errors: Vec<String>,
    /// Last error timestamp
    /// Optional last error
    pub last_error: Option<SystemTime>,
}

impl Default for ErrorMetrics {
    fn default() -> Self {
        Self {
            total_errors: 0,
            critical_errors: 0,
            warnings: 0,
            error_rate_per_minute: 0.0,
            recent_errors: Vec::new(),
            last_error: None,
        }
    }
}

/// Business metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessMetrics {
    /// Total transactions processed
    /// Number of total_transactions
    pub total_transactions: u64,
    /// Successful transactions
    /// Number of successful_transactions
    pub successful_transactions: u64,
    /// Failed transactions
    /// Number of failed_transactions
    pub failed_transactions: u64,
    /// Revenue generated
    /// The revenue value
    pub revenue: f64,
    /// Customer count
    /// Number of customer
    pub customer_count: u64,
}

impl Default for BusinessMetrics {
    fn default() -> Self {
        Self {
            total_transactions: 0,
            successful_transactions: 0,
            failed_transactions: 0,
            revenue: 0.0,
            customer_count: 0,
        }
    }
}

/// Application-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMetrics {
    /// Request count
    /// Number of request
    pub request_count: u64,
    /// Response time percentiles
    pub response_time_p50: f64,
    pub response_time_p95: f64,
    pub response_time_p99: f64,
    /// Cache hit rate
    /// The cache hit rate value
    pub cache_hit_rate: f64,
    /// Database query count
    /// Number of database_queries
    pub database_queries: u64,
}

impl Default for ApplicationMetrics {
    fn default() -> Self {
        Self {
            request_count: 0,
            response_time_p50: 0.0,
            response_time_p95: 0.0,
            response_time_p99: 0.0,
            cache_hit_rate: 0.0,
            database_queries: 0,
        }
    }
}

/// Comprehensive metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub performance: PerformanceMetrics,
    /// Security metrics
    /// The security value
    pub security: SecurityMetrics,
    /// Error metrics
    /// The errors value
    pub errors: ErrorMetrics,
    /// Business metrics
    /// The business value
    pub business: BusinessMetrics,
    /// Application metrics
    /// The application value
    pub application: ApplicationMetrics,
    /// Custom metrics
    /// Mapping of custom
    pub custom: HashMap<String, f64>,
    /// Timestamp of metrics collection
    pub timestamp: SystemTime,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            performance: PerformanceMetrics::default(),
            security: SecurityMetrics::default(),
            errors: ErrorMetrics::default(),
            business: BusinessMetrics::default(),
            application: ApplicationMetrics::default(),
            custom: HashMap::new(),
            timestamp: SystemTime::now(),
        }
    }
}

impl SystemMetrics {
    /// Create new system metrics with current timestamp
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Update timestamp to current time
    /// Updates timestamp
    pub fn update_timestamp(&mut self) {
        self.timestamp = SystemTime::now();
    }

    /// Add a custom metric
    pub fn add_custom_metric(&mut self, name: String, value: f64) {
        self.custom.insert(name, value);
    }

    /// Get a custom metric value
    /// Gets custom_metric
    pub fn get_custom_metric(&self, name: &str) -> Option<f64> {
        self.custom.get(name).copied()
    }

    /// Calculate overall health score (0.0 to 1.0)
    pub fn health_score(&self) -> f64 {
        let cpu_score = (100.0 - self.performance.cpu_usage_percent) / 100.0;
        let error_score = if self.errors.total_errors == 0 {
            1.0
        } else {
            (1.0 / (self.errors.error_rate_per_minute + 1.0)).min(1.0)
        };
        let security_score = if self.security.security_violations == 0 {
            1.0
        } else {
            0.5
        };

        (cpu_score + error_score + security_score) / 3.0
    }

    /// Get metrics summary as string
    pub fn summary(&self) -> String {
        format!(
            "CPU: {:.1}%, Memory: {:.1}MB, Errors: {}, Health: {:.2}",
            self.performance.cpu_usage_percent,
            self.performance.memory_usage_mb,
            self.errors.total_errors,
            self.health_score()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // PerformanceMetrics tests
    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();
        
        assert_eq!(metrics.cpu_usage_percent, 0.0);
        assert_eq!(metrics.memory_usage_mb, 0.0);
        assert_eq!(metrics.disk_usage_percent, 0.0);
        assert_eq!(metrics.network_throughput_mbps, 0.0);
        assert_eq!(metrics.operations_per_second, 0.0);
    }

    #[test]
    fn test_performance_metrics_custom() {
        let metrics = PerformanceMetrics {
            cpu_usage_percent: 45.5,
            memory_usage_mb: 2048.0,
            disk_usage_percent: 75.0,
            network_throughput_mbps: 125.5,
            operations_per_second: 1500.0,
        };
        
        assert_eq!(metrics.cpu_usage_percent, 45.5);
        assert_eq!(metrics.memory_usage_mb, 2048.0);
        assert!(metrics.disk_usage_percent > 50.0);
    }

    // SecurityMetrics tests
    #[test]
    fn test_security_metrics_default() {
        let metrics = SecurityMetrics::default();
        
        assert_eq!(metrics.failed_auth_attempts, 0);
        assert_eq!(metrics.successful_auths, 0);
        assert_eq!(metrics.security_violations, 0);
        assert_eq!(metrics.blocked_requests, 0);
        assert!(metrics.last_security_event.is_none());
    }

    #[test]
    fn test_security_metrics_with_events() {
        let metrics = SecurityMetrics {
            failed_auth_attempts: 5,
            successful_auths: 100,
            security_violations: 2,
            blocked_requests: 10,
            last_security_event: Some(SystemTime::now()),
        };
        
        assert_eq!(metrics.failed_auth_attempts, 5);
        assert_eq!(metrics.successful_auths, 100);
        assert!(metrics.last_security_event.is_some());
        
        let success_rate = metrics.successful_auths as f64 
            / (metrics.successful_auths + metrics.failed_auth_attempts) as f64;
        assert!(success_rate > 0.95);
    }

    // ErrorMetrics tests
    #[test]
    fn test_error_metrics_default() {
        let metrics = ErrorMetrics::default();
        
        assert_eq!(metrics.total_errors, 0);
        assert_eq!(metrics.critical_errors, 0);
        assert_eq!(metrics.warnings, 0);
        assert_eq!(metrics.error_rate_per_minute, 0.0);
        assert!(metrics.recent_errors.is_empty());
        assert!(metrics.last_error.is_none());
    }

    #[test]
    fn test_error_metrics_with_errors() {
        let metrics = ErrorMetrics {
            total_errors: 50,
            critical_errors: 5,
            warnings: 20,
            error_rate_per_minute: 2.5,
            recent_errors: vec!["Error 1".to_string(), "Error 2".to_string()],
            last_error: Some(SystemTime::now()),
        };
        
        assert_eq!(metrics.total_errors, 50);
        assert_eq!(metrics.critical_errors, 5);
        assert_eq!(metrics.recent_errors.len(), 2);
        assert!(metrics.last_error.is_some());
        
        let critical_rate = metrics.critical_errors as f64 / metrics.total_errors as f64;
        assert_eq!(critical_rate, 0.1);
    }

    // BusinessMetrics tests
    #[test]
    fn test_business_metrics_creation() {
        let metrics = BusinessMetrics {
            total_transactions: 10000,
            successful_transactions: 9950,
            failed_transactions: 50,
            revenue: 50000.0,
            average_transaction_value: 5.0,
        };
        
        assert_eq!(metrics.total_transactions, 10000);
        assert_eq!(metrics.successful_transactions, 9950);
        
        let success_rate = metrics.successful_transactions as f64 / metrics.total_transactions as f64;
        assert!(success_rate > 0.99);
    }

    // CustomMetrics tests
    #[test]
    fn test_custom_metrics_empty() {
        let metrics = CustomMetrics {
            metrics: HashMap::new(),
        };
        
        assert!(metrics.metrics.is_empty());
    }

    #[test]
    fn test_custom_metrics_with_data() {
        let mut metrics_map = HashMap::new();
        metrics_map.insert("cache_hit_rate".to_string(), 0.85);
        metrics_map.insert("queue_depth".to_string(), 150.0);
        
        let metrics = CustomMetrics {
            metrics: metrics_map,
        };
        
        assert_eq!(metrics.metrics.len(), 2);
        assert_eq!(metrics.metrics.get("cache_hit_rate"), Some(&0.85));
    }

    // SystemMetrics tests
    #[test]
    fn test_system_metrics_creation() {
        let system_metrics = SystemMetrics {
            performance: PerformanceMetrics::default(),
            security: SecurityMetrics::default(),
            errors: ErrorMetrics::default(),
            business: BusinessMetrics {
                total_transactions: 0,
                successful_transactions: 0,
                failed_transactions: 0,
                revenue: 0.0,
                average_transaction_value: 0.0,
            },
            custom: CustomMetrics {
                metrics: HashMap::new(),
            },
            timestamp: SystemTime::now(),
        };
        
        assert_eq!(system_metrics.performance.cpu_usage_percent, 0.0);
        assert_eq!(system_metrics.security.failed_auth_attempts, 0);
        assert_eq!(system_metrics.errors.total_errors, 0);
    }

    #[test]
    fn test_system_metrics_health_score() {
        let system_metrics = SystemMetrics {
            performance: PerformanceMetrics {
                cpu_usage_percent: 50.0,
                memory_usage_mb: 1024.0,
                disk_usage_percent: 60.0,
                network_throughput_mbps: 100.0,
                operations_per_second: 1000.0,
            },
            security: SecurityMetrics {
                failed_auth_attempts: 0,
                successful_auths: 100,
                security_violations: 0,
                blocked_requests: 0,
                last_security_event: None,
            },
            errors: ErrorMetrics::default(),
            business: BusinessMetrics {
                total_transactions: 100,
                successful_transactions: 100,
                failed_transactions: 0,
                revenue: 1000.0,
                average_transaction_value: 10.0,
            },
            custom: CustomMetrics {
                metrics: HashMap::new(),
            },
            timestamp: SystemTime::now(),
        };
        
        let health = system_metrics.health_score();
        assert!(health > 0.0);
        assert!(health <= 100.0);
    }

    // Serialization tests
    #[test]
    fn test_performance_metrics_serialization() {
        let metrics = PerformanceMetrics::default();
        let json = serde_json::to_string(&metrics);
        assert!(json.is_ok(), "Should be able to serialize PerformanceMetrics");
    }

    #[test]
    fn test_security_metrics_serialization() {
        let metrics = SecurityMetrics::default();
        let json = serde_json::to_string(&metrics);
        assert!(json.is_ok(), "Should be able to serialize SecurityMetrics");
    }

    #[test]
    fn test_error_metrics_serialization() {
        let metrics = ErrorMetrics::default();
        let json = serde_json::to_string(&metrics);
        assert!(json.is_ok(), "Should be able to serialize ErrorMetrics");
    }
}
