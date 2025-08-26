

use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalServiceRegistration {

    pub service_id: String,

    pub version: Version,

    pub metadata: ServiceMetadata,

    pub capabilities: Vec<String>,

    pub contact_info: ContactInfo,

    pub registered_at: DateTime<Utc>,

    pub health_endpoint: Option<String>,

    pub category: ServiceCategory,

    pub security_domain: SecurityDomain,
}

pub struct ServiceMetadata {

    pub name: String,

    pub description: String,

    pub documentation: Option<String>,

    pub license: String,

    pub tags: Vec<String>,

    pub properties: HashMap<String, String>,

    pub dependencies: Vec<String>,

pub struct ContactInfo {

    pub email: Option<String>,

    pub support_url: Option<String>,

    pub repository: Option<String>,

pub enum ServiceCategory {
    Security,
    Storage,
    Compute,
    Network,
    AI,
    Compliance,
    Monitoring,
    Other(String),

pub enum SecurityDomain {

    Cryptography,

    IAM,

    ThreatDetection,

    DataProtection,

    General,

pub struct UniversalRequest {

    pub request_id: String,

    pub parameters: HashMap<String, serde_json::Value>,

    pub timestamp: DateTime<Utc>,

    pub source: Option<String>,

    pub target: Option<String>,

pub struct UniversalResponse {

    pub success: bool,

    pub data: serde_json::Value,

    pub error: Option<String>,

impl Default for ServiceCategory {}

    fn default() -> Self {
        Self::Other("unknown".to_string())
    }
impl Default for SecurityDomain {
        Self::General
