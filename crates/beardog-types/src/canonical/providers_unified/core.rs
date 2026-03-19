// SPDX-License-Identifier: AGPL-3.0-only

// Core Provider Types
//
// Fundamental provider configuration types and enums.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core provider settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CoreProviderSettings {
    /// Provider identifier
    pub provider_id: String,

    /// Provider type classification
    pub provider_type: ProviderType,

    /// Provider name
    /// Name of the item
    pub name: String,

    /// Provider version
    /// The version value
    pub version: String,

    /// Provider enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Provider priority (lower = higher priority)
    /// Number of priority
    pub priority: u32,

    /// Provider capabilities
    /// Collection of capabilities
    pub capabilities: Vec<ProviderCapability>,

    /// Provider metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, serde_json::Value>,

    /// Collection of tags
    pub tags: Vec<String>,
}

/// Provider type classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
/// Types of provider
pub enum ProviderType {
    /// Security providers (auth, crypto, etc.)
    Security,

    /// Storage providers (database, cache, etc.)
    Storage,

    /// Network providers (HTTP, gRPC, etc.)
    Network,

    /// Monitoring providers (metrics, logging, etc.)
    Monitoring,

    /// AI/ML providers (inference, training, etc.)
    AI,

    /// Workflow providers (orchestration, scheduling, etc.)
    Workflow,

    /// Integration providers (adapters, connectors, etc.)
    Integration,

    /// Compute providers (processing, calculation, etc.)
    Compute,

    /// Communication providers (messaging, notifications, etc.)
    Communication,

    /// Analytics providers (reporting, analysis, etc.)
    Analytics,

    /// Backup providers (backup, restore, etc.)
    Backup,

    /// Custom provider type
    Custom(String),

    #[default]
    /// Generic provider type
    Generic,
}

/// Provider capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapability {
    /// Capability name
    /// Name of the item
    pub name: String,

    /// Capability version
    /// The version value
    pub version: String,

    /// Capability description
    /// Optional description
    pub description: Option<String>,

    /// Required parameters
    /// Collection of parameters
    pub parameters: Vec<CapabilityParameter>,

    /// Optional features
    /// Collection of features
    pub features: Vec<String>,
}

/// Capability parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityParameter {
    /// Parameter name
    /// Name of the item
    pub name: String,

    /// Parameter type
    /// The parameter type value
    pub parameter_type: ParameterType,

    /// Required parameter
    /// Whether required is enabled
    pub required: bool,

    /// Default value
    /// Optional default value
    pub default_value: Option<serde_json::Value>,

    /// Parameter description
    /// Optional description
    pub description: Option<String>,

    /// Validation rules
    pub validation: Vec<ValidationRule>,
}

/// Parameter types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of parameter
pub enum ParameterType {
    /// String variant
    String,
    /// Integer variant
    Integer,
    /// Float variant
    Float,
    /// Boolean variant
    Boolean,
    /// Array variant
    Array,
    /// Object variant
    Object,
    /// Enumeration with possible values
    Enum(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRule {
    MinValue(f64),

    MaxValue(f64),

    MinLength(usize),

    MaxLength(usize),

    Pattern(String),

    /// Custom validation function name
    Custom(String),
}

// Missing types that are imported elsewhere
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderRegistryEntry {
    /// Provider Id
    pub provider_id: String,
    /// Name
    /// Name of the item
    pub name: String,
    /// Status
    /// Current status of the component
    pub status: ProviderStatus,
    /// Health
    /// The health value
    pub health: ProviderHealth,
}

/// Status of a provider in the system
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ProviderStatus {
    /// Provider is active and available
    #[default]
    /// Active or enabled state
    Active,
    /// Inactive variant
    Inactive,
    /// Error variant
    Error,
    /// Maintenance variant
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderHealth {
    /// Healthy
    /// Whether healthy is enabled
    pub healthy: bool,
    /// Last Check
    /// Optional last check
    pub last_check: Option<String>,
    /// Message
    /// Optional message
    pub message: Option<String>,
}
