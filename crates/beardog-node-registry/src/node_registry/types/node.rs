

use std::collections::HashMap;
use std::time::SystemTime;
use crate::BearDogResult;
use super::trust::TrustLevel;

#[derive(Debug, Clone)]
pub struct NodeInfo {

    pub id: String,

    pub name: String,

    pub node_type: String,

    pub version: String,

    pub capabilities: Vec<String>,

    pub endpoints: Vec<String>,

    pub public_key: Vec<u8>,

    pub region: String,

    pub registered_at: SystemTime,

    pub last_seen: SystemTime,

    pub trust_level: TrustLevel,

    pub metadata: HashMap<String, String>,
}
impl NodeInfo {

    pub fn new(id: &str, name: &str, node_type: &str) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            name,
            node_type,
            version: "1.0.0".to_string(),
            capabilities: Vec::new(),
            endpoints: Vec::new(),
            public_key: Vec::new(),
            region: "default".to_string(),
            registered_at: now,
            last_seen: now,
            trust_level: TrustLevel::Unknown,
            metadata: HashMap::with_capacity(16),
        }
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now();

    pub fn is_online(&self, threshold_seconds: u64) -> bool {
        if let Ok(elapsed) = self.last_seen.elapsed() {
            elapsed.as_secs() <= threshold_seconds
        } else {
            false

    pub fn add_capability(&mut self, capability: &str) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);

    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|c| c != capability);

    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.contains(&capability.to_string())

    pub fn add_endpoint(&mut self, endpoint: &str) {
        if !self.endpoints.contains(&endpoint) {
            self.endpoints.push(endpoint);

    pub fn remove_endpoint(&mut self, endpoint: &str) {
        self.endpoints.retain(|e| e != endpoint);

    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key, value);

    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)

    pub fn update_trust_level(&mut self, trust_level: TrustLevel) {
        self.trust_level = trust_level;

    pub fn validate(&self) -> BearDogResult<()> {
        if self.id.is_empty() {
            return Err(crate::error::BearDogError::validation("id", "Node ID cannot be empty"));
        if self.name.is_empty() {
            return Err(crate::error::BearDogError::validation("name", "Node name cannot be empty"));
        if self.node_type.is_empty() {
            return Err(crate::error::BearDogError::validation("node_type", "Node type cannot be empty"));
        if self.endpoints.is_empty() {
            return Err(crate::error::BearDogError::validation("endpoints", "Node must have at least one endpoint"));
        if self.public_key.is_empty() {
            return Err(crate::error::BearDogError::validation("public_key", "Node must have a public key"));
        Ok(())

pub struct NodeRegistration {

    pub node_info: NodeInfo,

    pub proof: Vec<u8>,}

impl NodeRegistration {

    pub fn new(node_info: NodeInfo, proof: Vec<u8>) -> Self {
        Self { node_info, proof }

    pub fn validate_proof(&self) -> BearDogResult<bool> {

        Ok(!self.proof.is_empty())

pub struct NodeEntry {
    pub info: NodeInfo,

    pub status: NodeStatus,

    pub health_check_results: Vec<HealthCheckResult>,}

impl NodeEntry {

    pub fn new(info: NodeInfo) -> Self {
            info,
            registered_at: SystemTime::now(),
            status: NodeStatus::Active,
            health_check_results: Vec::new(),

    pub fn update_status(&mut self, status: NodeStatus) {
        self.status = status;

    pub fn add_health_check_result(&mut self, result: HealthCheckResult) {
        self.health_check_results.push(result);

        if self.health_check_results.len() > 10 {
            self.health_check_results.remove(0);

    pub fn latest_health_check(&self) -> Option<&HealthCheckResult> {
        self.health_check_results.last()

    pub fn is_healthy(&self) -> bool {
        match self.status {
            NodeStatus::Active => {
                if let Some(health_check) = self.latest_health_check() {
                    health_check.is_healthy
                } else {
                    true // No health check results yet, assume healthy
                }
            }
            _ => false,

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeStatus {

    Active,

    Inactive,

    Suspended,

    Banned,

    Maintenance,}

impl std::fmt::Display for NodeStatus {}

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeStatus::Active => write!(f, "Active"),
            NodeStatus::Inactive => write!(f, "Inactive"),
            NodeStatus::Suspended => write!(f, "Suspended"),
            NodeStatus::Banned => write!(f, "Banned"),
            NodeStatus::Maintenance => write!(f, "Maintenance"),

pub struct HealthCheckResult {

    pub timestamp: SystemTime,

    pub is_healthy: bool,

    pub message: String,

    pub response_time_ms: u64,

    pub metrics: HashMap<String, String>,}

impl HealthCheckResult {

    pub fn healthy(response_time_ms: u64) -> Self {
            timestamp: SystemTime::now(),
            is_healthy: true,
            message: "Health check passed".to_string(),
            response_time_ms,
            metrics: HashMap::with_capacity(16),

    pub fn unhealthy(message: &str, response_time_ms: u64) -> Self {
            is_healthy: false,
            message,

    pub fn with_metric(mut self, key: &str, value: &str) -> Self {
        self.metrics.insert(key, value);
        self

pub struct NodeType {

    pub type_name: String,

    pub display_name: String,

    pub description: String,

    pub default_capabilities: Vec<String>,

    pub required_capabilities: Vec<String>,

    pub optional_capabilities: Vec<String>,

impl NodeType {

    pub fn new(
        type_name: &str,
        display_name: &str,
        description: &str,
    ) -> Self {
            type_name,
            display_name,
            description,
            default_capabilities: Vec::new(),
            required_capabilities: Vec::new(),
            optional_capabilities: Vec::new(),

    pub fn with_default_capability(mut self, capability: &str) -> Self {
        self.default_capabilities.push(capability);

    pub fn with_required_capability(mut self, capability: &str) -> Self {
        self.required_capabilities.push(capability);

    pub fn with_optional_capability(mut self, capability: &str) -> Self {
        self.optional_capabilities.push(capability);
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {

    pub fn validate_capabilities(&self, capabilities: &[&str]) -> BearDogResult<bool> {

        for required in &self.required_capabilities {
            if !capabilities.contains(required) {
                return Ok(false);

        for capability in capabilities {
            if !self.required_capabilities.contains(capability) 
                && !self.optional_capabilities.contains(capability) {
        Ok(true)

    pub fn all_capabilities(&self) -> Vec<String> {
        let mut all_caps = self.required_capabilities.clone();
        all_caps.extend(self.optional_capabilities.clone());
        all_caps.sort();
        all_caps.dedup();
        all_caps

pub struct NodeTypeRegistry {

    pub types: HashMap<String, NodeType>,}

impl NodeTypeRegistry {

    pub fn new() -> Self {
        let mut registry = Self {
            types: HashMap::with_capacity(16),
        };

        registry.register_default_types();
        registry

    pub fn register_type(&mut self, node_type: NodeType) {
        self.types.insert(node_type.type_name.clone(), node_type);

    pub fn get_type(&self, type_name: &str) -> Option<&NodeType> {
        self.types.get(type_name)

    pub fn is_registered(&self, type_name: &str) -> bool {
        self.types.contains_key(type_name)

    pub fn get_all_types(&self) -> Vec<&NodeType> {
        self.types.values().collect()

    pub fn validate_node(&self, node: &NodeInfo) -> BearDogResult<bool> {
        if let Some(node_type) = self.get_type(&node.node_type) {
            node_type.validate_capabilities(&node.capabilities)
            Ok(false) // Unknown node type

    fn register_default_types(&mut self) {

        let security_type = NodeType::new(
            "security".to_string(),
            "Security Node".to_string(),
            "Node providing security services".to_string(),
        )
        .with_required_capability("authentication".to_string())
        .with_required_capability("authorization".to_string())
        .with_optional_capability("encryption".to_string())
        .with_optional_capability("threat_detection".to_string());

        let phonebook_type = NodeType::new(
            "phonebook".to_string(),
            "Phonebook Node".to_string(),
            "Node providing discovery services".to_string(),
        .with_required_capability("discovery".to_string())
        .with_optional_capability("caching".to_string());

        let compute_type = NodeType::new(
            "compute".to_string(),
            "Compute Node".to_string(),
            "Node providing computational services".to_string(),
        .with_required_capability("computation".to_string())
        .with_optional_capability("gpu_acceleration".to_string())
        .with_optional_capability("parallel_processing".to_string());

        let storage_type = NodeType::new(
            "storage".to_string(),
            "Storage Node".to_string(),
            "Node providing storage services".to_string(),
        .with_required_capability("storage".to_string())
        .with_optional_capability("replication".to_string())
        .with_optional_capability("backup".to_string());
        self.register_type(security_type);
        self.register_type(phonebook_type);
        self.register_type(compute_type);
        self.register_type(storage_type);
impl Default for NodeTypeRegistry {}

    fn default() -> Self {
        Self::new()

pub mod node_types {
    use beardog_types::constants::unified::nodes::*;

    pub const SECURITY: &str = SECURITY_SERVICE;

    pub const PHONEBOOK: &str = PHONEBOOK_SERVICE;

    pub const FEDERATION: &str = FEDERATION_SERVICE;

    pub const COMPUTE: &str = "compute";

    pub const STORAGE: &str = "storage";

    pub const RELAY: &str = "relay";

    pub const BACKUP: &str = "backup";

    pub const MONITORING: &str = "monitoring";

    pub const ANALYTICS: &str = "analytics";

    pub const GATEWAY: &str = "gateway";
#[cfg(test)]
mod tests {
    use super::*;
    #[test]}

    fn test_node_info_creation() {
        let node = NodeInfo::new(
            "test-node".to_string(),
            "Test Node".to_string(),
        );
        assert_eq!(node.id, "test-node");
        assert_eq!(node.name, "Test Node");
        assert_eq!(node.node_type, "security");
        assert_eq!(node.trust_level, TrustLevel::Unknown);
    fn test_node_capabilities() {
        let mut node = NodeInfo::new(
        
        node.add_capability("authentication".to_string());
        assert!(node.has_capability("authentication"));
        node.remove_capability("authentication");
        assert!(!node.has_capability("authentication"));}

    fn test_node_type_validation() {
        let node_type = NodeType::new(
            "test".to_string(),
            "Test Type".to_string(),
            "Test node type".to_string(),
        .with_required_capability("required_cap".to_string())
        .with_optional_capability("optional_cap".to_string());

        assert!(node_type.validate_capabilities(&["required_cap".to_string()]).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?);

        assert!(!node_type.validate_capabilities(&["optional_cap".to_string()]).map_err(|e| {

        assert!(node_type.validate_capabilities(&[
            "required_cap".to_string(),
            "optional_cap".to_string()
        ]).map_err(|e| {
    fn test_node_type_registry() {
        let registry = NodeTypeRegistry::new();

        assert!(registry.is_registered("security"));
        assert!(registry.is_registered("phonebook"));
        assert!(registry.is_registered("compute"));
        assert!(registry.is_registered("storage"));

        assert!(!registry.is_registered("unknown"));}

    fn test_health_check_result() {
        let healthy = HealthCheckResult::healthy(100);
        assert!(healthy.is_healthy);
        assert_eq!(healthy.response_time_ms, 100);
        let unhealthy = HealthCheckResult::unhealthy("Connection timeout during health check".to_string(), 1000);
        assert!(!unhealthy.is_healthy);
        assert_eq!(unhealthy.response_time_ms, 1000);
    fn test_node_entry() {
        let node_info = NodeInfo::new(
        let mut entry = NodeEntry::new(node_info);
        assert_eq!(entry.status, NodeStatus::Active);
        assert!(entry.is_healthy()); // No health checks yet, assume healthy
        entry.add_health_check_result(HealthCheckResult::unhealthy("Service unavailable - connection refused".to_string(), 1000));
        assert!(!entry.is_healthy());
} 
