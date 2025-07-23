//! Service Registration Types
//!
//! Universal service registration types for ecosystem integration

use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Universal service registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalServiceRegistration {
    /// Service identifier
    pub service_id: String,
    /// Service version
    pub version: Version,
    /// Service metadata
    pub metadata: ServiceMetadata,
    /// Capabilities offered
    pub capabilities: Vec<String>,
    /// Contact information
    pub contact_info: ContactInfo,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    /// Health check endpoint
    pub health_endpoint: Option<String>,
    /// Service category
    pub category: ServiceCategory,
    /// Security domain
    pub security_domain: SecurityDomain,
}

/// Service metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    /// Human-readable name
    pub name: String,
    /// Service description
    pub description: String,
    /// Documentation URL
    pub documentation: Option<String>,
    /// License information
    pub license: String,
    /// Tags for discovery
    pub tags: Vec<String>,
    /// Custom properties
    pub properties: HashMap<String, String>,
    /// Dependencies
    pub dependencies: Vec<String>,
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    /// Maintainer email
    pub email: Option<String>,
    /// Support URL
    pub support_url: Option<String>,
    /// Repository URL
    pub repository: Option<String>,
}

/// Service category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceCategory {
    Security,
    Storage,
    Compute,
    Network,
    AI,
    Compliance,
    Monitoring,
    Other(String),
}

/// Security domain classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityDomain {
    /// Cryptographic operations
    Cryptography,
    /// Identity and access management
    IAM,
    /// Compliance and audit
    Compliance,
    /// Threat detection
    ThreatDetection,
    /// Data protection
    DataProtection,
    /// General security
    General,
}

/// Universal request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRequest {
    /// Request identifier
    pub request_id: String,
    /// Request parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
    /// Source service
    pub source: Option<String>,
    /// Target service
    pub target: Option<String>,
}

/// Universal response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalResponse {
    /// Response to request ID
    pub request_id: String,
    /// Success indicator
    pub success: bool,
    /// Response data
    pub data: serde_json::Value,
    /// Error message if failed
    pub error: Option<String>,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
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
