# 🔧 Configuration System Design - BearDog
**Date**: November 6, 2025  
**Status**: 🎯 **DESIGN PHASE**  
**Goal**: Eliminate 507+ hardcoded values  
**Target**: Zero hardcoding with sensible defaults

---

## 📋 EXECUTIVE SUMMARY

This document outlines the comprehensive design for BearDog's configuration system that will eliminate all hardcoded values while maintaining ease of use and deployment flexibility.

### Design Goals
1. **Zero Hardcoding**: All configurable values in config files/env vars
2. **Sensible Defaults**: Works out-of-box with secure defaults
3. **Hierarchy**: CLI args > env vars > config file > platform defaults > fallbacks
4. **Type Safety**: Compile-time validation where possible
5. **Platform Aware**: Auto-detect platform-specific paths/settings
6. **Migration Friendly**: Easy transition from current hardcoded values

---

## 🏗️ ARCHITECTURE OVERVIEW

### Configuration Hierarchy

```
┌─────────────────────────────────────────┐
│  1. Command-Line Arguments              │ ← Highest Priority
│     --port 8080 --config custom.toml    │
├─────────────────────────────────────────┤
│  2. Environment Variables               │
│     BEARDOG_API_PORT=8080               │
├─────────────────────────────────────────┤
│  3. Configuration File                  │
│     ~/.config/beardog/config.toml       │
├─────────────────────────────────────────┤
│  4. Platform Auto-Detection             │
│     /usr/lib/... (Linux detected)       │
├─────────────────────────────────────────┤
│  5. Secure Fallback Defaults            │
│     127.0.0.1:8080 (localhost only)     │
└─────────────────────────────────────────┘
```

### Crate Structure

```
beardog-config/
├── Cargo.toml
├── src/
│   ├── lib.rs                   # Public API
│   ├── loader.rs                # Configuration loading logic
│   ├── hierarchy.rs             # Hierarchy resolution
│   ├── validation.rs            # Validation rules
│   ├── defaults.rs              # Platform-aware defaults
│   ├── discovery.rs             # Path/resource discovery
│   ├── domains/                 # Domain-specific configs
│   │   ├── network.rs           # Network configuration
│   │   ├── hsm.rs               # HSM configuration
│   │   ├── paths.rs             # Path configuration
│   │   ├── limits.rs            # Limits/timeouts
│   │   ├── crypto.rs            # Cryptographic parameters
│   │   ├── security.rs          # Security settings
│   │   └── monitoring.rs        # Monitoring/logging
│   ├── serialization/           # Format support
│   │   ├── toml.rs
│   │   ├── json.rs
│   │   └── yaml.rs
│   └── tests/
│       ├── integration.rs
│       └── hierarchy_tests.rs
└── examples/
    ├── basic_usage.rs
    └── custom_config.rs
```

---

## 📦 CORE COMPONENTS

### 1. Main Configuration Structure

```rust
/// Master configuration for BearDog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogConfig {
    /// Network configuration
    pub network: NetworkConfig,
    
    /// Path configuration
    pub paths: PathConfig,
    
    /// HSM configuration
    pub hsm: HsmConfig,
    
    /// Limits and timeouts
    pub limits: LimitsConfig,
    
    /// Cryptographic parameters
    pub crypto: CryptoConfig,
    
    /// Security settings
    pub security: SecurityConfig,
    
    /// Monitoring and logging
    pub monitoring: MonitoringConfig,
    
    /// Service discovery
    pub discovery: DiscoveryConfig,
}

impl BearDogConfig {
    /// Load configuration with full hierarchy
    pub fn load() -> Result<Self, ConfigError> {
        ConfigLoader::new()
            .with_defaults()
            .with_platform_detection()
            .with_config_file()?
            .with_env_vars()
            .build()
    }
    
    /// Load from specific file
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        ConfigLoader::new()
            .with_defaults()
            .with_file(path)?
            .with_env_vars()
            .build()
    }
    
    /// Create with custom overrides
    pub fn with_overrides(overrides: ConfigOverrides) -> Result<Self, ConfigError> {
        ConfigLoader::new()
            .with_defaults()
            .with_platform_detection()
            .with_config_file()?
            .with_env_vars()
            .with_overrides(overrides)
            .build()
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.network.validate()?;
        self.paths.validate()?;
        self.hsm.validate()?;
        self.limits.validate()?;
        self.crypto.validate()?;
        self.security.validate()?;
        self.monitoring.validate()?;
        self.discovery.validate()?;
        Ok(())
    }
}
```

### 2. Network Configuration

```rust
/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// API server configuration
    pub api: ApiConfig,
    
    /// Service discovery configuration
    pub discovery: ServiceDiscoveryConfig,
    
    /// Admin interface configuration
    pub admin: AdminConfig,
    
    /// Operation timeouts
    pub timeouts: TimeoutsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Bind address (default: 127.0.0.1 for security)
    #[serde(default = "default_api_bind_address")]
    pub bind_address: IpAddr,
    
    /// API port (default: 8080)
    #[serde(default = "default_api_port")]
    pub port: u16,
    
    /// Enable TLS
    #[serde(default)]
    pub tls_enabled: bool,
    
    /// TLS certificate path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_cert_path: Option<PathBuf>,
    
    /// TLS key path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_key_path: Option<PathBuf>,
    
    /// Max concurrent connections
    #[serde(default = "default_max_connections")]
    pub max_connections: usize,
}

fn default_api_bind_address() -> IpAddr {
    env::var("BEARDOG_API_BIND_ADDRESS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| IpAddr::V4(Ipv4Addr::LOCALHOST))
}

fn default_api_port() -> u16 {
    env::var("BEARDOG_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080)
}

fn default_max_connections() -> usize {
    env::var("BEARDOG_MAX_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100)
}

impl ApiConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.port == 0 {
            return Err(ConfigError::Validation("API port cannot be 0".into()));
        }
        
        if self.tls_enabled {
            if self.tls_cert_path.is_none() || self.tls_key_path.is_none() {
                return Err(ConfigError::Validation(
                    "TLS enabled but certificate/key paths not provided".into()
                ));
            }
        }
        
        Ok(())
    }
}
```

### 3. Path Configuration with Discovery

```rust
/// Path configuration with auto-discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathConfig {
    /// Configuration directory
    #[serde(default = "default_config_dir")]
    pub config_dir: PathBuf,
    
    /// Data directory
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
    
    /// Log directory
    #[serde(default = "default_log_dir")]
    pub log_dir: PathBuf,
    
    /// PKCS#11 library paths (auto-discovered if empty)
    #[serde(default)]
    pub pkcs11_library_paths: Vec<PathBuf>,
    
    /// Specific PKCS#11 library to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkcs11_library: Option<PathBuf>,
}

impl PathConfig {
    /// Discover PKCS#11 libraries on the system
    pub fn discover_pkcs11_libraries() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        match env::consts::OS {
            "linux" => {
                let search_paths = vec![
                    "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so",
                    "/usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so",
                    "/usr/lib/softhsm/libsofthsm2.so",
                    "/usr/local/lib/softhsm/libsofthsm2.so",
                ];
                
                for path_str in search_paths {
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        paths.push(path);
                    }
                }
            }
            "macos" => {
                let search_paths = vec![
                    "/usr/local/lib/opensc-pkcs11.so",
                    "/opt/homebrew/lib/opensc-pkcs11.so",
                    "/usr/local/lib/softhsm/libsofthsm2.so",
                ];
                
                for path_str in search_paths {
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        paths.push(path);
                    }
                }
            }
            "windows" => {
                let search_paths = vec![
                    "C:\\Program Files\\OpenSC Project\\OpenSC\\pkcs11\\opensc-pkcs11.dll",
                    "C:\\SoftHSM2\\lib\\softhsm2.dll",
                ];
                
                for path_str in search_paths {
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        paths.push(path);
                    }
                }
            }
            _ => {}
        }
        
        paths
    }
    
    pub fn validate(&self) -> Result<(), ConfigError> {
        if let Some(ref lib) = self.pkcs11_library {
            if !lib.exists() {
                return Err(ConfigError::Validation(
                    format!("PKCS#11 library not found: {}", lib.display())
                ));
            }
        }
        
        Ok(())
    }
}

fn default_config_dir() -> PathBuf {
    if let Ok(path) = env::var("BEARDOG_CONFIG_DIR") {
        return PathBuf::from(path);
    }
    
    #[cfg(target_family = "unix")]
    {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let xdg_config = env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(&home).join(".config"));
        xdg_config.join("beardog")
    }
    
    #[cfg(target_family = "windows")]
    {
        let appdata = env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(appdata).join("beardog")
    }
}
```

### 4. HSM Configuration

```rust
/// HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// Auto-detect available HSMs
    #[serde(default = "default_true")]
    pub auto_detect: bool,
    
    /// Prefer hardware HSMs over software
    #[serde(default = "default_true")]
    pub prefer_hardware: bool,
    
    /// Required capabilities
    #[serde(default)]
    pub required_capabilities: Vec<String>,
    
    /// PKCS#11 configuration
    #[serde(default)]
    pub pkcs11: Pkcs11Config,
    
    /// Software HSM configuration
    #[serde(default)]
    pub software: SoftwareHsmConfig,
    
    /// TPM configuration
    #[serde(default)]
    pub tpm: TpmConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pkcs11Config {
    /// Enable PKCS#11 support
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Auto-fallback to SoftHSM if no hardware available
    #[serde(default = "default_true")]
    pub auto_fallback_softhsm: bool,
    
    /// Auto-initialize SoftHSM if needed
    #[serde(default)]
    pub auto_init_softhsm: bool,
    
    /// Specific slots to use (empty = auto-detect)
    #[serde(default)]
    pub slots: Vec<u64>,
    
    /// PIN for authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,
}

fn default_true() -> bool {
    true
}
```

### 5. Limits Configuration

```rust
/// Limits and timeouts configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitsConfig {
    /// Operation timeout in seconds
    #[serde(default = "default_operation_timeout")]
    pub operation_timeout_secs: u64,
    
    /// Maximum retries for failed operations
    #[serde(default = "default_max_retries")]
    pub max_retries: usize,
    
    /// Buffer size for I/O operations
    #[serde(default = "default_buffer_size")]
    pub buffer_size: usize,
    
    /// Maximum concurrent operations
    #[serde(default = "default_max_concurrent_operations")]
    pub max_concurrent_operations: usize,
    
    /// Rate limits
    #[serde(default)]
    pub rate_limits: RateLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimits {
    /// API requests per second
    #[serde(default = "default_api_rate_limit")]
    pub api_requests_per_second: u32,
    
    /// HSM operations per second
    #[serde(default = "default_hsm_rate_limit")]
    pub hsm_operations_per_second: u32,
}

impl Default for RateLimits {
    fn default() -> Self {
        Self {
            api_requests_per_second: default_api_rate_limit(),
            hsm_operations_per_second: default_hsm_rate_limit(),
        }
    }
}

fn default_operation_timeout() -> u64 {
    env::var("BEARDOG_OPERATION_TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30)
}

fn default_max_retries() -> usize {
    env::var("BEARDOG_MAX_RETRIES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3)
}

fn default_buffer_size() -> usize {
    env::var("BEARDOG_BUFFER_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8192)
}

fn default_max_concurrent_operations() -> usize {
    env::var("BEARDOG_MAX_CONCURRENT_OPS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100)
}

fn default_api_rate_limit() -> u32 {
    env::var("BEARDOG_API_RATE_LIMIT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000)
}

fn default_hsm_rate_limit() -> u32 {
    env::var("BEARDOG_HSM_RATE_LIMIT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100)
}
```

---

## 📝 CONFIGURATION FILE FORMAT

### Example: `beardog.toml`

```toml
# BearDog Configuration File
# All settings are optional - sensible defaults will be used

[network.api]
# Bind address (default: 127.0.0.1 for security)
bind_address = "127.0.0.1"
port = 8080
tls_enabled = false
max_connections = 100

[network.discovery]
# Service discovery port
port = 9090
# Discovery backends to use
backends = ["dns-sd", "static"]
# Multicast address for mDNS
multicast_address = "239.255.0.1"

[network.admin]
# Admin interface
bind_address = "127.0.0.1"
port = 9091
enabled = true

[paths]
# Paths (auto-detected if not specified)
config_dir = "~/.config/beardog"
data_dir = "~/.local/share/beardog"
log_dir = "~/.local/share/beardog/logs"

# PKCS#11 library (auto-discovered if not specified)
# pkcs11_library = "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so"

[hsm]
# HSM configuration
auto_detect = true
prefer_hardware = true
required_capabilities = ["random_generation", "key_storage", "signing"]

[hsm.pkcs11]
enabled = true
auto_fallback_softhsm = true
auto_init_softhsm = false
# Specific slots (empty = auto-detect)
slots = []
# PIN can also be set via BEARDOG_PKCS11_PIN env var
# pin = "1234"

[hsm.software]
enabled = true
# Software HSM data directory
data_dir = "~/.local/share/beardog/softhsm"

[hsm.tpm]
enabled = false
# TPM device path
device = "/dev/tpm0"

[limits]
operation_timeout_secs = 30
max_retries = 3
buffer_size = 8192
max_concurrent_operations = 100

[limits.rate_limits]
api_requests_per_second = 1000
hsm_operations_per_second = 100

[crypto]
# Cryptographic parameters (industry standards)
rsa_key_size = 2048
ec_curve = "P-256"
hash_algorithm = "SHA3-256"
# Key derivation function
kdf = "HKDF-SHA256"

[security]
# Security settings
enforce_tls = false
require_mutual_tls = false
# Certificate validation
validate_certificates = true
# Allowed certificate authorities
trusted_ca_paths = ["/etc/ssl/certs"]

[monitoring]
# Logging configuration
log_level = "info"
log_format = "json"  # or "text"
log_output = "file"   # or "stdout", "stderr"

# Metrics
metrics_enabled = true
metrics_port = 9092

# Tracing
tracing_enabled = false
tracing_endpoint = "http://localhost:4317"

[discovery]
# Service discovery configuration
enabled = true
# Discovery interval in seconds
interval_secs = 60
# Service filters
service_types = ["beardog-node", "beardog-gateway"]
```

---

## 🔄 MIGRATION STRATEGY

### Phase 1: Create Configuration Crate (Week 1)

**Tasks**:
1. Create `beardog-config` crate
2. Define all configuration structures
3. Implement hierarchy loading
4. Add validation logic
5. Write comprehensive tests

**Deliverables**:
- Working configuration system
- Example configurations
- Migration documentation

### Phase 2: Integrate with Existing Code (Week 2-3)

**Tasks**:
1. Update `beardog-core` to use config
2. Update `beardog-tunnel` to use config
3. Update `beardog-cli` to accept config options
4. Remove hardcoded values systematically

**Approach**:
```rust
// OLD (hardcoded)
let port = 8080;
let bind_addr = "127.0.0.1";

// NEW (configured)
let config = BearDogConfig::load()?;
let port = config.network.api.port;
let bind_addr = config.network.api.bind_address;
```

### Phase 3: Validation & Testing (Week 4)

**Tasks**:
1. Test all configuration combinations
2. Validate defaults work correctly
3. Test platform-specific auto-detection
4. Integration testing
5. Documentation updates

---

## 🧪 TESTING STRATEGY

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = BearDogConfig::default();
        assert_eq!(config.network.api.port, 8080);
        assert_eq!(config.network.api.bind_address, IpAddr::V4(Ipv4Addr::LOCALHOST));
    }
    
    #[test]
    fn test_config_from_file() {
        let config = BearDogConfig::from_file("test_config.toml").unwrap();
        config.validate().unwrap();
    }
    
    #[test]
    fn test_env_var_override() {
        env::set_var("BEARDOG_API_PORT", "9000");
        let config = BearDogConfig::load().unwrap();
        assert_eq!(config.network.api.port, 9000);
        env::remove_var("BEARDOG_API_PORT");
    }
    
    #[test]
    fn test_hierarchy() {
        // Test that CLI args > env vars > config file
        env::set_var("BEARDOG_API_PORT", "9000");
        let mut config = BearDogConfig::load().unwrap();
        
        // Simulate CLI override
        let overrides = ConfigOverrides {
            api_port: Some(10000),
            ..Default::default()
        };
        config = config.with_overrides(overrides).unwrap();
        
        assert_eq!(config.network.api.port, 10000);
        env::remove_var("BEARDOG_API_PORT");
    }
}
```

### Integration Tests

```rust
#[test]
fn test_full_config_lifecycle() {
    // 1. Create config file
    let config_path = "/tmp/beardog_test.toml";
    let config_content = r#"
        [network.api]
        port = 7000
    "#;
    fs::write(config_path, config_content).unwrap();
    
    // 2. Load with hierarchy
    env::set_var("BEARDOG_API_BIND_ADDRESS", "0.0.0.0");
    let config = BearDogConfig::from_file(config_path).unwrap();
    
    // 3. Verify hierarchy worked
    assert_eq!(config.network.api.port, 7000); // From file
    assert_eq!(config.network.api.bind_address, "0.0.0.0".parse().unwrap()); // From env
    
    // Cleanup
    fs::remove_file(config_path).ok();
    env::remove_var("BEARDOG_API_BIND_ADDRESS");
}
```

---

## 📊 ELIMINATION TRACKING

### Hardcoded Values by Category

| Category | Count | Config Section | Priority |
|----------|-------|----------------|----------|
| **Network IPs** | 267 | `network.*` | 🔴 Critical |
| **Port Numbers** | 116 | `network.*` | 🔴 Critical |
| **File Paths** | 124 | `paths.*` | 🔴 Critical |
| **Timeouts** | ~45 | `limits.*` | 🟡 High |
| **Buffer Sizes** | ~20 | `limits.*` | 🟡 High |
| **Crypto Params** | ~30 | `crypto.*` | 🟢 Medium |

**Total**: 507+ hardcoded values

### Migration Checklist

- [ ] Network configuration (267 + 116 = 383 values)
- [ ] Path configuration (124 values)
- [ ] Timeout/limits configuration (45 values)
- [ ] Buffer sizes (20 values)
- [ ] Cryptographic parameters (30 values)
- [ ] HSM parameters (~20 values)
- [ ] Monitoring/logging (~15 values)

---

## 🎯 SUCCESS CRITERIA

### Must Have
- ✅ Zero hardcoded network addresses/ports
- ✅ Zero hardcoded file paths
- ✅ Sensible secure defaults
- ✅ Environment variable overrides
- ✅ Config file support (TOML)
- ✅ Validation with clear errors
- ✅ Platform auto-detection

### Should Have
- ✅ Multiple config format support (TOML, JSON, YAML)
- ✅ Configuration migration tool
- ✅ Config file generation
- ✅ Hot-reload capability
- ✅ Comprehensive documentation

### Nice to Have
- Runtime configuration changes (where safe)
- Configuration UI/TUI
- Configuration profiles (dev, staging, prod)
- Configuration versioning

---

## 📈 TIMELINE

### Week 1: Foundation
- Days 1-2: Create crate structure, define types
- Days 3-4: Implement loading and hierarchy
- Day 5: Validation and testing

### Week 2: Integration Part 1
- Days 1-3: Migrate network configuration
- Days 4-5: Migrate path configuration

### Week 3: Integration Part 2
- Days 1-2: Migrate HSM configuration
- Days 3-4: Migrate limits/crypto configuration
- Day 5: Testing and validation

### Week 4: Polish & Documentation
- Days 1-2: Integration testing
- Days 3-4: Documentation and examples
- Day 5: Migration guide and tools

**Total**: 4 weeks (60-80 hours)

---

## 🎓 BEST PRACTICES

### 1. Secure Defaults
```rust
// Always default to secure settings
fn default_api_bind_address() -> IpAddr {
    IpAddr::V4(Ipv4Addr::LOCALHOST)  // ✅ Secure: localhost only
    // NOT: IpAddr::V4(Ipv4Addr::UNSPECIFIED)  // ❌ Promiscuous: 0.0.0.0
}
```

### 2. Clear Validation
```rust
impl ApiConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.port == 0 {
            return Err(ConfigError::Validation(
                "API port cannot be 0. Please specify a valid port (1-65535)".into()
            ));
        }
        
        if self.bind_address == IpAddr::V4(Ipv4Addr::UNSPECIFIED) {
            warn!("Binding to 0.0.0.0 - API will be accessible from any network interface");
        }
        
        Ok(())
    }
}
```

### 3. Platform Awareness
```rust
fn discover_pkcs11_libraries() -> Vec<PathBuf> {
    match env::consts::OS {
        "linux" => discover_linux_pkcs11(),
        "macos" => discover_macos_pkcs11(),
        "windows" => discover_windows_pkcs11(),
        _ => Vec::new(),
    }
}
```

### 4. Migration Compatibility
```rust
// Support old environment variable names during transition
fn legacy_api_port() -> Option<u16> {
    env::var("BEARDOG_PORT")  // Old name
        .ok()
        .and_then(|s| s.parse().ok())
        .or_else(|| {
            env::var("BEARDOG_API_PORT")  // New name
                .ok()
                .and_then(|s| s.parse().ok())
        })
}
```

---

## 💡 RECOMMENDATIONS

### Immediate Actions
1. Create `beardog-config` crate skeleton
2. Define configuration structures
3. Implement basic loading
4. Write initial tests

### Short-term Actions
1. Migrate network configuration first (biggest impact)
2. Update CLI to accept config options
3. Create example configurations
4. Write migration guide

### Long-term Actions
1. Add hot-reload capability
2. Create configuration UI
3. Implement configuration profiles
4. Add configuration versioning

---

**Status**: 🎯 **DESIGN COMPLETE - READY FOR IMPLEMENTATION**  
**Next Step**: Create `beardog-config` crate  
**Owner**: Development Team  
**Timeline**: 4 weeks to zero hardcoding

🐻🔧 **BearDog: Configuration System - Path to Zero Hardcoding** 🔧🐻

