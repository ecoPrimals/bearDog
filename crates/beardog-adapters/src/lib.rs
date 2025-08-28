pub use beardog_types::canonical::providers::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use beardog_errors::BearDogError;

pub mod adapters;
pub mod ecosystem_integration;
pub mod universal;

pub use universal::capability_adapter::{
    BearDogCapabilityAdapter, ServiceCapability, ServiceMeshConnector,
};

// Import UniversalProvider directly from beardog-traits canonical
pub use beardog_traits::canonical::UniversalProvider;

pub use universal::service_registration::{ServiceMetadata, UniversalServiceRegistration};

pub use ecosystem_integration::EcosystemIntegration;
// Use canonical types instead of local definitions
pub use beardog_types::canonical::services::{UniversalRequest, UniversalResponse};

pub use universal::{
    BridgeAdapter, BridgeConfig, HttpAdapter, ProtocolAdapter, SecurityProviderBridge,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdapterError {
    AdapterNotFound { adapter_id: String },
    ExternalSystemError { system: String, error: String },
    AdapterError { adapter: String, error: String },
    ProtocolError { protocol: String, message: String },
}

// Helper functions for creating adapter errors
impl AdapterError {
    pub fn capability_not_found<S: Into<String>>(capability: S) -> Self {
        Self::AdapterNotFound {
            adapter_id: capability.into(),
        }
    }

    pub fn registration_failed<S: Into<String>>(reason: S) -> Self {
        Self::ExternalSystemError {
            system: "registration".to_string(),
            error: reason.into(),
        }
    }

    pub fn integration_failed<S: Into<String>>(reason: S) -> Self {
        Self::AdapterError {
            adapter: "integration".to_string(),
            error: reason.into(),
        }
    }

    pub fn protocol_error<S: Into<String>>(message: S) -> Self {
        Self::ProtocolError {
            protocol: "universal".to_string(),
            message: message.into(),
        }
    }
}

impl From<AdapterError> for BearDogError {
    fn from(error: AdapterError) -> Self {
        match error {
            AdapterError::AdapterNotFound { adapter_id } => Self::Configuration {
                message: format!("Adapter not found: {}", adapter_id),
                category: beardog_errors::ConfigurationErrorCategory::General,
            },
            AdapterError::ExternalSystemError { system, error } => Self::System {
                message: format!("External system error from {}: {}", system, error),
                category: beardog_errors::SystemErrorCategory::Service,
            },
            AdapterError::AdapterError { adapter, error } => Self::System {
                message: format!("Adapter error in {}: {}", adapter, error),
                category: beardog_errors::SystemErrorCategory::Resource,
            },
            AdapterError::ProtocolError { protocol, message } => Self::Network {
                message: format!("Protocol error in {}: {}", protocol, message),
                category: beardog_errors::NetworkErrorCategory::Connection,
            },
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
{
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

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            cpu_percent: 0.0,
            memory_mb: 0.0,
            network_kb: 0.0,
            disk_kb: 0.0,
        }
    }
}
