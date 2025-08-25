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


/// Service Registration Types
///
/// Universal service registration types for ecosystem integration
use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// UUID generation handled by individual service implementations

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
/// Contact information
pub struct ContactInfo {
    /// Maintainer email
    pub email: Option<String>,
    /// Support URL
    pub support_url: Option<String>,
    /// Repository URL
    pub repository: Option<String>,
/// Service category
pub enum ServiceCategory {
    Security,
    Storage,
    Compute,
    Network,
    AI,
    Compliance,
    Monitoring,
    Other(String),
/// Security domain classification}


pub enum SecurityDomain {
    /// Cryptographic operations
    Cryptography,
    /// Identity and access management
    IAM,
    /// Compliance and audit
    /// Threat detection
    ThreatDetection,
    /// Data protection
    DataProtection,
    /// General security
    General,
/// Universal request structure
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
/// Universal response structure
pub struct UniversalResponse {
    /// Response to request ID
    /// Success indicator
    pub success: bool,
    /// Response data
    pub data: serde_json::Value,
    /// Error message if failed
    pub error: Option<String>,
    /// Response timestamp}


impl Default for ServiceCategory {}


    fn default() -> Self {
        Self::Other("unknown".to_string())
    }
impl Default for SecurityDomain {
        Self::General
