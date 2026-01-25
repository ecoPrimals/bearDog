# 🎯 Hardcoding Elimination Strategy

**Date**: January 25, 2026  
**Status**: 🔄 IN PROGRESS  
**Priority**: **HIGHEST IMPACT**  
**Estimated Time**: 8-10 hours

---

## 📊 HARDCODING AUDIT SUMMARY

From previous analysis (DEEP_EVOLUTION_STATUS.md):
- **468 IP addresses** identified
- **172 file paths** identified
- **640 total instances** across codebase

---

## 🎯 CLASSIFICATION

### 1. Production Code (HIGH PRIORITY)
**Files identified**:
- `beardog-adapters/src/universal/primal_runtime_discovery.rs` - localhost fallback
- `beardog-integration/src/lib.rs` - localhost/port defaults
- `beardog-config/src/domains/network.rs` - bind address defaults

**Impact**: Direct production behavior

### 2. Test Code (MEDIUM - For Future)
**Status**: Tests legitimately need hardcoded values for determinism
**Action**: Use test helpers + constants, not inline strings

### 3. Examples (LOW - Acceptable)
**Status**: Examples can have hardcoded values for clarity
**Action**: Add comments explaining they're examples

### 4. Documentation (LOW - Acceptable)  
**Status**: Doc examples need concrete values
**Action**: Ensure docs explain configuration options

---

## 🔧 ELIMINATION PATTERNS

### Pattern 1: Localhost Fallbacks → Explicit Error or Config
**Before**:
```rust
.unwrap_or_else(|| "localhost".to_string())
```

**After**:
```rust
.ok_or_else(|| BearDogError::discovery("No address provided by mDNS"))
```

**Rationale**: Silent localhost fallback masks discovery failures

---

### Pattern 2: Default Ports → Configuration Hierarchy
**Before**:
```rust
let url = "https://localhost:8080";
```

**After**:
```rust
let url = std::env::var("BEARDOG_UPA_URL")
    .unwrap_or_else(|_| config.upa_url.clone());
```

**Hierarchy**: CLI args > ENV > Config file > Platform defaults > Error

---

### Pattern 3: Bind Addresses → Environment-Aware Defaults
**Before**:
```rust
bind_address: "127.0.0.1".to_string()
```

**After**:
```rust
bind_address: std::env::var("BEARDOG_BIND_ADDRESS")
    .unwrap_or_else(|_| {
        if cfg!(debug_assertions) {
            "127.0.0.1".to_string()  // Secure default for dev
        } else {
            "0.0.0.0".to_string()     // Production default
        }
    })
```

---

### Pattern 4: File Paths → Platform-Aware Discovery
**Before**:
```rust
let config_path = "/etc/beardog/config.toml";
```

**After**:
```rust
let config_path = std::env::var("BEARDOG_CONFIG_PATH")
    .ok()
    .or_else(|| beardog_utils::platform::default_config_path())
    .unwrap_or_else(|| PathBuf::from("config.toml"));
```

---

## 📋 EXECUTION PLAN

### Phase 1: Critical Production Fixes (2-3h)
**Target**: Fix hardcoded values that directly affect production behavior

#### 1.1 Fix primal_runtime_discovery.rs (30 min)
- **File**: `crates/beardog-adapters/src/universal/primal_runtime_discovery.rs:127`
- **Issue**: `localhost` fallback in mDNS discovery
- **Fix**: Return error if no address provided
- **Test**: Ensure discovery fails cleanly without valid address

#### 1.2 Fix beardog-integration lib.rs (45 min)
- **File**: `crates/beardog-integration/src/lib.rs:81,137`
- **Issue**: Hardcoded localhost URLs and hosts
- **Fix**: Use configuration hierarchy (ENV > Config > Default)
- **Test**: Verify ENV vars override defaults

#### 1.3 Fix network domain config (30 min)
- **File**: `crates/beardog-config/src/domains/network.rs:118`
- **Issue**: Hard-coded bind address default
- **Fix**: Environment-aware defaults (dev vs prod)
- **Test**: Verify different defaults in debug/release

#### 1.4 Create Configuration Types (45 min)
- Create `NetworkEndpointConfig` struct
- Add env var parsing helpers
- Document configuration hierarchy
- Update relevant docs

---

### Phase 2: Configuration Infrastructure (3-4h)
**Target**: Build robust configuration system

#### 2.1 Configuration Hierarchy Helper (1h)
- Create `beardog-config/src/hierarchy.rs`
- Implement `ConfigValue<T>` with priority resolution
- CLI args > ENV > Config file > Platform defaults
- Type-safe with validation

#### 2.2 Environment Variable Standards (1h)
- Document all ENV vars in ENVIRONMENT_VARIABLES.md
- Prefix: `BEARDOG_*`
- Naming convention: `BEARDOG_<COMPONENT>_<SETTING>`
- Examples: `BEARDOG_BIND_ADDRESS`, `BEARDOG_UPA_URL`

#### 2.3 Platform-Aware Defaults (1-2h)
- Create `beardog-utils/src/platform/defaults.rs`
- Linux defaults (XDG_CONFIG_HOME, /etc/beardog)
- macOS defaults (~/Library/Application Support)
- Windows defaults (%APPDATA%)
- Fallback defaults for unknown platforms

---

### Phase 3: Test Infrastructure Updates (2-3h)
**Target**: Clean up test hardcoding

#### 3.1 Test Constants Module (30 min)
- Create `tests/support/test_constants.rs`
- Define all test IPs, ports, paths in one place
- Document why specific values chosen

#### 3.2 Test Helper Functions (1h)
- `get_test_bind_address()` - Returns unique ephemeral addr
- `get_test_config()` - Returns test-safe config
- `with_temp_config()` - Creates temp config file

#### 3.3 Update Tests to Use Helpers (1-2h)
- Replace inline strings with constants/helpers
- Ensure tests are isolation-safe
- Verify no port conflicts

---

### Phase 4: Documentation & Validation (1-2h)
**Target**: Document and verify changes

#### 4.1 Configuration Documentation (45 min)
- Update CONFIGURATION.md with hierarchy
- Document all ENV vars
- Provide examples for common scenarios
- Migration guide from hardcoded values

#### 4.2 Validation & Testing (45 min)
- Run full test suite
- Test with different ENV var combinations
- Verify production defaults are secure
- Check no regressions

---

## 🎯 SUCCESS CRITERIA

### Measurable Outcomes:
- [ ] Zero hardcoded IPs/ports in production code paths
- [ ] All defaults configurable via ENV vars
- [ ] Platform-aware path resolution
- [ ] Secure defaults (127.0.0.1 dev, 0.0.0.0 prod)
- [ ] Documented configuration hierarchy
- [ ] All tests pass with new patterns
- [ ] No test port conflicts

### Quality Gates:
- [ ] `grep -r "localhost" crates/*/src/ | grep -v test | grep -v example` → Minimal results
- [ ] All `unwrap_or("some_url")` replaced with config hierarchy
- [ ] ENV var documentation complete
- [ ] Migration path documented

---

## 📈 IMPACT ASSESSMENT

### Before:
- ❌ 640 hardcoded instances
- ❌ Silent fallbacks to localhost
- ❌ No configuration flexibility
- ❌ Platform-agnostic file paths
- ❌ Impossible to override defaults

### After:
- ✅ Production code: 0 hardcoded instances
- ✅ Explicit errors for missing config
- ✅ Full configuration hierarchy
- ✅ Platform-aware defaults
- ✅ All defaults overrideable

### Benefits:
1. **Deployment Flexibility**: Configure once, deploy anywhere
2. **Security**: No accidental production localhost
3. **Testing**: Easy to isolate test environments
4. **Sovereignty**: Users control all addresses/paths
5. **Debugging**: Clear errors when config missing

---

## 🚀 STARTING WITH PHASE 1

**Current Focus**: Critical production fixes (2-3h)
**First Target**: `primal_runtime_discovery.rs` localhost fallback

**Philosophy**: "Hardcoding is tech debt. Configuration is sovereignty. Explicit errors beat silent fallbacks. Users control their systems."

---

## 📋 TRACKING

- [ ] Phase 1: Critical fixes (2-3h)
  - [ ] 1.1: primal_runtime_discovery.rs
  - [ ] 1.2: beardog-integration lib.rs
  - [ ] 1.3: network domain config  
  - [ ] 1.4: Configuration types
- [ ] Phase 2: Infrastructure (3-4h)
- [ ] Phase 3: Test cleanup (2-3h)
- [ ] Phase 4: Documentation (1-2h)

**Total**: 8-12 hours
**Status**: Ready to begin Phase 1.1

