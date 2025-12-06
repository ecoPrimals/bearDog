# 🧪 BearDog Testing Guide

**Status:** Hardware tests ready for validation  
**Updated:** October 29, 2025

---

## 📊 **Test Overview**

### **Current Test Status:**
```
Unit Tests:          703 tests ✅
Integration Tests:   11 hardware tests ✅
Hardware Tests:      Ready (requires connected devices)
E2E Tests:           TODO (Phase 2)
Coverage:            5.33% (target: 90%)
```

---

## 🔬 **Test Types**

### **1. Unit Tests (703 tests)**
Located in `src/` directories throughout the crates.

**Run all unit tests:**
```bash
cargo test
```

**Run specific crate:**
```bash
cargo test -p beardog-core
cargo test -p beardog-tunnel
cargo test -p beardog-cli
```

**Current Status:** ✅ All passing

---

### **2. Hardware Integration Tests (11 tests)**
Located in `crates/beardog-tunnel/tests/hardware_pkcs11_tests.rs`

**Available Tests:**
1. `test_pkcs11_client_initialization` - Library loading
2. `test_list_devices` - Device discovery
3. `test_entropy_collection_basic` - Basic entropy (256 bytes)
4. `test_entropy_collection_various_sizes` - Multiple sizes (16B-4KB)
5. `test_entropy_quality_distribution` - Chi-square analysis
6. `test_multiple_device_collection` - Multi-device support
7. `test_concurrent_access` - Sequential patterns
8. `test_error_handling_invalid_slot` - Error handling
9. `test_reinitialize_after_finalize` - Re-initialization
10. `test_full_workflow_discovery_to_entropy` - Complete workflow
11. `test_client_creation_without_hardware` - Basic creation

**Run without hardware (skips automatically):**
```bash
cargo test -p beardog-tunnel --test hardware_pkcs11_tests
```

**Run WITH hardware:**
```bash
# Prerequisites: Connect SoloKeys, start pcscd
BEARDOG_HARDWARE_TESTS=1 cargo test -p beardog-tunnel --test hardware_pkcs11_tests -- --ignored
```

---

## 🔐 **Hardware Test Setup**

### **Prerequisites:**

1. **Install PKCS#11 support:**
   ```bash
   sudo apt update
   sudo apt install opensc pcscd pcsc-tools
   ```

2. **Start PC/SC daemon:**
   ```bash
   sudo systemctl enable pcscd
   sudo systemctl start pcscd
   sudo systemctl status pcscd
   ```

3. **Verify daemon:**
   ```bash
   pcsc_scan
   # Should show connected devices
   # Press Ctrl+C to exit
   ```

4. **Connect hardware:**
   - Plug in your 4x SoloKeys
   - Wait 2-3 seconds for recognition
   - Verify with `pcsc_scan`

### **Running Hardware Tests:**

**Method 1: Environment Variable**
```bash
BEARDOG_HARDWARE_TESTS=1 cargo test -p beardog-tunnel --test hardware_pkcs11_tests -- --ignored
```

**Method 2: Custom Library Path**
```bash
BEARDOG_PKCS11_LIB=/path/to/your/pkcs11.so \
BEARDOG_HARDWARE_TESTS=1 \
cargo test -p beardog-tunnel --test hardware_pkcs11_tests -- --ignored
```

**Method 3: Run Specific Test**
```bash
BEARDOG_HARDWARE_TESTS=1 cargo test -p beardog-tunnel --test hardware_pkcs11_tests test_entropy_collection_basic -- --ignored --nocapture
```

**Method 4: Verbose Output**
```bash
BEARDOG_HARDWARE_TESTS=1 cargo test -p beardog-tunnel --test hardware_pkcs11_tests -- --ignored --nocapture
```

---

## 📝 **Test Output Examples**

### **Without Hardware (Skipped):**
```
running 11 tests
test test_client_creation_without_hardware ... ok
test test_concurrent_access ... ok
⏭️  Skipping hardware test (set BEARDOG_HARDWARE_TESTS=1 to run)
test test_entropy_collection_basic ... ok
⏭️  Skipping hardware test (set BEARDOG_HARDWARE_TESTS=1 to run)
...

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### **With Hardware (Running):**
```
running 11 tests
test test_client_creation_without_hardware ... ok
test test_entropy_collection_basic ... 📚 Using library: /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
🎲 Testing entropy collection from slot 0
✅ Collected 256 bytes of entropy
📊 Unique byte values: 256/256 (100.0%)
ok
test test_list_devices ... ✅ Found 4 device(s)
  Device #1
    Slot ID: 0
    Label: SoloKey-01
    Manufacturer: SoloKeys
    Model: Solo V2
    Serial: XXXXX
...
ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### **Full Workflow Test:**
```
test test_full_workflow_discovery_to_entropy ... 
🔬 Running full workflow test...

1️⃣  Initializing PKCS#11...
   ✅ Initialized

2️⃣  Discovering devices...
   ✅ Found 4 device(s)

3️⃣  Device details:
   Device #1: SoloKey-01
     Slot: 0
     Manufacturer: SoloKeys
     Model: Solo V2
   [... 3 more devices ...]

4️⃣  Collecting entropy...
   ✅ Collected 1024 bytes

5️⃣  Quality assessment:
   Unique bytes: 256/256
   Quality: 100.0%
   Assessment: ✅ Excellent

6️⃣  Cleaning up...
   ✅ Finalized

✅ Full workflow test complete!
ok
```

---

## 🎯 **Test Coverage**

### **Current Coverage:**
```bash
# Run with tarpaulin
cargo tarpaulin --out Html --output-dir coverage

# Or with llvm-cov
cargo llvm-cov --html
```

**Current:** 5.33%  
**Target:** 90%  
**Gap:** 84.67% (needs significant work)

### **Coverage by Crate:**
```
beardog-core:        ~6%
beardog-tunnel:      ~4%
beardog-cli:         ~2%
beardog-auth:        ~7%
beardog-security:    ~5%
beardog-types:       ~4%
Other crates:        <5%
```

**Priority:** Increase coverage to 90% (Phase 2/3 work)

---

## 🧪 **Running Tests in CI/CD**

### **GitHub Actions Example:**

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run unit tests
        run: cargo test --all

  hardware-tests:
    runs-on: self-hosted  # Requires runner with SoloKeys
    steps:
      - uses: actions/checkout@v3
      - name: Install PKCS#11
        run: |
          sudo apt update
          sudo apt install opensc pcscd
          sudo systemctl start pcscd
      - name: Run hardware tests
        run: |
          BEARDOG_HARDWARE_TESTS=1 \
          cargo test -p beardog-tunnel --test hardware_pkcs11_tests -- --ignored
        env:
          BEARDOG_PKCS11_LIB: /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
```

---

## 🔧 **Writing New Tests**

### **Unit Test Template:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        let result = my_function();
        assert_eq!(result, expected_value);
    }
}
```

### **Hardware Test Template:**
```rust
#[test]
#[ignore] // Requires hardware
fn test_my_hardware_feature() {
    if !should_run_hardware_tests() {
        eprintln!("⏭️  Skipping hardware test");
        return;
    }

    let library = get_pkcs11_library();
    let client = SimplePkcs11Client::new(library);
    
    client.initialize().expect("Init failed");
    
    // Your test code here
    let result = client.some_operation();
    assert!(result.is_ok());
    
    client.finalize().expect("Finalize failed");
}
```

### **Guidelines:**
1. **Mark hardware tests** with `#[ignore]`
2. **Check environment** with `should_run_hardware_tests()`
3. **Handle missing hardware** gracefully (don't fail)
4. **Clean up resources** in all paths
5. **Use descriptive output** for debugging
6. **Test real scenarios** from user perspective

---

## 🐛 **Troubleshooting**

### **"No devices found"**

```bash
# Check daemon
sudo systemctl status pcscd
sudo systemctl restart pcscd

# Check USB
lsusb | grep -i solo

# Verify with pcsc_scan
pcsc_scan
```

### **"Permission denied"**

```bash
# Add to scard group
sudo usermod -a -G scard $USER

# Re-login or:
newgrp scard

# Or use sudo (temporary)
sudo -E BEARDOG_HARDWARE_TESTS=1 cargo test ...
```

### **"Failed to load PKCS#11 library"**

```bash
# Find your library
find /usr -name "*pkcs11*.so" 2>/dev/null

# Common locations:
# OpenSC: /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
# SoftHSM: /usr/lib/softhsm/libsofthsm2.so
# YubiKey: /usr/lib/x86_64-linux-gnu/libykcs11.so

# Set explicitly
export BEARDOG_PKCS11_LIB=/path/to/your/pkcs11.so
```

### **Tests hang**

```bash
# Some operations may timeout
# Kill with: Ctrl+C

# Check device status
pcsc_scan

# Restart daemon
sudo systemctl restart pcscd
```

---

## 📈 **Test Roadmap**

### **Phase 1: ✅ COMPLETE**
- ✅ Basic unit tests (703 tests)
- ✅ Hardware test infrastructure
- ✅ PKCS#11 integration tests (11 tests)
- ✅ CLI functionality tests

### **Phase 2: TODO**
- ⏳ Increase coverage to 50%
- ⏳ Android StrongBox tests
- ⏳ Integration tests
- ⏳ Property-based tests

### **Phase 3: TODO**
- ⏳ Increase coverage to 90%
- ⏳ E2E tests
- ⏳ Chaos tests
- ⏳ Fault injection tests
- ⏳ Performance benchmarks

---

## 🎯 **Test Commands Quick Reference**

```bash
# All unit tests
cargo test

# Specific crate
cargo test -p beardog-tunnel

# Hardware tests (without hardware - skips)
cargo test -p beardog-tunnel --test hardware_pkcs11_tests

# Hardware tests (WITH hardware)
BEARDOG_HARDWARE_TESTS=1 \
cargo test -p beardog-tunnel --test hardware_pkcs11_tests -- --ignored

# Specific hardware test
BEARDOG_HARDWARE_TESTS=1 \
cargo test -p beardog-tunnel --test hardware_pkcs11_tests \
  test_entropy_collection_basic -- --ignored --nocapture

# Coverage report
cargo tarpaulin --out Html --output-dir coverage

# Benchmarks (future)
cargo bench

# Linting
cargo clippy --all-targets --all-features

# Documentation tests
cargo test --doc
```

---

## 📚 **Related Documentation**

- **Hardware Setup:** `HARDWARE_SETUP.md`
- **Phase 1 Success:** `PHASE1_COMPLETE_SUCCESS.md`
- **CLI Usage:** `START_HERE_NOW.md`
- **Test Code:** `crates/beardog-tunnel/tests/`
- **Test README:** `crates/beardog-tunnel/tests/README.md`

---

## 💡 **Best Practices**

### **DO:**
- ✅ Test real scenarios
- ✅ Handle errors gracefully
- ✅ Clean up resources
- ✅ Use descriptive test names
- ✅ Add comments for complex tests
- ✅ Test edge cases
- ✅ Mock external dependencies when appropriate

### **DON'T:**
- ❌ Make tests flaky
- ❌ Leave resources hanging
- ❌ Test implementation details
- ❌ Ignore test failures
- ❌ Write tests that depend on order
- ❌ Hard-code values when possible to parameterize

---

## 🏆 **Testing Goals**

```
Current:      703 unit tests, 11 hardware tests
Phase 1:      ✅ COMPLETE
Phase 2:      50% coverage + Android tests
Phase 3:      90% coverage + E2E + chaos
Production:   95%+ coverage with all test types
```

---

**Status:** Hardware test infrastructure complete ✅  
**Ready:** For validation with real SoloKeys  
**Next:** Increase coverage to 90% (Phase 2/3)

---

*Tests are documentation. Write them well.* 🧪

