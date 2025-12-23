

use super::super::traits::*;

#[derive(Debug, Clone)]
    /// Current status of the health
    pub health_status: HealthStatus,

    /// The last health check value
    pub last_health_check: chrono::DateTime<chrono::Utc>,


    pub response_time_ms: u64,

    /// The availability percentage value
    pub availability_percentage: f64,

    /// The error rate percentage value
    pub error_rate_percentage: f64,
} 
