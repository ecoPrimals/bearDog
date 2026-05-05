# 🔧 Unified Configuration Architecture
## BearDog Ecosystem Configuration System Design

**Version**: 1.0.0  
**Date**: November 6, 2025  
**Status**: DESIGN SPECIFICATION

---

## 📋 Executive Summary

### Problem Statement

**Current State**: 441 hardcoded values scattered across the codebase
- Port numbers (3000, 8080, 443, 9000, etc.)
- Primal IDs ("songbird-001", "beardog-primary", etc.)
- Timeout values (30, 5000, 10000, etc.)
- Buffer sizes (1024, 4096, 8192, etc.)
- Retry counts (3, 5, 10, etc.)
- IP addresses ("127.0.0.1", "0.0.0.0", etc.)

**Impact**:
- ❌ Difficult to configure for different environments
- ❌ Impossible to change without recompilation
- ❌ No runtime configuration updates
- ❌ Testing requires code changes
- ❌ Deployment flexibility limited

### Solution Overview

**Unified Configuration System** with:
1. **Hierarchical Config Sources** (12-factor app methodology)
2. **Type-Safe Configuration** (compile-time validation)
3. **Environment-Specific Overrides** (dev/staging/prod)
4. **Runtime Hot-Reload** (zero-downtime updates)
5. **Validation & Defaults** (fail-fast with clear errors)
6. **Discovery Integration** (zero-hardcoding for services)

---

## 🏗️ Architecture

### Configuration Hierarchy (Priority Order)

```
1. Environment Variables (highest priority)
   ↓ overrides
2. Config File (.toml/.yaml/.json)
   ↓ overrides
3. Discovery Results (dynamic endpoints)
   ↓ overrides
4. Compiled Defaults (lowest priority, fallback)
```

### Core Principles

1. **Zero Hardcoding**: All values configurable
2. **Sensible Defaults**: Works out-of-box for development
3. **Environment Awareness**: Auto-detect dev/staging/prod
4. **Type Safety**: Compile-time validation where possible
5. **Runtime Flexibility**: Hot-reload without restart
6. **Discovery First**: Prefer dynamic over static
7. **Fail Fast**: Clear errors on misconfiguration

---

## 📁 Configuration Structure

### File Format: TOML (Primary)

```toml
# beardog.toml
[environment]
name = "production"  # dev, staging, production
log_level = "info"   # trace, debug, info, warn, error

[network]
api_host = "0.0.0.0"
api_port = 8080
discovery_endpoint = "auto"  # "auto" = use discovery, or explicit URL
enable_tls = true
cert_path = "/etc/beardog/certs/server.pem"
key_path = "/etc/beardog/certs/server.key"

[network.timeouts]
connect_ms = 5000
request_ms = 30000
idle_ms = 60000

[discovery]
enabled = true
methods = ["mdns", "dns-sd", "static"]
refresh_interval_secs = 300
cache_ttl_secs = 600

[discovery.static_endpoints]
songbird = ["https://songbird-001.local:9443", "https://songbird-002.local:9443"]
parrot = ["https://parrot-001.local:9444"]

[hsm]
provider = "auto"  # auto, software, pkcs11, tpm, ios, android, aws-kms, etc.
discovery_enabled = true
fallback_to_software = true

[hsm.software]
crypto_backend = "ring"  # ring, openssl, rustcrypto
key_storage = "database"  # memory, file, database
memory_protection = "high"  # low, medium, high, maximum

[hsm.software.storage]
database_url = "${DATABASE_URL}"  # env var interpolation
backup_path = "/var/lib/beardog/hsm/backup"
rotation_interval_hours = 720  # 30 days

[security]
require_mutual_tls = true
max_auth_attempts = 5
auth_timeout_secs = 300
session_duration_secs = 28800  # 8 hours

[security.encryption]
algorithm = "aes-256-gcm"
key_derivation = "hkdf-sha256"
min_key_bits = 256

[monitoring]
enabled = true
metrics_port = 9190
health_check_port = 9091
alert_webhook_url = "${ALERT_WEBHOOK_URL}"

[monitoring.thresholds]
cpu_warning_percent = 80
cpu_critical_percent = 95
memory_warning_percent = 85
memory_critical_percent = 95
error_rate_per_min = 10

[performance]
worker_threads = 0  # 0 = auto (num_cpus)
max_connections = 10000
buffer_size_bytes = 8192
enable_zero_copy = true

[genetics]
enabled = true
adaptation_rate = 0.05
fitness_threshold = 0.7
mutation_probability = 0.1
```

### Environment Variable Overrides

**Format**: `BEARDOG_<SECTION>_<KEY>=<value>`

```bash
# Override network port
export BEARDOG_NETWORK_API_PORT=9000

# Override HSM provider
export BEARDOG_HSM_PROVIDER=pkcs11

# Override discovery endpoint
export BEARDOG_DISCOVERY_ENDPOINT=https://discovery.example.com

# Override security settings
export BEARDOG_SECURITY_REQUIRE_MUTUAL_TLS=false

# Secret interpolation
export DATABASE_URL=postgresql://user:pass@localhost/beardog
export ALERT_WEBHOOK_URL=https://alerts.example.com/webhook
```

---

## 🔧 Implementation Design

### Core Types

```rust
// crates/beardog-types/src/canonical/config/unified/mod.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// Root configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogConfig {
    pub environment: EnvironmentConfig,
    pub network: NetworkConfig,
    pub discovery: DiscoveryConfig,
    pub hsm: HsmConfig,
    pub security: SecurityConfig,
    pub monitoring: MonitoringConfig,
    pub performance: PerformanceConfig,
    pub genetics: GeneticsConfig,
}

impl BearDogConfig {
    /// Load configuration from all sources (hierarchy)
    pub fn load() -> Result<Self, ConfigError> {
        ConfigLoader::new()
            .with_defaults()
            .with_file_if_exists("beardog.toml")?
            .with_file_if_exists("/etc/beardog/config.toml")?
            .with_env_vars("BEARDOG_")?
            .with_discovery()?
            .validate()?
            .build()
    }
    
    /// Load with custom file path
    pub fn load_from(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        ConfigLoader::new()
            .with_defaults()
            .with_file(path)?
            .with_env_vars("BEARDOG_")?
            .validate()?
            .build()
    }
    
    /// Load with environment name
    pub fn load_for_env(env: Environment) -> Result<Self, ConfigError> {
        let file = match env {
            Environment::Development => "beardog.dev.toml",
            Environment::Staging => "beardog.staging.toml",
            Environment::Production => "beardog.prod.toml",
        };
        
        ConfigLoader::new()
            .with_defaults()
            .with_file_if_exists(file)?
            .with_env_vars("BEARDOG_")?
            .validate()?
            .build()
    }
}

/// Environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub name: Environment,
    pub log_level: LogLevel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub api_host: String,
    pub api_port: u16,
    pub discovery_endpoint: DiscoveryEndpoint,
    pub enable_tls: bool,
    pub cert_path: Option<PathBuf>,
    pub key_path: Option<PathBuf>,
    pub timeouts: TimeoutConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiscoveryEndpoint {
    Auto,
    Url(String),
}

/// Timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    #[serde(with = "duration_millis")]
    pub connect: Duration,
    #[serde(with = "duration_millis")]
    pub request: Duration,
    #[serde(with = "duration_millis")]
    pub idle: Duration,
}

// ... similar for other config sections ...
```

### Configuration Loader

```rust
// crates/beardog-utils/src/config/loader.rs

use beardog_types::canonical::config::unified::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use std::path::Path;

pub struct ConfigLoader {
    config: BearDogConfig,
    sources: Vec<ConfigSource>,
}

impl ConfigLoader {
    pub fn new() -> Self {
        Self {
            config: BearDogConfig::default(),
            sources: Vec::new(),
        }
    }
    
    /// Load defaults (compiled-in fallbacks)
    pub fn with_defaults(mut self) -> Self {
        self.config = BearDogConfig::defaults();
        self.sources.push(ConfigSource::Defaults);
        self
    }
    
    /// Load from file (TOML/YAML/JSON)
    pub fn with_file(mut self, path: impl AsRef<Path>) -> BearDogResult<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| BearDogError::configuration(
                format!("Failed to read config file: {}", e)
            ))?;
        
        let file_config: BearDogConfig = toml::from_str(&content)
            .map_err(|e| BearDogError::configuration(
                format!("Failed to parse config file: {}", e)
            ))?;
        
        // Merge file config into existing config
        self.config = self.config.merge(file_config);
        self.sources.push(ConfigSource::File(path.as_ref().to_path_buf()));
        Ok(self)
    }
    
    /// Load from environment variables
    pub fn with_env_vars(mut self, prefix: &str) -> BearDogResult<Self> {
        let env_config = BearDogConfig::from_env(prefix)?;
        self.config = self.config.merge(env_config);
        self.sources.push(ConfigSource::Environment);
        Ok(self)
    }
    
    /// Load from discovery (dynamic endpoints)
    pub fn with_discovery(mut self) -> BearDogResult<Self> {
        if self.config.discovery.enabled {
            // Query discovery for dynamic endpoints
            // This is async, so we need a runtime here
            let discovered_config = tokio::runtime::Runtime::new()?
                .block_on(async {
                    discover_endpoints(&self.config.discovery).await
                })?;
            
            self.config = self.config.merge(discovered_config);
            self.sources.push(ConfigSource::Discovery);
        }
        Ok(self)
    }
    
    /// Validate configuration
    pub fn validate(self) -> BearDogResult<Self> {
        self.config.validate()?;
        Ok(self)
    }
    
    /// Build final configuration
    pub fn build(self) -> BearDogResult<BearDogConfig> {
        Ok(self.config)
    }
}
```

### Hot Reload Support

```rust
// crates/beardog-utils/src/config/watcher.rs

use beardog_types::canonical::config::unified::BearDogConfig;
use notify::{Watcher, RecursiveMode, Event};
use tokio::sync::watch;
use std::path::Path;
use std::sync::Arc;

pub struct ConfigWatcher {
    config: Arc<watch::Sender<BearDogConfig>>,
    _watcher: Box<dyn Watcher>,
}

impl ConfigWatcher {
    /// Start watching configuration file for changes
    pub fn watch(
        path: impl AsRef<Path>
    ) -> BearDogResult<(Self, watch::Receiver<BearDogConfig>)> {
        let (tx, rx) = watch::channel(BearDogConfig::load_from(&path)?);
        let tx = Arc::new(tx);
        
        let path_clone = path.as_ref().to_path_buf();
        let tx_clone = Arc::clone(&tx);
        
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(_event) = res {
                // Reload configuration on file change
                if let Ok(new_config) = BearDogConfig::load_from(&path_clone) {
                    let _ = tx_clone.send(new_config);
                    info!("📝 Configuration reloaded from file");
                }
            }
        })?;
        
        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;
        
        Ok((
            Self {
                config: tx,
                _watcher: Box::new(watcher),
            },
            rx
        ))
    }
}

/// Component that needs to react to config changes
pub trait ConfigurableComponent {
    fn apply_config(&mut self, config: &BearDogConfig) -> BearDogResult<()>;
}
```

---

## 🚀 Migration Strategy

### Phase 1: Foundation (Week 1)

**Goal**: Core configuration system operational

1. **Day 1-2**: Implement core types and loader
   - Create `BearDogConfig` structure
   - Implement `ConfigLoader` with hierarchy
   - Add TOML/YAML parsing
   - Add environment variable support

2. **Day 3**: Validation and defaults
   - Implement `validate()` method
   - Define sensible defaults for all values
   - Add config validation tests

3. **Day 4-5**: Integration with existing code
   - Replace network hardcoding
   - Replace timeout hardcoding
   - Replace HSM configuration hardcoding

### Phase 2: Discovery Integration (Week 2)

**Goal**: Dynamic service discovery configuration

1. **Day 1-2**: Discovery-driven configuration
   - Implement `discover_endpoints()`
   - Integrate with Universal Discovery
   - Handle discovery failures gracefully

2. **Day 3-4**: Testing and validation
   - Test discovery fallback
   - Test static + dynamic hybrid
   - Test discovery cache behavior

### Phase 3: Hot Reload (Week 3)

**Goal**: Zero-downtime configuration updates

1. **Day 1-2**: File watching
   - Implement `ConfigWatcher`
   - Add configuration update events
   - Test reload behavior

2. **Day 3-4**: Component integration
   - Implement `ConfigurableComponent` trait
   - Update components to react to config changes
   - Test hot reload scenarios

### Phase 4: Complete Migration (Week 4)

**Goal**: Zero hardcoded values

1. **Day 1-3**: Migrate remaining hardcoding
   - Security settings
   - Monitoring thresholds
   - Performance tuning
   - Genetics parameters

2. **Day 4-5**: Documentation and testing
   - Update all documentation
   - Create configuration guide
   - Add end-to-end config tests

---

## 🧪 Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config_valid() {
        let config = BearDogConfig::defaults();
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_env_var_override() {
        std::env::set_var("BEARDOG_NETWORK_API_PORT", "9000");
        let config = BearDogConfig::load().unwrap();
        assert_eq!(config.network.api_port, 9000);
    }
    
    #[test]
    fn test_file_priority() {
        // File overrides defaults
        let config = BearDogConfig::load_from("test.toml").unwrap();
        // Verify file values used
    }
    
    #[test]
    fn test_validation_catches_invalid() {
        let mut config = BearDogConfig::defaults();
        config.network.api_port = 0;  // Invalid
        assert!(config.validate().is_err());
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_config_hot_reload() {
    let temp_file = create_temp_config();
    let (watcher, mut rx) = ConfigWatcher::watch(&temp_file).unwrap();
    
    // Modify config file
    update_temp_config(&temp_file, "api_port = 9000");
    
    // Wait for update
    let updated = tokio::time::timeout(
        Duration::from_secs(5),
        rx.changed()
    ).await.unwrap();
    
    assert_eq!(rx.borrow().network.api_port, 9000);
}
```

---

## 📊 Impact Analysis

### Before Migration

```rust
// ❌ Hardcoded everywhere
let port = 8080;
let timeout = Duration::from_secs(30);
let endpoint = "https://songbird-001.local:9443";
```

### After Migration

```rust
// ✅ Fully configurable
let config = BearDogConfig::load()?;
let port = config.network.api_port;
let timeout = config.network.timeouts.request;
let endpoint = config.discovery.resolve("songbird")?;
```

### Benefits

1. **Flexibility**: Configure without recompilation
2. **Testing**: Easy to test with different configs
3. **Deployment**: Environment-specific settings
4. **Operations**: Hot-reload for zero-downtime updates
5. **Discovery**: Automatic endpoint resolution
6. **Validation**: Catch misconfiguration early
7. **Documentation**: Config is self-documenting

---

## 🔐 Security Considerations

### Secrets Management

**Never commit secrets to config files!**

```toml
# ❌ Bad: Plain text secrets
database_password = "secret123"

# ✅ Good: Environment variable reference
database_password = "${DATABASE_PASSWORD}"

# ✅ Good: Secrets provider reference
database_password = "@vault:secret/data/beardog/db#password"
```

### Secrets Providers Integration

```rust
// Support multiple secrets backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecretsProvider {
    Environment,
    Vault { endpoint: String, token: String },
    AwsSecretsManager { region: String },
    UniversalSecretsManagement { endpoint: String },
}
```

---

## 📚 Example Configurations

### Development

```toml
[environment]
name = "development"
log_level = "debug"

[network]
api_host = "127.0.0.1"
api_port = 8080
enable_tls = false

[hsm]
provider = "software"
[hsm.software]
crypto_backend = "ring"
key_storage = "memory"
```

### Production

```toml
[environment]
name = "production"
log_level = "info"

[network]
api_host = "0.0.0.0"
api_port = 443
enable_tls = true
cert_path = "/etc/beardog/certs/server.pem"
key_path = "/etc/beardog/certs/server.key"

[hsm]
provider = "auto"  # Auto-detect best available
discovery_enabled = true

[security]
require_mutual_tls = true
max_auth_attempts = 3
```

---

## ✅ Success Criteria

### Definition of Done

- [ ] All 441 hardcoded values migrated
- [ ] Configuration loads from file + env vars
- [ ] Validation catches all invalid configs
- [ ] Hot reload working for all components
- [ ] Discovery integration complete
- [ ] Documentation complete
- [ ] All tests passing
- [ ] Zero hardcoded values in production code

### Metrics

- **Hardcoded Values**: 441 → 0
- **Configuration Sources**: 4 (defaults, file, env, discovery)
- **Hot Reload Support**: ✅
- **Type Safety**: ✅ (compile-time validation)
- **Documentation**: ✅ (complete guide)

---

## 🎯 Conclusion

This unified configuration architecture will:
1. ✅ Eliminate all 441 hardcoded values
2. ✅ Enable flexible deployment
3. ✅ Support hot-reload (zero-downtime)
4. ✅ Integrate with discovery (zero-hardcoding)
5. ✅ Maintain type safety
6. ✅ Provide sensible defaults
7. ✅ Support secrets management

**Effort**: ~4 weeks (one developer)  
**Priority**: High (blocks production flexibility)  
**Risk**: Low (additive change, no breaking changes)

---

**Document Version**: 1.0.0  
**Last Updated**: November 6, 2025  
**Next Review**: After Phase 1 completion

