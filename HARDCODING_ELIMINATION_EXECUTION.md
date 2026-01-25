# Hardcoding Elimination Strategy - Execution Plan
**Date**: January 25, 2026  
**Priority**: 🔴 HIGH  
**Target**: Zero hardcoded values

---

## 🎯 STRATEGY OVERVIEW

### Philosophy
**Instead of**: Hardcoded constants scattered everywhere  
**Use**: Runtime configuration + platform discovery + capability-based resolution

### Three-Layer Approach
1. **Config Layer**: User-configurable values (ports, IPs, paths)
2. **Discovery Layer**: Platform-aware auto-detection
3. **Resolution Layer**: Runtime capability-based service discovery

---

## 📊 CURRENT STATE (Audit Results)

### Hardcoded Values Found
- **838 IP addresses** (127.0.0.1, localhost, etc.)
- **139 port numbers** (:8080, :9090, etc.)
- **~40 file paths** (library paths, config paths)
- **~45 timeouts** (Duration::from_secs hardcoded)

### Categories by Priority

#### 🔴 CRITICAL (Week 1)
**Network Endpoints** - 838 IPs, 139 ports
- Most pervasive
- Breaks deployment flexibility
- Prevents capability-based discovery

#### 🟡 HIGH (Week 2)
**Socket Paths** - /tmp/beardog.sock → /primal/beardog
- IPC protocol compliance
- Interprimal communication

#### 🟢 MEDIUM (Week 3)
**File Paths** - Platform-specific library paths
- Cross-platform compatibility
- XDG compliance

#### 🔵 LOW (Week 4)
**Timeouts & Limits** - Durations, retry counts
- Operational flexibility
- Performance tuning

---

## 🚀 EXECUTION PHASES

### Phase 3.1: Network Configuration (Week 1 - Days 1-3)

#### Step 1: Audit Network Hardcoding
```bash
# Find all hardcoded IPs
grep -r "127\.0\.0\.1\|localhost\|0\.0\.0\.0" crates/ --include="*.rs" \
  | grep -v test | grep -v "\.toml" > hardcoded_ips.txt

# Find all hardcoded ports
grep -r ":8080\|:9090\|:3000" crates/ --include="*.rs" \
  | grep -v test > hardcoded_ports.txt
```

#### Step 2: Create NetworkConfig Centralization
```rust
// crates/beardog-config/src/domains/network.rs

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub api: ApiConfig,
    pub discovery: DiscoveryConfig,
    pub admin: AdminConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    #[serde(default = "default_api_host")]
    pub host: IpAddr,
    
    #[serde(default = "default_api_port")]
    pub port: u16,
    
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
}

fn default_api_host() -> IpAddr {
    // Check environment first
    std::env::var("BEARDOG_API_HOST")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST))
}

fn default_api_port() -> u16 {
    std::env::var("BEARDOG_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080)
}

fn default_timeout_secs() -> u64 {
    std::env::var("BEARDOG_TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30)
}
```

#### Step 3: Replace Hardcoded Values
```rust
// ❌ Before: Hardcoded
let addr = "127.0.0.1:8080";
let timeout = Duration::from_secs(30);

// ✅ After: Config-driven
use beardog_config::global::BEARDOG_CONFIG;
let host = BEARDOG_CONFIG.network.api.host;
let port = BEARDOG_CONFIG.network.api.port;
let addr = format!("{}:{}", host, port);
let timeout = Duration::from_secs(BEARDOG_CONFIG.network.api.timeout_secs);
```

### Phase 3.2: Socket Path Migration (Week 1 - Days 4-5)

#### Step 1: Update Socket Paths
```rust
// ❌ Before: Custom paths
pub const SOCKET_PATH: &str = "/tmp/beardog.sock";
pub const BEARDOG_SOCKET: &str = "/tmp/beardog-nat0.sock";

// ✅ After: Primal IPC Protocol standard
pub const SOCKET_PATH: &str = "/primal/beardog";

// For family-specific instances
pub fn family_socket_path(family_id: &str) -> String {
    format!("/primal/beardog-{}", family_id)
}
```

#### Step 2: Update Server Binding
```rust
// crates/beardog-tunnel/src/unix_socket_ipc/server.rs

// ❌ Before
let listener = UnixListener::bind("/tmp/beardog.sock").await?;

// ✅ After
use beardog_ipc::BEARDOG_SOCKET_PATH;
let listener = UnixListener::bind(BEARDOG_SOCKET_PATH).await?;
```

#### Step 3: Update Client Connections
```rust
// All client code

// ❌ Before
let stream = UnixStream::connect("/tmp/beardog.sock").await?;

// ✅ After: Use discovery!
let client = SongbirdClient::connect().await?;
let beardog = client.resolve("beardog").await?;
let stream = UnixStream::connect(&beardog.endpoint).await?;
```

### Phase 3.3: Capability-Based Discovery (Week 2)

#### Replace Direct Connections with Discovery
```rust
// ❌ Before: Hardcoded primal knowledge
pub async fn get_crypto_service() -> Result<CryptoClient> {
    let client = connect_to_beardog("localhost:8080").await?;
    Ok(CryptoClient::new(client))
}

// ✅ After: Capability-based
pub async fn get_crypto_service() -> Result<CryptoClient> {
    let songbird = SongbirdClient::connect().await?;
    let services = songbird.find_capability("crypto").await?;
    
    let service = services.first()
        .ok_or_else(|| Error::ServiceNotFound("crypto"))?;
    
    let stream = UnixStream::connect(&service.endpoint).await?;
    Ok(CryptoClient::new(stream))
}
```

### Phase 3.4: Platform-Aware Paths (Week 3)

#### Step 1: XDG Base Directory Support
```rust
// crates/beardog-config/src/paths.rs

use std::path::PathBuf;

pub struct PathConfig {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub runtime_dir: PathBuf,
}

impl PathConfig {
    pub fn new() -> Self {
        Self {
            config_dir: Self::config_dir(),
            data_dir: Self::data_dir(),
            cache_dir: Self::cache_dir(),
            runtime_dir: Self::runtime_dir(),
        }
    }
    
    fn config_dir() -> PathBuf {
        // 1. Environment variable
        if let Ok(dir) = std::env::var("BEARDOG_CONFIG_DIR") {
            return PathBuf::from(dir);
        }
        
        // 2. XDG_CONFIG_HOME
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            return PathBuf::from(xdg).join("beardog");
        }
        
        // 3. Platform default
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("beardog")
    }
    
    // Similar for data_dir, cache_dir, runtime_dir
}
```

#### Step 2: Library Path Discovery
```rust
// crates/beardog-config/src/library_discovery.rs

pub struct LibraryDiscovery;

impl LibraryDiscovery {
    pub fn find_pkcs11_library() -> Option<PathBuf> {
        // 1. Environment variable
        if let Ok(path) = std::env::var("PKCS11_LIBRARY") {
            let p = PathBuf::from(path);
            if p.exists() {
                return Some(p);
            }
        }
        
        // 2. Platform-specific search paths
        let search_paths = match std::env::consts::OS {
            "linux" => vec![
                "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so",
                "/usr/local/lib/opensc-pkcs11.so",
                "/usr/lib/softhsm/libsofthsm2.so",
            ],
            "macos" => vec![
                "/usr/local/lib/opensc-pkcs11.so",
                "/opt/homebrew/lib/opensc-pkcs11.so",
            ],
            "windows" => vec![
                "C:\\Program Files\\OpenSC\\pkcs11\\opensc-pkcs11.dll",
            ],
            _ => vec![],
        };
        
        // 3. Search for first existing path
        search_paths.iter()
            .map(PathBuf::from)
            .find(|p| p.exists())
    }
}
```

---

## 📋 IMPLEMENTATION CHECKLIST

### Week 1: Network & Sockets
- [ ] Day 1: Audit all hardcoded IPs/ports
- [ ] Day 1: Create NetworkConfig structure
- [ ] Day 2: Migrate API server config
- [ ] Day 2: Migrate client config
- [ ] Day 3: Update socket paths to /primal/*
- [ ] Day 3: Test with mock Songbird
- [ ] Day 4: Update all UnixSocket binds
- [ ] Day 4: Update all UnixSocket connects
- [ ] Day 5: Integration testing
- [ ] Day 5: Documentation update

### Week 2: Discovery Integration
- [ ] Day 1: Replace direct connections with discovery
- [ ] Day 2: Add discovery caching
- [ ] Day 3: Handle service unavailability
- [ ] Day 4: Add fallback mechanisms
- [ ] Day 5: End-to-end testing

### Week 3: Path Configuration
- [ ] Day 1: Implement XDG support
- [ ] Day 2: Platform-aware library discovery
- [ ] Day 3: Remove hardcoded paths
- [ ] Day 4: Test on multiple platforms
- [ ] Day 5: Documentation

### Week 4: Timeouts & Limits
- [ ] Day 1: Audit timeout values
- [ ] Day 2: Create TimeoutConfig
- [ ] Day 3: Replace hardcoded timeouts
- [ ] Day 4: Add environment overrides
- [ ] Day 5: Final verification

---

## 🎯 SUCCESS CRITERIA

### Must Have
- ✅ Zero hardcoded IPs in production code
- ✅ Zero hardcoded ports in production code
- ✅ All socket paths use /primal/* namespace
- ✅ Platform-aware path discovery
- ✅ Configuration via env vars or files

### Should Have
- ✅ Runtime service discovery working
- ✅ XDG Base Directory compliance
- ✅ Graceful fallbacks
- ✅ Clear error messages

### Nice to Have
- ⭐ Auto-detection of optimal values
- ⭐ Config validation on startup
- ⭐ Config migration tool

---

## 📊 METRICS TRACKING

### Before
```
Hardcoded IPs:     838 instances
Hardcoded Ports:   139 instances
Socket Paths:      Custom (/tmp/*)
Config System:     Partial
Discovery:         None
Grade:             C+ (60% eliminated)
```

### Target (End of Phase 3)
```
Hardcoded IPs:     0 instances ✅
Hardcoded Ports:   0 instances ✅
Socket Paths:      Standard (/primal/*) ✅
Config System:     Complete ✅
Discovery:         Runtime ✅
Grade:             A (100% eliminated) ✅
```

---

## 💡 GUIDING PRINCIPLES

### 1. Configuration Hierarchy
```
1. Command-line arguments (highest priority)
2. Environment variables
3. Config file
4. Platform detection
5. Sensible defaults (lowest priority)
```

### 2. Fail Fast with Clear Errors
```rust
// ❌ Bad: Silent fallback
let port = config.port.unwrap_or(8080);

// ✅ Good: Clear error
let port = config.port
    .ok_or_else(|| ConfigError::missing("API port not configured"))?;
```

### 3. Document Defaults
```toml
# configs/beardog-config-template.toml

[network.api]
# API server port
# Default: 8080
# Environment: BEARDOG_API_PORT
port = 8080
```

---

**Status**: 🟢 READY TO EXECUTE  
**Next**: Begin network config migration  
**Timeline**: 4 weeks to zero hardcoding

🐻🐕 **BearDog: From hardcoded to dynamic!** ✨

