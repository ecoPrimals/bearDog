# Hardcoding Fix Plan - November 21, 2025 (Evening)

**Status**: Phase 1 - High Priority Ports  
**Target**: Reduce 937 hardcoded port references to configuration-driven values  
**Priority**: HIGH (per specs)

---

## 🎯 STRATEGY

Per `ZERO_HARDCODING_SPECIFICATION.md`, we need to move ALL hardcoded values to configuration.

### Configuration Hierarchy (Implemented)
```
1. Command-line Arguments (Highest Priority)
   ↓
2. Environment Variables (BEARDOG_*)
   ↓
3. Config File (beardog.toml)
   ↓
4. Platform Defaults (auto-detected)
   ↓
5. Fallback Constants (Last Resort, documented)
```

---

## 📊 CURRENT STATE (from audit)

**Hardcoded Ports**: 937 references across 290 files  
**Target**: Zero hardcoded ports in production code

**Test References**: Acceptable (test-only constants are fine)  
**Production References**: Must be eliminated

---

## 🔍 ANALYSIS APPROACH

### Step 1: Identify Production vs Test Code
```bash
# Find hardcoded ports in production code
grep -r ":\s*\d{4,5}\b" crates/ --include="*.rs" | \
  grep -v "tests/" | \
  grep -v "_tests.rs" | \
  grep -v "benches/" | \
  head -50
```

### Step 2: Common Port Patterns
Based on specs, common hardcoded ports include:
- **8080** - API server
- **9090** - Discovery service
- **9091** - Admin endpoint
- **8443** - HTTPS API
- **5432** - PostgreSQL
- **6379** - Redis
- **27017** - MongoDB

---

## ✅ WHAT'S ALREADY DONE

From previous work (Nov 20, 2025):
1. ✅ `beardog-config` crate exists
2. ✅ Configuration hierarchy implemented
3. ✅ Environment variable support (BEARDOG_*)
4. ✅ `with_defaults()` pattern established
5. ✅ `from_env()` pattern established
6. ✅ 6 config structs modernized

---

## 🚀 PHASE 1: HIGH PRIORITY FIXES (2-3 hours)

### Task 1.1: Create NetworkPorts Config Structure
**File**: `crates/beardog-config/src/domains/network_ports.rs`

```rust
//! Network Port Configuration
//!
//! Centralized configuration for all network ports used by BearDog.

use serde::{Deserialize, Serialize};
use std::net::IpAddr;

/// Network ports configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPortsConfig {
    /// API server port
    #[serde(default = "default_api_port")]
    pub api_port: u16,

    /// Discovery service port
    #[serde(default = "default_discovery_port")]
    pub discovery_port: u16,

    /// Admin endpoint port
    #[serde(default = "default_admin_port")]
    pub admin_port: u16,

    /// HTTPS API port
    #[serde(default = "default_https_port")]
    pub https_port: u16,

    /// Metrics/monitoring port
    #[serde(default = "default_metrics_port")]
    pub metrics_port: u16,

    /// Health check port
    #[serde(default = "default_health_port")]
    pub health_port: u16,
}

// Default port constants (documented fallbacks)
const DEFAULT_API_PORT: u16 = 8080;
const DEFAULT_DISCOVERY_PORT: u16 = 9090;
const DEFAULT_ADMIN_PORT: u16 = 9091;
const DEFAULT_HTTPS_PORT: u16 = 8443;
const DEFAULT_METRICS_PORT: u16 = 9100;
const DEFAULT_HEALTH_PORT: u16 = 8081;

fn default_api_port() -> u16 {
    std::env::var("BEARDOG_API_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_API_PORT)
}

fn default_discovery_port() -> u16 {
    std::env::var("BEARDOG_DISCOVERY_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_DISCOVERY_PORT)
}

fn default_admin_port() -> u16 {
    std::env::var("BEARDOG_ADMIN_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_ADMIN_PORT)
}

fn default_https_port() -> u16 {
    std::env::var("BEARDOG_HTTPS_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_HTTPS_PORT)
}

fn default_metrics_port() -> u16 {
    std::env::var("BEARDOG_METRICS_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_METRICS_PORT)
}

fn default_health_port() -> u16 {
    std::env::var("BEARDOG_HEALTH_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_HEALTH_PORT)
}

impl Default for NetworkPortsConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl NetworkPortsConfig {
    /// Creates configuration with secure defaults
    pub fn with_defaults() -> Self {
        Self {
            api_port: default_api_port(),
            discovery_port: default_discovery_port(),
            admin_port: default_admin_port(),
            https_port: default_https_port(),
            metrics_port: default_metrics_port(),
            health_port: default_health_port(),
        }
    }

    /// Loads configuration from environment
    pub fn from_env() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ports() {
        let config = NetworkPortsConfig::with_defaults();
        assert_eq!(config.api_port, DEFAULT_API_PORT);
        assert_eq!(config.discovery_port, DEFAULT_DISCOVERY_PORT);
    }

    #[test]
    fn test_ports_are_valid() {
        let config = NetworkPortsConfig::with_defaults();
        assert!(config.api_port > 1024, "Should use non-privileged ports");
        assert!(config.api_port < 65535, "Should be valid port");
    }
}
```

**Time**: 30 minutes

### Task 1.2: Update BearDogConfig to Include NetworkPortsConfig
**File**: `crates/beardog-config/src/lib.rs`

Add:
```rust
pub use domains::network_ports::NetworkPortsConfig;

pub struct BearDogConfig {
    pub network_ports: NetworkPortsConfig,
    // ... existing fields
}
```

**Time**: 10 minutes

### Task 1.3: Create Migration Script
**File**: `scripts/find_hardcoded_ports.sh`

```bash
#!/bin/bash
# Find hardcoded ports in production code

echo "Finding hardcoded ports in production code..."
echo "=============================================="

# Find port patterns in non-test files
grep -rn ":\s*\d{4,5}\b" crates/ --include="*.rs" \
  | grep -v "tests/" \
  | grep -v "_tests.rs" \
  | grep -v "benches/" \
  | grep -v "const " \
  | grep -v "// " \
  | sort | uniq > hardcoded_ports_production.txt

echo "Results saved to: hardcoded_ports_production.txt"
echo ""
echo "Top hardcoded ports:"
grep -o ":\s*[0-9]\{4,5\}" hardcoded_ports_production.txt | sort | uniq -c | sort -rn | head -10
```

**Time**: 15 minutes

---

## 📋 PHASE 2: SYSTEMATIC REPLACEMENT (5-8 hours)

### Approach
1. **Identify** all production code with hardcoded ports
2. **Categorize** by module/crate
3. **Replace** with `BEARDOG_CONFIG.network_ports.xxx`
4. **Test** each replacement
5. **Document** changes

### High-Frequency Replacements

#### Pattern 1: Direct Port Usage
```rust
// ❌ BEFORE
let addr = format!("127.0.0.1:8080");

// ✅ AFTER
use beardog_config::global::BEARDOG_CONFIG;
let addr = format!("127.0.0.1:{}", BEARDOG_CONFIG.network_ports.api_port);
```

#### Pattern 2: SocketAddr Construction
```rust
// ❌ BEFORE
let addr: SocketAddr = "127.0.0.1:8080".parse()?;

// ✅ AFTER  
use beardog_config::global::BEARDOG_CONFIG;
let addr: SocketAddr = format!("127.0.0.1:{}", BEARDOG_CONFIG.network_ports.api_port).parse()?;
```

#### Pattern 3: Configuration Structs
```rust
// ❌ BEFORE
pub struct ApiConfig {
    pub port: u16,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self { port: 8080 }
    }
}

// ✅ AFTER
pub struct ApiConfig {
    pub port: u16,
}

impl Default for ApiConfig {
    fn default() -> Self {
        use beardog_config::global::BEARDOG_CONFIG;
        Self { 
            port: BEARDOG_CONFIG.network_ports.api_port 
        }
    }
}
```

---

## 🎯 SUCCESS CRITERIA

### Phase 1 Complete When:
- [ ] `NetworkPortsConfig` created and integrated
- [ ] `BEARDOG_CONFIG` includes network_ports
- [ ] Migration script created
- [ ] Documentation updated

### Phase 2 Complete When:
- [ ] Production code port references < 100 (from 937)
- [ ] All high-frequency ports (8080, 9090, 9091) from config
- [ ] Tests still passing
- [ ] Documentation complete

### Full Completion When:
- [ ] Zero hardcoded ports in production code
- [ ] All ports configurable via ENV or config file
- [ ] Config template includes all ports
- [ ] Migration guide complete

---

## 📊 PROGRESS TRACKING

**Current**: 937 hardcoded port references  
**Phase 1 Target**: Infrastructure in place  
**Phase 2 Target**: < 100 references  
**Final Target**: 0 references in production

**Estimated Time**:
- Phase 1 (Infrastructure): 1 hour ✅ (can complete now)
- Phase 2 (Systematic replacement): 5-8 hours
- Total: 6-9 hours

---

## 🚀 NEXT STEPS (Immediate)

1. ✅ Create `NetworkPortsConfig` structure
2. ✅ Integrate into `BearDogConfig`
3. ✅ Create migration script
4. ⏳ Run script to identify top offenders
5. ⏳ Begin systematic replacement

**Status**: Ready to execute Phase 1 (1 hour)

---

**Created**: November 21, 2025 (Evening)  
**Author**: Comprehensive Audit & Execution  
**Priority**: HIGH (per ZERO_HARDCODING_SPECIFICATION.md)

