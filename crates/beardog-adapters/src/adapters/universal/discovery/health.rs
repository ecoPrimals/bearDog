//! Health Monitoring
//!
//! Health monitoring for discovered services

use super::super::traits::*;

/// Ecosystem service health information
#[derive(Debug, Clone)]
pub struct EcosystemServiceHealth {
    /// Unique identifier for the service
    pub service_id: String,
    /// Current health status of the service
    pub health_status: HealthStatus,
    /// When the last health check was performed
    pub last_health_check: chrono::DateTime<chrono::Utc>,
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Availability percentage (0-100)
    pub availability_percentage: f64,
    /// Error rate percentage (0-100)
    pub error_rate_percentage: f64,
} 