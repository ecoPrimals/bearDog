//! ZFS Configuration Management
//! 
//! Advanced configuration patterns with ZFS-specific settings

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use nestgate_core::{Result, NestGateError, StorageTier};

/// Main ZFS configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsConfig {
    /// API endpoint for ZFS service
    pub api_endpoint: String,
    /// Default pool name for operations
    pub default_pool: String,
    /// Use real ZFS commands (false = mock mode for testing)
    pub use_real_zfs: bool,
    /// Tier configurations
    pub tiers: TierConfigurations,
    /// Pool discovery settings
    pub pool_discovery: PoolDiscoveryConfig,
    /// Health monitoring configuration
    pub health_monitoring: HealthMonitoringConfig,
    /// Metrics collection settings
    pub metrics: MetricsConfig,
    /// Migration settings
    pub migration: MigrationConfig,
    /// Security settings
    pub security: SecurityConfig,
    /// Enable AI integration features
    pub enable_ai_integration: Option<bool>,
}

/// Tier-specific configurations for hot/warm/cold storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfigurations {
    pub hot: TierConfig,
    pub warm: TierConfig,
    pub cold: TierConfig,
}

/// Individual tier configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfig {
    /// Tier name
    pub name: String,
    /// Pool name for this tier
    pub pool_name: String,
    /// Dataset prefix for this tier
    pub dataset_prefix: String,
    /// ZFS properties for this tier
    pub properties: HashMap<String, String>,
    /// Performance profile
    pub performance_profile: PerformanceProfile,
    /// Migration rules
    pub migration_rules: MigrationRules,
    /// Capacity limits
    pub capacity_limits: CapacityLimits,
}

/// Performance profile for tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceProfile {
    HighPerformance,    // Hot tier - optimized for speed
    Balanced,           // Warm tier - balance of speed and compression
    HighCompression,    // Cold tier - optimized for space efficiency
}

/// Migration rules between tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRules {
    /// Age threshold for migration (in days)
    pub age_threshold_days: u32,
    /// Access frequency threshold
    pub access_frequency_threshold: f64,
    /// Size threshold for migration
    pub size_threshold_bytes: u64,
    /// Enable automatic migration
    pub auto_migration_enabled: bool,
    /// Migration schedule (cron-like expression)
    pub migration_schedule: Option<String>,
}

/// Capacity limits for tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityLimits {
    /// Maximum capacity percentage (0.0 - 1.0)
    pub max_utilization: f64,
    /// Warning threshold percentage
    pub warning_threshold: f64,
    /// Reserved space in bytes
    pub reserved_bytes: u64,
}

/// Pool discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolDiscoveryConfig {
    /// Enable automatic pool discovery
    pub auto_discovery: bool,
    /// Pools to explicitly include
    pub include_pools: Vec<String>,
    /// Pools to explicitly exclude
    pub exclude_pools: Vec<String>,
    /// Discovery interval in seconds
    pub discovery_interval_seconds: u64,
    /// Validate pool health on discovery
    pub validate_health: bool,
}

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringConfig {
    /// Enable health monitoring
    pub enabled: bool,
    /// Check interval in seconds
    pub check_interval_seconds: u64,
    /// Failure threshold before marking unhealthy
    pub failure_threshold: u32,
    /// Recovery threshold before marking healthy
    pub recovery_threshold: u32,
    /// Enable alerting
    pub alerting_enabled: bool,
    /// Alert endpoints
    pub alert_endpoints: Vec<String>,
}

/// Metrics collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Collection interval in seconds
    pub collection_interval_seconds: u64,
    /// Retention period in days
    pub retention_days: u32,
    /// Metrics storage path
    pub storage_path: Option<PathBuf>,
    /// Export format (prometheus, json, etc.)
    pub export_format: MetricsFormat,
}

/// Migration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    /// Enable background migration
    pub background_migration: bool,
    /// Maximum concurrent migrations
    pub max_concurrent_migrations: u32,
    /// Migration bandwidth limit (bytes per second)
    pub bandwidth_limit_bps: Option<u64>,
    /// Migration queue size
    pub queue_size: u32,
    /// Retry attempts for failed migrations
    pub retry_attempts: u32,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable encryption for new datasets
    pub enable_encryption: bool,
    /// Default encryption algorithm
    pub encryption_algorithm: String,
    /// Key management settings
    pub key_management: KeyManagementConfig,
    /// Access control settings
    pub access_control: AccessControlConfig,
}

/// Key management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementConfig {
    /// Key storage location
    pub key_storage_path: PathBuf,
    /// Key rotation interval in days
    pub rotation_interval_days: u32,
    /// Backup key locations
    pub backup_locations: Vec<PathBuf>,
}

/// Access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    /// Default permissions for new datasets
    pub default_permissions: String,
    /// User access rules
    pub user_rules: HashMap<String, Vec<String>>,
    /// Group access rules
    pub group_rules: HashMap<String, Vec<String>>,
}

/// Metrics export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricsFormat {
    Prometheus,
    Json,
    InfluxDb,
}

impl Default for ZfsConfig {
    fn default() -> Self {
        Self {
            api_endpoint: "http://localhost:8080".to_string(),
            default_pool: "nestpool".to_string(),
            use_real_zfs: true,
            tiers: TierConfigurations::default(),
            pool_discovery: PoolDiscoveryConfig::default(),
            health_monitoring: HealthMonitoringConfig::default(),
            metrics: MetricsConfig::default(),
            migration: MigrationConfig::default(),
            security: SecurityConfig::default(),
            enable_ai_integration: Some(true),
        }
    }
}

impl Default for TierConfigurations {
    fn default() -> Self {
        Self {
            hot: TierConfig::hot_tier_default(),
            warm: TierConfig::warm_tier_default(),
            cold: TierConfig::cold_tier_default(),
        }
    }
}

impl TierConfig {
    /// Default configuration for hot tier (high performance)
    pub fn hot_tier_default() -> Self {
        let mut properties = HashMap::new();
        properties.insert("compression".to_string(), "lz4".to_string());
        properties.insert("recordsize".to_string(), "128K".to_string());
        properties.insert("atime".to_string(), "off".to_string());
        properties.insert("primarycache".to_string(), "all".to_string());
        properties.insert("secondarycache".to_string(), "all".to_string());
        
        Self {
            name: "hot".to_string(),
            pool_name: "nestpool".to_string(),
            dataset_prefix: "hot".to_string(),
            properties,
            performance_profile: PerformanceProfile::HighPerformance,
            migration_rules: MigrationRules::hot_tier_defaults(),
            capacity_limits: CapacityLimits::hot_tier_defaults(),
        }
    }
    
    /// Default configuration for warm tier (balanced)
    pub fn warm_tier_default() -> Self {
        let mut properties = HashMap::new();
        properties.insert("compression".to_string(), "zstd".to_string());
        properties.insert("recordsize".to_string(), "1M".to_string());
        properties.insert("atime".to_string(), "on".to_string());
        properties.insert("primarycache".to_string(), "metadata".to_string());
        properties.insert("secondarycache".to_string(), "metadata".to_string());
        
        Self {
            name: "warm".to_string(),
            pool_name: "nestpool".to_string(),
            dataset_prefix: "warm".to_string(),
            properties,
            performance_profile: PerformanceProfile::Balanced,
            migration_rules: MigrationRules::warm_tier_defaults(),
            capacity_limits: CapacityLimits::warm_tier_defaults(),
        }
    }
    
    /// Default configuration for cold tier (high compression)
    pub fn cold_tier_default() -> Self {
        let mut properties = HashMap::new();
        properties.insert("compression".to_string(), "gzip-9".to_string());
        properties.insert("recordsize".to_string(), "1M".to_string());
        properties.insert("atime".to_string(), "off".to_string());
        properties.insert("primarycache".to_string(), "metadata".to_string());
        properties.insert("secondarycache".to_string(), "none".to_string());
        
        Self {
            name: "cold".to_string(),
            pool_name: "nestpool".to_string(),
            dataset_prefix: "cold".to_string(),
            properties,
            performance_profile: PerformanceProfile::HighCompression,
            migration_rules: MigrationRules::cold_tier_defaults(),
            capacity_limits: CapacityLimits::cold_tier_defaults(),
        }
    }

    /// Create production-optimized hot tier configuration
    pub fn hot_tier_production() -> Self {
        let mut config = Self::hot_tier_default();
        config.pool_name = "nestpool-prod".to_string();
        
        // Production-optimized properties for NVMe
        config.properties.insert("recordsize".to_string(), "128K".to_string());
        config.properties.insert("compression".to_string(), "lz4".to_string());
        config.properties.insert("primarycache".to_string(), "all".to_string());
        config.properties.insert("secondarycache".to_string(), "all".to_string());
        config.properties.insert("logbias".to_string(), "throughput".to_string());
        config.properties.insert("sync".to_string(), "standard".to_string());
        
        // Aggressive migration rules for hot tier
        config.migration_rules.age_threshold_days = 7;
        config.migration_rules.access_frequency_threshold = 10.0;
        config.migration_rules.auto_migration_enabled = true;
        
        config
    }
    
    /// Create production-optimized warm tier configuration
    pub fn warm_tier_production() -> Self {
        let mut config = Self::warm_tier_default();
        config.pool_name = "nestpool-prod".to_string();
        
        // Balanced properties for warm tier
        config.properties.insert("recordsize".to_string(), "1M".to_string());
        config.properties.insert("compression".to_string(), "gzip-6".to_string());
        config.properties.insert("primarycache".to_string(), "all".to_string());
        config.properties.insert("secondarycache".to_string(), "metadata".to_string());
        config.properties.insert("logbias".to_string(), "latency".to_string());
        
        // Moderate migration rules
        config.migration_rules.age_threshold_days = 30;
        config.migration_rules.access_frequency_threshold = 1.0;
        config.migration_rules.auto_migration_enabled = true;
        
        config
    }
    
    /// Create production-optimized cold tier configuration
    pub fn cold_tier_production() -> Self {
        let mut config = Self::cold_tier_default();
        config.pool_name = "nestpool-prod".to_string();
        
        // Space-optimized properties for cold tier
        config.properties.insert("recordsize".to_string(), "1M".to_string());
        config.properties.insert("compression".to_string(), "gzip-9".to_string());
        config.properties.insert("primarycache".to_string(), "metadata".to_string());
        config.properties.insert("secondarycache".to_string(), "none".to_string());
        config.properties.insert("logbias".to_string(), "throughput".to_string());
        config.properties.insert("dedup".to_string(), "on".to_string());
        
        // Conservative migration rules
        config.migration_rules.age_threshold_days = 90;
        config.migration_rules.access_frequency_threshold = 0.1;
        config.migration_rules.auto_migration_enabled = true;
        
        config
    }
    
    /// Auto-detect hot tier configuration for any pool
    pub fn auto_detect_hot(pool_name: &str) -> Self {
        let mut config = Self::hot_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = format!("{}/hot", pool_name);
        config
    }
    
    /// Auto-detect warm tier configuration for any pool
    pub fn auto_detect_warm(pool_name: &str) -> Self {
        let mut config = Self::warm_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = format!("{}/warm", pool_name);
        config
    }
    
    /// Auto-detect cold tier configuration for any pool
    pub fn auto_detect_cold(pool_name: &str) -> Self {
        let mut config = Self::cold_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = format!("{}/cold", pool_name);
        config
    }
}

impl MigrationRules {
    pub fn hot_tier_defaults() -> Self {
        Self {
            age_threshold_days: 7,      // Move to warm after 7 days
            access_frequency_threshold: 10.0,  // High access frequency
            size_threshold_bytes: 1024 * 1024 * 100, // 100MB
            auto_migration_enabled: true,
            migration_schedule: Some("0 2 * * *".to_string()), // Daily at 2 AM
        }
    }
    
    pub fn warm_tier_defaults() -> Self {
        Self {
            age_threshold_days: 30,     // Move to cold after 30 days
            access_frequency_threshold: 1.0,   // Low access frequency
            size_threshold_bytes: 1024 * 1024 * 1024, // 1GB
            auto_migration_enabled: true,
            migration_schedule: Some("0 3 * * 0".to_string()), // Weekly on Sunday at 3 AM
        }
    }
    
    pub fn cold_tier_defaults() -> Self {
        Self {
            age_threshold_days: 365,    // Archive after 1 year
            access_frequency_threshold: 0.1,   // Very low access frequency
            size_threshold_bytes: u64::MAX,    // No size limit
            auto_migration_enabled: false,     // Manual archival only
            migration_schedule: None,
        }
    }
}

impl CapacityLimits {
    pub fn hot_tier_defaults() -> Self {
        Self {
            max_utilization: 0.8,      // 80% max utilization
            warning_threshold: 0.7,    // Warning at 70%
            reserved_bytes: 1024 * 1024 * 1024 * 10, // 10GB reserved
        }
    }
    
    pub fn warm_tier_defaults() -> Self {
        Self {
            max_utilization: 0.9,      // 90% max utilization
            warning_threshold: 0.8,    // Warning at 80%
            reserved_bytes: 1024 * 1024 * 1024 * 5,  // 5GB reserved
        }
    }
    
    pub fn cold_tier_defaults() -> Self {
        Self {
            max_utilization: 0.95,     // 95% max utilization
            warning_threshold: 0.9,    // Warning at 90%
            reserved_bytes: 1024 * 1024 * 1024 * 2,  // 2GB reserved
        }
    }
}

impl Default for PoolDiscoveryConfig {
    fn default() -> Self {
        Self {
            auto_discovery: true,
            include_pools: vec![],
            exclude_pools: vec!["rpool".to_string()], // Exclude system pools
            discovery_interval_seconds: 300, // 5 minutes
            validate_health: true,
        }
    }
}

impl Default for HealthMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval_seconds: 30,
            failure_threshold: 3,
            recovery_threshold: 2,
            alerting_enabled: false,
            alert_endpoints: vec![],
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval_seconds: 60,
            retention_days: 30,
            storage_path: None,
            export_format: MetricsFormat::Prometheus,
        }
    }
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            background_migration: true,
            max_concurrent_migrations: 2,
            bandwidth_limit_bps: None,
            queue_size: 100,
            retry_attempts: 3,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_encryption: false,
            encryption_algorithm: "aes-256-gcm".to_string(),
            key_management: KeyManagementConfig::default(),
            access_control: AccessControlConfig::default(),
        }
    }
}

impl Default for KeyManagementConfig {
    fn default() -> Self {
        Self {
            key_storage_path: PathBuf::from("/etc/nestgate/zfs/keys"),
            rotation_interval_days: 90,
            backup_locations: vec![],
        }
    }
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            default_permissions: "755".to_string(),
            user_rules: HashMap::new(),
            group_rules: HashMap::new(),
        }
    }
}

impl ZfsConfig {
    /// Load configuration from file with advanced integration patterns
    pub async fn load_from_file(path: &std::path::Path) -> Result<Self> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| NestGateError::Configuration(format!("Failed to read config file: {}", e)))?;
        
        // Support multiple formats (YAML, JSON)
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") || 
           path.extension().and_then(|s| s.to_str()) == Some("yml") {
            serde_yaml::from_str(&content)
                .map_err(|e| NestGateError::Configuration(format!("YAML parsing error: {}", e)))
        } else {
            serde_json::from_str(&content)
                .map_err(|e| NestGateError::Configuration(format!("JSON parsing error: {}", e)))
        }
    }
    
    /// Save configuration to file
    pub async fn save_to_file(&self, path: &std::path::Path) -> Result<()> {
        let content = if path.extension().and_then(|s| s.to_str()) == Some("yaml") || 
                         path.extension().and_then(|s| s.to_str()) == Some("yml") {
            serde_yaml::to_string(self)
                .map_err(|e| NestGateError::Configuration(format!("YAML serialization error: {}", e)))?
        } else {
            serde_json::to_string_pretty(self)
                .map_err(|e| NestGateError::Configuration(format!("JSON serialization error: {}", e)))?
        };
        
        tokio::fs::write(path, content)
            .await
            .map_err(|e| NestGateError::Configuration(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }
    
    /// Get tier configuration by storage tier
    pub fn get_tier_config(&self, tier: &StorageTier) -> &TierConfig {
        match tier {
            StorageTier::Hot => &self.tiers.hot,
            StorageTier::Warm => &self.tiers.warm,
            StorageTier::Cold => &self.tiers.cold,
            StorageTier::Cache => &self.tiers.hot, // Cache uses hot tier config
        }
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate API endpoint
        url::Url::parse(&self.api_endpoint)
            .map_err(|e| NestGateError::Configuration(format!("Invalid API endpoint: {}", e)))?;
        
        // Validate pool name
        if self.default_pool.is_empty() {
            return Err(NestGateError::Configuration("Default pool name cannot be empty".to_string()));
        }
        
        // Validate tier configurations
        self.tiers.hot.validate()?;
        self.tiers.warm.validate()?;
        self.tiers.cold.validate()?;
        
        Ok(())
    }

    /// Create a production configuration with auto-detected pools
    pub async fn production_config() -> Result<Self> {
        let mut config = Self::default();
        
        // Auto-detect available ZFS pools
        let available_pools = Self::detect_available_pools().await?;
        
        // Prefer production pool if available
        if available_pools.contains(&"nestpool-prod".to_string()) {
            config.default_pool = "nestpool-prod".to_string();
            config.tiers = TierConfigurations::production_tiers();
        } else if available_pools.contains(&"nestpool".to_string()) {
            config.default_pool = "nestpool".to_string();
            config.tiers = TierConfigurations::default();
        } else if !available_pools.is_empty() {
            // Use first available pool
            config.default_pool = available_pools[0].clone();
            config.tiers = TierConfigurations::auto_detect_tiers(&config.default_pool);
        }
        
        // Enable production-optimized settings
        config.use_real_zfs = true;
        config.health_monitoring.enabled = true;
        config.metrics.enabled = true;
        config.migration.background_migration = true;
        
        Ok(config)
    }
    
    /// Detect available ZFS pools on the system
    async fn detect_available_pools() -> Result<Vec<String>> {
        let output = tokio::process::Command::new("zpool")
            .args(&["list", "-H", "-o", "name"])
            .output()
            .await
            .map_err(|e| NestGateError::Internal(format!("Failed to list ZFS pools: {}", e)))?;
        
        if !output.status.success() {
            return Ok(Vec::new());
        }
        
        let pools: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        
        Ok(pools)
    }
    
    /// Check if a pool has the expected tier structure
    pub async fn validate_pool_structure(&self, pool_name: &str) -> Result<bool> {
        let output = tokio::process::Command::new("zfs")
            .args(&["list", "-H", "-o", "name", "-r", pool_name])
            .output()
            .await
            .map_err(|e| NestGateError::Internal(format!("Failed to list pool datasets: {}", e)))?;
        
        if !output.status.success() {
            return Ok(false);
        }
        
        let datasets: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        
        // Check for expected tier datasets
        let expected_tiers = vec![
            format!("{}/hot", pool_name),
            format!("{}/warm", pool_name),
            format!("{}/cold", pool_name),
        ];
        
        for tier in expected_tiers {
            if !datasets.contains(&tier) {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Get the best available pool for NestGate operations
    pub async fn get_best_pool() -> Result<String> {
        let available_pools = Self::detect_available_pools().await?;
        
        // Priority order: production pool > test pool > any other pool
        let preferred_pools = vec!["nestpool-prod", "nestpool"];
        
        for preferred in preferred_pools {
            if available_pools.contains(&preferred.to_string()) {
                return Ok(preferred.to_string());
            }
        }
        
        // Return first available pool if no preferred pools found
        available_pools.first()
            .cloned()
            .ok_or_else(|| NestGateError::Internal("No ZFS pools found".to_string()))
    }
}

impl TierConfig {
    /// Validate tier configuration
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(NestGateError::Configuration("Tier name cannot be empty".to_string()));
        }
        
        if self.pool_name.is_empty() {
            return Err(NestGateError::Configuration("Pool name cannot be empty".to_string()));
        }
        
        // Validate capacity limits
        if self.capacity_limits.max_utilization > 1.0 || self.capacity_limits.max_utilization <= 0.0 {
            return Err(NestGateError::Configuration("Max utilization must be between 0.0 and 1.0".to_string()));
        }
        
        if self.capacity_limits.warning_threshold > self.capacity_limits.max_utilization {
            return Err(NestGateError::Configuration("Warning threshold cannot exceed max utilization".to_string()));
        }
        
        Ok(())
    }
}

impl TierConfigurations {
    /// Create production-optimized tier configurations
    pub fn production_tiers() -> Self {
        Self {
            hot: TierConfig::hot_tier_production(),
            warm: TierConfig::warm_tier_production(),
            cold: TierConfig::cold_tier_production(),
        }
    }
    
    /// Auto-detect tier configurations for a given pool
    pub fn auto_detect_tiers(pool_name: &str) -> Self {
        Self {
            hot: TierConfig::auto_detect_hot(pool_name),
            warm: TierConfig::auto_detect_warm(pool_name),
            cold: TierConfig::auto_detect_cold(pool_name),
        }
    }
} 