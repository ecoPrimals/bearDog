# BearDog Tunnel Integration Tests

This directory contains integration tests for the `beardog-tunnel` crate, including real hardware tests for PKCS#11 devices.

## Test Types

### 1. **Unit Tests** (in `src/`)
Standard tests within the source files - run automatically with `cargo test`.

### 2. **Integration Tests** (in `tests/`)
- `hardware_pkcs11_tests.rs` - Real PKCS#11 hardware tests (requires connected devices)

## Running Hardware Tests

### Prerequisites

1. **Install PKCS#11 support:**
   ```bash
   sudo apt install opensc pcscd pcsc-tools
   sudo systemctl start pcscd
   ```

2. **Connect hardware:**
   - Connect your SoloKeys or other PKCS#11 devices
   - Verify with `pcsc_scan`

### Running Tests

**Skip hardware tests (default):**
```bash
cargo test --test hardware_pkcs11_tests
# All hardware tests are #[ignore]'d by default
```

**Run hardware tests:**
```bash
# Method 1: Use --ignored flag
cargo test --test hardware_pkcs11_tests -- --ignored

# Method 2: Set environment variable
BEARDOG_HARDWARE_TESTS=1 cargo test --test hardware_pkcs11_tests

# Method 3: Run specific test
cargo test --test hardware_pkcs11_tests test_list_devices -- --ignored
```

**Custom PKCS#11 library:**
```bash
BEARDOG_PKCS11_LIB=/path/to/your/pkcs11.so \
BEARDOG_HARDWARE_TESTS=1 \
cargo test --test hardware_pkcs11_tests -- --ignored
```

## Available Hardware Tests

### Basic Tests
- `test_pkcs11_client_initialization` - Test PKCS#11 library loading
- `test_list_devices` - Discover connected HSM devices
- `test_client_creation_without_hardware` - Creation without hardware (always runs)

### Entropy Tests
- `test_entropy_collection_basic` - Collect 256 bytes
- `test_entropy_collection_various_sizes` - Test 16B to 4KB
- `test_entropy_quality_distribution` - Chi-square test for randomness
- `test_multiple_device_collection` - Collect from multiple devices

### Robustness Tests
- `test_concurrent_access` - Sequential access patterns
- `test_error_handling_invalid_slot` - Invalid slot handling
- `test_reinitialize_after_finalize` - Re-initialization behavior

### Workflow Tests
- `test_full_workflow_discovery_to_entropy` - Complete CLI simulation

## Test Output

**Successful test:**
```
test test_entropy_collection_basic ... ok
📊 Unique byte values: 256/256 (100.0%)
✅ Collected 256 bytes of entropy
```

**Skipped test (no hardware):**
```
test test_list_devices ... ok
⏭️  Skipping hardware test (set BEARDOG_HARDWARE_TESTS=1 to run)
```

**Failed test:**
```
test test_entropy_quality_distribution ... FAILED
assertion failed: unique_bytes >= 200
Poor entropy: only 150 unique bytes
```

## CI/CD Integration

### GitHub Actions

```yaml
# .github/workflows/test.yml
jobs:
  test-hardware:
    runs-on: self-hosted  # Requires runner with hardware
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
          cargo test --test hardware_pkcs11_tests -- --ignored
        env:
          BEARDOG_PKCS11_LIB: /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
```

### Local Development

Add to `.cargo/config.toml`:
```toml
[env]
BEARDOG_PKCS11_LIB = "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so"

[alias]
test-hw = "test --test hardware_pkcs11_tests -- --ignored"
test-hw-verbose = "test --test hardware_pkcs11_tests -- --ignored --nocapture"
```

Then run:
```bash
BEARDOG_HARDWARE_TESTS=1 cargo test-hw
```

## Troubleshooting

### "No devices found"

**Check PC/SC daemon:**
```bash
sudo systemctl status pcscd
sudo systemctl restart pcscd
```

**Verify USB devices:**
```bash
lsusb | grep -i solo
pcsc_scan
```

**Check permissions:**
```bash
sudo usermod -a -G scard $USER
newgrp scard
```

### "Failed to load PKCS#11 library"

**Find your library:**
```bash
find /usr -name "*pkcs11*.so" 2>/dev/null
```

**Common locations:**
- OpenSC: `/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so`
- SoftHSM: `/usr/lib/softhsm/libsofthsm2.so`
- YubiKey: `/usr/lib/x86_64-linux-gnu/libykcs11.so`

**Set explicitly:**
```bash
export BEARDOG_PKCS11_LIB=/path/to/your/pkcs11.so
```

### "Permission denied"

**Run with sudo (temporary):**
```bash
sudo -E BEARDOG_HARDWARE_TESTS=1 cargo test --test hardware_pkcs11_tests -- --ignored
```

**Or fix permissions:**
```bash
sudo usermod -a -G scard $USER
# Log out and back in
```

## Writing New Hardware Tests

### Template

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
    
    client.finalize().expect("Finalize failed");
}
```

### Guidelines

1. **Always check `should_run_hardware_tests()`** at the start
2. **Mark with `#[ignore]`** so tests don't run by default
3. **Handle missing hardware gracefully** - don't fail if no devices
4. **Clean up** - always call `finalize()`
5. **Use descriptive output** - print progress for debugging
6. **Test real scenarios** - simulate actual CLI usage

## Performance Benchmarks

Run benchmarks with hardware:
```bash
BEARDOG_HARDWARE_TESTS=1 cargo bench --bench pkcs11_performance
```

(Benchmarks to be implemented in `benches/` directory)

## Documentation

- **Project Overview:** `../../START_HERE.md`
- **Architecture:** `../../ARCHITECTURE.md`

## Contributing

When adding new hardware tests:
1. Follow the template above
2. Add entry to this README
3. Test with and without hardware
4. Document any special requirements
5. Include in CI/CD if applicable

---

**Last Updated:** May 4, 2026  
**Status:** Hardware test infrastructure in place

