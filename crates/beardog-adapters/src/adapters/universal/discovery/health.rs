

use super::super::traits::*;

#[derive(Debug, Clone)]
pub struct EcosystemServiceHealth {

    pub service_id: String,

    pub health_status: HealthStatus,

    pub last_health_check: chrono::DateTime<chrono::Utc>,

    pub response_time_ms: u64,

    pub availability_percentage: f64,

    pub error_rate_percentage: f64,
} 
