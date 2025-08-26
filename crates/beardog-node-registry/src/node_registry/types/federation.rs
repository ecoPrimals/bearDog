

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use super::trust::TrustLevel;

#[derive(Debug, Clone)]
pub struct DistributedRegistryInfo {

    pub registry_id: String,

    pub registry_name: String,

    pub endpoint: String,

    pub public_key: Vec<u8>,

    pub capabilities: Vec<String>,

    pub region: String,

    pub federation_status: FederationStatus,

    pub last_seen: SystemTime,

    pub trust_level: TrustLevel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FederationStatus {

    NotFederated,

    Connecting,

    Federated,

    Failed,

    Disconnected,

pub struct ServiceAdvertisement {

    pub service_id: String,

    pub service_name: String,

    pub service_type: String,

    pub version: String,

    pub endpoints: Vec<String>,

    pub metadata: HashMap<String, String>,

    pub ttl: Duration,

    pub health_status: ServiceHealthStatus,

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ServiceHealthStatus {

    Healthy,

    Degraded,

    Unavailable,

    Unknown,

pub mod service_types {
    use beardog_types::constants::unified::nodes::*;

    pub const SECURITY: &str = SECURITY_SERVICE;

    pub const PHONEBOOK: &str = PHONEBOOK_SERVICE;

    pub const FEDERATION: &str = FEDERATION_SERVICE;

    pub const COMPUTE: &str = "compute";

    pub const STORAGE: &str = "storage";

    pub const RELAY: &str = "relay";

    pub const BACKUP: &str = "backup";}

impl ServiceAdvertisement {

    pub fn new(service_id: &str, service_name: &str, service_type: &str) -> Self {
        Self {
            service_id,
            service_name,
            service_type,
            version: "1.0.0".to_string(),
            endpoints: Vec::new(),
            capabilities: Vec::new(),
            metadata: HashMap::with_capacity(16),
            region: "default".to_string(),
            ttl: Duration::from_secs(300), // 5 minutes
            health_status: ServiceHealthStatus::Unknown,
        }
    }

    pub fn with_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoints.push(endpoint);
        self

    pub fn with_capability(mut self, capability: &str) -> Self {
        self.capabilities.push(capability);

    pub fn with_region(mut self, region: &str) -> Self {
        self.region = region;

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = ttl;

    pub fn with_health_status(mut self, status: ServiceHealthStatus) -> Self {
        self.health_status = status;

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key, value);
impl DistributedRegistryInfo {

    pub fn new(registry_id: &str, registry_name: &str, endpoint: &str) -> Self {
            registry_id,
            registry_name,
            endpoint,
            public_key: Vec::new(),
            federation_status: FederationStatus::NotFederated,
            last_seen: SystemTime::now(),
            trust_level: TrustLevel::Unknown,

    pub fn with_public_key(mut self, public_key: Vec<u8>) -> Self {
        self.public_key = public_key;

    pub fn with_federation_status(mut self, status: FederationStatus) -> Self {
        self.federation_status = status;

    pub fn with_trust_level(mut self, trust_level: TrustLevel) -> Self {
        self.trust_level = trust_level;

    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now();

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
