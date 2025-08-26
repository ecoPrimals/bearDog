

use crate::config::{
    database::UnifiedDatabaseConfig as DatabaseConfig, 
    monitoring::BasicMonitoringConfig as MonitoringConfig,
    performance::GeneralPerformanceConfig as PerformanceConfig,
    security_unified::UnifiedSecurityConfig as SecurityConfig,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub monitoring: MonitoringConfig,
    pub performance: PerformanceConfig,
    pub security: SecurityConfig,
}

