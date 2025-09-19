

use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub enum TrustLevel {

    /// Represents unknown = 0 variant
    Unknown = 0,

    /// Represents basic = 1 variant
    Basic = 1,

    /// Represents medium = 2 variant
    Medium = 2,

    /// Represents high = 3 variant
    High = 3,

    /// Represents explicit = 4 variant
    Explicit = 4,
}
impl TrustLevel {

/// As U8 operation.
    /// Returns as u8
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }


/// From U8 operation.
    /// Creates instance from u8
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(TrustLevel::Unknown),
            1 => Some(TrustLevel::Basic),
            2 => Some(TrustLevel::Medium),
            3 => Some(TrustLevel::High),
            4 => Some(TrustLevel::Explicit),
            _ => None,
        }


    pub fn can_perform_sensitive_operations(&self) -> bool {
        *self >= TrustLevel::High}


/// Requires Monitoring operation.
    pub fn requires_monitoring(&self) -> bool {
        *self <= TrustLevel::Medium


/// Next operation.
    pub fn next(&self) -> Option<Self> {
        match self {
            TrustLevel::Unknown => Some(TrustLevel::Basic),
            TrustLevel::Basic => Some(TrustLevel::Medium),
            TrustLevel::Medium => Some(TrustLevel::High),
            TrustLevel::High => Some(TrustLevel::Explicit),
            TrustLevel::Explicit => None,}

impl std::fmt::Display for TrustLevel {}

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            TrustLevel::Unknown => write!(f, "Unknown"),
            TrustLevel::Basic => write!(f, "Basic"),
            TrustLevel::Medium => write!(f, "Medium"),
            TrustLevel::High => write!(f, "High"),
            TrustLevel::Explicit => write!(f, "Explicit"),

#[derive(Debug, Clone)]
    pub min_trust_level: TrustLevel,

    pub decay_factor: f64,

    pub enabled: bool,

    pub max_age: Duration,}

impl Default for TrustPropagationConfig {
    fn default(3,
            min_trust_level: TrustLevel::Medium,
            decay_factor: 0.8,
            enabled: true,
            max_age: Duration::from_secs(HashMap<String, HashMap<String, TrustLevel>>,

    pub(TrustPropagationConfig,}

impl TrustStore {

/// New operation.
    /// Creates a new instance
    pub fn new(config: TrustPropagationConfig) -> Self {
            relationships: HashMap::with_capacity(config,}

/// Add Relationship operation.
    pub fn add_relationship(&str, to: &str, trust_level: TrustLevel) {
        self.relationships
            .entry(from)
            .or_insert_with(HashMap::new)
            .insert(&str, to: &str) -> Option<TrustLevel> {
            .get(&str, to: &str) {
        if let Some(targets) = self.relationships.get_mut(from) {
            targets.remove(to);
            if targets.is_empty() {
                self.relationships.remove(from);
            }


/// Get Relationships operation.
    /// Gets relationships
    /// Gets relationships
    pub fn get_relationships(&self, node: &str) -> HashMap<String, TrustLevel> {
            .get(&str, to: &str) -> Option<TrustLevel> {
        if let Some(direct_trust) = self.get_trust_level(from, to) {
            return Some(direct_trust);
        if !self.propagation_config.enabled {
            return None;

        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((from.to_string(), TrustLevel::Explicit, 0));
        while let Some(TrustLevel, relationship_trust: TrustLevel, hops: u8) -> TrustLevel {
        let min_trust = std::cmp::min(current_trust, relationship_trust);
        let decay = self.propagation_config.decay_factor.powi(hops as i32);
        let propagated_value = (min_trust.as_u8() as f64 * decay) as u8;

        TrustLevel::from_u8(propagated_value).unwrap_or(TrustLevel::Unknown)}


/// Get Trusting Nodes operation.
    /// Gets trusting_nodes
    /// Gets trusting_nodes
    pub fn get_trusting_nodes(&self, node_id: &str) -> Vec<String> {
        let mut trusting_nodes = Vec::new();
        for (from_node, targets) in &self.relationships {
            if targets.contains_key(node_id) {
                trusting_nodes.push(&from_node);
        trusting_nodes


/// Get Trusted Nodes operation.
    /// Gets trusted_nodes
    /// Gets trusted_nodes
    pub fn get_trusted_nodes(&self, node_id: &str) -> Vec<String> {
            .get(node_id)
            .map(|targets| targets.keys().cloned().collect())}


/// Relationship Count operation.
    pub fn relationship_count(&self) -> usize {
        self.relationships.values().map(|targets| targets.len()).sum()

/// Clear All operation.
    pub fn clear_all(&mut self) {
        self.relationships.clear();
impl Default for TrustStore {
        Self::new(TrustPropagationConfig::default(String,

    pub to_node: String,

    pub trust_level: TrustLevel,

    pub created_at: chrono::DateTime<chrono::Utc>,

    pub updated_at: chrono::DateTime<chrono::Utc>,}

impl TrustRelationship {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, to_node: &str, trust_level: TrustLevel) -> Self {
        let now = chrono::Utc::now(now,
            updated_at: now,}

/// Update Trust Level operation.
    /// Updates trust_level
    /// Updates trust_level
    pub fn update_trust_level(&mut self, trust_level: TrustLevel) {
        self.trust_level = trust_level;
        self.updated_at = chrono::Utc::now();


/// Age operation.
    pub fn age(&self) -> chrono::Duration {
        chrono::Utc::now(bool,

    pub message: String,

    pub metadata: HashMap<String, String>,}

impl NodeVerificationResult {

/// Success operation.
    pub fn success(trust_level: TrustLevel) -> Self {
            verified: true,
            message: "Node verified successfully".to_string(),
            metadata: HashMap::with_capacity(16),}


/// Failure operation.
    pub fn failure(message: &str) -> Self {
            verified: false,
            message,
            trust_level: TrustLevel::Unknown,


/// With Metadata operation.
    /// Creates instance with metadata
    pub fn with_metadata(&str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.into());
        self
#[cfg(test)]
mod tests {
    use super::*;
    #[test]}


    fn test_trust_level_progression() {
        assert_eq!(TrustLevel::Unknown.next(), Some(TrustLevel::Basic));
        assert_eq!(TrustLevel::Basic.next(), Some(TrustLevel::Medium));
        assert_eq!(TrustLevel::Medium.next(), Some(TrustLevel::High));
        assert_eq!(TrustLevel::High.next(), Some(TrustLevel::Explicit));
        assert_eq!(TrustLevel::Explicit.next(), None);}


    fn test_trust_level_conversion() {
        assert_eq!(TrustLevel::from_u8(0), Some(TrustLevel::Unknown));
        assert_eq!(TrustLevel::from_u8(1), Some(TrustLevel::Basic));
        assert_eq!(TrustLevel::from_u8(2), Some(TrustLevel::Medium));
        assert_eq!(TrustLevel::from_u8(3), Some(TrustLevel::High));
        assert_eq!(TrustLevel::from_u8(4), Some(TrustLevel::Explicit));
        assert_eq!(TrustLevel::from_u8(5), None);}


    fn test_trust_store_operations() {
        let mut store = TrustStore::default();
        store.add_relationship("node1".to_string(), "node2".to_string(), TrustLevel::High);
        assert_eq!(store.get_trust_level("node1", "node2"), Some(TrustLevel::High));
        store.remove_relationship("node1", "node2");
        assert_eq!(store.get_trust_level("node1", "node2"), None);}


    fn test_trust_relationship_age() {
        let rel = TrustRelationship::new("node1".to_string(), "node2".to_string(), TrustLevel::Medium);
        assert!(rel.age().num_seconds() >= 0);
}
