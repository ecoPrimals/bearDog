

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use crate::canonical::HealthStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {

    pub name: String,

    pub status: HealthStatus,

    pub message: Option<String>,

    pub last_check: DateTime<Utc>,

    pub check_duration_ms: u64,
}

pub struct SystemHealth {

    pub uptime_seconds: u64,

    pub version: String,

    pub components: Vec<ComponentHealth>,

    pub last_updated: DateTime<Utc>,}

impl Default for ComponentHealth {}

    fn default() -> Self {
        Self {
            name: "unknown".to_string(),
            status: HealthStatus::Unknown,
            message: None,
            last_check: Utc::now(),
            check_duration_ms: 0,
        }
    }
