

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use beardog_types::providers::ServiceHealth;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshInfo {

    pub name: String,

    pub endpoint: String,

    pub capabilities: Vec<String>,

    pub api_version: String,

    pub health: ServiceHealth,

    pub last_health_check: Option<chrono::DateTime<chrono::Utc>>,

    pub metadata: HashMap<String, String>,

    pub priority: u8,
}

pub struct RegistrationInfo {

    pub registration_id: String,

    pub service_info: beardog_types::canonical::services::UniversalServiceMetadata,

    pub registered_at: chrono::DateTime<chrono::Utc>,

    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,

    pub health_check_config: Option<HealthCheckConfig>,

    pub tags: Vec<String>,
}

pub use beardog_types::canonical::monitoring::HealthCheckConfig;

pub struct DiscoveredService {

    pub service_id: String,

    pub service_name: String,

    pub service_type: String,

    pub capabilities: Vec<ServiceMeshCapability>,

    pub version: Option<String>,

    pub discovered_at: chrono::DateTime<chrono::Utc>,
}

pub struct ServiceMeshCapability {

    pub version: String,

    pub description: Option<String>,

    pub parameters: HashMap<String, serde_json::Value>,
}

pub struct ServiceRegistrationRequest {

    pub service: beardog_types::canonical::services::UniversalServiceMetadata,
    pub health_check: Option<HealthCheckConfig>,

    pub ttl_seconds: Option<u64>,
}

pub struct ServiceLookupRequest {

    pub service_name: Option<String>,

    pub service_type: Option<String>,

    pub required_capabilities: Vec<String>,

    pub health_filter: Option<ServiceHealth>,

    pub limit: Option<u32>,
}

pub struct ServiceUpdateRequest {

    pub tags: Option<Vec<String>>,

    pub metadata: Option<HashMap<String, String>>,
}

pub struct ServiceMeshResult<T> {

    pub success: bool,

    pub data: Option<T>,

    pub error: Option<String>,

    pub timestamp: chrono::DateTime<chrono::Utc>,

    pub operation_id: Uuid,
}

pub struct ServiceMeshStats {

    pub total_services: u64,

    pub healthy_services: u64,

    pub unhealthy_services: u64,

    pub mesh_nodes: u64,

    pub requests_per_second: f64,

    pub avg_response_time_ms: f64,
}

impl Default for ServiceMeshInfo {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            endpoint: std::env::var("BEARDOG_SONGBIRD_ENDPOINT")
            .unwrap_or_else(|_| beardog_types::constants::network::TEST_LOCALHOST_HTTP.to_string()),
            capabilities: vec![],
            api_version: "v1".to_string(),
            health: ServiceHealth {
                is_healthy: false,
                last_check: chrono::Utc::now(),
                response_time_ms: 0.0,
                error_message: Some("Unknown status".to_string()),
            },
            last_health_check: None,
            metadata: std::collections::HashMap::default(),
            priority: 0,
        }
    }
}
