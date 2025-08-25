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


/// Federation and service advertisement types
///
/// This module contains types for federation management and service discovery
/// within the distributed node registry system.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use super::trust::TrustLevel;
/// Distributed registry information
#[derive(Debug, Clone)]
pub struct DistributedRegistryInfo {
    /// Registry unique identifier
    pub registry_id: String,
    /// Registry name
    pub registry_name: String,
    /// Registry endpoint
    pub endpoint: String,
    /// Registry public key
    pub public_key: Vec<u8>,
    /// Registry capabilities
    pub capabilities: Vec<String>,
    /// Registry region
    pub region: String,
    /// Federation status
    pub federation_status: FederationStatus,
    /// Last seen timestamp
    pub last_seen: SystemTime,
    /// Trust level for this registry
    pub trust_level: TrustLevel,
}
/// Federation status for a registry
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FederationStatus {
    /// Not federated
    NotFederated,
    /// Attempting to federate
    Connecting,
    /// Successfully federated
    Federated,
    /// Federation failed
    Failed,
    /// Temporarily disconnected
    Disconnected,
/// Service advertisement for discovery}


pub struct ServiceAdvertisement {
    /// Service unique identifier
    pub service_id: String,
    /// Service name
    pub service_name: String,
    /// Service type
    pub service_type: String,
    /// Service version
    pub version: String,
    /// Service endpoints
    pub endpoints: Vec<String>,
    /// Service capabilities
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Geographic region
    /// Advertisement TTL
    pub ttl: Duration,
    /// Service health status
    pub health_status: ServiceHealthStatus,
/// Service health status
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ServiceHealthStatus {
    /// Service is healthy
    Healthy,
    /// Service is degraded
    Degraded,
    /// Service is unavailable
    Unavailable,
    /// Service health is unknown
    Unknown,
/// Standard service type constants for node registry}


pub mod service_types {
    use beardog_types::constants::unified::nodes::*;
    
    /// Security service type for authentication and authorization
    pub const SECURITY: &str = SECURITY_SERVICE;
    /// Phonebook service type for node discovery  
    pub const PHONEBOOK: &str = PHONEBOOK_SERVICE;
    /// Federation service type for network federation
    pub const FEDERATION: &str = FEDERATION_SERVICE;
    /// Compute service type for distributed computing
    pub const COMPUTE: &str = "compute";
    /// Storage service type for distributed storage
    pub const STORAGE: &str = "storage";
    /// Relay service type for message relaying
    pub const RELAY: &str = "relay";
    /// Backup service type for data backup
    pub const BACKUP: &str = "backup";}


impl ServiceAdvertisement {
    /// Create a new service advertisement}


    pub fn new(service_id: String, service_name: String, service_type: String) -> Self {
        Self {
            service_id,
            service_name,
            service_type,
            version: "1.0.0".to_string(),
            endpoints: Vec::new(),
            capabilities: Vec::new(),
            metadata: HashMap::new(),
            region: "default".to_string(),
            ttl: Duration::from_secs(300), // 5 minutes
            health_status: ServiceHealthStatus::Unknown,
        }
    }
    /// Add endpoint
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoints.push(endpoint);
        self
    /// Add capability}


    pub fn with_capability(mut self, capability: String) -> Self {
        self.capabilities.push(capability);
    /// Set region
    pub fn with_region(mut self, region: String) -> Self {
        self.region = region;
    /// Set TTL}


    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = ttl;
    /// Set health status
    pub fn with_health_status(mut self, status: ServiceHealthStatus) -> Self {
        self.health_status = status;
    /// Add metadata}


    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
impl DistributedRegistryInfo {
    /// Create a new distributed registry info}


    pub fn new(registry_id: String, registry_name: String, endpoint: String) -> Self {
            registry_id,
            registry_name,
            endpoint,
            public_key: Vec::new(),
            federation_status: FederationStatus::NotFederated,
            last_seen: SystemTime::now(),
            trust_level: TrustLevel::Unknown,
    /// Set public key}


    pub fn with_public_key(mut self, public_key: Vec<u8>) -> Self {
        self.public_key = public_key;
    /// Set federation status
    pub fn with_federation_status(mut self, status: FederationStatus) -> Self {
        self.federation_status = status;
    /// Set trust level}


    pub fn with_trust_level(mut self, trust_level: TrustLevel) -> Self {
        self.trust_level = trust_level;
    /// Update last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now();
    /// Check if registry is online}


    pub fn is_online(&self, threshold_seconds: u64) -> bool {
        if let Ok(elapsed) = self.last_seen.elapsed() {
            elapsed.as_secs() <= threshold_seconds
        } else {
            false
impl std::fmt::Display for FederationStatus {}


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FederationStatus::NotFederated => write!(f, "Not Federated"),
            FederationStatus::Connecting => write!(f, "Connecting"),
            FederationStatus::Federated => write!(f, "Federated"),
            FederationStatus::Failed => write!(f, "Failed"),
            FederationStatus::Disconnected => write!(f, "Disconnected"),}


impl std::fmt::Display for ServiceHealthStatus {
            ServiceHealthStatus::Healthy => write!(f, "Healthy"),
            ServiceHealthStatus::Degraded => write!(f, "Degraded"),
            ServiceHealthStatus::Unavailable => write!(f, "Unavailable"),
            ServiceHealthStatus::Unknown => write!(f, "Unknown"),}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_service_advertisement_creation() {
        let ad = ServiceAdvertisement::new(
            "test-service".to_string(),
            "Test Service".to_string(),
            "security".to_string(),
        )
        .with_endpoint("http://localhost:8080".to_string())
        .with_capability("authentication".to_string())
        .with_region("us-east".to_string());
        assert_eq!(ad.service_id, "test-service");
        assert_eq!(ad.service_name, "Test Service");
        assert_eq!(ad.service_type, "security");
        assert!(ad.endpoints.contains(&"http://localhost:8080".to_string()));
        assert!(ad.capabilities.contains(&"authentication".to_string()));
        assert_eq!(ad.region, "us-east");}


    fn test_distributed_registry_info() {
        let info = DistributedRegistryInfo::new(
            "registry-1".to_string(),
            "Registry One".to_string(),
            "http://registry.example.com:8080".to_string(),
        .with_capability("federation".to_string())
        .with_trust_level(TrustLevel::High);
        assert_eq!(info.registry_id, "registry-1");
        assert_eq!(info.registry_name, "Registry One");
        assert_eq!(info.endpoint, "http://registry.example.com:8080");
        assert!(info.capabilities.contains(&"federation".to_string()));
        assert_eq!(info.trust_level, TrustLevel::High);
    fn test_federation_status_display() {
        assert_eq!(FederationStatus::NotFederated.to_string(), "Not Federated");
        assert_eq!(FederationStatus::Connecting.to_string(), "Connecting");
        assert_eq!(FederationStatus::Federated.to_string(), "Federated");
        assert_eq!(FederationStatus::Failed.to_string(), "Failed");
        assert_eq!(FederationStatus::Disconnected.to_string(), "Disconnected");}


    fn test_service_health_status_display() {
        assert_eq!(ServiceHealthStatus::Healthy.to_string(), "Healthy");
        assert_eq!(ServiceHealthStatus::Degraded.to_string(), "Degraded");
        assert_eq!(ServiceHealthStatus::Unavailable.to_string(), "Unavailable");
        assert_eq!(ServiceHealthStatus::Unknown.to_string(), "Unknown");
} 
