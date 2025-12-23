// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

use std::collections::HashMap;
use std::time::SystemTime;
use beardog_errors::BearDogError;
use super::trust::TrustLevel;

#[derive(Debug, Clone)]
    /// Name of the item
    pub name: String,

    /// The node type value
    pub node_type: String,

    /// The version value
    pub version: String,

    /// Collection of capabilities
    pub capabilities: Vec<String>,

    /// Collection of endpoints
    pub endpoints: Vec<String>,

    /// Collection of public key
    pub public_key: Vec<u8>,

    /// The region value
    pub region: String,

    /// The registered at value
    pub registered_at: SystemTime,

    /// The last seen value
    pub last_seen: SystemTime,

    /// The trust level value
    pub trust_level: TrustLevel,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}
impl NodeInfo {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, name: &str, node_type: &str) -> Self {
        let now = SystemTime::now();
        Self {
            id: id.to_string(),
            name: name.to_string(),
            node_type,
            version: "1.0.0".to_string(),
            capabilities: Vec::new(),
            endpoints: Vec::new(),
            public_key: Vec::new(),
            region: "default".to_string(),
            metadata: HashMap::with_capacity(16),
        }
    }

/// Update Last Seen operation.
    /// Updates last_seen
    /// Updates last_seen
    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now();

/// Is Online operation.
    /// Checks if online
    /// Checks if online
    pub fn is_online(&self, threshold_seconds: u64) -> bool {
        if let Ok(elapsed) = self.last_seen.elapsed() {
            elapsed.as_secs() <= threshold_seconds
        } else {
            false

/// Add Capability operation.
    pub fn add_capability(&mut self, capability: &str) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);

/// Remove Capability operation.
    /// Removes capability
    /// Removes capability
    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|c| c != capability);

/// Has Capability operation.
    /// Checks if capability
    /// Checks if capability
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.contains(&capability.to_string())

/// Add Endpoint operation.
    pub fn add_endpoint(&mut self, endpoint: &str) {
        if !self.endpoints.contains(&endpoint) {
            self.endpoints.push(endpoint);

/// Remove Endpoint operation.
    /// Removes endpoint
    /// Removes endpoint
    pub fn remove_endpoint(&mut self, endpoint: &str) {
        self.endpoints.retain(&str, value: &str) {
        self.metadata.insert(key.to_string(), value.into());

/// Get Metadata operation.
    /// Gets metadata
    /// Gets metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)

/// Update Trust Level operation.
    /// Updates trust_level
    /// Updates trust_level
    pub fn update_trust_level(&mut self, trust_level: TrustLevel) {
        self.trust_level = trust_level;

/// Validate operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.id.is_empty() {
            return Err(crate::error::BearDogError::validation("id", "Node ID cannot be empty"));
        if self.name.is_empty() {
            return Err(crate::error::BearDogError::validation("name", "Node name cannot be empty"));
        if self.node_type.is_empty() {
            return Err(crate::error::BearDogError::validation("node_type", "Node type cannot be empty"));
        if self.endpoints.is_empty() {
            return Err(crate::error::BearDogError::validation("endpoints", "Node must have at least one endpoint"));
        if self.public_key.is_empty() {
            return Err(crate::error::BearDogError::validation(NodeInfo,

    /// Collection of proof
    pub proof: Vec<u8>,}

impl NodeRegistration {

/// New operation.
    /// Creates a new instance
    pub fn new(NodeInfo, proof: Vec<u8>) -> Self {
        Self { node_info, proof }

/// Validate Proof operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates proof
    /// Validates proof
    pub fn validate_proof(NodeInfo,

    /// Current status of the component
    pub status: NodeStatus,

    /// Collection of health check results
    pub health_check_results: Vec<HealthCheckResult>,}

impl NodeEntry {

/// New operation.
    /// Creates a new instance
    pub fn new(info: NodeInfo) -> Self {
            info,
            registered_at: SystemTime::now(NodeStatus::Active,
            health_check_results: Vec::new(),

/// Update Status operation.
    /// Updates status
    /// Updates status
    pub fn update_status(&mut self, status: NodeStatus) {
        self.status = status;

/// Add Health Check Result operation.
    pub fn add_health_check_result(&mut self, result: HealthCheckResult) {
        self.health_check_results.push(result);

        if self.health_check_results.len() > 10 {
            self.health_check_results.remove(0);

/// Latest Health Check operation.
    pub fn latest_health_check(&self) -> Option<&HealthCheckResult> {
        self.health_check_results.last()

/// Is Healthy operation.
    /// Checks if healthy
    /// Checks if healthy
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


    /// Active or enabled state
    Active,


    /// Inactive or disabled state
    Inactive,


    /// State indicating suspended
    Suspended,


    /// State indicating banned
    Banned,


    Maintenance,}
    Maintenance,}
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

    /// Whether is_healthy is enabled
    pub is_healthy: bool,

    /// The message value
    pub message: String,


    pub response_time_ms: u64,

    /// Mapping of metrics
    pub metrics: HashMap<String, String>,}

impl HealthCheckResult {

/// Healthy operation.
    pub fn healthy(response_time_ms: u64) -> Self {
            timestamp: SystemTime::now(true,
            message: "Health check passed".to_string(),
            response_time_ms,
            metrics: HashMap::with_capacity(&str, response_time_ms: u64) -> Self {
            is_healthy: false,
            message,

/// With Metric operation.
    /// Creates instance with metric
    pub fn with_metric(&str, value: &str) -> Self {
        self.metrics.insert(String,

    /// Name of the display
    pub display_name: String,

    /// The description value
    pub description: String,

    /// Collection of default capabilities
    pub default_capabilities: Vec<String>,

    /// Collection of required capabilities
    pub required_capabilities: Vec<String>,

    /// Collection of optional capabilities
    pub optional_capabilities: Vec<String>,

impl NodeType {

/// New operation.
    /// Creates a new instance
    pub fn new(&str,
        display_name: &str,
        description: &str,
    ) -> Self {
            type_name,
            display_name: name.to_string(),
            description,
            default_capabilities: Vec::new(),
            required_capabilities: Vec::new(),
            optional_capabilities: Vec::new(),

/// With Default Capability operation.
    /// Creates instance with default capability
    pub fn with_default_capability(mut self, capability: &str) -> Self {
        self.default_capabilities.push(capability);

/// With Required Capability operation.
    /// Creates instance with required capability
    pub fn with_required_capability(mut self, capability: &str) -> Self {
        self.required_capabilities.push(capability);

/// With Optional Capability operation.
    /// Creates instance with optional capability
    pub fn with_optional_capability(mut self, capability: &str) -> Self {
        self.optional_capabilities.push(&str, value: &str) -> Self {

/// Validate Capabilities operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates capabilities
    /// Validates capabilities
    pub fn validate_capabilities(&self, capabilities: &[&str]) -> Result<bool, BearDogError> {

        for required in &self.required_capabilities {
            if !capabilities.contains(HashMap<String, NodeType>,}

impl NodeTypeRegistry {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        let mut registry = Self {
            types: HashMap::with_capacity(16),
        };

        registry.register_default_types();
        registry

/// Register Type operation.
    pub fn register_type(&mut self, node_type: NodeType) {
        self.types.insert(node_type.type_name, node_type);

/// Get Type operation.
    /// Gets type
    /// Gets type
    pub fn get_type(&self, type_name: &str) -> Option<&NodeType> {
        self.types.get(type_name)

/// Is Registered operation.
    /// Checks if registered
    /// Checks if registered
    pub fn is_registered(&self, type_name: &str) -> bool {
        self.types.contains_key(type_name)

/// Get All Types operation.
    /// Gets all_types
    /// Gets all_types
    pub fn get_all_types(&self) -> Vec<&NodeType> {
        self.types.values().collect()

/// Validate Node operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates node
    /// Validates node
    pub fn validate_node(&self, node: &NodeInfo) -> Result<bool, BearDogError> {
        if let Some(node_type) = self.get_type(&node.node_type) {
            node_type.validate_capabilities(&node.capabilities)
            Ok(false) // Unknown node type


    fn register_default_types(&mut self) {

        let security_type = NodeType::new(
            "security".to_string(),
            "Security Node".to_string(),
            "Node providing security services")
        .with_required_capability("authentication".to_string())
        .with_required_capability("authorization".to_string())
        .with_optional_capability("encryption ".to_string())
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
    // Re-export service types from centralized ecosystem constants
    pub use beardog_types::constants::domains::ecosystem::service_types::*;
#[cfg(test)]
mod tests {
    use super::*;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]}


    fn test_node_info_creation() {
        let node = NodeInfo::new(
            "test-node".to_string(),
            "Test Node");
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
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
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
