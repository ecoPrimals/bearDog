//! BearDog Universal Ecosystem Integration Adapters
//!
//! This module provides capability-based integration with the ecoPrimals ecosystem,
//! following the Universal Primal Architecture Standard. Instead of hardcoded service
//! integrations, this uses dynamic capability discovery and universal API patterns.

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub mod universal;
pub mod ecosystem_integration;

// Re-export main universal types
pub use universal::*;
pub use ecosystem_integration::*;

/// Universal Ecosystem Integration Result
pub type EcosystemResult<T> = Result<T, EcosystemError>;

/// Universal ecosystem integration errors
#[derive(Debug, thiserror::Error)]
pub enum EcosystemError {
    #[error("Capability not found: {capability}")]
    CapabilityNotFound { capability: String },
    
    #[error("Service registration failed: {reason}")]
    RegistrationFailed { reason: String },
    
    #[error("Integration error: {message}")]
    IntegrationError { message: String },
    
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    
    #[error("Network error: {source}")]
    NetworkError { 
        #[from] 
        source: reqwest::Error 
    },
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
    T: Default 
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
            performance_indicators: HashMap::new(),
            context: HashMap::new(),
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
