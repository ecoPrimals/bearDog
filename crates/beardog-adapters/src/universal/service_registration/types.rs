// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
    /// The version value
    pub version: String, // Changed from Version to String to avoid serialization issues

    /// The metadata value
    pub metadata: ServiceMetadata,

    /// Collection of capabilities
    pub capabilities: Vec<String>,

    /// The contact info value
    pub contact_info: ContactInfo,

    /// The registered at value
    pub registered_at: DateTime<Utc>,

    /// Optional health endpoint
    pub health_endpoint: Option<String>,

    /// The category value
    pub category: ServiceCategory,

    /// The security domain value
    pub security_domain: SecurityDomain,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceMetadata {
    /// Name of the item
    pub name: String,
    /// The description value
    pub description: String,
    /// Optional documentation
    pub documentation: Option<String>,
    /// The license value
    pub license: String,
    /// Collection of tags
    pub tags: Vec<String>,
    /// Mapping of properties
    pub properties: HashMap<String, String>,
    /// Collection of dependencies
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContactInfo {
    /// Optional email
    pub email: Option<String>,

    /// Optional support url
    pub support_url: Option<String>,
    /// Optional repository
    pub repository: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ServiceCategory {
    /// Represents security variant
    Security,
    /// Represents storage variant
    Storage,
    /// Represents compute variant
    Compute,
    /// Represents network variant
    Network,
    /// Represents a i variant
    AI,
    /// Represents compliance variant
    Compliance,
    /// Currently monitoring
    Monitoring,
    /// Represents other variant
    Other(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SecurityDomain {
    /// Represents cryptography variant
    Cryptography,
    /// Represents iam variant
    Iam,
    /// Represents threat detection variant
    ThreatDetection,
    /// Represents data protection variant
    DataProtection,
    /// Represents general variant
    General,
}

pub struct UniversalRequest {
    pub request_id: String,

    /// Mapping of parameters
    pub parameters: HashMap<String, serde_json::Value>,


    pub timestamp: DateTime<Utc>,

    /// Optional source
    pub source: Option<String>,

    /// Optional target
    pub target: Option<String>,
}

pub struct UniversalResponse {
    /// Whether success is enabled
    pub success: bool,
    /// The data value
    pub data: serde_json::Value,
    /// Optional error
    pub error: Option<String>,
}

impl Default for ServiceCategory {
    fn default() -> Self {
        Self::Other("unknown".to_string())
    }
}

impl Default for SecurityDomain {
    fn default() -> Self {
        Self::General
    }
}
