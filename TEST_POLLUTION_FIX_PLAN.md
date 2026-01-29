# 🧪 Test Pollution Fix - Implementation Plan

**Date**: January 27, 2026  
**Status**: IN PROGRESS  
**Approach**: Systematic `#[serial]` annotation  
**Estimated Effort**: 2-4 hours

---

## 📊 SCOPE

**Files with env var manipulation**: 60 files identified  
**Strategy**: Add `#[serial]` to tests that manipulate environment variables

---

## 🎯 IMPLEMENTATION STRATEGY

### Phase 1: Test Files (High Priority) ✅

Add `#[serial]` to all test functions in:
- `*_comprehensive_tests.rs` files
- `*_tests.rs` files in test directories
- Integration test files

**Pattern**:
```rust
#[test]
#[serial_test::serial]  // ← Add this
fn test_something() {
    env::set_var("VAR", "value");
    // ...
}
```

---

### Phase 2: Production Code (Verification)

Review production code files for env manipulation:
- Most should be reading only (safe)
- Any `env::set_var` in production is suspicious
- Document any legitimate cases

---

## 📋 FILES TO FIX

### beardog-config (High Priority)

1. ✅ `crypto_comprehensive_tests.rs` (1 test fixed)
2. ⏳ `security_comprehensive_tests.rs`
3. ⏳ `paths_comprehensive_tests.rs`
4. ⏳ `capacity_comprehensive_tests.rs`
5. ⏳ `limits_comprehensive_tests.rs`
6. ⏳ `timeouts_new/builder_comprehensive_tests.rs`
7. ⏳ `network_coverage_extension.rs`
8. ⏳ `loader_tests.rs`

### beardog-tunnel (Medium Priority)

9. ⏳ `tunnel/hsm/manager/mod.rs` (has EnvCleanup but needs serial)
10. ⏳ `unix_socket_ipc_btsp_tests.rs`
11. ⏳ `unix_socket_ipc_schema_tests.rs`
12. ⏳ `tests/btsp_contact_exchange_tests.rs`

### beardog-core (Medium Priority)

13. ⏳ `primal_self_knowledge.rs`
14. ⏳ `self_knowledge.rs`
15. ⏳ `zero_knowledge_bootstrap/self_discovery_tests.rs`
16. ⏳ `zero_knowledge_bootstrap/tests.rs`
17. ⏳ `tests/self_discovery_extended_tests.rs`

### beardog-types (Lower Priority)

18-30. ⏳ Various config and canonical tests

### Other Crates (Lower Priority)

31-60. ⏳ Auth, CLI, integration tests

---

## 🤖 AUTOMATION APPROACH

### Script to Add Serial Annotations

```bash
#!/bin/bash
# add_serial_tests.sh

for file in $(grep -l "env::set_var\|env::remove_var" crates/**/*.rs); do
  # Skip if not a test file
  if ! grep -q "#\[test\]" "$file"; then
    continue
  fi
  
  # Add serial_test to imports if not present
  if ! grep -q "use serial_test::serial" "$file"; then
    # Add after other use statements
    sed -i '/^use /a use serial_test::serial;' "$file"
  fi
  
  # Add #[serial] before #[test] if not present
  sed -i '/#\[test\]/i\    #[serial]' "$file"
done
```

**Risk**: May over-annotate, but safe (serial execution is conservative)

---

## 🎯 PRIORITY ORDER

1. **HIGH**: beardog-config (where pollution occurs)
2. **MEDIUM**: beardog-tunnel (HSM tests)
3. **MEDIUM**: beardog-core (discovery tests)
4. **LOW**: Other crates (less impactful)

---

## ✅ VALIDATION

After applying fixes:

```bash
# Test that all tests pass in parallel
cargo test --lib --workspace

# Expected: 1373/1373 pass
```

---

## 📊 PROGRESS TRACKING

- [ ] Phase 1: beardog-config (8 files)
- [ ] Phase 2: beardog-tunnel (4 files)
- [ ] Phase 3: beardog-core (5 files)
- [ ] Phase 4: beardog-types (12 files)
- [ ] Phase 5: Other crates (31 files)
- [ ] Validation: Full test suite passes

---

**Status**: STARTING  
**ETA**: 2-4 hours

🐻 BearDog: Test Pollution Fix - Systematic Approach 🧪

