

use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
    /// The bind address value
    pub bind_address: String,

    /// Number of port
    pub port: u16,

    /// Number of cache_size
    pub cache_size: usize,

    /// Number of cache_ttl_seconds
    pub cache_ttl_seconds: u64,

    /// Number of refresh_interval_seconds
    pub refresh_interval_seconds: u64,

    /// Number of max_tracked_nodes
    pub max_tracked_nodes: usize,

    /// The node entry ttl value
    pub node_entry_ttl: Duration,

    /// Collection of default regions
    pub default_regions: Vec<String>,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}
impl Default for PhonebookConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            bind_address: std::env::var("BEARDOG_PHONEBOOK_BIND_ADDRESS")
                .or_else(|_| std::env::var("BEARDOG_BIND_ADDRESS"))
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            cache_size: std::env::var("BEARDOG_PHONEBOOK_CACHE_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10000), // 10K entries default
            cache_ttl_seconds: std::env::var("BEARDOG_PHONEBOOK_CACHE_TTL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(300), // 5 minutes default
            refresh_interval_seconds: std::env::var("BEARDOG_PHONEBOOK_REFRESH_INTERVAL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60), // 1 minute default
            max_tracked_nodes: std::env::var("BEARDOG_MAX_TRACKED_NODES")
                .ok()
                .and_then(|n| n.parse().ok())
                .unwrap_or(10000), // 10K nodes default
            node_entry_ttl: Duration::from_secs(
                std::env::var("BEARDOG_NODE_ENTRY_TTL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300) // 5 minutes default
            ),
            default_regions: vec!["default".to_string()],
            metadata: HashMap::with_capacity(16),
        }
    }
impl PhonebookConfig {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()

/// With Enabled operation.
    /// Creates instance with enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self

/// With Cache Size operation.
    /// Creates instance with cache size
    pub fn with_cache_size(mut self, size: usize) -> Self {
        self.cache_size = size;

/// With Cache Ttl operation.
    /// Creates instance with cache ttl
    pub fn with_cache_ttl(mut self, ttl_seconds: u64) -> Self {
        self.cache_ttl_seconds = ttl_seconds;

/// With Refresh Interval operation.
    /// Creates instance with refresh interval
    pub fn with_refresh_interval(mut self, interval_seconds: u64) -> Self {
        self.refresh_interval_seconds = interval_seconds;

/// With Bind Address operation.
    /// Creates instance with bind address
    pub fn with_bind_address(mut self, bind_address: &str) -> Self {
        self.bind_address = bind_address;

/// With Port operation.
    /// Creates instance with port
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;

/// With Max Tracked Nodes operation.
    /// Creates instance with max tracked nodes
    pub fn with_max_tracked_nodes(mut self, max_tracked_nodes: usize) -> Self {
        self.max_tracked_nodes = max_tracked_nodes;

/// With Node Entry Ttl operation.
    /// Creates instance with node entry ttl
    pub fn with_node_entry_ttl(mut self, ttl_seconds: u64) -> Self {
        self.node_entry_ttl = Duration::from_secs(ttl_seconds);

/// With Default Region operation.
    /// Creates instance with default region
    pub fn with_default_region(mut self, region: &str) -> Self {
        self.default_regions.push(&str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.into());

/// Cache Ttl operation.
    pub fn cache_ttl(&self) -> Duration {
        Duration::from_secs(self.cache_ttl_seconds)

/// Refresh Interval operation.
    pub fn refresh_interval(&self) -> Duration {
        Duration::from_secs(self.refresh_interval_seconds)

/// Full Address operation.
    pub fn full_address(&self) -> String {
        format!("{}:{}", self.bind_address, self.port)

/// Validate operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), String> {
        if self.cache_size == 0 {
            return Err("Cache size cannot be zero".to_string());
        if self.cache_ttl_seconds == 0 {
            return Err("Cache TTL cannot be zero".to_string());
        if self.refresh_interval_seconds == 0 {
            return Err("Refresh interval cannot be zero".to_string());
        if self.default_regions.is_empty() {
            return Err("Default regions cannot be empty".to_string());
        if self.max_tracked_nodes == 0 {
            return Err("Max tracked nodes cannot be zero".to_string());
        if self.port == 0 {
            return Err("Port cannot be zero".to_string());
        if self.bind_address.is_empty() {
            return Err("Bind address cannot be empty".to_string());
        Ok(())
#[cfg(test)]
mod tests {
    use super::*;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]}


    fn test_phonebook_config_default() {
        let config = PhonebookConfig::default();
        assert!(config.enabled);
        assert_eq!(config.bind_address, "0.0.0.0");
        assert_eq!(config.port, 8844);
        assert_eq!(config.cache_size, 10000);
        assert_eq!(config.cache_ttl_seconds, 300);
        assert_eq!(config.refresh_interval_seconds, 60);
        assert_eq!(config.max_tracked_nodes, 10000);
        assert!(!config.default_regions.is_empty());
        assert!(config.validate().is_ok());
    fn test_phonebook_config_builder() {
        let config = PhonebookConfig::new()
            .with_cache_size(5000)
            .with_cache_ttl(600)
            .with_default_region("us-east".to_string());
        assert_eq!(config.cache_size, 5000);
        assert_eq!(config.cache_ttl_seconds, 600);
        assert!(config.default_regions.contains(&"us-east".to_string()));}


    fn test_phonebook_config_durations() {
        let config = PhonebookConfig::default()
            .with_cache_ttl(900)
            .with_refresh_interval(120)
            .with_node_entry_ttl(600);
        assert_eq!(config.cache_ttl(), Duration::from_secs(900));
        assert_eq!(config.refresh_interval(), Duration::from_secs(120));
        assert_eq!(config.node_entry_ttl, Duration::from_secs(600));
    fn test_phonebook_config_validation() {
        let mut config = PhonebookConfig::default();
        
        config.cache_size = 0;
        assert!(config.validate().is_err());
        config.cache_size = 1000;
        config.cache_ttl_seconds = 0;
        config.cache_ttl_seconds = 300;
        config.refresh_interval_seconds = 0;
        config.refresh_interval_seconds = 60;
        config.default_regions.clear();
        config.default_regions.push("default".to_string());
        config.max_tracked_nodes = 0;
        config.max_tracked_nodes = 1000;
        config.port = 0;
        config.port = 8844;
        config.bind_address = "".to_string();}


    fn test_phonebook_config_network() {
            .with_bind_address("127.0.0.1".to_string())
            .with_port(9999);
        assert_eq!(config.bind_address, "127.0.0.1");
        assert_eq!(config.port, 9999);
        assert_eq!(config.full_address(), "127.0.0.1:9999");
    fn test_phonebook_config_metadata() {
            .with_metadata("region".to_string(), "us-west".to_string())
            .with_metadata("tier".to_string(), "production".to_string());
        assert_eq!(config.metadata.get("region"), Some("us-west".to_string()));
        assert_eq!(config.metadata.get("tier"), Some("production".to_string()));}


    fn test_phonebook_config_regions() {
            .with_default_region("us-east".to_string())
            .with_default_region("us-west".to_string())
            .with_default_region("eu-central".to_string());
        assert!(config.default_regions.contains(&"us-west".to_string()));
        assert!(config.default_regions.contains(&"eu-central".to_string()));
        assert!(config.default_regions.len() >= 4); // Default + 3 added
} 
