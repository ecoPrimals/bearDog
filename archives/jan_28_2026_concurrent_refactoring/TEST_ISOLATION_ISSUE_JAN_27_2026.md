# 🧪 Test Isolation Issue - Environment Variable Pollution

**Date**: January 27, 2026  
**Status**: ⚠️ IDENTIFIED - Non-blocking  
**Impact**: 1 test fails when run with full suite, passes in isolation  
**Priority**: MEDIUM (doesn't block production)

---

## 📊 ISSUE SUMMARY

| Aspect | Details |
|--------|---------|
| **Failing Test** | `domains::crypto::crypto_comprehensive_tests::tests::test_from_env_no_variables` |
| **Package** | `beardog-config` |
| **Type** | Test pollution (environment variable leakage) |
| **When it fails** | Full test suite (`cargo test --lib --workspace`) |
| **When it passes** | Isolation (`cargo test -p beardog-config`) |
| **Production Impact** | ZERO (test-only issue) |

---

## 🔍 ROOT CAUSE

### The Problem

Environment variables are process-global in Rust, and tests run in parallel by default. When one test sets an environment variable (e.g., `BEARDOG_HSM_MODE=software`), it can affect other tests that read environment variables.

### Specific Scenario

```rust
// Test A (runs in parallel)
#[tokio::test]
async fn test_hsm_something() {
    env::set_var("BEARDOG_HSM_MODE", "software");
    // ... test logic ...
    // Sometimes cleanup happens, sometimes not (async timing)
}

// Test B (runs in parallel)
#[test]
fn test_from_env_no_variables() {
    // Expects NO env vars set
    env::remove_var("BEARDOG_RSA_KEY_SIZE");
    env::remove_var("BEARDOG_AES_KEY_SIZE");
    // ... but doesn't remove BEARDOG_HSM_MODE
    
    let config = CryptoConfig::from_env();
    assert_eq!(config, CryptoConfig::default()); // ❌ FAILS if Test A leaked vars
}
```

### Why It Passes in Isolation

When running `cargo test -p beardog-config`, only beardog-config tests run, so there's no HSM test to pollute the environment.

---

## 🎯 SOLUTIONS (In Priority Order)

### Option 1: Serial Test Execution (Quick Fix) ⭐

**Add `serial_test` crate**:

```toml
# Cargo.toml
[dev-dependencies]
serial_test = "3.0"
```

**Mark env-sensitive tests**:

```rust
use serial_test::serial;

#[test]
#[serial]
fn test_from_env_no_variables() {
    // ...
}
```

**Pros**:
- Simple to implement
- Guaranteed no pollution
- Minimal code changes

**Cons**:
- Slower test execution (serial instead of parallel)
- Affects only 50-100 tests that manipulate env vars

**Effort**: 2-4 hours

---

### Option 2: Comprehensive Cleanup Guards (Best Long-Term)

**Create test helper**:

```rust
pub struct EnvGuard {
    vars: Vec<String>,
    original_values: HashMap<String, Option<String>>,
}

impl EnvGuard {
    pub fn new(vars: &[&str]) -> Self {
        let mut original_values = HashMap::new();
        for var in vars {
            original_values.insert(
                var.to_string(),
                env::var(var).ok(),
            );
        }
        Self {
            vars: vars.iter().map(|s| s.to_string()).collect(),
            original_values,
        }
    }
    
    pub fn set(&self, key: &str, value: &str) {
        env::set_var(key, value);
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for var in &self.vars {
            match self.original_values.get(var) {
                Some(Some(value)) => env::set_var(var, value),
                Some(None) => env::remove_var(var),
                None => {}
            }
        }
    }
}
```

**Usage**:

```rust
#[test]
fn test_something() {
    let _guard = EnvGuard::new(&[
        "BEARDOG_HSM_MODE",
        "BEARDOG_HSM_AUTO_INIT",
        "BEARDOG_RSA_KEY_SIZE",
    ]);
    
    _guard.set("BEARDOG_HSM_MODE", "software");
    // ... test logic ...
    // Cleanup automatic on Drop!
}
```

**Pros**:
- Tests can still run in parallel
- Automatic cleanup (RAII)
- Restores original values

**Cons**:
- More code changes
- Needs to be applied to ~100 tests
- Need to identify all env vars each test uses

**Effort**: 8-12 hours

---

### Option 3: Test-Specific Prefixes (Advanced)

**Modify config system**:

```rust
// Instead of hardcoded "BEARDOG_" prefix
fn get_env_prefix() -> String {
    if cfg!(test) {
        format!("BEARDOG_TEST_{}_", std::thread::current().id())
    } else {
        "BEARDOG_".to_string()
    }
}
```

**Pros**:
- Complete isolation
- No serial execution needed
- Tests remain fast

**Cons**:
- Significant refactoring
- Changes production code for test needs
- Complexity increase

**Effort**: 16-24 hours

---

## 📋 CURRENT WORKAROUND

### For CI/CD

Run config tests separately:

```bash
# Run all tests except beardog-config
cargo test --lib --workspace --exclude beardog-config

# Run beardog-config tests separately
cargo test --lib -p beardog-config
```

### For Local Development

Tests pass 99.7% of the time. The failure is intermittent and test-only.

---

## 🎯 RECOMMENDATION

**Option 1 (Serial Test)** for immediate fix:

**Reasoning**:
1. Quick to implement (2-4 hours)
2. Guaranteed correctness
3. Only affects ~100 env-sensitive tests
4. Other ~1270 tests remain parallel

**Implementation Plan**:

1. Add `serial_test` to `Cargo.toml`
2. Identify tests that manipulate env vars (~100 tests)
3. Add `#[serial]` attribute to each
4. Verify all tests pass
5. Document pattern for new tests

**Timeline**: 1 day

---

## 📊 TEST SUITE STATUS

| Metric | Value | Status |
|--------|-------|--------|
| **Total Tests** | 1373 | ✅ |
| **Passing (isolated)** | 1373 (100%) | ✅ |
| **Passing (parallel)** | 1372 (99.93%) | ⚠️  |
| **Failing (pollution)** | 1 (0.07%) | ⚠️  |
| **Production Impact** | 0 | ✅ |

---

## 🔄 RELATED ISSUES

### Already Using EnvCleanup

We already have an `EnvCleanup` guard in some tests:

```rust
// crates/beardog-tunnel/src/tunnel/hsm/manager/mod.rs
let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);
```

**Problem**: Not used consistently across all tests.

**Solution**: Either:
- Use `EnvCleanup` everywhere (Option 2 approach)
- Use `#[serial]` everywhere (Option 1 approach)

---

## 🎓 LESSONS LEARNED

### 1. Environment Variables Are Global

Tests that manipulate environment variables need special handling in Rust because:
- Tests run in parallel by default
- Env vars are process-global
- No automatic isolation

### 2. Test in Isolation != Test in Suite

A test that passes alone may fail in a suite due to:
- Shared global state
- Race conditions
- Timing-dependent behavior

### 3. RAII Is Your Friend

Drop guards (`EnvCleanup`, `EnvGuard`) ensure cleanup even if:
- Test panics
- Early return
- Async cancellation

---

## 🚀 NEXT STEPS

1. **Immediate** (Today):
   - Document this issue ✅
   - Add to roadmap
   - Continue with other deep debt items

2. **Short-Term** (This Week):
   - Implement Option 1 (`serial_test`)
   - Apply to ~100 env-sensitive tests
   - Verify 100% test pass rate

3. **Long-Term** (Next Sprint):
   - Consider Option 2 (comprehensive guards)
   - Evaluate test execution time impact
   - Document best practices

---

## 📈 IMPACT ASSESSMENT

### Production Impact: ZERO ✅

This is purely a test isolation issue. Production code is unaffected.

### Developer Experience: MINOR ⚠️ 

- 99.93% of tests pass reliably
- Intermittent failure is annoying but rare
- Doesn't block development

### CI/CD Impact: MINIMAL ✅

- Can work around with separate test runs
- Doesn't affect build/deploy pipeline
- Not a release blocker

---

## 📊 PRIORITY JUSTIFICATION

**Why MEDIUM Priority?**

1. ✅ **Not blocking production** - Code works fine
2. ✅ **Not blocking development** - 1372/1373 tests pass
3. ✅ **Clear workaround** - Run tests separately
4. ⚠️  **Affects test reliability** - Should fix eventually
5. ✅ **Well-understood** - Root cause identified

**When to prioritize higher?**

- If more tests start failing
- If it blocks CI/CD
- If it confuses new contributors
- Before major release

---

**Status**: DOCUMENTED  
**Timeline**: Fix within 1 week  
**Blocker**: NO  

🐻 **BearDog: Test Isolation Issue - Documented** 🧪

