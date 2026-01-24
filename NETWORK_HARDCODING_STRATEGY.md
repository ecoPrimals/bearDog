# Network Hardcoding Elimination Strategy

**Target**: 527 instances across 112 files → 0  
**Week 1 Goal**: 527 → <400 instances (eliminate ~130, 25% reduction)

---

## 🎯 Strategy: Systematic Replacement

### Phase 1: Constants Module Cleanup (High Impact)
**File**: `crates/beardog-types/src/constants/domains/network.rs`  
**Status**: Partially migrated, needs completion

**Current State**:
- ✅ Functions use `BEARDOG_CONFIG` (good!)
- ❌ Still has `FALLBACK_*` private constants (bad!)
- ❌ Still has `LOCALHOST_IPV4`, `LOCALHOST_NAME` constants (should come from config)

**Action**: Remove ALL hardcoded fallback constants, rely 100% on config hierarchy

### Phase 2: Service IDs (Medium Impact)
**Files**:
- `crates/beardog-core/src/zero_copy_service_ids.rs` (15 instances)
- `crates/beardog-core/src/zero_copy_service_ids_expanded.rs` (15 instances)

**Issue**: Service discovery endpoints hardcoded

**Action**: Use capability-based discovery via Songbird

### Phase 3: Test Fixtures (Acceptable, Document)
**Files**:
- `crates/beardog-types/src/canonical/config/test_fixtures.rs` (23 instances)
- Various `*_tests.rs` files

**Issue**: Test constants for fixtures

**Action**: Document as acceptable for tests, ensure not used in production

### Phase 4: Network Endpoints (High Priority)
**Files**:
- `crates/beardog-types/src/canonical/network/universal_endpoints.rs` (10 instances)
- `crates/beardog-types/src/canonical/config/runtime_config.rs` (14 instances)

**Action**: Replace with config-based endpoint resolution

---

## 📋 Execution Plan

### Step 1: Clean Constants Module (30 min)
Remove all `FALLBACK_*` constants:
```rust
// ❌ REMOVE:
const FALLBACK_API_PORT: u16 = 8080;
const FALLBACK_METRICS_PORT: u16 = 9090;

// ✅ REPLACE with direct config access:
pub fn default_api_port() -> u16 {
    BEARDOG_CONFIG.network.api.port
}
```

### Step 2: Remove Localhost Constants (15 min)
```rust
// ❌ REMOVE:
pub const LOCALHOST_IPV4: &str = "127.0.0.1";
pub const LOCALHOST_NAME: &str = "localhost";

// ✅ These should come from config or platform detection
```

### Step 3: Update Callers to Use Config (2-3 hours)
Systematic replacement across 43 files:
```rust
// ❌ OLD:
use beardog_types::constants::domains::network::config::LOCALHOST_IPV4;
let addr = LOCALHOST_IPV4;

// ✅ NEW:
use beardog_config::global::BEARDOG_CONFIG;
let addr = BEARDOG_CONFIG.network.api.bind_address;
```

### Step 4: Service IDs to Capability Discovery (3-4 hours)
```rust
// ❌ OLD:
const DISCOVERY_ENDPOINT: &str = "http://127.0.0.1:8080/discovery";

// ✅ NEW:
let discovery = songbird::discover_capability("service.discovery").await?;
```

---

## 🎯 Quick Wins (1-2 hours)

### 1. Remove FALLBACK constants (30 min)
Impact: Removes 5+ hardcoded values  
Risk: Low (functions already use config)

### 2. Remove LOCALHOST constants (15 min)
Impact: Forces all callers to use config  
Risk: Medium (need to verify all callers have config access)

### 3. Document test constants (15 min)
Impact: Clarifies which hardcoding is acceptable  
Risk: None (documentation only)

### 4. Update top 5 high-use files (1-2 hours)
Impact: Removes ~50 instances  
Risk: Medium (need testing)

---

## 📊 Expected Results

### Week 1 Target
- **Before**: 527 instances
- **Quick Wins**: -20 instances (constants cleanup)
- **Top 5 Files**: -50 instances
- **Service IDs**: -30 instances
- **Documentation**: -30 (reclassified as acceptable test code)
- **After**: ~400 instances (-24% reduction)

### Test Strategy
1. Run `cargo check --workspace` after each file
2. Run specific crate tests: `cargo test -p beardog-types`
3. Integration test at end of phase
4. Full test suite before commit

---

## 🚫 What NOT to Change

### Industry Standard Ports (Keep)
```rust
pub const DEFAULT_POSTGRES_PORT: u16 = 5432;  // ✅ OK - Industry standard
pub const DEFAULT_GRAFANA_PORT: u16 = 3000;   // ✅ OK - Industry standard  
pub const HTTPS_PORT: u16 = 443;              // ✅ OK - IETF standard
```

These are **protocol constants**, not configuration.

### Test-Only Constants (Document, Keep)
```rust
#[cfg(test)]
const TEST_HOST: &str = "127.0.0.1";  // ✅ OK - Test fixture
```

Document as acceptable, ensure `#[cfg(test)]` guards.

---

## 🔄 Migration Pattern

### Before (Hardcoded)
```rust
fn connect_to_api() -> Result<Connection> {
    let addr = "127.0.0.1:8080".parse()?;
    Connection::new(addr)
}
```

### After (Config-Based)
```rust
fn connect_to_api() -> Result<Connection> {
    use beardog_config::global::BEARDOG_CONFIG;
    let addr = format!(
        "{}:{}",
        BEARDOG_CONFIG.network.api.bind_address,
        BEARDOG_CONFIG.network.api.port
    ).parse()?;
    Connection::new(addr)
}
```

### Best (Capability-Based)
```rust
async fn connect_to_api() -> Result<Connection> {
    let api = songbird::discover_capability("service.api").await?;
    Connection::new(api.endpoint)
}
```

---

**Ready to execute!** Starting with constants module cleanup for immediate impact.

