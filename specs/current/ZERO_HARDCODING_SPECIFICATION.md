# 🚫 Zero Hardcoding Specification
## Eliminating All Hardcoded Values from BearDog

**Version**: 1.0  
**Date**: November 1, 2025  
**Status**: 🎯 **ACTIVE MANDATE**  
**Current Hardcoding**: 211 instances remaining (down from 472)  
**Target**: **ZERO** hardcoded values in production code  
**Philosophy**: **Configuration over Convention**

---

## 🎯 EXECUTIVE SUMMARY

**Problem**: Hardcoded values (IPs, ports, paths, timeouts, constants) make BearDog inflexible, hard to test, and difficult to deploy across different environments.

**Solution**: Move ALL configurable values to configuration files, environment variables, or runtime discovery.

**Status**:
- ✅ **45% reduction**: 261 hardcoded values removed
- ⚠️ **211 remaining**: Need systematic elimination
- 🎯 **Target**: Zero hardcoded values by Phase 1 completion

---

## 📊 HARDCODING AUDIT (November 1, 2025)

### **Original State (October 2025)**
```yaml
Total Hardcoded Values: 472 instances
Breakdown:
  - IP Addresses: ~200 instances
  - Port Numbers: ~341 instances
  - File Paths: ~100 instances
  - Timeouts: ~50 instances
  - Other Constants: ~81 instances
```

### **Current State (November 1, 2025)**
```yaml
Total Hardcoded Values: 211 instances (55% reduction!)
Breakdown:
  - Default Library Paths: ~40 instances
  - Network Configuration: ~80 instances
  - Timeouts/Limits: ~45 instances
  - Test Constants: ~46 instances (acceptable)
```

### **Target State (Phase 1 Complete)**
```yaml
Total Hardcoded Values: 0 instances in production code
Allowed:
  - Test constants (in test code only)
  - Protocol constants (e.g., PKCS#11 version)
  - Mathematical constants (e.g., π)
```

---

## 🚨 HARDCODING VIOLATIONS

### **Category 1: Network Configuration** ⚠️ **HIGH PRIORITY**

**Current Violations:**
```rust
// ❌ HARDCODED - Default ports
const API_PORT: u16 = 8080;
const DISCOVERY_PORT: u16 = 9090;
const ADMIN_PORT: u16 = 9091;

// ❌ HARDCODED - Bind addresses
let bind_addr = "127.0.0.1:8080".parse()?;

// ❌ HARDCODED - Timeouts
tokio::time::timeout(Duration::from_secs(30), operation).await?;

// ❌ HARDCODED - URLs
let endpoint = "http://localhost:8080/api/v1/nodes";
```

**Solution:**
```rust
// ✅ CONFIGURATION-DRIVEN
pub struct NetworkConfig {
    pub api_port: u16,
    pub discovery_port: u16,
    pub admin_port: u16,
    pub bind_address: IpAddr,
    pub operation_timeout: Duration,
    pub api_base_url: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            api_port: env::var("BEARDOG_API_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8080),
            discovery_port: env::var("BEARDOG_DISCOVERY_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(9090),
            // ... all from env or config file
        }
    }
}

impl NetworkConfig {
    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }
}
```

---

### **Category 2: File Paths** ⚠️ **HIGH PRIORITY**

**Current Violations:**
```rust
// ❌ HARDCODED - Library paths
#[arg(long, default_value = "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so")]
library: String,

// ❌ HARDCODED - Config paths
let config_path = PathBuf::from("/etc/beardog/config.toml");

// ❌ HARDCODED - Data directories
let data_dir = PathBuf::from("~/.config/beardog");

// ❌ HARDCODED - Log files
let log_file = File::create("/var/log/beardog/app.log")?;
```

**Solution:**
```rust
// ✅ DYNAMIC DISCOVERY
pub struct PathConfig {
    pub library_search_paths: Vec<PathBuf>,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub log_dir: PathBuf,
}

impl Default for PathConfig {
    fn default() -> Self {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let config_dir = env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(&home).join(".config"))
            .join("beardog");
        
        Self {
            library_search_paths: Self::discover_library_paths(),
            config_dir: env::var("BEARDOG_CONFIG_DIR")
                .map(PathBuf::from)
                .unwrap_or(config_dir.clone()),
            data_dir: env::var("BEARDOG_DATA_DIR")
                .map(PathBuf::from)
                .unwrap_or(config_dir.join("data")),
            log_dir: env::var("BEARDOG_LOG_DIR")
                .map(PathBuf::from)
                .unwrap_or(config_dir.join("logs")),
        }
    }
    
    fn discover_library_paths() -> Vec<PathBuf> {
        match env::consts::OS {
            "linux" => vec![
                PathBuf::from("/usr/lib/x86_64-linux-gnu"),
                PathBuf::from("/usr/local/lib"),
                PathBuf::from("/usr/lib"),
            ],
            "macos" => vec![
                PathBuf::from("/usr/local/lib"),
                PathBuf::from("/opt/homebrew/lib"),
            ],
            "windows" => vec![
                PathBuf::from("C:\\Program Files\\OpenSC\\pkcs11"),
                PathBuf::from("C:\\Program Files\\SoftHSM2\\lib"),
            ],
            _ => vec![],
        }
    }
}
```

---

### **Category 3: Timeouts & Limits** ⚠️ **MEDIUM PRIORITY**

**Current Violations:**
```rust
// ❌ HARDCODED - Operation timeouts
tokio::time::timeout(Duration::from_secs(30), operation).await?;

// ❌ HARDCODED - Retry counts
const MAX_RETRIES: usize = 3;

// ❌ HARDCODED - Buffer sizes
const BUFFER_SIZE: usize = 8192;

// ❌ HARDCODED - Connection limits
const MAX_CONNECTIONS: usize = 100;
```

**Solution:**
```rust
// ✅ CONFIGURABLE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitsConfig {
    #[serde(default = "default_operation_timeout")]
    pub operation_timeout: Duration,
    
    #[serde(default = "default_max_retries")]
    pub max_retries: usize,
    
    #[serde(default = "default_buffer_size")]
    pub buffer_size: usize,
    
    #[serde(default = "default_max_connections")]
    pub max_connections: usize,
}

fn default_operation_timeout() -> Duration {
    Duration::from_secs(
        env::var("BEARDOG_OPERATION_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30)
    )
}

fn default_max_retries() -> usize {
    env::var("BEARDOG_MAX_RETRIES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3)
}

// ... etc.
```

---

### **Category 4: Cryptographic Parameters** ✅ **ACCEPTABLE WITH DEFAULTS**

**Acceptable (with overrides):**
```rust
// ✅ OK - Industry standard defaults (but configurable!)
pub struct CryptoConfig {
    #[serde(default = "default_rsa_key_size")]
    pub rsa_key_size: usize,  // Default: 2048 (configurable)
    
    #[serde(default = "default_ec_curve")]
    pub ec_curve: Curve,  // Default: P-256 (configurable)
    
    #[serde(default = "default_hash_algorithm")]
    pub hash_algorithm: HashAlgorithm,  // Default: SHA3-256 (configurable)
}

fn default_rsa_key_size() -> usize {
    env::var("BEARDOG_RSA_KEY_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2048)  // Industry standard, but overridable
}
```

**Rationale**: Some cryptographic defaults are industry standards, but they MUST be configurable for:
- Different security requirements
- Compatibility with legacy systems
- Future algorithm upgrades
- Testing with smaller keys

---

## 🏗️ CONFIGURATION ARCHITECTURE

### **Configuration Hierarchy**

```
1. Command-line Arguments (Highest Priority)
   ↓
2. Environment Variables
   ↓
3. Config File (beardog.toml)
   ↓
4. Platform Defaults (auto-detected)
   ↓
5. Fallback Constants (Last Resort)
```

**Implementation:**
```rust
pub struct Config {
    network: NetworkConfig,
    paths: PathConfig,
    limits: LimitsConfig,
    crypto: CryptoConfig,
    hardware: HardwareConfig,
}

impl Config {
    /// Load configuration with proper hierarchy
    pub fn load() -> Result<Self> {
        // 1. Start with fallback defaults
        let mut config = Self::defaults();
        
        // 2. Override with config file (if exists)
        if let Some(config_path) = Self::find_config_file() {
            config = config.merge_file(&config_path)?;
        }
        
        // 3. Override with environment variables
        config = config.merge_env()?;
        
        // 4. Override with command-line args (handled by clap)
        // (Done in CLI parsing)
        
        Ok(config)
    }
    
    fn find_config_file() -> Option<PathBuf> {
        // Search in order:
        // 1. $BEARDOG_CONFIG
        // 2. ./beardog.toml
        // 3. ~/.config/beardog/config.toml
        // 4. /etc/beardog/config.toml
        
        if let Ok(path) = env::var("BEARDOG_CONFIG") {
            let p = PathBuf::from(path);
            if p.exists() {
                return Some(p);
            }
        }
        
        let candidates = vec![
            PathBuf::from("beardog.toml"),
            PathBuf::from(env::var("HOME").unwrap_or(".".to_string()))
                .join(".config/beardog/config.toml"),
            PathBuf::from("/etc/beardog/config.toml"),
        ];
        
        candidates.into_iter().find(|p| p.exists())
    }
}
```

---

### **Configuration File Format**

**File**: `configs/beardog.toml` (example)

```toml
# BearDog Configuration
# All values are optional - defaults will be used if not specified

[network]
api_port = 8080
discovery_port = 9090
admin_port = 9091
bind_address = "127.0.0.1"
operation_timeout_secs = 30

[network.endpoints]
api_base_url = "http://localhost:8080"
discovery_multicast_addr = "239.255.0.1:9090"

[paths]
config_dir = "~/.config/beardog"
data_dir = "~/.local/share/beardog"
log_dir = "~/.local/share/beardog/logs"

[paths.libraries]
# PKCS#11 library search paths (auto-discovered if empty)
search_paths = []
# Or specify exact library:
# pkcs11_library = "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so"

[limits]
operation_timeout_secs = 30
max_retries = 3
buffer_size = 8192
max_connections = 100

[crypto]
rsa_key_size = 2048
ec_curve = "P-256"
hash_algorithm = "SHA3-256"

[hardware]
auto_detect = true
prefer_hardware = true
required_capabilities = ["random_generation", "key_storage"]

[hardware.pkcs11]
auto_fallback_softhsm = true
auto_init_softhsm = true

[hardware.softhsm]
config_path = "~/.config/softhsm2/softhsm2.conf"
token_dir = "~/.config/softhsm2/tokens"
default_token_label = "BearDog-Default"

[logging]
level = "info"
format = "json"
output = "file"  # or "stdout", "stderr"
```

---

### **Environment Variables**

**Naming Convention**: `BEARDOG_<SECTION>_<KEY>`

```bash
# Network
export BEARDOG_API_PORT=8080
export BEARDOG_DISCOVERY_PORT=9090
export BEARDOG_BIND_ADDRESS=127.0.0.1
export BEARDOG_OPERATION_TIMEOUT_SECS=30

# Paths
export BEARDOG_CONFIG_DIR=~/.config/beardog
export BEARDOG_DATA_DIR=~/.local/share/beardog
export BEARDOG_LOG_DIR=~/.local/share/beardog/logs
export BEARDOG_PKCS11_LIBRARY=/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so

# Limits
export BEARDOG_MAX_RETRIES=3
export BEARDOG_BUFFER_SIZE=8192
export BEARDOG_MAX_CONNECTIONS=100

# Crypto
export BEARDOG_RSA_KEY_SIZE=2048
export BEARDOG_EC_CURVE=P-256
export BEARDOG_HASH_ALGORITHM=SHA3-256

# Hardware
export BEARDOG_AUTO_DETECT_HARDWARE=true
export BEARDOG_PREFER_HARDWARE=true

# SoftHSM
export SOFTHSM2_CONF=~/.config/softhsm2/softhsm2.conf

# Logging
export BEARDOG_LOG_LEVEL=info
export RUST_LOG=beardog=debug  # Standard Rust logging
```

---

## 📋 IMPLEMENTATION CHECKLIST

### **Phase 1: Foundation (Week 1)**

- [ ] Create `beardog-config` crate
  - [ ] Define all config structs
  - [ ] Implement hierarchy (file → env → args)
  - [ ] Add validation
  - [ ] Add serialization (TOML, JSON, YAML)

- [ ] Create `configs/beardog-config-template.toml`
  - [ ] Document all options
  - [ ] Provide sensible defaults
  - [ ] Include comments/examples

- [ ] Update CLI to use config system
  - [ ] Remove all hardcoded values
  - [ ] Add `--config` flag
  - [ ] Environment variable support

### **Phase 2: Systematic Replacement (Week 2)**

- [ ] Network configuration
  - [ ] Replace hardcoded ports
  - [ ] Replace hardcoded IPs
  - [ ] Replace hardcoded timeouts
  - [ ] Replace hardcoded URLs

- [ ] Path configuration
  - [ ] Replace hardcoded library paths
  - [ ] Replace hardcoded data directories
  - [ ] Replace hardcoded log paths
  - [ ] Implement path discovery

- [ ] Limits configuration
  - [ ] Replace hardcoded timeouts
  - [ ] Replace hardcoded retry counts
  - [ ] Replace hardcoded buffer sizes
  - [ ] Replace hardcoded connection limits

### **Phase 3: Validation & Testing (Week 3)**

- [ ] Configuration validation
  - [ ] Type checking
  - [ ] Range checking
  - [ ] Dependency checking
  - [ ] Error messages

- [ ] Testing
  - [ ] Unit tests for config loading
  - [ ] Integration tests with different configs
  - [ ] Environment variable tests
  - [ ] Default value tests

- [ ] Documentation
  - [ ] Configuration guide
  - [ ] Environment variable reference
  - [ ] Migration guide (old → new)
  - [ ] Examples for common scenarios

---

## 🎯 SUCCESS CRITERIA

### **Zero Hardcoding Goals**

| Category | Current | Target | Priority |
|----------|---------|--------|----------|
| **Network** | 80 instances | 0 | HIGH |
| **Paths** | 40 instances | 0 | HIGH |
| **Timeouts** | 45 instances | 0 | MEDIUM |
| **Test Constants** | 46 instances | Allowed | LOW |

### **Configuration Coverage**

- ✅ 100% of network values configurable
- ✅ 100% of paths configurable
- ✅ 100% of limits configurable
- ✅ 100% of crypto parameters configurable (with secure defaults)
- ✅ Zero hardcoded values in production code

### **User Experience**

- ✅ Works with zero configuration (sensible defaults)
- ✅ Easy to override any value (env vars or config file)
- ✅ Clear error messages for invalid config
- ✅ Auto-detection of platform-specific paths
- ✅ Migration path from old hardcoded values

---

## 💡 BEST PRACTICES

### **1. Secure Defaults**
```rust
// Use secure defaults that work out of the box
impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),  // ✅ Secure default
            // NOT 0.0.0.0 (promiscuous)
        }
    }
}
```

### **2. Validation**
```rust
impl NetworkConfig {
    pub fn validate(&self) -> Result<()> {
        if self.api_port == 0 {
            return Err(ConfigError::InvalidPort("API port cannot be 0"));
        }
        if self.api_port == self.admin_port {
            return Err(ConfigError::PortConflict("API and admin ports must differ"));
        }
        Ok(())
    }
}
```

### **3. Clear Documentation**
```toml
# API server bind address
# Default: 127.0.0.1 (localhost only - secure)
# Set to 0.0.0.0 to accept connections from any interface (less secure)
bind_address = "127.0.0.1"
```

### **4. Migration Support**
```rust
// Support old config keys for backward compatibility
#[serde(alias = "api_server_port")]  // Old name
pub api_port: u16,  // New name
```

---

## 🐻 BOTTOM LINE

### **Current State**
- ⚠️ 211 hardcoded values remaining
- ⚠️ 45% reduction from original 472
- ⚠️ Still requires manual configuration
- ⚠️ Not deployment-friendly

### **Target State**
- ✅ **ZERO** hardcoded values in production
- ✅ Works with zero configuration
- ✅ Easy to customize (env vars or file)
- ✅ Auto-detects platform specifics
- ✅ Deployment-friendly

### **Timeline**
- **Week 1**: Configuration system foundation
- **Week 2**: Systematic hardcoding removal
- **Week 3**: Testing and documentation
- **Total**: 3 weeks to zero hardcoding

### **Philosophy**
> **"Configuration over Convention, but Convention when Sensible"**
> 
> - Provide sensible defaults that work
> - Allow configuration of everything
> - Auto-detect what can be detected
> - Make the right thing easy, everything possible

---

**Last Updated**: November 1, 2025  
**Next Review**: Week 1 completion  
**Owner**: BearDog Core Team  

🐻🚫 **BearDog: Zero Hardcoding, Maximum Flexibility!**

