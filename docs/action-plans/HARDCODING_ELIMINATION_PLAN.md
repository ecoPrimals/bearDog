# 🔧 HARDCODING ELIMINATION PLAN
## Environment-Driven Configuration

**Current State**: 342 hardcoded IPs/ports across 101 files  
**Production Hardcoding**: ~170 instances (excluding tests)  
**Target**: 0 hardcoded configuration  
**Timeline**: 6 weeks (parallel with other work)

---

## 📊 CURRENT ANALYSIS

### **Total Breakdown**
- Total instances: 342
- Test files: ~170 (50%) ✅ **Acceptable in tests**
- Production code: ~172 (50%) ⚠️ **Must fix**

### **By Type**

**IP Addresses**: ~180 instances
- `localhost`: ~90
- `127.0.0.1`: ~70
- `0.0.0.0`: ~20

**Ports**: ~162 instances
- `8080`: ~40 (common default)
- `8081`: ~25 (ToadStool)
- `8082`: ~20 (Songbird)
- `3000`: ~15 (API server)
- `5432`: ~12 (PostgreSQL)
- `6379`: ~10 (Redis)
- `9090`: ~10 (Metrics)
- `27017`: ~8 (MongoDB)
- Other: ~22

---

## 🎯 PRIORITY FILES

### **Critical (Must Fix Immediately)**

#### 1. **`runtime_config.rs`** - 16 instances 🚨
**Location**: `crates/beardog-types/src/canonical/config/runtime_config.rs`

**Issues**:
- Hardcoded default ports
- Hardcoded localhost addresses
- Fallback values that should come from environment

**Fix Strategy**:
```rust
// BEFORE
pub const DEFAULT_API_PORT: u16 = 8080;
pub const DEFAULT_METRICS_PORT: u16 = 9090;
pub const DEFAULT_HOST: &str = "localhost";

// AFTER
use std::env;

fn default_api_port() -> u16 {
    env::var("BEARDOG_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080)  // Keep as last-resort fallback
}

fn default_host() -> String {
    env::var("BEARDOG_HOST")
        .unwrap_or_else(|_| "localhost".to_string())
}
```

#### 2. **`constants/domains/network.rs`** - 20 instances 🚨
**Location**: `crates/beardog-types/src/constants/domains/network.rs`

**Issues**:
- Primal service ports hardcoded
- **Conflicts with "infant discovery" specification**
- Should use service discovery, not hardcoded ports

**Fix Strategy**:
```rust
// BEFORE (VIOLATES INFANT DISCOVERY)
pub const TOADSTOOL_PORT: u16 = 8081;
pub const SONGBIRD_PORT: u16 = 8082;
pub const SQUIRREL_PORT: u16 = 8083;

// AFTER (ENVIRONMENT-DRIVEN WITH DISCOVERY FALLBACK)
pub fn primal_port(service: &str) -> Option<u16> {
    // Try environment first
    let env_key = format!("{}_PORT", service.to_uppercase());
    if let Ok(port_str) = env::var(&env_key) {
        if let Ok(port) = port_str.parse() {
            return Some(port);
        }
    }
    
    // Use service discovery
    None  // Force discovery, no hardcoded fallback
}
```

#### 3. **`env_config.rs`** - 11 instances ⚠️
**Location**: `crates/beardog-utils/src/env_config.rs`

**Issues**:
- Default values hardcoded
- Should provide environment variable names as constants

**Fix Strategy**:
```rust
// BEFORE
let db_url = env::var("DATABASE_URL")
    .unwrap_or_else(|_| "postgres://localhost:5432/beardog".to_string());

// AFTER
pub const DATABASE_URL_ENV: &str = "DATABASE_URL";
pub const DATABASE_PORT_ENV: &str = "DATABASE_PORT";

let db_url = env::var(DATABASE_URL_ENV)
    .map_err(|_| BearDogError::config("DATABASE_URL not set - required for production"))?;
```

---

## 🎯 MIGRATION STRATEGY

### **Phase 1: Critical Configuration (Week 1-2)**

**Week 1: Core Configuration**
- [ ] Fix `runtime_config.rs` (16 instances)
- [ ] Fix `constants/domains/network.rs` (20 instances)
- [ ] Fix `env_config.rs` (11 instances)
- [ ] Create environment variable documentation

**Week 2: Network Configuration**
- [ ] Fix `network.rs` (11 instances)
- [ ] Fix `network_discovery.rs` (11 instances)
- [ ] Fix `discovery.rs` configuration (2 instances)
- [ ] Update service discovery to use environment

---

### **Phase 2: Service Configuration (Week 3-4)**

**Focus**: Adapter and service endpoint configuration

- [ ] Fix adapter configurations
- [ ] Fix universal adapter endpoints
- [ ] Fix primal capability adapter
- [ ] Remove hardcoded service URLs

**Files**:
```
crates/beardog-adapters/src/universal/
crates/beardog-core/src/ecosystem_integration/
crates/beardog-node-registry/src/node_registry/types/config/
```

---

### **Phase 3: HSM & Security (Week 5)**

**Focus**: HSM provider endpoints

- [ ] Fix HSM provider URLs
- [ ] Fix cloud provider endpoints
- [ ] Fix PKCS#11 library paths
- [ ] Environment-driven HSM configuration

---

### **Phase 4: Final Cleanup (Week 6)**

**Focus**: Remaining instances and validation

- [ ] Audit all remaining hardcoding
- [ ] Validate all environment variables documented
- [ ] Create .env.example file
- [ ] Update deployment documentation

---

## 📋 ENVIRONMENT VARIABLE STRATEGY

### **Naming Convention**

```bash
# Format: BEARDOG_{DOMAIN}_{SETTING}

# Core
BEARDOG_HOST=localhost
BEARDOG_API_PORT=8080
BEARDOG_METRICS_PORT=9090

# Database
BEARDOG_DATABASE_URL=postgres://localhost:5432/beardog
BEARDOG_DATABASE_PORT=5432
BEARDOG_DATABASE_MAX_CONNECTIONS=100

# Redis
BEARDOG_REDIS_URL=redis://localhost:6379
BEARDOG_REDIS_PORT=6379

# Service Discovery
BEARDOG_DISCOVERY_ENABLED=true
BEARDOG_DISCOVERY_TIMEOUT_MS=5000

# Primal Services (for discovery fallback)
TOADSTOOL_HOST=localhost
TOADSTOOL_PORT=8081
SONGBIRD_HOST=localhost
SONGBIRD_PORT=8082
SQUIRREL_HOST=localhost
SQUIRREL_PORT=8083

# HSM Configuration
BEARDOG_HSM_PROVIDER=software
BEARDOG_HSM_PKCS11_LIB=/usr/lib/softhsm/libsofthsm2.so

# Security
BEARDOG_JWT_SECRET_KEY=<secret>
BEARDOG_ENCRYPTION_KEY=<secret>

# Monitoring
BEARDOG_PROMETHEUS_PORT=9090
BEARDOG_GRAFANA_URL=http://localhost:3000
```

### **Configuration File** `.env.example`

```bash
# BearDog Environment Configuration
# Copy to .env and customize for your environment

# ===================================================================
# CORE CONFIGURATION
# ===================================================================

BEARDOG_HOST=localhost
BEARDOG_API_PORT=8080
BEARDOG_METRICS_PORT=9090
BEARDOG_LOG_LEVEL=info

# ===================================================================
# DATABASE CONFIGURATION
# ===================================================================

BEARDOG_DATABASE_URL=postgres://localhost:5432/beardog
# Or individual components:
# BEARDOG_DATABASE_HOST=localhost
# BEARDOG_DATABASE_PORT=5432
# BEARDOG_DATABASE_NAME=beardog
# BEARDOG_DATABASE_USER=beardog
# BEARDOG_DATABASE_PASSWORD=secret

# ===================================================================
# REDIS CONFIGURATION
# ===================================================================

BEARDOG_REDIS_URL=redis://localhost:6379
# Or:
# BEARDOG_REDIS_HOST=localhost
# BEARDOG_REDIS_PORT=6379

# ===================================================================
# SERVICE DISCOVERY
# ===================================================================

# Enable/disable service discovery
BEARDOG_DISCOVERY_ENABLED=true
BEARDOG_DISCOVERY_TIMEOUT_MS=5000

# Discovery methods (comma-separated)
BEARDOG_DISCOVERY_METHODS=mdns,http,env

# ===================================================================
# PRIMAL SERVICES
# (Used as fallback if discovery fails)
# ===================================================================

TOADSTOOL_HOST=localhost
TOADSTOOL_PORT=8081

SONGBIRD_HOST=localhost
SONGBIRD_PORT=8082

SQUIRREL_HOST=localhost
SQUIRREL_PORT=8083

# ===================================================================
# HSM CONFIGURATION
# ===================================================================

BEARDOG_HSM_PROVIDER=software
# For PKCS#11:
# BEARDOG_HSM_PKCS11_LIB=/usr/lib/softhsm/libsofthsm2.so
# BEARDOG_HSM_PKCS11_SLOT=0

# For cloud providers:
# BEARDOG_HSM_AWS_REGION=us-east-1
# BEARDOG_HSM_AZURE_VAULT=my-vault

# ===================================================================
# SECURITY
# ===================================================================

BEARDOG_JWT_SECRET_KEY=<generate-a-secure-key>
BEARDOG_ENCRYPTION_KEY=<generate-a-secure-key>

# ===================================================================
# MONITORING
# ===================================================================

BEARDOG_PROMETHEUS_PORT=9090
BEARDOG_GRAFANA_URL=http://localhost:3000

# ===================================================================
# DEPLOYMENT
# ===================================================================

BEARDOG_ENVIRONMENT=development  # development, staging, production
BEARDOG_NAMESPACE=default
```

---

## 🔍 AUTOMATED DETECTION

### **Find All Hardcoded Values**

```bash
#!/bin/bash
# find_hardcoding.sh

echo "Finding hardcoded IPs and ports..."

# Find IPs
grep -rE "(localhost|127\.0\.0\.1|0\.0\.0\.0)" crates/ \
  --include="*.rs" \
  --exclude-dir="tests" \
  | grep -v "test\|example\|doc" \
  > hardcoded_ips.txt

# Find common ports
grep -rE ":(8080|8081|8082|3000|5432|6379|9090|27017)\b" crates/ \
  --include="*.rs" \
  --exclude-dir="tests" \
  | grep -v "test\|example\|doc" \
  > hardcoded_ports.txt

echo "Results saved to hardcoded_*.txt"
```

---

## 📊 TRACKING TEMPLATE

```markdown
## Week [N]: [DATE RANGE]

### Hardcoded Values Fixed
- IPs: [number]
- Ports: [number]
- URLs: [number]
- Total: [number]

### Files Modified
1. [file] - [description]
2. [file] - [description]

### Environment Variables Added
- BEARDOG_[VAR]: [description]
- BEARDOG_[VAR]: [description]

### Remaining
- Production hardcoding: [number]
- Percentage complete: X%

### Documentation Updated
- [ ] .env.example
- [ ] Deployment docs
- [ ] Configuration guide
```

---

## ✅ VALIDATION

### **After Each Change**

1. **Check no hardcoded values remain**:
```bash
grep -E "localhost|127\.0\.0\.1|:8080|:8081" <file>
```

2. **Test with environment variables**:
```bash
export BEARDOG_API_PORT=9999
cargo test <relevant_test>
```

3. **Test without environment variables** (should have sensible defaults or fail gracefully):
```bash
unset BEARDOG_API_PORT
cargo test <relevant_test>
```

4. **Update documentation**:
- Add variable to .env.example
- Document in configuration guide

---

## 🎯 SUCCESS METRICS

### **Weekly Targets**
- Week 1: 50 instances fixed
- Week 2: 40 instances fixed
- Week 3: 30 instances fixed
- Week 4: 25 instances fixed
- Week 5: 20 instances fixed
- Week 6: 7 instances fixed + cleanup

### **Final Validation**
```bash
# Should return only test files
grep -rE "localhost|127\.0\.0\.1|:8080" crates/ \
  --include="*.rs" \
  --exclude-dir="tests"
```

---

## 🚀 QUICK WINS (This Week)

### **Easy Fixes** (3 hours, 30-40 instances)

1. **Environment variable defaults** - Change const to fn
2. **Port definitions** - Use env::var with fallback
3. **Host addresses** - Environment-driven

**Start with**:
```rust
// crates/beardog-types/src/canonical/config/runtime_config.rs
// Change all pub const to pub fn with env::var
```

---

**Created**: October 21, 2025  
**Status**: Ready to Execute  
**First Fixes**: This week (top 50 instances)  
**Next Review**: October 27, 2025

Configuration should be environment-driven, not code-driven! ⚙️✨

