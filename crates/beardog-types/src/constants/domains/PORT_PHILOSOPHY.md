# 🔌 Port Configuration Philosophy
**BearDog Port Management Strategy**

---

## 🎯 Core Principle

> **"BearDog-configurable ports are environment-driven. Industry-standard ports remain as compile-time constants."**

---

## 📊 Port Categories

### 1. ✅ **Environment-Driven Ports** (BearDog-Specific)
**Pattern:** Functions that check environment variables first, fallback to sensible defaults

#### BearDog Service Ports:
```rust
// ✅ CORRECT: Environment-aware
pub fn default_api_port() -> u16 {
    std::env::var("BEARDOG_API_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}

// ❌ WRONG: Hardcoded constant
pub const API_PORT: u16 = 8080;
```

**Covered Ports:**
- `default_api_port()` → `BEARDOG_API_PORT` (default: 8080)
- `default_metrics_port()` → `BEARDOG_METRICS_PORT` (default: 9190)
- `default_health_port()` → `BEARDOG_HEALTH_PORT` (default: 8081)
- `default_admin_port()` → `BEARDOG_ADMIN_PORT` (default: 8082)
- `default_debug_port()` → `BEARDOG_DEBUG_PORT` (default: 8083)
- `default_node_discovery_port()` → `BEARDOG_NODE_DISCOVERY_PORT` (default: 8090)
- `default_cluster_port()` → `BEARDOG_CLUSTER_PORT` (default: 8091)

**Deprecated Legacy Constants:**
All corresponding `DEFAULT_*_PORT` constants are deprecated with migration notes pointing to the functions.

---

### 2. ✅ **Compile-Time Constants** (Industry Standards)
**Pattern:** Well-known ports that never change

```rust
// ✅ CORRECT: Industry standard ports
pub const HTTP_PORT: u16 = 80;
pub const HTTPS_PORT: u16 = 443;
pub const SSH_PORT: u16 = 22;
pub const DNS_PORT: u16 = 53;
pub const POSTGRES_PORT: u16 = 5432;
```

**Why:** These are international standards (RFC, IANA) that applications expect to be fixed.

**Covered Ports:**
- `HTTP_PORT` (80) - RFC 7230
- `HTTPS_PORT` (443) - RFC 2818
- `SSH_PORT` (22) - RFC 4251
- `FTP_PORT` (21) - RFC 959
- `SMTP_PORT` (25) - RFC 5321
- `DNS_PORT` (53) - RFC 1035
- `DHCP_SERVER_PORT` (67) - RFC 2131
- `DHCP_CLIENT_PORT` (68) - RFC 2131
- `SNMP_PORT` (161) - RFC 3411
- `SYSLOG_PORT` (514) - RFC 3164

---

### 3. ✅ **Port Range Categories** (Classification Constants)
**Pattern:** Categorization, not actual port assignments

```rust
// ✅ CORRECT: Port range definitions
pub const WELL_KNOWN_PORT_MIN: u16 = 1;
pub const WELL_KNOWN_PORT_MAX: u16 = 1023;
pub const REGISTERED_PORT_MIN: u16 = 1024;
pub const REGISTERED_PORT_MAX: u16 = 49151;
pub const DYNAMIC_PORT_MIN: u16 = 49152;
pub const DYNAMIC_PORT_MAX: u16 = 65535;
```

**Why:** These define IANA port categories, not specific service assignments.

---

## 🚫 Anti-Patterns to Avoid

### ❌ **Hardcoded Service Ports**
```rust
// ❌ BAD: Service-specific hardcoded port
let api_url = format!("http://localhost:8080/api");

// ✅ GOOD: Environment-driven
let api_port = beardog_types::constants::domains::network::defaults::default_api_port();
let api_url = format!("http://localhost:{}/api", api_port);

// ✅ BETTER: Use RuntimeNetworkConfig
use beardog_types::canonical::config::runtime_config::RuntimeNetworkConfig;
let config = RuntimeNetworkConfig::from_env();
let api_url = format!("http://{}:{}/api", config.api_host, config.api_port);
```

### ❌ **Inline Port Numbers**
```rust
// ❌ BAD: Magic number
server.bind("0.0.0.0:9090")?;

// ✅ GOOD: Named function
let metrics_port = default_metrics_port();
server.bind(format!("0.0.0.0:{}", metrics_port))?;

// ✅ BETTER: RuntimeNetworkConfig
let config = RuntimeNetworkConfig::from_env();
server.bind(format!("{}:{}", config.api_host, config.metrics_port))?;
```

---

## 📚 Migration Guide

### For Application Code:

#### Old Pattern:
```rust
use beardog_types::constants::domains::network::defaults::DEFAULT_API_PORT;
let url = format!("http://localhost:{}/health", DEFAULT_API_PORT);
```

#### New Pattern (Simple):
```rust
use beardog_types::constants::domains::network::defaults::default_api_port;
let url = format!("http://localhost:{}/health", default_api_port());
```

#### New Pattern (Comprehensive):
```rust
use beardog_types::canonical::config::runtime_config::RuntimeNetworkConfig;
let config = RuntimeNetworkConfig::from_env();
let url = format!("http://{}:{}/health", config.api_host, config.health_port);
```

---

## 🎯 Decision Tree: "Should this be a constant or function?"

```
Is this port number for a BearDog service?
├─ YES → Use environment-aware function
│         Example: default_api_port()
│         Reason: Deployments need different ports
│
└─ NO → Is it an industry-standard well-known port?
         ├─ YES → Use compile-time constant
         │         Example: HTTPS_PORT = 443
         │         Reason: International standard
         │
         └─ NO → Is it a port range/category definition?
                  ├─ YES → Use compile-time constant
                  │         Example: WELL_KNOWN_PORT_MAX = 1023
                  │         Reason: Classification, not assignment
                  │
                  └─ NO → Use RuntimeNetworkConfig or environment function
                            Example: std::env::var("CUSTOM_PORT")
                            Reason: Runtime configuration
```

---

## 🔧 Environment Variables

### Primary Configuration:
```bash
# BearDog service ports
export BEARDOG_API_PORT=8080
export BEARDOG_METRICS_PORT=9090
export BEARDOG_HEALTH_PORT=8081
export BEARDOG_ADMIN_PORT=8082
export BEARDOG_DEBUG_PORT=8083

# Node clustering
export BEARDOG_NODE_DISCOVERY_PORT=8090
export BEARDOG_CLUSTER_PORT=8091

# Network binding
export BEARDOG_BIND_ADDRESS=0.0.0.0
export BEARDOG_API_HOST=127.0.0.1
export BEARDOG_ENABLE_TLS=true
```

### For Testing:
```bash
# Use high ephemeral ports to avoid conflicts
export BEARDOG_API_PORT=50001
export BEARDOG_METRICS_PORT=50002
export BEARDOG_HEALTH_PORT=50003
```

---

## 📈 Benefits

### 1. **Deployment Flexibility**
```bash
# Development
BEARDOG_API_PORT=8080

# Staging  
BEARDOG_API_PORT=8888

# Production
BEARDOG_API_PORT=443

# Same code, different environments!
```

### 2. **Multi-Instance Deployment**
```bash
# Instance 1
BEARDOG_API_PORT=8080 beardog &

# Instance 2
BEARDOG_API_PORT=8081 beardog &

# No port conflicts!
```

### 3. **Kubernetes/Docker Friendly**
```yaml
env:
  - name: BEARDOG_API_PORT
    valueFrom:
      configMapKeyRef:
        name: beardog-config
        key: api.port
```

### 4. **Testing Without Conflicts**
```rust
#[test]
fn test_api_server() {
    std::env::set_var("BEARDOG_API_PORT", "0"); // OS assigns port
    let server = start_server();
    // No conflicts with other tests!
}
```

---

## ✅ Current Status

### Fully Migrated (Environment-Aware):
- ✅ `default_api_port()` 
- ✅ `default_metrics_port()`
- ✅ `default_health_port()`
- ✅ `default_admin_port()`
- ✅ `default_debug_port()`
- ✅ `default_node_discovery_port()` (NEW)
- ✅ `default_cluster_port()` (NEW)

### Correctly Kept as Constants:
- ✅ Well-known ports (HTTP, HTTPS, SSH, etc.)
- ✅ Port ranges (WELL_KNOWN, REGISTERED, DYNAMIC)

### Deprecated (Migration Path Provided):
- ⚠️ `DEFAULT_API_PORT` → use `default_api_port()`
- ⚠️ `DEFAULT_METRICS_PORT` → use `default_metrics_port()`
- ⚠️ `DEFAULT_HEALTH_PORT` → use `default_health_port()`
- ⚠️ `DEFAULT_ADMIN_PORT` → use `default_admin_port()`
- ⚠️ `DEFAULT_DEBUG_PORT` → use `default_debug_port()`
- ⚠️ `DEFAULT_NODE_DISCOVERY_PORT` → use `default_node_discovery_port()`
- ⚠️ `DEFAULT_CLUSTER_PORT` → use `default_cluster_port()`

---

## 🎓 Philosophy Summary

### The Three Rules:

1. **BearDog Services** → Environment-driven functions
   - API, Metrics, Health, Admin, Debug, Node, Cluster
   - Configurable via `BEARDOG_*_PORT` variables
   - Fallback to sensible defaults

2. **Industry Standards** → Compile-time constants
   - HTTP (80), HTTPS (443), SSH (22), etc.
   - Never change, defined by RFCs/IANA
   - Reference-only, not for binding

3. **Port Categories** → Compile-time constants
   - Well-known (1-1023), Registered (1024-49151), etc.
   - Classification, not assignment
   - Used for validation logic

---

## 🚀 Next Steps for Developers

### When Adding a New Port:

1. **Ask:** Is this a BearDog service?
   - YES → Create environment-aware function
   - NO → Proceed to step 2

2. **Ask:** Is this an industry standard?
   - YES → Use existing constant or add with RFC reference
   - NO → Use `RuntimeNetworkConfig` or custom env var

3. **Ask:** Does this need runtime configuration?
   - YES → Use `RuntimeNetworkConfig` or environment function
   - NO → Reconsider if it's truly a constant

### Example: Adding WebSocket Port

```rust
// Step 1: Add fallback constant (private)
const FALLBACK_WS_PORT: u16 = 3000;

// Step 2: Add environment-aware function
/// Get default WebSocket port from environment or fallback to 3000
///
/// Checks `BEARDOG_WS_PORT` environment variable first.
#[must_use]
pub fn default_ws_port() -> u16 {
    std::env::var("BEARDOG_WS_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(FALLBACK_WS_PORT)
}

// Step 3: Deprecate any legacy constant
#[deprecated(
    since = "3.1.0",
    note = "Use default_ws_port() for environment-aware configuration"
)]
pub const DEFAULT_WS_PORT: u16 = FALLBACK_WS_PORT;

// Step 4: Add to RuntimeNetworkConfig
pub struct RuntimeNetworkConfig {
    // ...
    pub ws_port: u16,  // from BEARDOG_WS_PORT
}
```

---

**Status:** ✅ **Port Philosophy Complete**  
**Environment-Aware Ports:** 7 (API, Metrics, Health, Admin, Debug, Node Discovery, Cluster)  
**Standard Ports:** 11 (HTTP, HTTPS, SSH, FTP, SMTP, DNS, DHCP×2, SNMP, Syslog, PostgreSQL)  
**Port Categories:** 6 (ranges for well-known, registered, dynamic ports)

🐻 **"Configurable where needed, constant where standard."** 🔐

---

*Last Updated: October 29, 2025*  
*Next Review: When adding new BearDog services*

