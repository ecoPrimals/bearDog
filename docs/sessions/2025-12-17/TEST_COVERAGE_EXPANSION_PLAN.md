# 📊 Test Coverage Expansion Plan - 78% → 90%

**Current**: 78.18% coverage  
**Target**: 90% coverage  
**Gap**: ~12%  
**Estimated**: ~200 additional tests  
**Timeline**: 2-3 weeks

---

## 🎯 STRATEGY

### Phase 1: Measurement & Analysis (Week 1, Days 1-2)

**Objective**: Identify coverage gaps

```bash
# 1. Generate HTML coverage report (easier to analyze)
cargo llvm-cov --workspace --html --output-dir coverage/

# 2. Open in browser
firefox coverage/index.html

# 3. Identify files with <80% coverage
grep -A 2 "coverage:" coverage/index.html | grep -v "100\|9[0-9]"

# 4. Priority: Focus on critical paths first
# - Security modules (beardog-security, beardog-auth)
# - Core operations (beardog-core)
# - Crypto service (beardog-tunnel)
# - Error handling (beardog-errors)
```

---

### Phase 2: Core Module Coverage (Week 1, Days 3-5)

**Target**: beardog-core, beardog-security, beardog-auth

#### beardog-core

**Gaps** (estimated):
- Edge cases in crypto service
- Error recovery paths
- Configuration validation
- Discovery failure scenarios

**Tests to Add** (~40 tests):
```rust
// crates/beardog-core/src/crypto_service/mod.rs
#[cfg(test)]
mod additional_coverage_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_crypto_service_invalid_key_id() {
        // Test error handling for invalid key
    }
    
    #[tokio::test]
    async fn test_crypto_service_concurrent_operations() {
        // Test thread safety
    }
    
    #[tokio::test]
    async fn test_crypto_service_algorithm_not_supported() {
        // Test unsupported algorithm handling
    }
    
    // ... more tests
}
```

#### beardog-security

**Gaps** (estimated):
- HSM fallback scenarios
- Key rotation edge cases
- Permission denial paths
- Entropy exhaustion

**Tests to Add** (~30 tests):
```rust
// crates/beardog-security/src/hsm/manager.rs
#[cfg(test)]
mod coverage_expansion_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_hsm_manager_all_providers_fail() {
        // Test fallback when all HSMs unavailable
    }
    
    #[tokio::test]
    async fn test_hsm_manager_partial_capability() {
        // Test when HSM has partial capabilities
    }
    
    // ... more tests
}
```

#### beardog-auth

**Gaps** (estimated):
- Authentication failure paths
- Token expiration scenarios
- Permission edge cases
- Concurrent auth requests

**Tests to Add** (~25 tests):

---

### Phase 3: Type System & Utilities (Week 2, Days 1-2)

**Target**: beardog-types, beardog-utils

#### beardog-types

**Tests to Add** (~30 tests):
- Serialization/deserialization edge cases
- Type conversions
- Validation failures
- Default implementations

#### beardog-utils

**Tests to Add** (~20 tests):
- Zero-copy edge cases
- Buffer pool exhaustion
- Memory pressure scenarios
- Concurrent access patterns

---

### Phase 4: Integration & Adapters (Week 2, Days 3-5)

**Target**: beardog-adapters, beardog-integrations

#### beardog-adapters

**Tests to Add** (~30 tests):
- Provider discovery failures
- Capability mismatches
- Network errors
- Timeout scenarios

#### beardog-integration-tests

**Tests to Add** (~15 tests):
- Cross-module integration
- E2E failure scenarios
- Recovery validation

---

### Phase 5: Specialized Modules (Week 3)

**Target**: beardog-genetics, beardog-monitoring, beardog-workflows

#### beardog-genetics

**Tests to Add** (~20 tests):
- Constraint violation scenarios
- Evolution edge cases
- Validation failures

#### beardog-monitoring

**Tests to Add** (~15 tests):
- Metric collection failures
- Alert threshold edge cases
- Health check timeouts

#### beardog-workflows

**Tests to Add** (~10 tests):
- Workflow state transitions
- Error recovery
- Timeout handling

---

## 📋 SYSTEMATIC APPROACH

### For Each Module:

1. **Identify Gaps**:
   ```bash
   # Generate coverage for specific crate
   cargo llvm-cov --package beardog-core --html --output-dir coverage/core
   
   # Review uncovered lines
   open coverage/core/index.html
   ```

2. **Prioritize Tests**:
   - ✅ Critical error paths first
   - ✅ Security-sensitive code
   - ✅ Edge cases and boundaries
   - ✅ Concurrent scenarios
   - ✅ Integration points

3. **Write Tests**:
   ```rust
   #[cfg(test)]
   mod coverage_expansion_week1 {
       use super::*;
       
       // Group tests by coverage goal
       mod error_path_coverage {
           // Error handling tests
       }
       
       mod edge_case_coverage {
           // Edge case tests
       }
       
       mod integration_coverage {
           // Integration tests
       }
   }
   ```

4. **Verify**:
   ```bash
   # Run new tests
   cargo test --package beardog-core coverage_expansion_week1
   
   # Measure improvement
   cargo llvm-cov --package beardog-core --text
   ```

5. **Iterate**:
   - Document coverage increase
   - Identify remaining gaps
   - Repeat for next module

---

## 🎯 COVERAGE TARGETS BY MODULE

| Module | Current | Target | Gap | Tests Needed |
|--------|---------|--------|-----|--------------|
| beardog-core | ~75% | 90% | 15% | ~40 |
| beardog-security | ~80% | 95% | 15% | ~30 |
| beardog-auth | ~78% | 90% | 12% | ~25 |
| beardog-types | ~70% | 85% | 15% | ~30 |
| beardog-utils | ~75% | 85% | 10% | ~20 |
| beardog-adapters | ~72% | 85% | 13% | ~30 |
| beardog-genetics | ~65% | 80% | 15% | ~20 |
| beardog-monitoring | ~70% | 80% | 10% | ~15 |
| beardog-workflows | ~75% | 85% | 10% | ~10 |
| **Total** | **78.18%** | **90%** | **~12%** | **~220** |

---

## 🛠️ TEST PATTERNS

### Pattern 1: Error Path Coverage

```rust
#[tokio::test]
async fn test_operation_with_invalid_input() {
    let service = create_test_service();
    
    let result = service.operation(invalid_input()).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        BearDogError::ValidationFailed { field, reason } => {
            assert_eq!(field, "expected_field");
            assert!(reason.contains("expected message"));
        }
        _ => panic!("Wrong error type"),
    }
}
```

### Pattern 2: Edge Case Coverage

```rust
#[tokio::test]
async fn test_operation_with_boundary_values() {
    let service = create_test_service();
    
    // Test minimum boundary
    let result = service.operation(MIN_VALUE).await;
    assert!(result.is_ok());
    
    // Test maximum boundary
    let result = service.operation(MAX_VALUE).await;
    assert!(result.is_ok());
    
    // Test beyond boundaries
    let result = service.operation(MAX_VALUE + 1).await;
    assert!(result.is_err());
}
```

### Pattern 3: Concurrent Coverage

```rust
#[tokio::test]
async fn test_operation_concurrent_access() {
    let service = Arc::new(create_test_service());
    let mut handles = vec![];
    
    // Spawn 100 concurrent operations
    for i in 0..100 {
        let service = service.clone();
        handles.push(tokio::spawn(async move {
            service.operation(i).await
        }));
    }
    
    // All should succeed
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}
```

### Pattern 4: Integration Coverage

```rust
#[tokio::test]
async fn test_cross_module_integration() {
    // Set up multiple components
    let hsm = create_test_hsm();
    let crypto = create_crypto_service(hsm);
    let auth = create_auth_service(crypto);
    
    // Test end-to-end flow
    let result = auth.authenticate_and_sign(credentials, data).await;
    
    assert!(result.is_ok());
    verify_signature(result.unwrap());
}
```

### Pattern 5: Recovery Coverage

```rust
#[tokio::test]
async fn test_operation_recovery_after_failure() {
    let service = create_test_service();
    
    // Inject failure
    inject_transient_failure();
    
    // First attempt fails
    let result = service.operation(data).await;
    assert!(result.is_err());
    
    // Service recovers
    clear_failure();
    
    // Second attempt succeeds
    let result = service.operation(data).await;
    assert!(result.is_ok());
}
```

---

## 📊 PROGRESS TRACKING

### Week 1 Checklist:

- [ ] Day 1: Generate initial coverage report
- [ ] Day 1: Identify top 10 files needing coverage
- [ ] Day 2: Add 40 tests to beardog-core
- [ ] Day 3: Add 30 tests to beardog-security
- [ ] Day 4: Add 25 tests to beardog-auth
- [ ] Day 5: Verify Week 1 coverage improvement

**Target**: 78% → 82% (+4%)

### Week 2 Checklist:

- [ ] Day 1: Add 30 tests to beardog-types
- [ ] Day 2: Add 20 tests to beardog-utils
- [ ] Day 3: Add 30 tests to beardog-adapters
- [ ] Day 4: Add 15 integration tests
- [ ] Day 5: Verify Week 2 coverage improvement

**Target**: 82% → 86% (+4%)

### Week 3 Checklist:

- [ ] Day 1-2: Add 20 tests to beardog-genetics
- [ ] Day 2-3: Add 15 tests to beardog-monitoring
- [ ] Day 3-4: Add 10 tests to beardog-workflows
- [ ] Day 4-5: Fill remaining gaps
- [ ] Day 5: Final coverage verification

**Target**: 86% → 90% (+4%)

---

## 🚀 QUICK START

### Start Today:

```bash
# 1. Generate coverage report
cargo llvm-cov --workspace --html --output-dir coverage/

# 2. Identify lowest coverage files
ls coverage/*.html | xargs grep -l "coverage: [0-6][0-9]\.[0-9]%"

# 3. Pick one file, add 5 tests
# Focus on error paths and edge cases

# 4. Verify improvement
cargo test --package <package-name>
cargo llvm-cov --package <package-name> --text

# 5. Repeat daily
```

### Daily Goal: +5-10 tests

- 10 tests/day = 50 tests/week
- 3 weeks = 150 tests minimum
- 200 tests = realistic target

---

## 🎯 SUCCESS CRITERIA

### Final Verification:

```bash
# Generate final coverage report
cargo llvm-cov --workspace --html --output-dir coverage/final/

# Verify overall coverage
cargo llvm-cov --workspace --text | grep "TOTAL"

# Should show:
# TOTAL   xxx    yyy    90.xx%
```

### Acceptance:

- [ ] Overall coverage ≥ 90%
- [ ] No critical modules < 85%
- [ ] All error paths tested
- [ ] All public APIs tested
- [ ] Integration tests expanded
- [ ] Documentation updated

---

## 📚 RESOURCES

### Commands:

```bash
# Coverage for single crate
cargo llvm-cov --package beardog-core --html

# Coverage for specific test
cargo llvm-cov --test crypto_service_tests --html

# Text output (faster)
cargo llvm-cov --workspace --text

# Open in browser
open coverage/index.html
```

### Documentation:

- `cargo-llvm-cov` docs: https://github.com/taiki-e/cargo-llvm-cov
- Testing best practices: `docs/guides/`
- Chaos testing examples: `tests/chaos_*.rs`

---

## 🐻 BOTTOM LINE

**Goal**: 78% → 90% coverage  
**Method**: Systematic, module-by-module  
**Timeline**: 2-3 weeks  
**Daily Target**: 5-10 new tests  
**Success**: 90%+ coverage, all critical paths tested

**Start small, iterate daily, verify frequently!** 🚀

---

**Document Version**: 1.0  
**Created**: December 17, 2025  
**Status**: Ready to Execute

🐻 **BearDog: Comprehensive Test Coverage Through Systematic Expansion** 📊

