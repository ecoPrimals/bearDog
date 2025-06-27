# BearDog Configuration Management Specification

**Version:** 1.0  
**Date:** January 2025  
**Status:** SPECIFICATION  
**Priority:** HIGH  

## 🎯 **Overview**

BearDog's configuration management system provides:
- **Secure-by-default** configuration
- **Zero hardcoding** - everything configurable
- **Multi-source configuration** with priority handling
- **Runtime configuration updates**
- **Configuration validation** and schema enforcement
- **Environment-specific configuration**

## ⚙️ **Configuration Architecture**

### **Configuration Sources (Priority Order)**
1. **Command Line Arguments** (highest priority)
2. **Environment Variables**
3. **Configuration Files** (TOML/YAML/JSON)
4. **Consul/etcd** (distributed configuration)
5. **Secure Defaults** (lowest priority)

### **Core Configuration Structure**
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogConfig {
    pub core: CoreConfig,
    pub encryption: EncryptionConfig,
    pub security: SecurityConfig,
    pub hsm: Option<HsmConfig>,
    pub audit: AuditConfig,
    pub threat_detection: ThreatDetectionConfig,
    pub compliance: ComplianceConfig,
    pub multi_party: MultiPartyConfig,
    pub network: NetworkConfig,
    pub logging: LoggingConfig,
    pub performance: PerformanceConfig,
    pub integrations: IntegrationConfig,
}

// Secure defaults implementation
impl Default for BearDogConfig {
    fn default() -> Self {
        Self {
            core: CoreConfig::secure_default(),
            encryption: EncryptionConfig::secure_default(),
            security: SecurityConfig::secure_default(),
            hsm: None, // Disabled by default
            audit: AuditConfig::secure_default(),
            threat_detection: ThreatDetectionConfig::secure_default(),
            compliance: ComplianceConfig::secure_default(),
            multi_party: MultiPartyConfig::secure_default(),
            network: NetworkConfig::secure_default(),
            logging: LoggingConfig::secure_default(),
            performance: PerformanceConfig::secure_default(),
            integrations: IntegrationConfig::secure_default(),
        }
    }
}
```

### **Core Configuration**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    pub service_id: String,
    pub instance_id: String,
    pub bind_address: String,
    pub port: u16,
    pub enable_tls: bool,
    pub tls_config: Option<TlsConfig>,
    pub worker_threads: Option<usize>,
    pub shutdown_timeout_seconds: u64,
    pub health_check_interval_seconds: u64,
}

impl CoreConfig {
    pub fn secure_default() -> Self {
        Self {
            service_id: env::var("BEARDOG_SERVICE_ID")
                .unwrap_or_else(|_| "beardog-security-manager".to_string()),
            instance_id: uuid::Uuid::new_v4().to_string(),
            bind_address: env::var("BEARDOG_BIND_ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1".to_string()), // Localhost only
            port: env::var("BEARDOG_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8443), // HTTPS port
            enable_tls: env::var("BEARDOG_ENABLE_TLS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true), // TLS enabled by default
            tls_config: Some(TlsConfig::secure_default()),
            worker_threads: None, // Auto-detect CPU cores
            shutdown_timeout_seconds: 30,
            health_check_interval_seconds: 60,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
    pub ca_cert_path: Option<PathBuf>,
    pub require_client_cert: bool,
    pub min_tls_version: TlsVersion,
    pub cipher_suites: Vec<String>,
}

impl TlsConfig {
    pub fn secure_default() -> Self {
        Self {
            cert_path: env::var("BEARDOG_TLS_CERT")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("./certs/beardog.crt")),
            key_path: env::var("BEARDOG_TLS_KEY")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("./certs/beardog.key")),
            ca_cert_path: env::var("BEARDOG_TLS_CA")
                .ok()
                .map(PathBuf::from),
            require_client_cert: env::var("BEARDOG_REQUIRE_CLIENT_CERT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true), // mTLS by default
            min_tls_version: TlsVersion::V1_3,
            cipher_suites: vec![
                "TLS_AES_256_GCM_SHA384".to_string(),
                "TLS_CHACHA20_POLY1305_SHA256".to_string(),
                "TLS_AES_128_GCM_SHA256".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TlsVersion {
    V1_2,
    V1_3,
}
```

### **Encryption Configuration**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub default_algorithm: String,
    pub key_derivation: KeyDerivationConfig,
    pub key_rotation: KeyRotationConfig,
    pub key_storage: KeyStorageConfig,
    pub performance: EncryptionPerformanceConfig,
}

impl EncryptionConfig {
    pub fn secure_default() -> Self {
        Self {
            default_algorithm: env::var("BEARDOG_DEFAULT_ALGORITHM")
                .unwrap_or_else(|_| "aes-256-gcm".to_string()),
            key_derivation: KeyDerivationConfig::secure_default(),
            key_rotation: KeyRotationConfig::secure_default(),
            key_storage: KeyStorageConfig::secure_default(),
            performance: EncryptionPerformanceConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationConfig {
    pub function: String, // "argon2id", "pbkdf2", "scrypt"
    pub iterations: u32,
    pub memory_cost: Option<u32>, // For Argon2
    pub parallelism: Option<u32>, // For Argon2
    pub salt_size: usize,
}

impl KeyDerivationConfig {
    pub fn secure_default() -> Self {
        Self {
            function: "argon2id".to_string(),
            iterations: 3,
            memory_cost: Some(65536), // 64MB
            parallelism: Some(4),
            salt_size: 32, // 256 bits
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationConfig {
    pub auto_rotation_enabled: bool,
    pub rotation_interval_days: u32,
    pub rotation_schedule: Option<String>, // Cron expression
    pub grace_period_days: u32,
    pub require_approval: bool,
}

impl KeyRotationConfig {
    pub fn secure_default() -> Self {
        Self {
            auto_rotation_enabled: env::var("BEARDOG_AUTO_ROTATION")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(false), // Manual by default
            rotation_interval_days: 90,
            rotation_schedule: None,
            grace_period_days: 30,
            require_approval: true,
        }
    }
}
```

## 📁 **Configuration Loading**

### **Configuration Manager**
```rust
use tokio::fs;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ConfigurationManager {
    config: Arc<RwLock<BearDogConfig>>,
    sources: Vec<Box<dyn ConfigurationSource>>,
    validators: Vec<Box<dyn ConfigurationValidator>>,
    watchers: Vec<Box<dyn ConfigurationWatcher>>,
    notification_handlers: Vec<Box<dyn ConfigurationChangeHandler>>,
}

impl ConfigurationManager {
    pub async fn new() -> Result<Self> {
        let mut sources: Vec<Box<dyn ConfigurationSource>> = Vec::new();
        
        // Add configuration sources in priority order
        sources.push(Box::new(EnvironmentConfigurationSource::new()));
        sources.push(Box::new(FileConfigurationSource::new()));
        sources.push(Box::new(ConsulConfigurationSource::new().await?));
        sources.push(Box::new(DefaultConfigurationSource::new()));
        
        let validators = vec![
            Box::new(SchemaValidator::new()) as Box<dyn ConfigurationValidator>,
            Box::new(SecurityValidator::new()) as Box<dyn ConfigurationValidator>,
            Box::new(NetworkValidator::new()) as Box<dyn ConfigurationValidator>,
        ];
        
        Ok(Self {
            config: Arc::new(RwLock::new(BearDogConfig::default())),
            sources,
            validators,
            watchers: Vec::new(),
            notification_handlers: Vec::new(),
        })
    }
    
    pub async fn load_configuration(&mut self) -> Result<BearDogConfig> {
        let mut merged_config = BearDogConfig::default();
        
        // Load from all sources in priority order
        for source in &self.sources {
            if let Ok(partial_config) = source.load_configuration().await {
                merged_config = self.merge_configurations(merged_config, partial_config)?;
            }
        }
        
        // Validate the merged configuration
        for validator in &self.validators {
            validator.validate(&merged_config)?;
        }
        
        // Update the cached configuration
        *self.config.write().await = merged_config.clone();
        
        // Start watching for changes
        self.start_configuration_watchers().await?;
        
        Ok(merged_config)
    }
    
    pub async fn get_configuration(&self) -> BearDogConfig {
        self.config.read().await.clone()
    }
    
    pub async fn update_configuration(&self, new_config: BearDogConfig) -> Result<()> {
        // Validate the new configuration
        for validator in &self.validators {
            validator.validate(&new_config)?;
        }
        
        let old_config = self.config.read().await.clone();
        *self.config.write().await = new_config.clone();
        
        // Notify change handlers
        for handler in &self.notification_handlers {
            handler.handle_configuration_change(&old_config, &new_config).await?;
        }
        
        Ok(())
    }
    
    fn merge_configurations(
        &self,
        base: BearDogConfig,
        overlay: BearDogConfig,
    ) -> Result<BearDogConfig> {
        // Implement deep merge logic with overlay taking precedence
        // This is a simplified version - real implementation would be more comprehensive
        
        Ok(BearDogConfig {
            core: if overlay.core != CoreConfig::secure_default() {
                overlay.core
            } else {
                base.core
            },
            encryption: if overlay.encryption != EncryptionConfig::secure_default() {
                overlay.encryption
            } else {
                base.encryption
            },
            // ... merge other sections
            ..overlay
        })
    }
}

#[async_trait]
pub trait ConfigurationSource: Send + Sync {
    async fn load_configuration(&self) -> Result<BearDogConfig>;
    fn source_name(&self) -> &str;
    fn priority(&self) -> u32;
}

#[async_trait]
pub trait ConfigurationValidator: Send + Sync {
    fn validate(&self, config: &BearDogConfig) -> Result<()>;
    fn validator_name(&self) -> &str;
}

#[async_trait]
pub trait ConfigurationWatcher: Send + Sync {
    async fn start_watching(&mut self, callback: Arc<dyn ConfigurationChangeCallback>) -> Result<()>;
    async fn stop_watching(&mut self) -> Result<()>;
}

#[async_trait]
pub trait ConfigurationChangeHandler: Send + Sync {
    async fn handle_configuration_change(
        &self,
        old_config: &BearDogConfig,
        new_config: &BearDogConfig,
    ) -> Result<()>;
}
```

### **Environment Variable Configuration Source**
```rust
pub struct EnvironmentConfigurationSource;

impl EnvironmentConfigurationSource {
    pub fn new() -> Self {
        Self
    }
    
    fn load_core_config_from_env(&self) -> CoreConfig {
        CoreConfig {
            service_id: env::var("BEARDOG_SERVICE_ID")
                .unwrap_or_else(|_| CoreConfig::secure_default().service_id),
            bind_address: env::var("BEARDOG_BIND_ADDRESS")
                .unwrap_or_else(|_| CoreConfig::secure_default().bind_address),
            port: env::var("BEARDOG_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(CoreConfig::secure_default().port),
            enable_tls: env::var("BEARDOG_ENABLE_TLS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(CoreConfig::secure_default().enable_tls),
            // ... load other fields from environment
            ..CoreConfig::secure_default()
        }
    }
}

#[async_trait]
impl ConfigurationSource for EnvironmentConfigurationSource {
    async fn load_configuration(&self) -> Result<BearDogConfig> {
        Ok(BearDogConfig {
            core: self.load_core_config_from_env(),
            // Load other sections from environment variables
            ..BearDogConfig::default()
        })
    }
    
    fn source_name(&self) -> &str {
        "environment"
    }
    
    fn priority(&self) -> u32 {
        1000 // High priority
    }
}
```

### **File Configuration Source**
```rust
pub struct FileConfigurationSource {
    config_paths: Vec<PathBuf>,
}

impl FileConfigurationSource {
    pub fn new() -> Self {
        let mut config_paths = Vec::new();
        
        // Check common configuration file locations
        if let Ok(config_file) = env::var("BEARDOG_CONFIG_FILE") {
            config_paths.push(PathBuf::from(config_file));
        }
        
        config_paths.extend_from_slice(&[
            PathBuf::from("./beardog.toml"),
            PathBuf::from("./config/beardog.toml"),
            PathBuf::from("/etc/beardog/beardog.toml"),
            PathBuf::from("./beardog.yaml"),
            PathBuf::from("./config/beardog.yaml"),
            PathBuf::from("./beardog.json"),
        ]);
        
        Self { config_paths }
    }
    
    async fn load_toml_config(&self, path: &PathBuf) -> Result<BearDogConfig> {
        let content = fs::read_to_string(path).await?;
        let config: BearDogConfig = toml::from_str(&content)
            .map_err(|e| BearDogError::ConfigurationError(format!("TOML parse error: {}", e)))?;
        Ok(config)
    }
    
    async fn load_yaml_config(&self, path: &PathBuf) -> Result<BearDogConfig> {
        let content = fs::read_to_string(path).await?;
        let config: BearDogConfig = serde_yaml::from_str(&content)
            .map_err(|e| BearDogError::ConfigurationError(format!("YAML parse error: {}", e)))?;
        Ok(config)
    }
    
    async fn load_json_config(&self, path: &PathBuf) -> Result<BearDogConfig> {
        let content = fs::read_to_string(path).await?;
        let config: BearDogConfig = serde_json::from_str(&content)
            .map_err(|e| BearDogError::ConfigurationError(format!("JSON parse error: {}", e)))?;
        Ok(config)
    }
}

#[async_trait]
impl ConfigurationSource for FileConfigurationSource {
    async fn load_configuration(&self) -> Result<BearDogConfig> {
        for path in &self.config_paths {
            if path.exists() {
                return match path.extension().and_then(|ext| ext.to_str()) {
                    Some("toml") => self.load_toml_config(path).await,
                    Some("yaml") | Some("yml") => self.load_yaml_config(path).await,
                    Some("json") => self.load_json_config(path).await,
                    _ => {
                        // Try to infer format from content
                        let content = fs::read_to_string(path).await?;
                        if content.trim_start().starts_with('{') {
                            serde_json::from_str(&content)
                                .map_err(|e| BearDogError::ConfigurationError(format!("JSON parse error: {}", e)))
                        } else if content.contains("---") || content.contains(":") {
                            serde_yaml::from_str(&content)
                                .map_err(|e| BearDogError::ConfigurationError(format!("YAML parse error: {}", e)))
                        } else {
                            toml::from_str(&content)
                                .map_err(|e| BearDogError::ConfigurationError(format!("TOML parse error: {}", e)))
                        }
                    }
                };
            }
        }
        
        Err(BearDogError::ConfigurationError("No configuration file found".to_string()))
    }
    
    fn source_name(&self) -> &str {
        "file"
    }
    
    fn priority(&self) -> u32 {
        500 // Medium priority
    }
}
```

## ✅ **Configuration Validation**

### **Schema Validator**
```rust
use jsonschema::{JSONSchema, Draft};

pub struct SchemaValidator {
    schema: JSONSchema,
}

impl SchemaValidator {
    pub fn new() -> Self {
        let schema_json = include_str!("../schemas/beardog-config-schema.json");
        let schema_value: serde_json::Value = serde_json::from_str(schema_json)
            .expect("Invalid JSON schema");
        let schema = JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema_value)
            .expect("Invalid JSON schema");
        
        Self { schema }
    }
}

impl ConfigurationValidator for SchemaValidator {
    fn validate(&self, config: &BearDogConfig) -> Result<()> {
        let config_json = serde_json::to_value(config)
            .map_err(|e| BearDogError::ConfigurationError(format!("Serialization error: {}", e)))?;
        
        if let Err(errors) = self.schema.validate(&config_json) {
            let error_messages: Vec<String> = errors
                .map(|error| format!("{}: {}", error.instance_path, error))
                .collect();
            
            return Err(BearDogError::ConfigurationError(format!(
                "Schema validation failed: {}",
                error_messages.join(", ")
            )));
        }
        
        Ok(())
    }
    
    fn validator_name(&self) -> &str {
        "schema"
    }
}
```

### **Security Validator**
```rust
pub struct SecurityValidator;

impl SecurityValidator {
    pub fn new() -> Self {
        Self
    }
    
    fn validate_tls_config(&self, tls_config: &TlsConfig) -> Result<()> {
        // Check if certificate files exist and are readable
        if !tls_config.cert_path.exists() {
            return Err(BearDogError::ConfigurationError(
                format!("TLS certificate file not found: {:?}", tls_config.cert_path)
            ));
        }
        
        if !tls_config.key_path.exists() {
            return Err(BearDogError::ConfigurationError(
                format!("TLS key file not found: {:?}", tls_config.key_path)
            ));
        }
        
        // Validate TLS version
        match tls_config.min_tls_version {
            TlsVersion::V1_2 => {
                tracing::warn!("TLS 1.2 is deprecated, consider upgrading to TLS 1.3");
            }
            TlsVersion::V1_3 => {} // Good
        }
        
        // Validate cipher suites
        for cipher in &tls_config.cipher_suites {
            if !self.is_secure_cipher_suite(cipher) {
                return Err(BearDogError::ConfigurationError(
                    format!("Insecure cipher suite: {}", cipher)
                ));
            }
        }
        
        Ok(())
    }
    
    fn is_secure_cipher_suite(&self, cipher: &str) -> bool {
        // List of approved secure cipher suites
        const SECURE_CIPHERS: &[&str] = &[
            "TLS_AES_256_GCM_SHA384",
            "TLS_CHACHA20_POLY1305_SHA256",
            "TLS_AES_128_GCM_SHA256",
        ];
        
        SECURE_CIPHERS.contains(&cipher)
    }
    
    fn validate_security_config(&self, security_config: &SecurityConfig) -> Result<()> {
        // Ensure strong defaults
        if security_config.jwt_expiration_minutes > 480 { // 8 hours
            return Err(BearDogError::ConfigurationError(
                "JWT expiration too long (max 8 hours)".to_string()
            ));
        }
        
        if security_config.max_failed_attempts > 10 {
            return Err(BearDogError::ConfigurationError(
                "Max failed attempts too high (max 10)".to_string()
            ));
        }
        
        if security_config.rate_limit_requests_per_minute > 10000 {
            tracing::warn!("High rate limit configured, ensure DDoS protection is in place");
        }
        
        Ok(())
    }
}

impl ConfigurationValidator for SecurityValidator {
    fn validate(&self, config: &BearDogConfig) -> Result<()> {
        // Validate TLS configuration
        if config.core.enable_tls {
            if let Some(ref tls_config) = config.core.tls_config {
                self.validate_tls_config(tls_config)?;
            } else {
                return Err(BearDogError::ConfigurationError(
                    "TLS enabled but no TLS configuration provided".to_string()
                ));
            }
        }
        
        // Validate security configuration
        self.validate_security_config(&config.security)?;
        
        // Validate network binding
        if config.core.bind_address != "127.0.0.1" && config.core.bind_address != "localhost" {
            tracing::warn!(
                "Binding to non-localhost address: {}. Ensure proper firewall rules are in place.",
                config.core.bind_address
            );
        }
        
        Ok(())
    }
    
    fn validator_name(&self) -> &str {
        "security"
    }
}
```

## 📄 **Configuration Templates**

### **Production Configuration Template**
```toml
# beardog.toml - Production Configuration Template

[core]
service_id = "beardog-security-manager"
bind_address = "0.0.0.0"  # Adjust for production
port = 8443
enable_tls = true
worker_threads = 8  # Adjust based on server capacity

[core.tls]
cert_path = "/etc/beardog/certs/beardog.crt"
key_path = "/etc/beardog/certs/beardog.key"
ca_cert_path = "/etc/beardog/certs/ca.crt"
require_client_cert = true
min_tls_version = "v1.3"

[encryption]
default_algorithm = "aes-256-gcm"
key_rotation_days = 90
auto_rotation = false  # Manual rotation for production

[encryption.key_derivation]
function = "argon2id"
iterations = 3
memory_cost = 65536
parallelism = 4
salt_size = 32

[security]
require_mutual_tls = true
jwt_expiration_minutes = 60
max_failed_attempts = 3
lockout_duration_minutes = 30
enable_audit_log = true
rate_limit_requests_per_minute = 1000

[hsm]
enabled = true
provider = "pkcs11"
library_path = "/usr/lib/libpkcs11.so"
slot_id = 0
pin_env_var = "HSM_PIN"

[audit]
enable_audit = true
audit_level = "comprehensive"
encrypt_audit_logs = true
sign_audit_logs = true
retention_days = 2555

[audit.destinations]
file_enabled = true
file_path = "/var/log/beardog/audit.jsonl"
syslog_enabled = true
syslog_endpoint = "localhost:514"
database_enabled = true
database_url_env_var = "AUDIT_DATABASE_URL"

[threat_detection]
enable_real_time = true
behavioral_threshold = 0.7
anomaly_threshold = 0.8
ml_threshold = 0.6

[compliance]
enabled_standards = ["gdpr", "hipaa", "sox", "pci"]
audit_retention_days = 2555
report_schedule = "monthly"

[multi_party]
require_approval_for = ["key_rotation", "key_deletion", "policy_change"]
min_approvers = 2
approval_timeout_hours = 24

[network]
timeout_seconds = 30
max_retries = 3
connection_pool_size = 20

[logging]
level = "info"
format = "json"
output = "/var/log/beardog/beardog.log"
enable_structured_logging = true

[performance]
max_memory_mb = 1024
cache_size = 10000
cache_ttl_minutes = 60
metrics_collection_interval_seconds = 60

[integrations]
# NestGate integration
nestgate_enabled = true
nestgate_endpoint = "https://nestgate.internal:8081"
nestgate_cert_path = "/etc/beardog/certs/nestgate-client.crt"
nestgate_key_path = "/etc/beardog/certs/nestgate-client.key"

# SongBird integration
songbird_enabled = true
songbird_endpoint = "https://songbird.internal:8080"
songbird_cert_path = "/etc/beardog/certs/songbird-client.crt"
songbird_key_path = "/etc/beardog/certs/songbird-client.key"
```

### **Development Configuration Template**
```toml
# beardog-dev.toml - Development Configuration

[core]
service_id = "beardog-dev"
bind_address = "127.0.0.1"
port = 8443
enable_tls = false  # Simplified for development
worker_threads = 2

[encryption]
default_algorithm = "aes-256-gcm"
key_rotation_days = 7  # Faster rotation for testing

[security]
require_mutual_tls = false
jwt_expiration_minutes = 480  # Longer for development
max_failed_attempts = 10  # More lenient for development
rate_limit_requests_per_minute = 10000

[hsm]
enabled = false  # Use software keys for development

[audit]
enable_audit = true
audit_level = "standard"
encrypt_audit_logs = false
retention_days = 30

[audit.destinations]
file_enabled = true
file_path = "./logs/audit.jsonl"
syslog_enabled = false

[threat_detection]
enable_real_time = false  # Simplified for development

[logging]
level = "debug"
format = "pretty"
output = "stdout"

[performance]
max_memory_mb = 256
cache_size = 100
```

## 🔄 **Runtime Configuration Updates**

### **Configuration Hot Reload**
```rust
use tokio::sync::watch;

pub struct ConfigurationHotReload {
    config_tx: watch::Sender<BearDogConfig>,
    config_rx: watch::Receiver<BearDogConfig>,
    file_watcher: Option<tokio::fs::File>,
}

impl ConfigurationHotReload {
    pub async fn new(initial_config: BearDogConfig) -> Self {
        let (config_tx, config_rx) = watch::channel(initial_config);
        
        Self {
            config_tx,
            config_rx,
            file_watcher: None,
        }
    }
    
    pub async fn start_watching(&mut self, config_file_path: PathBuf) -> Result<()> {
        use tokio::time::{interval, Duration};
        
        let config_tx = self.config_tx.clone();
        let file_path = config_file_path.clone();
        
        tokio::spawn(async move {
            let mut last_modified = std::fs::metadata(&file_path)
                .ok()
                .and_then(|meta| meta.modified().ok());
            
            let mut interval = interval(Duration::from_secs(5)); // Check every 5 seconds
            
            loop {
                interval.tick().await;
                
                if let Ok(meta) = std::fs::metadata(&file_path) {
                    if let Ok(modified) = meta.modified() {
                        if last_modified.map_or(true, |last| modified > last) {
                            last_modified = Some(modified);
                            
                            // Reload configuration
                            if let Ok(new_config) = Self::load_config_from_file(&file_path).await {
                                if let Err(e) = config_tx.send(new_config) {
                                    tracing::error!("Failed to send updated configuration: {}", e);
                                    break;
                                }
                                
                                tracing::info!("Configuration reloaded from file: {:?}", file_path);
                            }
                        }
                    }
                }
            }
        });
        
        Ok(())
    }
    
    pub fn subscribe(&self) -> watch::Receiver<BearDogConfig> {
        self.config_rx.clone()
    }
    
    async fn load_config_from_file(path: &PathBuf) -> Result<BearDogConfig> {
        let content = tokio::fs::read_to_string(path).await?;
        
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") => toml::from_str(&content)
                .map_err(|e| BearDogError::ConfigurationError(format!("TOML parse error: {}", e))),
            Some("yaml") | Some("yml") => serde_yaml::from_str(&content)
                .map_err(|e| BearDogError::ConfigurationError(format!("YAML parse error: {}", e))),
            Some("json") => serde_json::from_str(&content)
                .map_err(|e| BearDogError::ConfigurationError(format!("JSON parse error: {}", e))),
            _ => Err(BearDogError::ConfigurationError("Unsupported file format".to_string())),
        }
    }
}
```

---

**Next Steps**: Implement distributed configuration sources (Consul/etcd), configuration encryption, and dynamic reconfiguration handlers. 