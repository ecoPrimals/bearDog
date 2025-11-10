# DiscoveryConfig Consolidation Plan (Pilot)

**Date**: November 10, 2025  
**Status**: 🎯 Ready to Execute  
**Estimated Time**: 2-3 hours

---

## 🔍 Complete Field Inventory

### Instance 1: beardog-types/canonical/config/domains/adapter.rs (CANONICAL)
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

### Instance 2 & 3: beardog-adapters (2 identical instances)
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

### Instance 4: beardog-utils/env_config.rs (MINIMAL)
```rust
pub struct DiscoveryConfig {
    pub endpoint: String,
    pub timeout_secs: u64,
    pub retry_attempts: u32,
}
```

### Instance 5: beardog-tunnel/universal_hsm_discovery/mod.rs
```rust
pub struct DiscoveryConfig {
    pub enable_cloud_discovery: bool,
    pub enable_pkcs11_discovery: bool,
    pub enable_smartphone_discovery: bool,
    pub discovery_timeout_seconds: u64,
    pub enable_capability_detection: bool,
}
```

### Instance 6: beardog-tunnel/tunnel/hsm/universal_discovery/mod.rs (EXTENDED HSM)
```rust
pub struct DiscoveryConfig {
    pub enable_cloud_kms: bool,
    pub enable_network_hsm: bool,
    pub enable_usb_hsm: bool,
    pub enable_software_hsm: bool,
    pub enable_mobile_hsm: bool,
    pub enable_tpm: bool,
    pub discovery_timeout_seconds: u32,
    pub enable_human_entropy_elevation: bool,
    pub minimum_entropy_quality: f64,
}
```

### Instance 7: beardog-types/canonical/providers_unified/discovery.rs
```rust
pub struct DiscoveryConfig {
    pub enabled: bool,
    pub discovery_type: DiscoveryType,
    pub registry_endpoints: Vec<String>,
    pub registration_enabled: bool,
    pub health_check_interval: Duration,
    pub service_metadata: HashMap<String, String>,
    pub refresh_interval: Duration,
}
```

### Instance 8: beardog-core/biome_sovereignty/mixed_lineage.rs
```rust
pub struct DiscoveryConfig {
    pub auto_discovery: bool,
    pub discovery_timeout_secs: u64,
    pub max_discovery_attempts: u32,
}
```

---

## 🎯 Consolidation Strategy: Domain Extensions

### Base: Core DiscoveryConfig (All-domains fields)
Location: `beardog-types/src/canonical/config/domains/discovery.rs` (NEW FILE)

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Core discovery configuration for all discovery types
/// 
/// This is the canonical base configuration. Domain-specific extensions
/// should wrap or compose this struct rather than redefining discovery config.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveryConfig {
    // Core Discovery
    /// Enable discovery feature
    pub enabled: bool,
    
    /// Discovery timeout
    pub timeout: Duration,
    
    /// Maximum discovery attempts
    pub max_attempts: u32,
    
    /// Maximum concurrent discovery operations
    pub max_concurrent: usize,
    
    /// Discovery interval
    pub discovery_interval: Duration,
    
    /// Discovery refresh interval
    pub refresh_interval: Duration,
    
    // Caching
    /// Enable discovery result caching
    pub cache_enabled: bool,
    
    /// Cache TTL
    pub cache_ttl: Duration,
    
    // Endpoints
    /// Discovery service endpoints
    pub endpoints: Vec<String>,
    
    // Health & Registration
    /// Health check interval
    pub health_check_interval: Duration,
    
    /// Enable automatic registration
    pub auto_register: bool,
    
    // Metadata
    /// Service metadata for registration
    pub service_metadata: HashMap<String, String>,
    
    // Features
    /// Enable predictive discovery
    pub predictive_enabled: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(5),
            max_attempts: 3,
            max_concurrent: 10,
            discovery_interval: Duration::from_secs(30),
            refresh_interval: Duration::from_secs(60),
            cache_enabled: true,
            cache_ttl: Duration::from_secs(300),
            endpoints: vec![],
            health_check_interval: Duration::from_secs(60),
            auto_register: true,
            service_metadata: HashMap::new(),
            predictive_enabled: false,
        }
    }
}
```

### Extension 1: HsmDiscoveryConfig
Location: `beardog-types/src/canonical/hsm/discovery.rs`

```rust
use super::super::config::domains::discovery::DiscoveryConfig;

/// HSM-specific discovery configuration
#[derive(Debug, Clone)]
pub struct HsmDiscoveryConfig {
    /// Base discovery configuration
    pub base: DiscoveryConfig,
    
    // HSM Type Enablement
    pub enable_cloud_kms: bool,
    pub enable_network_hsm: bool,
    pub enable_usb_hsm: bool,
    pub enable_software_hsm: bool,
    pub enable_mobile_hsm: bool,
    pub enable_tpm: bool,
    pub enable_pkcs11_discovery: bool,
    
    // Capability Detection
    pub enable_capability_detection: bool,
    
    // Entropy & Quality
    pub enable_human_entropy_elevation: bool,
    pub minimum_entropy_quality: f64,
}

impl Default for HsmDiscoveryConfig {
    fn default() -> Self {
        Self {
            base: DiscoveryConfig::default(),
            enable_cloud_kms: true,
            enable_network_hsm: true,
            enable_usb_hsm: true,
            enable_software_hsm: true,
            enable_mobile_hsm: true,
            enable_tpm: true,
            enable_pkcs11_discovery: true,
            enable_capability_detection: true,
            enable_human_entropy_elevation: true,
            minimum_entropy_quality: 0.8,
        }
    }
}
```

### Extension 2: BiomeDiscoveryConfig
Location: `beardog-types/src/canonical/biome/discovery.rs`

```rust
use super::super::config::domains::discovery::DiscoveryConfig;

/// Biome sovereignty-specific discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeDiscoveryConfig {
    /// Base discovery configuration
    pub base: DiscoveryConfig,
    
    /// Whether to automatically discover potential partners
    pub auto_discovery_partners: bool,
}

impl Default for BiomeDiscoveryConfig {
    fn default() -> Self {
        Self {
            base: DiscoveryConfig::default(),
            auto_discovery_partners: true,
        }
    }
}
```

---

## 📋 Migration Steps

### Step 1: Create New Canonical DiscoveryConfig ✅
1. Create `beardog-types/src/canonical/config/domains/discovery.rs`
2. Implement base DiscoveryConfig with ALL common fields
3. Implement Default trait
4. Add comprehensive documentation
5. Export from `beardog-types/src/canonical/config/domains/mod.rs`

### Step 2: Create Domain Extensions ✅
1. Create `HsmDiscoveryConfig` in `beardog-types/src/canonical/hsm/discovery.rs`
2. Create `BiomeDiscoveryConfig` in `beardog-types/src/canonical/biome/discovery.rs`
3. Export appropriately

### Step 3: Migrate Instance 1 (adapter.rs) ✅
**File**: `crates/beardog-types/src/canonical/config/domains/adapter.rs:79`

**Before**:
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

**After**:
```rust
// Remove struct, add re-export
pub use super::discovery::DiscoveryConfig;
```

### Step 4: Migrate Instances 2 & 3 (beardog-adapters) ✅
**Files**:
- `crates/beardog-adapters/src/universal/capability_discovery/discovery/config.rs:7`
- `crates/beardog-adapters/src/universal/capability_discovery.rs:33`

**Before**: Local struct definition

**After**:
```rust
// Remove local struct
use beardog_types::canonical::config::domains::discovery::DiscoveryConfig;

// Update field access if needed (likely some conversions)
```

### Step 5: Migrate Instance 4 (beardog-utils) ✅
**File**: `crates/beardog-utils/src/env_config.rs:100`

**Before**: Minimal 3-field struct

**After**:
```rust
use beardog_types::canonical::config::domains::discovery::DiscoveryConfig;

// from_env() method will map to canonical fields
```

### Step 6: Migrate Instances 5 & 6 (HSM - beardog-tunnel) ✅
**Files**:
- `crates/beardog-tunnel/src/universal_hsm_discovery/mod.rs:148`
- `crates/beardog-tunnel/src/tunnel/hsm/universal_discovery/mod.rs:37`

**Before**: HSM-specific flags only

**After**:
```rust
use beardog_types::canonical::hsm::discovery::HsmDiscoveryConfig;

// Use HsmDiscoveryConfig instead of local DiscoveryConfig
// Access base fields via .base.field
```

### Step 7: Migrate Instance 7 (providers_unified) ✅
**File**: `crates/beardog-types/src/canonical/providers_unified/discovery.rs:11`

**Before**: Service discovery variant

**After**:
```rust
// This might already BE the canonical one, or needs merge
// Check if we keep both or consolidate
```

### Step 8: Migrate Instance 8 (biome_sovereignty) ✅
**File**: `crates/beardog-core/src/biome_sovereignty/mixed_lineage.rs:71`

**Before**: Biome-specific 3-field struct

**After**:
```rust
use beardog_types::canonical::biome::discovery::BiomeDiscoveryConfig;

// Use BiomeDiscoveryConfig instead
```

---

## ⚙️ Implementation Order

1. ✅ Create canonical base `DiscoveryConfig`
2. ✅ Create `HsmDiscoveryConfig` extension
3. ✅ Create `BiomeDiscoveryConfig` extension
4. ✅ Migrate easiest first (Instance 4 - beardog-utils, minimal)
5. ✅ Migrate Instance 1 (adapter.rs - already canonical location)
6. ✅ Migrate Instances 2 & 3 (beardog-adapters - identical, do together)
7. ✅ Migrate Instances 5 & 6 (HSM configs)
8. ✅ Migrate Instance 8 (biome config)
9. ✅ Review Instance 7 (might already be correct)
10. ✅ Compile and test

---

## 🧪 Validation Checklist

- [ ] All 8 instances migrated
- [ ] No local DiscoveryConfig definitions remain (except canonical)
- [ ] `cargo check --workspace` passes
- [ ] `cargo test --workspace --lib` passes
- [ ] All imports resolved
- [ ] No functionality broken
- [ ] Commit with clear message

---

## 📊 Expected Results

### Before
- 8 fragmented DiscoveryConfig structs
- 3 different field sets
- No single source of truth
- Type confusion

### After
- 1 canonical DiscoveryConfig
- 2 domain extensions (HSM, Biome)
- Single source of truth
- Clear type hierarchy
- All imports from canonical location

---

## 🎯 Success Criteria

1. **Zero local DiscoveryConfig definitions** (except canonical + extensions)
2. **Build passes** with no new errors
3. **Tests pass** with no regressions
4. **Imports simplified** (all from beardog-types::canonical)
5. **Documentation updated** (inline docs explain extensions)

---

**Status**: 🟢 READY TO EXECUTE  
**Next**: Create canonical DiscoveryConfig file  
**Estimated Time**: 2-3 hours total

