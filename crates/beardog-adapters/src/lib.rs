// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # BearDog Universal Adapters
///
/// **CANONICAL ADAPTER SYSTEM** - Universal integration patterns for all primals
/// This crate provides the unified adapter architecture that enables BearDog to
/// integrate with any primal ecosystem through capability-based discovery and
/// universal provider patterns.

// Core re-exports for adapter implementations
pub use beardog_types::canonical::providers::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// Import BearDogResult from errors crate
pub mod adapters;
pub mod ecosystem_integration;
pub mod universal;

// Re-export key universal types (corrected imports)
pub use universal::capability_adapter::{
    BearDogCapabilityAdapter, ServiceCapability, ServiceMeshConnector, UniversalServiceProvider,
};

// Import ServiceMetadata from service_registration
pub use universal::service_registration::{ServiceMetadata, UniversalServiceRegistration};

// Import UniversalRequest and UniversalResponse from http_adapter
pub use universal::http_adapter::{UniversalRequest, UniversalResponse};
pub use ecosystem_integration::EcosystemIntegration;

pub use universal::{
    BridgeAdapter, BridgeConfig, HttpAdapter, ProtocolAdapter, SecurityProviderBridge,
};

// Use canonical BearDogResult instead of custom EcosystemResult
pub use beardog_errors::BearDogResult;

// Convert EcosystemError variants to use BearDogError instead
pub use beardog_errors::BearDogError;

// Helper functions to convert from EcosystemError patterns to BearDogError
impl BearDogError {
    pub fn capability_not_found(capability: impl Into<String>) -> Self {
        Self::NotFound {
            message: format!("Capability not found: {}", capability.into()),
        }
    }
    
    pub fn registration_failed(reason: impl Into<String>) -> Self {
        Self::External {
            message: format!("Service registration failed: {}", reason.into()),
        }
    }
    
    pub fn integration_failed(reason: impl Into<String>) -> Self {
        Self::External {
            message: format!("Integration failed: {}", reason.into()),
        }
    }
    
    pub fn protocol_error(message: impl Into<String>) -> Self {
        Self::Protocol {
            message: message.into(),
        }
    }
}

/// Universal AI-First response format following ecosystem standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFirstResponse<T> {
    /// Operation success status (machine-readable)
    pub success: bool,
    /// Strongly-typed response data
    pub data: T,
    /// AI-optimized error information
    pub error: Option<AIFirstError>,
    /// Unique request identifier for tracing and correlation
    pub request_id: Uuid,
    /// Processing time in milliseconds for performance monitoring
    pub processing_time_ms: u64,
    /// AI-specific metadata for decision making
    pub ai_metadata: AIResponseMetadata,
    /// Confidence score for AI decision making (0.0 - 1.0)
    pub confidence_score: f64,
    /// Suggested next actions for AI agents
    pub suggested_actions: Vec<SuggestedAction>,
}

/// AI-optimized error structure with automation hints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFirstError {
    /// Machine-readable error code
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Suggested resolution for AI agents
    pub resolution_hint: Option<String>,
    /// Whether this error is retryable
    pub retryable: bool,
    /// Retry strategy if applicable
    pub retry_strategy: Option<RetryStrategy>,
}

/// AI response metadata for decision making
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponseMetadata {
    /// Operation type that was performed
    pub operation_type: String,
    /// Resource consumption metrics
    pub resource_usage: ResourceUsage,
    /// Performance indicators
    pub performance_indicators: HashMap<String, f64>,
    /// Context for next operations
    pub context: HashMap<String, serde_json::Value>,
}

/// Suggested action for AI agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedAction {
    /// Action type
    pub action_type: String,
    /// Action description
    pub description: String,
    /// Priority (0-10)
    pub priority: u8,
    /// Parameters for the action
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Retry strategy for failed operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryStrategy {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Base delay in milliseconds
    pub base_delay_ms: u64,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Whether to use exponential backoff
    pub exponential_backoff: bool,
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// CPU usage percentage
    pub cpu_percent: f64,
    /// Memory usage in MB
    pub memory_mb: f64,
    /// Network I/O in KB
    pub network_kb: f64,
    /// Disk I/O in KB
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
            performance_indicators: HashMap::new(),
            context: HashMap::new(),
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
