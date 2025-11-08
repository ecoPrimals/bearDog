# Config Unification Plan - November 8, 2025

**Generated**: Automated audit  
**Total Configs**: 928 structs  
**Duplicates Found**: 150+ instances (top 20 names)  
**Target**: <500 structs (45% reduction)

---

## 🔴 HIGH PRIORITY DUPLICATES

### Critical: Generic "Config" (13 instances)
**Issue**: 13 files use generic "Config" name - causes confusion

**Locations**: Need to analyze `/tmp/beardog_config_catalog.txt`

**Action**: Rename each to domain-specific name:
- `Config` in auth module → `AuthConfig`
- `Config` in network module → `NetworkConfig`
- etc.

---

### SecurityConfig (10 instances)
**Action Required**: Consolidate to single canonical version

**Steps**:
1. Review all 10 implementations
2. Identify common fields
3. Create `CanonicalSecurityConfig` in `beardog-types/src/canonical/config/domains/security/`
4. Add optional fields for domain-specific needs
5. Create type aliases for backwards compatibility
6. Migrate all references

---

### RetryConfig (10 instances)
**Action Required**: Consolidate to single version

**Pattern**:
```rust
// Canonical location: beardog-types/src/canonical/config/domains/retry.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalRetryConfig {
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
    pub max_backoff_ms: u64,
    pub backoff_multiplier: f64,
    #[serde(default)]
    pub retry_on_timeout: bool,
}

// Type alias for backwards compatibility
pub type RetryConfig = CanonicalRetryConfig;
```

---

### DiscoveryConfig (9 instances)
**Action Required**: Consolidate with careful review of discovery-specific needs

---

### TimeoutConfig (8 instances)
**Action Required**: Consolidate to single version

**Note**: May need separate timeouts for different operations, use fields:
```rust
pub struct CanonicalTimeoutConfig {
    pub connect_timeout_ms: u64,
    pub read_timeout_ms: u64,
    pub write_timeout_ms: u64,
    pub operation_timeout_ms: u64,
}
```

---

### PerformanceConfig (8 instances)
### HsmConfig (8 instances)
### OptimizationConfig (7 instances)
### NetworkConfig (7 instances)
### MonitoringConfig (7 instances)
### CircuitBreakerConfig (7 instances)
### CacheConfig (7 instances)

**Action Required**: Each needs consolidation analysis

---

## 📋 CONSOLIDATION STRATEGY

### Phase 1: Analysis (Completed)
- [x] Generate inventory (928 configs found)
- [x] Identify duplicates (150+ instances)
- [x] Prioritize by frequency

### Phase 2: Top 5 Duplicates (Next)
**Focus**: Config, SecurityConfig, RetryConfig, DiscoveryConfig, TimeoutConfig

**Steps for each**:
1. Extract all instances to temporary files
2. Compare implementations
3. Design canonical version with all fields
4. Create migration plan
5. Implement canonical version
6. Add type aliases
7. Update references
8. Test thoroughly

### Phase 3: Next 15 Duplicates
**Focus**: PerformanceConfig through ConnectionConfig

### Phase 4: Remaining Configs
**Focus**: Organize by domain, ensure no more duplicates

---

## 🎯 QUICK WINS

### Rename Generic "Config" (2 hours)
**Impact**: Eliminates 13 ambiguous names

```bash
# For each generic Config:
# 1. Determine domain from file path
# 2. Rename to domain-specific name
# 3. Update all references
# 4. Test build
```

### Consolidate RetryConfig (3 hours)
**Impact**: Eliminates 9 duplicate implementations

**Likely candidates for canonical fields**:
- max_retries
- initial_backoff_ms
- max_backoff_ms
- backoff_multiplier
- retry_on_timeout (optional)

---

## 📊 EXPECTED RESULTS

### Before
- 928 config structs
- 13 generic "Config" names
- 150+ duplicate names
- Configs scattered across 351 files

### After
- ~500 config structs (45% reduction)
- 0 generic "Config" names
- 0 duplicate names
- Configs organized in canonical hierarchy

---

## 🚀 NEXT STEPS

1. **Immediate**: Analyze top 5 duplicate configs in detail
2. **This Week**: Consolidate top 5 duplicates
3. **Next Week**: Consolidate remaining duplicates
4. **Following Week**: Establish final hierarchy

---

**Status**: 🟢 **ANALYSIS COMPLETE - READY FOR CONSOLIDATION**  
**Inventory**: `/tmp/beardog_config_catalog.txt`  
**Analysis**: This document

---

*Next: Begin Phase 1.2 - Constants Centralization*

