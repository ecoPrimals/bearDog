# Config Type Alias Consolidation Plan

**Date**: October 1, 2025  
**Current State**: 34 type aliases across 23 files  
**Target State**: <10 essential aliases  
**Strategy**: Remove duplicates, standardize naming, keep only compatibility aliases

---

## 📊 **Analysis Results**

### **Duplicate Aliases (MUST FIX)**

These have conflicting definitions:

1. **GlobalConfig** (2 definitions)
   - `= BearDogMasterConfig` (config/mod.rs)
   - `= UnifiedBearDogConfig` (unified_types.rs)
   - **Action**: Keep UnifiedBearDogConfig, remove BearDogMasterConfig

2. **MasterConfig** (2 definitions)
   - `= BearDogMasterConfig` (config/mod.rs)
   - `= UnifiedBearDogConfig` (unified_types.rs)
   - **Action**: Same as above

3. **HsmConfig** (2 definitions)
   - `= CanonicalHsmConfig` (hsm_unified/mod.rs)
   - `= crate::canonical::hsm_unified::CanonicalHsmConfig` (unified_types.rs)
   - **Action**: Keep one, remove duplicate

4. **ConfigurationOutcome<T>** (2 definitions)
   - Different Result types
   - **Action**: Standardize on one

### **Essential Aliases (KEEP)**

These provide backwards compatibility:

```rust
// Domain configs (KEEP for backwards compatibility)
pub type AppConfig = CanonicalAppConfig;
pub type ApplicationConfig = CanonicalAppConfig;
pub type AuthConfig = CanonicalAuthConfig;
pub type DatabaseConfig = CanonicalDatabaseConfig;
pub type NetworkConfig = CanonicalNetworkConfig;
pub type SecurityConfig = CanonicalSecurityConfig;
pub type MonitoringConfig = CanonicalMonitoringConfig;
pub type WorkflowConfig = CanonicalWorkflowConfig;
pub type PerformanceConfig = CanonicalPerformanceConfig;
pub type GeneticsConfig = CanonicalGeneticsConfig;
pub type ComplianceConfig = CanonicalComplianceConfig;
pub type CacheConfig = CanonicalCacheConfig;
```

**Justification**: Standard aliases for Canonical* types (12 aliases)

### **Unnecessary Aliases (REMOVE)**

```rust
// Duplicate master config aliases (REMOVE 2 of 3)
pub type UnifiedConfig = UnifiedBearDogConfig;  // REMOVE
pub type MasterConfig = UnifiedBearDogConfig;   // REMOVE
pub type GlobalConfig = UnifiedBearDogConfig;   // REMOVE
// Keep: Just use UnifiedBearDogConfig directly

// Deprecated aliases (REMOVE)
pub type WorkingUnifiedConfig = SimplifiedBearDogConfig;  // REMOVE (deprecated)
pub type CanonicalProductionConfig = UnifiedProductionConfig;  // REMOVE (not used)

// Specialized/internal (EVALUATE)
pub type EndpointConfig = CanonicalNetworkConfig;  // QUESTIONABLE
pub type RetryPolicyConfig = RetryConfig;  // REMOVE (not widely used)
pub type RateLimitingConfig = RateLimitConfig;  // REMOVE (not widely used)
pub type SecurityPolicyConfig = crate::canonical::SecurityConfig;  // REMOVE
pub type ThreatDetectionConfig = crate::canonical::monitoring::ThreatDetectionConfig;  // REMOVE
pub type SovereigntyConfig = SimpleEcosystemConfig;  // EVALUATE
pub type KeyManagerConfig = MemoryKeyConfig;  // EVALUATE
pub type GeneticHealingConfig = ...::TunnelConfig;  // EVALUATE
```

---

## 🎯 **Consolidation Strategy**

### **Phase 1: Remove Duplicates** (Immediate)

1. **Fix GlobalConfig/MasterConfig conflicts**
   - Choose: `UnifiedBearDogConfig` as canonical
   - Remove: `BearDogMasterConfig`, `GlobalConfig`, `MasterConfig` aliases
   - Update: All usages to use `UnifiedBearDogConfig` directly

2. **Fix HsmConfig duplicate**
   - Keep: `hsm_unified/mod.rs` definition
   - Remove: `unified_types.rs` duplicate

3. **Fix ConfigurationOutcome duplicate**
   - Standardize on: `Result<T, BearDogError>`
   - Remove alias (not needed)

### **Phase 2: Remove Unnecessary Aliases** (Quick Wins)

Remove these 8 aliases that add no value:
- `UnifiedConfig`
- `MasterConfig`  
- `GlobalConfig`
- `WorkingUnifiedConfig` (deprecated)
- `CanonicalProductionConfig`
- `EndpointConfig` (if not used)
- `RetryPolicyConfig`
- `RateLimitingConfig`

### **Phase 3: Keep Essential** (12 aliases)

Keep these standard domain config aliases:
- AppConfig, ApplicationConfig
- AuthConfig, CacheConfig
- DatabaseConfig, NetworkConfig
- SecurityConfig, MonitoringConfig
- WorkflowConfig, PerformanceConfig
- GeneticsConfig, ComplianceConfig

### **Phase 4: Evaluate Specialized** (5 aliases)

Review usage and decide:
- `SecurityPolicyConfig`
- `ThreatDetectionConfig`
- `SovereigntyConfig`
- `KeyManagerConfig`
- `GeneticHealingConfig`

---

## 📊 **Target Metrics**

| Metric | Before | After | Reduction |
|--------|--------|-------|-----------|
| Total aliases | 34 | <15 | 56% |
| Duplicate definitions | 4 | 0 | 100% |
| Unnecessary aliases | 8+ | 0 | 100% |
| Essential aliases | 12 | 12 | 0% |
| Files with aliases | 23 | <15 | 35% |

---

## 🚀 **Implementation Order**

### **Step 1: Remove Duplicate GlobalConfig/MasterConfig** ✅ NEXT
- File: `beardog-types/src/canonical/config/mod.rs`
- Action: Remove alias definitions
- Impact: Forces use of canonical `UnifiedBearDogConfig`

### **Step 2: Remove Duplicate HsmConfig**
- File: `beardog-types/src/unified_types.rs`
- Action: Remove duplicate alias

### **Step 3: Remove Unnecessary Aliases**
- Files: Various
- Action: Remove 8 unnecessary type aliases
- Verify: No breaking changes

### **Step 4: Document Canonical Patterns**
- Update: CODING_STANDARDS.md
- Add: Config naming conventions
- Document: When to use aliases vs direct types

---

## ✅ **Success Criteria**

- [ ] No duplicate config type definitions
- [ ] <15 total config type aliases
- [ ] All aliases serve clear purpose (backwards compat or convenience)
- [ ] Clean build with no new warnings
- [ ] Documentation updated

---

**Priority**: HIGH  
**Complexity**: LOW  
**Impact**: HIGH (reduces confusion, improves maintainability)  
**Estimated Time**: 1-2 hours 