

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use nestgate_core::{Result, NestGateError, StorageTier};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsConfig {

    pub api_endpoint: String,

    pub default_pool: String,

    pub use_real_zfs: bool,

    pub tiers: TierConfigurations,

    pub pool_discovery: PoolDiscoveryConfig,

    pub health_monitoring: HealthMonitoringConfig,

    pub metrics: MetricsConfig,

    pub migration: MigrationConfig,

    pub security: SecurityConfig,

    pub enable_ai_integration: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfigurations {
    pub hot: TierConfig,
    pub warm: TierConfig,
    pub cold: TierConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfig {

    pub name: String,

    pub pool_name: String,

    pub dataset_prefix: String,

    pub properties: HashMap<String, String>,

    pub performance_profile: PerformanceProfile,

    pub migration_rules: MigrationRules,

    pub capacity_limits: CapacityLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceProfile {
    HighPerformance,    // Hot tier - optimized for speed
    Balanced,           // Warm tier - balance of speed and compression
    HighCompression,    // Cold tier - optimized for space efficiency
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRules {

    pub age_threshold_days: u32,

    pub access_frequency_threshold: f64,

    pub size_threshold_bytes: u64,

    pub auto_migration_enabled: bool,

    pub migration_schedule: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityLimits {

    pub max_utilization: f64,

    pub warning_threshold: f64,

    pub reserved_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolDiscoveryConfig {

    pub auto_discovery: bool,

    pub include_pools: Vec<String>,

    pub exclude_pools: Vec<String>,

    pub discovery_interval_seconds: u64,

    pub validate_health: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringConfig {

    pub enabled: bool,

    pub check_interval_seconds: u64,

    pub failure_threshold: u32,

    pub recovery_threshold: u32,

    pub alerting_enabled: bool,

    pub alert_endpoints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {

    pub enabled: bool,

    pub collection_interval_seconds: u64,

    pub retention_days: u32,

    pub storage_path: Option<PathBuf>,

    pub export_format: MetricsFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {

    pub background_migration: bool,

    pub max_concurrent_migrations: u32,

    pub bandwidth_limit_bps: Option<u64>,

    pub queue_size: u32,

    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {

    pub enable_encryption: bool,

    pub encryption_algorithm: String,

    pub key_management: UnifiedProcessorConfig,

    pub access_control: AccessControlConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
pub struct KeyManagementConfig {

    pub key_storage_path: PathBuf,

    pub rotation_interval_days: u32,

    pub backup_locations: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {

    pub default_permissions: String,

    pub user_rules: HashMap<String, Vec<String>>,

    pub group_rules: HashMap<String, Vec<String>>,
}

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

    pub fn hot_tier_default() -> Self {
        let mut properties = HashMap::with_capacity(16);
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

    pub fn warm_tier_default() -> Self {
        let mut properties = HashMap::with_capacity(16);
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

    pub fn cold_tier_default() -> Self {
        let mut properties = HashMap::with_capacity(16);
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

    pub fn hot_tier_production() -> Self {
        let mut config = Self::hot_tier_default();
        config.pool_name = "nestpool-prod".to_string();

        config.properties.insert("recordsize".to_string(), "128K".to_string());
        config.properties.insert("compression".to_string(), "lz4".to_string());
        config.properties.insert("primarycache".to_string(), "all".to_string());
        config.properties.insert("secondarycache".to_string(), "all".to_string());
        config.properties.insert("logbias".to_string(), "throughput".to_string());
        config.properties.insert("sync".to_string(), "standard".to_string());

        config.migration_rules.age_threshold_days = 7;
        config.migration_rules.access_frequency_threshold = 10.0;
        config.migration_rules.auto_migration_enabled = true;
        
        config
    }

    pub fn warm_tier_production() -> Self {
        let mut config = Self::warm_tier_default();
        config.pool_name = "nestpool-prod".to_string();

        config.properties.insert("recordsize".to_string(), "1M".to_string());
        config.properties.insert("compression".to_string(), "gzip-6".to_string());
        config.properties.insert("primarycache".to_string(), "all".to_string());
        config.properties.insert("secondarycache".to_string(), "metadata".to_string());
        config.properties.insert("logbias".to_string(), "latency".to_string());

        config.migration_rules.age_threshold_days = 30;
        config.migration_rules.access_frequency_threshold = 1.0;
        config.migration_rules.auto_migration_enabled = true;
        
        config
    }

    pub fn cold_tier_production() -> Self {
        let mut config = Self::cold_tier_default();
        config.pool_name = "nestpool-prod".to_string();

        config.properties.insert("recordsize".to_string(), "1M".to_string());
        config.properties.insert("compression".to_string(), "gzip-9".to_string());
        config.properties.insert("primarycache".to_string(), "metadata".to_string());
        config.properties.insert("secondarycache".to_string(), "none".to_string());
        config.properties.insert("logbias".to_string(), "throughput".to_string());
        config.properties.insert("dedup".to_string(), "on".to_string());

        config.migration_rules.age_threshold_days = 90;
        config.migration_rules.access_frequency_threshold = 0.1;
        config.migration_rules.auto_migration_enabled = true;
        
        config
    }

    pub fn auto_detect_hot(pool_name: &str) -> Self {
        let mut config = Self::hot_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = format_args!("{}/hot", pool_name).to_string();
        config
    }

    pub fn auto_detect_warm(pool_name: &str) -> Self {
        let mut config = Self::warm_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = format_args!("{}/warm", pool_name).to_string();
        config
    }

    pub fn auto_detect_cold(pool_name: &str) -> Self {
        let mut config = Self::cold_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = format_args!("{}/cold", pool_name).to_string();
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
            key_management: UnifiedProcessorConfig::default(),
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
            user_rules: HashMap::with_capacity(16),
            group_rules: HashMap::with_capacity(16),
        }
    }
}

impl ZfsConfig {

    pub async fn load_from_file(path: &std::path::Path) -> Result<Self> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| NestGateError::Configuration(format_args!("Failed to read config file: {}", e).to_string()))?;

        if path.extension().and_then(|s| s.to_str()) == Some("yaml") || 
           path.extension().and_then(|s| s.to_str()) == Some("yml") {
            serde_yaml::from_str(&content)
                .map_err(|e| NestGateError::Configuration(format_args!("YAML parsing error: {}", e).to_string()))
        } else {
            serde_json::from_str(&content)
                .map_err(|e| NestGateError::Configuration(format_args!("JSON parsing error: {}", e).to_string()))
        }
    }

    pub async fn save_to_file(&self, path: &std::path::Path) -> Result<()> {
        let content = if path.extension().and_then(|s| s.to_str()) == Some("yaml") || 
                         path.extension().and_then(|s| s.to_str()) == Some("yml") {
            serde_yaml::to_string(self)
                .map_err(|e| NestGateError::Configuration(format_args!("YAML serialization error: {}", e).to_string()))?
        } else {
            serde_json::to_string_pretty(self)
                .map_err(|e| NestGateError::Configuration(format_args!("JSON serialization error: {}", e).to_string()))?
        };
        
        tokio::fs::write(path, content)
            .await
            .map_err(|e| NestGateError::Configuration(format_args!("Failed to write config file: {}", e).to_string()))?;
        
        Ok(())
    }

    pub fn get_tier_config(&self, tier: &StorageTier) -> &TierConfig {
        match tier {
            StorageTier::Hot => &self.tiers.hot,
            StorageTier::Warm => &self.tiers.warm,
            StorageTier::Cold => &self.tiers.cold,
            StorageTier::Cache => &self.tiers.hot, // Cache uses hot tier config
        }
    }

    pub fn validate(&self) -> Result<()> {

        url::Url::parse(&self.api_endpoint)
            .map_err(|e| NestGateError::Configuration(format_args!("Invalid API endpoint: {}", e).to_string()))?;

        if self.default_pool.is_empty() {
            return Err(NestGateError::Configuration("Default pool name cannot be empty".to_string()));
        }

        self.tiers.hot.validate()?;
        self.tiers.warm.validate()?;
        self.tiers.cold.validate()?;
        
        Ok(())
    }

    pub async fn production_config() -> Result<Self> {
        let mut config = Self::default();

        let available_pools = Self::detect_available_pools().await?;

        if available_pools.contains(&"nestpool-prod".to_string()) {
            config.default_pool = "nestpool-prod".to_string();
            config.tiers = TierConfigurations::production_tiers();
        } else if available_pools.contains(&"nestpool".to_string()) {
            config.default_pool = "nestpool".to_string();
            config.tiers = TierConfigurations::default();
        } else if !available_pools.is_empty() {

            config.default_pool = available_pools[0].clone();
            config.tiers = TierConfigurations::auto_detect_tiers(&config.default_pool);
        }

        config.use_real_zfs = true;
        config.health_monitoring.enabled = true;
        config.metrics.enabled = true;
        config.migration.background_migration = true;
        
        Ok(config)
    }

    async fn detect_available_pools() -> Result<Vec<String>> {
        let output = tokio::process::Command::new("zpool")
            .args(&["list", "-H", "-o", "name"])
            .output()
            .await
            .map_err(|e| NestGateError::Internal(format_args!("Failed to list ZFS pools: {}", e).to_string()))?;
        
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

    pub async fn validate_pool_structure(&self, pool_name: &str) -> Result<bool> {
        let output = tokio::process::Command::new("zfs")
            .args(&["list", "-H", "-o", "name", "-r", pool_name])
            .output()
            .await
            .map_err(|e| NestGateError::Internal(format_args!("Failed to list pool datasets: {}", e).to_string()))?;
        
        if !output.status.success() {
            return Ok(false);
        }
        
        let datasets: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        let expected_tiers = vec![
            format_args!("{}/hot", pool_name).to_string(),
            format_args!("{}/warm", pool_name).to_string(),
            format_args!("{}/cold", pool_name).to_string(),
        ];
        
        for tier in expected_tiers {
            if !datasets.contains(&tier) {
                return Ok(false);
            }
        }
        
        Ok(true)
    }

    pub async fn get_best_pool() -> Result<String> {
        let available_pools = Self::detect_available_pools().await?;

        let preferred_pools = vec!["nestpool-prod", "nestpool"];
        
        for preferred in preferred_pools {
            if available_pools.contains(&preferred.to_string()) {
                return Ok(preferred.to_string());
            }
        }

        available_pools.first()
            .cloned()
            .ok_or_else(|| NestGateError::Internal("No ZFS pools found".to_string()))
    }
}

impl TierConfig {

    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(NestGateError::Configuration("Tier name cannot be empty".to_string()));
        }
        
        if self.pool_name.is_empty() {
            return Err(NestGateError::Configuration("Pool name cannot be empty".to_string()));
        }

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

    pub fn production_tiers() -> Self {
        Self {
            hot: TierConfig::hot_tier_production(),
            warm: TierConfig::warm_tier_production(),
            cold: TierConfig::cold_tier_production(),
        }
    }

    pub fn auto_detect_tiers(pool_name: &str) -> Self {
        Self {
            hot: TierConfig::auto_detect_hot(pool_name),
            warm: TierConfig::auto_detect_warm(pool_name),
            cold: TierConfig::auto_detect_cold(pool_name),
        }
    }
} 