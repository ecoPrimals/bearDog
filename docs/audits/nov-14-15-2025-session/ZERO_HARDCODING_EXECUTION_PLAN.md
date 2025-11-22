# 🚀 Zero Hardcoding Execution Plan - Phase 2

**Date**: November 14, 2025  
**Status**: 🟢 **IN PROGRESS**  
**Target**: 492 hardcoded instances → 0  
**Timeline**: 2-3 weeks

---

## 📊 **BASELINE METRICS**

### Current State (Nov 14, 2025):
```
Hardcoded Values:         492 instances
- Network (IP/ports):     492 instances (127.0.0.1, localhost, ports)
- File paths:             ~40 instances (from spec)
- Timeouts:               ~45 instances (from spec)
- Test constants:         ~46 instances (acceptable)

Target:                   0 instances in production code
Spec Violation:           Yes (Zero Hardcoding Specification)
```

### Infrastructure Status:
```
✅ beardog-config crate exists and is well-structured
✅ Environment variable support implemented
✅ Hierarchical config loading ready
✅ Validation framework in place
✅ Builder pattern for testing
✅ Default values with security in mind
```

---

## 🎯 **EXECUTION STRATEGY**

### Week 1: Infrastructure + 90% Reduction (492 → <50)
**Focus**: High-impact network configuration

1. **Day 1-2**: Config utility functions (4 hours)
   - Create config access helpers
   - Add global config singleton usage
   - Documentation for developers

2. **Day 3-4**: Replace network hardcoding (6 hours)
   - Update all `127.0.0.1` / `localhost` references
   - Replace hardcoded ports (8080, 9090, 3000, etc.)
   - Update test files to use config builders

3. **Day 5**: Verification (2 hours)
   - Run full test suite
   - Verify no regressions
   - Update environment variable documentation

**Target**: 492 → <50 instances (90% reduction) ✅

### Week 2: Path to Excellence (<50 → <20)
**Focus**: Paths, timeouts, and edge cases

1. **Day 6-7**: File paths (4 hours)
   - Library search paths
   - Config directories
   - Log file paths
   - Data directories

2. **Day 8-9**: Timeouts and limits (4 hours)
   - Operation timeouts
   - Retry counts
   - Buffer sizes
   - Connection limits

3. **Day 10**: Validation (2 hours)
   - Integration tests with various configs
   - Environment variable testing
   - Edge case verification

**Target**: <50 → <20 instances (96% reduction) ✅

### Week 3: Spec Compliance (<20 → 0)
**Focus**: Final cleanup and validation

1. **Day 11-12**: Final sweep (4 hours)
   - Audit remaining instances
   - Replace or justify each one
   - Document any legitimate constants

2. **Day 13-14**: Testing & Documentation (4 hours)
   - Comprehensive config testing
   - Migration guide for users
   - Environment variable reference
   - Configuration examples

3. **Day 15**: Final validation (2 hours)
   - Zero hardcoding verification
   - Performance testing
   - Spec compliance check

**Target**: 0 instances (100% compliance) ✅

---

## 📋 **REPLACEMENT CHECKLIST**

### Priority 1: Network Configuration (492 instances)

#### Locations to Update:
```
✅ beardog-config (already has infrastructure)
□ beardog-core (13 instances - tests mostly)
□ beardog-types (network config constants)
□ beardog-tunnel (HSM network configs)
□ beardog-adapters (universal adapter endpoints)
□ beardog-monitoring (metrics endpoints)
□ beardog-node-registry (federation endpoints)
□ beardog-security-registry (security endpoints)
```

#### Pattern to Replace:
```rust
// ❌ BEFORE: Hardcoded
let endpoint = "http://localhost:8080";
let addr = "127.0.0.1:9090";
const PORT: u16 = 8080;

// ✅ AFTER: Config-driven
use beardog_config::config;
let endpoint = format!("http://{}:{}", 
    config().network.api.bind_address,
    config().network.api.port);
let addr = format!("{}:{}", 
    config().network.discovery.bind_address,
    config().network.discovery.port);
```

### Priority 2: File Paths (~40 instances)

#### Locations:
```
□ beardog-config (paths domain - enhance)
□ beardog-tunnel (HSM library paths)
□ beardog-security (key storage paths)
□ beardog-cli (config file discovery)
```

#### Pattern:
```rust
// ❌ BEFORE
let lib_path = "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so";
let config_dir = PathBuf::from("/etc/beardog");

// ✅ AFTER
let lib_path = config().paths.discover_pkcs11_library()
    .unwrap_or_else(|| config().paths.default_pkcs11_path());
let config_dir = config().paths.config_dir.clone();
```

### Priority 3: Timeouts & Limits (~45 instances)

#### Locations:
```
□ beardog-config (timeouts domain - enhance)
□ beardog-core (operation timeouts)
□ beardog-tunnel (HSM operation timeouts)
□ beardog-networking (connection timeouts)
```

#### Pattern:
```rust
// ❌ BEFORE
tokio::time::timeout(Duration::from_secs(30), operation).await?;
const MAX_RETRIES: usize = 3;

// ✅ AFTER
tokio::time::timeout(config().timeouts.operation, operation).await?;
let max_retries = config().limits.max_retries;
```

---

## 🛠️ **IMPLEMENTATION TOOLS**

### 1. Global Config Access
```rust
// Create in beardog-config/src/global.rs (already exists!)
use once_cell::sync::Lazy;
use crate::BearDogConfig;

pub static BEARDOG_CONFIG: Lazy<BearDogConfig> = Lazy::new(|| {
    BearDogConfig::load()
        .expect("Failed to load BearDog configuration")
});

pub fn config() -> &'static BearDogConfig {
    &BEARDOG_CONFIG
}
```

### 2. Search & Replace Strategy
```bash
# Find all hardcoded network values
grep -r "127\.0\.0\.1\|localhost" crates/ --include="*.rs" | grep -v test

# Find hardcoded ports
grep -r ":8080\|:9090\|:3000" crates/ --include="*.rs" | grep -v test

# Find Duration::from_secs
grep -r "Duration::from_secs" crates/ --include="*.rs" | grep -v test
```

### 3. Testing Strategy
```rust
// Use builders for tests (already available!)
use beardog_config::domains::network::ApiConfig;

#[test]
fn test_with_custom_port() {
    let config = ApiConfig::builder()
        .port(9999)
        .build();
    
    // Test with custom config
    assert_eq!(config.port, 9999);
}
```

---

## 📈 **PROGRESS TRACKING**

### Metrics to Monitor:
```bash
# Count remaining hardcoded values
grep -r "127\.0\.0\.1\|localhost" crates/ --include="*.rs" | \
    grep -v test | grep -v "//" | wc -l

# Verify config usage
grep -r "config()" crates/ --include="*.rs" | wc -l

# Check for unwrap on config
grep -r "config().*unwrap" crates/ --include="*.rs"
```

### Daily Targets:
```
Day 1:  492 → 400 (92 replaced - tooling setup)
Day 2:  400 → 300 (100 replaced - network core)
Day 3:  300 → 200 (100 replaced - network adapters)
Day 4:  200 → 100 (100 replaced - network tests)
Day 5:  100 → <50 (50+ replaced - cleanup)

Week 1 Target: <50 instances remaining
```

---

## ✅ **VERIFICATION CHECKLIST**

### Pre-Deployment Validation:
- [ ] All tests passing (`cargo test --workspace`)
- [ ] No hardcoded values in production code
- [ ] Config validation tests updated
- [ ] Environment variable documentation complete
- [ ] Migration guide written
- [ ] Example configurations provided
- [ ] Performance verified (no regression)
- [ ] Backward compatibility maintained (where possible)

### Post-Deployment Monitoring:
- [ ] Config loading errors logged
- [ ] Invalid config values detected early
- [ ] Helpful error messages for users
- [ ] Clear documentation for all env vars

---

## 📚 **DOCUMENTATION DELIVERABLES**

### 1. Environment Variable Reference
**File**: `docs/configuration/ENVIRONMENT_VARIABLES.md`
- Complete list of all `BEARDOG_*` variables
- Default values documented
- Examples for common scenarios
- Security considerations

### 2. Configuration Guide
**File**: `docs/configuration/CONFIGURATION_GUIDE.md`
- How to configure BearDog
- File-based vs environment-based
- Hierarchy explanation
- Best practices

### 3. Migration Guide
**File**: `docs/configuration/MIGRATION_GUIDE.md`
- Upgrading from hardcoded values
- Breaking changes (if any)
- Backward compatibility notes
- Example migrations

### 4. Configuration Examples
**Files**: `configs/examples/*.toml`
- Development config
- Production config
- Testing config
- High-security config

---

## 🎯 **SUCCESS CRITERIA**

### Technical:
- ✅ Zero hardcoded values in production code
- ✅ All values configurable via env vars or files
- ✅ Secure defaults that work out-of-box
- ✅ Platform-aware path discovery
- ✅ Comprehensive validation

### Quality:
- ✅ All tests passing
- ✅ No performance regression
- ✅ Clean clippy/fmt
- ✅ Documentation complete

### Specification:
- ✅ **Zero Hardcoding Specification compliance**
- ✅ Configuration over convention
- ✅ Easy to deploy across environments

---

## 🚀 **IMMEDIATE NEXT STEPS**

### Starting Now (Day 1):
1. ✅ Create this execution plan
2. → Enhance global config access
3. → Replace beardog-core hardcoded values (13 instances)
4. → Update test files to use config builders
5. → Document progress

**Let's begin execution!** 🐻🎯

---

**Status**: 📍 **READY TO EXECUTE**  
**Next Action**: Replace beardog-core network hardcoding  
**Tracking**: This document + git commits

