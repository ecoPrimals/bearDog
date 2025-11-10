# Config Consolidation Analysis

**Date**: November 10, 2025  
**Status**: 🔍 Analysis Complete - Ready for Consolidation

---

## 🎯 Executive Summary

**Problem**: Config structs are FRAGMENTED across crates, not duplicated.
- Same name (`DiscoveryConfig`) but different fields
- Each crate defines its own version
- No single source of truth
- Breaking the canonical type system

**Impact**:
- Maintainability nightmare
- Type confusion
- Import chaos
- Violates canonical architecture

**Solution**: Consolidate to canonical versions in `beardog-types/src/canonical/config/domains/`

---

## 📊 Fragmentation Evidence

### Example 1: DiscoveryConfig (8 instances)

#### Canonical Version (beardog-types)
**Location**: `crates/beardog-types/src/canonical/config/domains/adapter.rs:79`

```rust
pub struct DiscoveryConfig {
    pub timeout: Duration,
    pub max_attempts: u32,
    pub discovery_interval: Duration,
    pub cache_enabled: bool,
    pub cache_ttl: Duration,
    pub endpoints: Vec<String>,
    pub predictive_enabled: bool,
}
```

#### Fragmented Version 1 (beardog-adapters)
**Location**: `crates/beardog-adapters/src/universal/capability_discovery/discovery/config.rs:7`

```rust
pub struct DiscoveryConfig {
    pub timeout_ms: u64,
    pub max_concurrent: usize,
    pub cache_duration_ms: u64,
    pub health_check_interval_ms: u64,
    pub discovery_endpoints: Vec<String>,
    pub auto_register: bool,
}
```

**Issues**:
- Different field names (`timeout` vs `timeout_ms`)
- Different types (Duration vs u64)
- Different fields (`max_concurrent` vs `max_attempts`)
- Defines its OWN config instead of importing canonical

#### Fragmented Version 2 (beardog-tunnel)
**Location**: `crates/beardog-tunnel/src/universal_hsm_discovery/mod.rs:148`

```rust
pub struct DiscoveryConfig {
    pub enable_cloud_discovery: bool,
    pub enable_pkcs11_discovery: bool,
    pub enable_smartphone_discovery: bool,
    pub discovery_timeout_seconds: u64,
    pub enable_capability_detection: bool,
}
```

**Issues**:
- HSM-specific flags (should be in HsmDiscoveryConfig)
- Yet another timeout representation
- Domain-specific logic in general config

---

## 🔍 Complete Fragmentation Inventory

### High-Priority Consolidations

| Config Name | Instances | Canonical Location | Fragmented Locations |
|-------------|-----------|-------------------|---------------------|
| **DiscoveryConfig** | 8 | types/canonical/config/domains/adapter.rs | adapters, tunnel, utils, core |
| **SecurityConfig** | 7 | types/canonical/config/domains/security.rs | adapters, tunnel, monitoring, auth |
| **NetworkConfig** | 7 | types/canonical/config/domains/network/mod.rs | adapters, tunnel, networking |
| **TimeoutConfig** | 5 | types/canonical/providers_unified/resilience.rs | utils, config, types (3 places) |
| **RetryConfig** | 5 | types/canonical/providers_unified/resilience.rs | adapters, tunnel, types (2 places) |
| **HsmConfig** | 7 | types/canonical/hsm/config.rs | adapters, tunnel, security |
| **MonitoringConfig** | 5 | types/canonical/config/domains/monitoring.rs | monitoring, tunnel, workflows |

---

## 🎯 Consolidation Strategy

### Phase 1: Pilot with DiscoveryConfig (2 hours)
1. **Audit**: Review all 8 instances in detail
2. **Unify**: Ensure canonical version has ALL needed fields
3. **Migrate**: Replace fragmented versions with imports
4. **Test**: Validate build and tests pass
5. **Document**: Record learnings

### Phase 2: High-Volume Configs (3-4 hours)
- SecurityConfig (7 instances)
- NetworkConfig (7 instances)
- HsmConfig (7 instances)

### Phase 3: Supporting Configs (2-3 hours)
- TimeoutConfig (5 instances)
- RetryConfig (5 instances)
- MonitoringConfig (5 instances)
- Others (20+ configs with 3-4 instances each)

---

## 📋 Consolidation Checklist (Per Config)

### 1. Analysis
- [ ] List all instances with locations
- [ ] Compare field structures
- [ ] Identify canonical version
- [ ] Note domain-specific extensions needed

### 2. Canonical Enhancement
- [ ] Ensure canonical has all needed fields
- [ ] Add missing fields if necessary
- [ ] Use proper types (Duration, not u64)
- [ ] Add comprehensive documentation
- [ ] Implement Default trait

### 3. Migration
- [ ] Replace local struct with import
- [ ] Update field access if names changed
- [ ] Fix any type mismatches
- [ ] Run cargo check on that crate

### 4. Validation
- [ ] Full workspace compile: `cargo check --workspace`
- [ ] Run tests: `cargo test --package <package>`
- [ ] Check for regressions
- [ ] Update progress dashboard

### 5. Commit
- [ ] Atomic commit per config type
- [ ] Clear commit message
- [ ] Reference this analysis doc

---

## 🚀 Pilot: DiscoveryConfig Consolidation Plan

### Step 1: Read All 8 Instances
```bash
# Read each file to understand full structure
cat crates/beardog-adapters/src/universal/capability_discovery/discovery/config.rs
cat crates/beardog-adapters/src/universal/capability_discovery.rs
cat crates/beardog-utils/src/env_config.rs
cat crates/beardog-tunnel/src/universal_hsm_discovery/mod.rs
cat crates/beardog-tunnel/src/tunnel/hsm/universal_discovery/mod.rs
cat crates/beardog-types/src/canonical/config/domains/adapter.rs
cat crates/beardog-types/src/canonical/providers_unified/discovery.rs
cat crates/beardog-core/src/biome_sovereignty/mixed_lineage.rs
```

### Step 2: Identify Canonical Fields Union
Merge ALL fields from ALL instances into canonical version:
- timeout (Duration, not u64)
- max_attempts (u32)
- max_concurrent (usize) - ADD if missing
- discovery_interval (Duration)
- cache_enabled (bool)
- cache_ttl (Duration)
- endpoints (Vec<String>)
- predictive_enabled (bool)
- auto_register (bool) - ADD if missing
- health_check_interval (Duration) - ADD if missing

**Domain-Specific Extensions** (create separate structs):
- HsmDiscoveryConfig (for HSM-specific flags)
  - enable_cloud_discovery
  - enable_pkcs11_discovery
  - enable_smartphone_discovery
  - enable_capability_detection

### Step 3: Update Canonical
Edit: `crates/beardog-types/src/canonical/config/domains/adapter.rs`

### Step 4: Migrate Crates
For EACH fragmented location:
1. Remove local struct definition
2. Add import: `use beardog_types::canonical::config::domains::adapter::DiscoveryConfig;`
3. Update field access if necessary
4. Compile and test

### Step 5: Create Domain Extensions
If needed, create `HsmDiscoveryConfig` that WRAPS DiscoveryConfig:

```rust
pub struct HsmDiscoveryConfig {
    pub base: DiscoveryConfig,
    pub enable_cloud_discovery: bool,
    pub enable_pkcs11_discovery: bool,
    pub enable_smartphone_discovery: bool,
    pub enable_capability_detection: bool,
}
```

---

## 📊 Expected Outcomes

### Metrics
- **Before**: 8 fragmented DiscoveryConfig structs
- **After**: 1 canonical + 1 domain extension (if needed)
- **Files changed**: ~8
- **Import fixes**: ~15-20
- **Compilation**: Should pass

### Benefits
- ✅ Single source of truth
- ✅ Type safety enforced
- ✅ Easier maintenance
- ✅ Canonical architecture restored
- ✅ Import paths simplified

---

## 🎓 Lessons Learned (To Be Updated)

### What Worked
- TBD after pilot

### Challenges
- TBD after pilot

### Optimizations
- TBD after pilot

---

## 📈 Progress Tracking

### Completed
- [ ] DiscoveryConfig (8 → 1)
- [ ] SecurityConfig (7 → 1)
- [ ] NetworkConfig (7 → 1)
- [ ] HsmConfig (7 → 1)
- [ ] TimeoutConfig (5 → 1)
- [ ] RetryConfig (5 → 1)
- [ ] MonitoringConfig (5 → 1)

### In Progress
- DiscoveryConfig (pilot)

### Next Up
- SecurityConfig
- NetworkConfig

---

## ⚠️ Risk Mitigation

### Potential Issues
1. **Type mismatches**: Duration vs u64
   - Solution: Provide conversion helpers if needed

2. **Domain-specific fields**: HSM flags in general config
   - Solution: Create domain extension structs

3. **Breaking changes**: Field renames
   - Solution: Update all usage sites atomically

4. **Test failures**: Config shape changes
   - Solution: Update test fixtures

### Backup Plan
- Work on feature branch: `unification/config-consolidation`
- Atomic commits per config type
- Easy to revert if issues arise

---

**Status**: 🟢 READY TO EXECUTE PILOT  
**Next**: Start with DiscoveryConfig consolidation  
**Estimated Time**: 2 hours for pilot, 6-8 hours total

