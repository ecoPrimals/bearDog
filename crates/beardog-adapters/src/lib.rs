

pub use beardog_types::canonical::providers::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod adapters;
pub mod ecosystem_integration;
pub mod universal;

pub use universal::capability_adapter::{
    BearDogCapabilityAdapter, ServiceCapability, ServiceMeshConnector, UniversalServiceProvider,
};

pub use universal::service_registration::{ServiceMetadata, UniversalServiceRegistration};

pub use universal::http_adapter::{UniversalRequest, UniversalResponse};
pub use ecosystem_integration::EcosystemIntegration;

pub use universal::{
    BridgeAdapter, BridgeConfig, HttpAdapter, ProtocolAdapter, SecurityProviderBridge,
};

pub use beardog_errors::BearDogResult;

pub use beardog_errors::BearDogError;

impl BearDogError {
    pub fn capability_not_found(capability: impl Into<&str>) -> Self {
        Self::NotFound {
            message: format_args!("Capability not found: {}", capability.into().to_string()),
        }
    }
    
    pub fn registration_failed(reason: impl Into<&str>) -> Self {
        Self::External {
            message: format_args!("Service registration failed: {}", reason.into().to_string()),
        }
    }
    
    pub fn integration_failed(reason: impl Into<&str>) -> Self {
        Self::External {
            message: format_args!("Integration failed: {}", reason.into().to_string()),
        }
    }
    
    pub fn protocol_error(message: impl Into<&str>) -> Self {
        Self::Protocol {
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFirstResponse<T> {

    pub success: bool,

    pub data: T,

    pub error: Option<AIFirstError>,

    pub request_id: Uuid,

    pub processing_time_ms: u64,

    pub ai_metadata: AIResponseMetadata,

    pub confidence_score: f64,

    pub suggested_actions: Vec<SuggestedAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFirstError {

    pub code: String,

    pub message: String,

    pub resolution_hint: Option<String>,

    pub retryable: bool,

    pub retry_strategy: Option<RetryStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponseMetadata {

    pub operation_type: String,

    pub resource_usage: ResourceUsage,

    pub performance_indicators: HashMap<String, f64>,

    pub context: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedAction {

    pub action_type: String,

    pub description: String,

    pub priority: u8,

    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryStrategy {

    pub max_retries: u32,

    pub base_delay_ms: u64,

    pub backoff_multiplier: f64,

    pub exponential_backoff: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {

    pub cpu_percent: f64,

    pub memory_mb: f64,

    pub network_kb: f64,

    pub disk_kb: f64,
}

impl<T> Default for AIFirstResponse<T>
where
    T: Default,
{}

    fn default() -> Self {
        Self {
            success: true,
            data: T::default(),
            error: None,
            request_id: Uuid::new_v4(),
            processing_time_ms: 0,
            ai_metadata: AIResponseMetadata::default(),
            confidence_score: 1.0,
            suggested_actions: Vec::new(),
        }
    }
}

impl Default for AIResponseMetadata {
    fn default() -> Self {
        Self {
            operation_type: "default".to_string(),
            resource_usage: ResourceUsage::default(),
            performance_indicators: HashMap::with_capacity(16),
            context: HashMap::with_capacity(16),
        }
    }
}

impl Default for ResourceUsage {}

    fn default() -> Self {
        Self {
            cpu_percent: 0.0,
            memory_mb: 0.0,
            network_kb: 0.0,
            disk_kb: 0.0,
        }
    }
}
