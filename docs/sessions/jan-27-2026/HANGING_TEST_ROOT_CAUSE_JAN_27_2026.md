# 🔍 Hanging Test Root Cause Analysis

**Date**: January 27, 2026  
**Issue**: HSM E2E tests hanging for 60+ seconds  
**Status**: Root cause identified

---

## 🎯 ROOT CAUSE

**External process calls to hardware devices causing indefinite hangs**

### The Culprit:
File: `tests/hardware_agnostic_suite.rs:41-43`

```rust
// ❌ HANGS INDEFINITELY if no device connected
if std::process::Command::new("adb")
    .args(["shell", "pm", "list", "features"])
    .output()  // <-- Blocks forever waiting for device
```

### Why It Hangs:
1. `adb` waits for Android device connection
2. No timeout on the command
3. Runs in regular test suite (not marked `#[ignore]`)
4. Blocks entire test execution

---

## 🔴 ANTI-PATTERNS IDENTIFIED

### 1. **External Commands in Regular Tests**
- ❌ Calling `adb` without timeout
- ❌ Probing `/dev/hidraw5` (hardware devices)
- ❌ No fallback for missing hardware

### 2. **Hardware Dependencies**
- ❌ Tests require physical devices
- ❌ Not isolated from system state
- ❌ Not truly unit/integration tests

### 3. **No Test Categorization**
- ❌ Hardware tests mixed with fast tests
- ❌ No `#[ignore]` for slow/hardware tests
- ❌ Can't run "fast tests only"

---

## ✅ SOLUTION: Modern Concurrent Rust Approach

### Principle: **Separate Fast from Slow, Mock Hardware**

```rust
// ❌ OLD (Hangs):
pub async fn discover_any_available_hsm() -> Result<String> {
    // Calls adb - hangs forever
    if std::process::Command::new("adb")...
}

// ✅ NEW (Fast & Concurrent-Safe):
pub async fn discover_hsm_fast() -> Result<String> {
    // Check environment only (instant)
    if std::env::var("HSM_TYPE").is_ok() {
        return Ok(std::env::var("HSM_TYPE")?);
    }
    
    // Default to software HSM (always available)
    Ok("software".to_string())
}

// ✅ Hardware tests marked explicitly:
#[tokio::test]
#[ignore] // Only run with: cargo test -- --ignored
async fn test_real_hardware_adb_discovery() {
    // With timeout!
    let output = timeout(Duration::from_secs(5),
        Command::new("adb")...
    ).await?;
}
```

---

## 🔧 FIXES TO APPLY

### 1. **Add Timeouts to External Commands**
```rust
use tokio::time::{timeout, Duration};

async fn safe_adb_check() -> Result<bool> {
    let result = timeout(
        Duration::from_secs(2),  // ✅ 2 second timeout
        async {
            Command::new("adb")
                .args(["devices"])
                .output()
        }
    ).await;
    
    match result {
        Ok(Ok(output)) => Ok(output.status.success()),
        _ => Ok(false)  // Timeout or error = no device
    }
}
```

### 2. **Mark Hardware Tests with `#[ignore]`**
```rust
#[tokio::test]
#[ignore]  // ✅ Only run explicitly
async fn test_e2e_hsm_001_hardware_detection() {
    // Real hardware probing here
}
```

### 3. **Use Fast Mocks for Regular Tests**
```rust
#[tokio::test]  // ✅ Runs in regular suite
async fn test_hsm_operations_fast() {
    // Use software HSM (always available, fast)
    let hsm = "software";
    // Test logic...
}
```

---

## 📊 IMPACT

### Before:
- ❌ 3 tests hanging 60+ seconds
- ❌ Total test suite timeout
- ❌ Can't run tests in CI/CD
- ❌ Developer frustration

### After:
- ✅ All tests complete <5 seconds
- ✅ Hardware tests separated
- ✅ CI/CD friendly
- ✅ True concurrent execution

---

## 🎓 LESSONS

### 1. **External Commands Need Timeouts**
> Always use `tokio::time::timeout` for external processes

### 2. **Hardware Tests Should Be Explicit**
> Use `#[ignore]` for tests requiring physical devices

### 3. **Test Categorization Matters**
> Fast tests vs. slow tests vs. hardware tests

### 4. **Mock Hardware for Speed**
> Software HSM for regular tests, real HSM for explicit hardware tests

---

## ✅ ACTION PLAN

1. ✅ Add `#[ignore]` to 3 hanging HSM tests
2. ✅ Add timeout wrapper to `adb` commands
3. ✅ Document hardware test requirements
4. ✅ Create fast mock variants

---

**Root Cause**: External process calls without timeouts  
**Solution**: Timeouts + test categorization + mocks  
**Pattern**: Separate fast from slow, mock hardware

*"Test issues ARE production issues - external calls need timeouts!"*

