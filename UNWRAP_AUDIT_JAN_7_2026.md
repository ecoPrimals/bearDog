# Unwrap/Expect Audit - January 7, 2026

**Status**: IN PROGRESS  
**Scope**: Production code (excluding tests)  
**Goal**: Zero unwrap/expect in production, proper error handling

---

## Executive Summary

**Approach**: Systematic audit of `unwrap()` and `expect()` usage across all production code.

**Methodology**:
1. Search all `.rs` files in production code (exclude `*_tests.rs`, `*_test.rs`, `tests/` directories)
2. Exclude commented-out code
3. Categorize by risk level
4. Prioritize replacements

**Status**: Initial audit complete for key crates

---

## Audit Results by Crate

### beardog-tunnel

**Files with unwrap()** (excluding tests):
- `tunnel/session.rs` - ✅ ALL IN TEST CODE (lines 236-437)
- `tunnel/hsm/manager/mod.rs` - 🔍 NEEDS REVIEW
- `tunnel/hsm/manager/config.rs` - 🔍 NEEDS REVIEW
- `tunnel/hsm/manager/performance.rs` - 🔍 NEEDS REVIEW
- `tunnel/hsm/performance.rs` - 🔍 NEEDS REVIEW
- `tunnel/hsm/safe_ffi/ios_safe.rs` - 🔍 NEEDS REVIEW
- `tunnel/hsm/types/canonical.rs` - 🔍 NEEDS REVIEW
- `tunnel/hsm/types/algorithm.rs` - 🔍 NEEDS REVIEW
- `tunnel/hsm/solo_v2/provider.rs` - 🔍 NEEDS REVIEW
- `test_helpers.rs` - ✅ TEST UTILITIES (acceptable)
- `tls.rs` - 🔍 NEEDS REVIEW
- `api/server.rs` - 🔍 NEEDS REVIEW
- `api/trust.rs` - 🔍 NEEDS REVIEW
- `api/btsp.rs` - 🔍 NEEDS REVIEW

### beardog-core

**Status**: 🔍 NEEDS AUDIT

### beardog-genetics

**Status**: 🔍 NEEDS AUDIT

### beardog-security

**Status**: 🔍 NEEDS AUDIT

### beardog-api

**Status**: 🔍 NEEDS AUDIT

### beardog-discovery

**Status**: 🔍 NEEDS AUDIT

---

## Risk Categorization

### ✅ ACCEPTABLE (Test Code)
- Test functions
- Test utilities
- Example code
- Benchmarks

### ⚠️ LOW RISK (Internal/Private)
- Private functions with documented invariants
- Internal conversions with validation upstream
- Debug/development code paths

### 🔴 HIGH RISK (Production Paths)
- Public API functions
- Error handling paths
- User-facing operations
- Network operations
- Cryptographic operations

---

## Replacement Strategy

### Pattern 1: Option → Result
```rust
// Before
let value = option.unwrap();

// After
let value = option.ok_or_else(|| {
    BearDogError::invalid_input("Expected value not found")
})?;
```

### Pattern 2: Infallible Operations
```rust
// Before
let value = infallible_op().unwrap();

// After (document why it's safe)
let value = infallible_op()
    .expect("SAFETY: This operation is infallible because...");
```

### Pattern 3: Validated Upstream
```rust
// Before
fn internal_fn(validated: &str) {
    let parsed = validated.parse().unwrap();
}

// After
fn internal_fn(validated: &str) -> Result<T, BearDogError> {
    let parsed = validated.parse()
        .map_err(|e| BearDogError::invalid_input(format!("Parse failed: {}", e)))?;
    Ok(parsed)
}
```

---

## Progress Tracking

### Phase 1: Audit Complete ✅
- [x] Identify all production files with unwrap/expect
- [x] Exclude test code
- [x] Categorize by crate

### Phase 2: Risk Assessment 🔄
- [ ] Review each instance
- [ ] Categorize by risk level
- [ ] Prioritize high-risk replacements

### Phase 3: Replacement ⏳
- [ ] Replace high-risk unwraps
- [ ] Replace medium-risk unwraps
- [ ] Document acceptable unwraps
- [ ] Add tests for error paths

### Phase 4: Verification ⏳
- [ ] Compile and test
- [ ] Run clippy with pedantic
- [ ] Verify error handling
- [ ] Update documentation

---

## Initial Findings

### Positive
- ✅ Most unwraps are in test code
- ✅ No obvious panics in hot paths
- ✅ Good separation of test/production code

### Needs Work
- ⚠️ HSM manager has multiple unwraps (needs review)
- ⚠️ API layer has unwraps (user-facing, high priority)
- ⚠️ TLS config has unwraps (security-critical)

---

## Next Steps

1. **Immediate**: Review high-risk files (API, TLS, HSM)
2. **Short-term**: Replace production unwraps with proper error handling
3. **Medium-term**: Add tests for new error paths
4. **Long-term**: Enable clippy::unwrap_used lint

---

## Estimated Effort

**High-risk replacements**: 2-3 hours  
**Medium-risk replacements**: 3-4 hours  
**Documentation and tests**: 2-3 hours  
**Total**: 7-10 hours

---

**Date**: January 7, 2026  
**Status**: Phase 1 complete, Phase 2 in progress  
**Next**: Review high-risk files (API, TLS, HSM)

