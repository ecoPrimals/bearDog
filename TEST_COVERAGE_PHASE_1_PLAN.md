# Test Coverage Phase 1 Plan - October 9, 2025

**Goal**: Increase coverage from 21.4% → 30%+  
**Focus**: Unit tests for beardog-core and beardog-types  
**Duration**: This session + 1-2 more sessions

---

## 📊 Current State

### Coverage Analysis
- **Overall**: 21.4% (need +8.6% minimum)
- **beardog-core**: 144 source files, 2 test files
- **beardog-types**: 232 source files, 2 test files

### Test Infrastructure
- ✅ `cargo test` working
- ✅ Tarpaulin coverage tool available
- ✅ Test framework in place
- ✅ Existing test patterns to follow

---

## 🎯 Priority Modules for Testing

### Tier 1: Critical Business Logic ⭐⭐⭐

#### beardog-types (Foundation)
1. **health.rs / health_status.rs** - Core health types
2. **metrics.rs** - Telemetry types
3. **capabilities.rs** - Capability types
4. **crypto.rs** - Cryptography types
5. **ecosystem_relationships.rs** - Relationship types

**Impact**: High - these are used everywhere  
**Effort**: Low-Medium - mostly pure functions  
**Tests needed**: ~50-75 tests

#### beardog-core (Core Logic)
1. **discovery/** - Service discovery
   - `infant_discovery.rs`
   - `universal_infant_discovery.rs`
   - `vendor_agnostic_hsm.rs`

2. **ecosystem_storage/** - Storage layer
   - `backends.rs`
   - `cache.rs`
   - `operations.rs`

3. **ecosystem/** - Ecosystem coordination
   - `service_registration.rs`
   - `self_discovery.rs`

**Impact**: High - critical paths  
**Effort**: Medium - some async, some complex  
**Tests needed**: ~75-100 tests

---

### Tier 2: Important Supporting Code ⭐⭐

#### beardog-types
1. **hsm.rs** - HSM types
2. **genetics.rs** - Genetics types
3. **config/** - Configuration types

#### beardog-core
1. **ecosystem_integration/** - Integration layer
2. **external_functions/** - FFI layer
3. **universal_discovery/** - Universal discovery

**Impact**: Medium-High  
**Effort**: Medium  
**Tests needed**: ~50-75 tests

---

### Tier 3: Polish & Edge Cases ⭐

Everything else - lower priority for Phase 1

---

## 📋 Implementation Strategy

### Step 1: Test Infrastructure (30 min)
- [x] Create test plan
- [ ] Review existing test patterns
- [ ] Set up test utilities/helpers
- [ ] Document test standards

### Step 2: beardog-types Basic Tests (2-3 hours)
Priority order:
1. `health.rs` - Health status types
2. `metrics.rs` - Metric types  
3. `capabilities.rs` - Capability types
4. `crypto.rs` - Crypto types

**Target**: +2-3% coverage

### Step 3: beardog-core Discovery Tests (2-3 hours)
Priority order:
1. `discovery/infant_discovery.rs`
2. `discovery/universal_infant_discovery.rs`
3. `discovery/vendor_agnostic_hsm.rs`

**Target**: +2-3% coverage

### Step 4: beardog-core Storage Tests (2-3 hours)
Priority order:
1. `ecosystem_storage/cache.rs`
2. `ecosystem_storage/operations.rs`
3. `ecosystem_storage/backends.rs`

**Target**: +2-3% coverage

### Step 5: beardog-core Ecosystem Tests (2-3 hours)
Priority order:
1. `ecosystem/service_registration.rs`
2. `ecosystem/self_discovery.rs`

**Target**: +1-2% coverage

---

## 🎯 Success Criteria

### Minimum (Phase 1 Goal)
- ✅ Coverage: 21.4% → **30%+** (+8.6%)
- ✅ Tests added: **~125 tests minimum**
- ✅ All tests passing
- ✅ Zero build regressions

### Stretch (Exceptional)
- 🎯 Coverage: 21.4% → **35%+** (+13.6%)
- 🎯 Tests added: **~200 tests**
- 🎯 Property-based tests for key algorithms
- 🎯 Comprehensive error case testing

---

## 🧪 Test Patterns to Follow

### Pattern 1: Pure Function Tests
```rust
#[test]
fn test_function_happy_path() {
    let input = create_test_input();
    let result = function_under_test(input);
    assert_eq!(result, expected_output);
}

#[test]
fn test_function_error_case() {
    let invalid_input = create_invalid_input();
    let result = function_under_test(invalid_input);
    assert!(result.is_err());
}
```

### Pattern 2: Struct Tests
```rust
#[test]
fn test_struct_creation() {
    let instance = Struct::new(params);
    assert_eq!(instance.field, expected_value);
}

#[test]
fn test_struct_methods() {
    let mut instance = Struct::new(params);
    instance.method();
    assert_eq!(instance.state, expected_state);
}
```

### Pattern 3: Async Tests
```rust
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### Pattern 4: Error Handling Tests
```rust
#[test]
fn test_error_propagation() {
    let result = function_that_can_fail();
    match result {
        Ok(_) => panic!("Should have failed"),
        Err(e) => assert!(matches!(e, ExpectedErrorType)),
    }
}
```

---

## 📊 Coverage Tracking

### Baseline
- **Date**: October 9, 2025 (start)
- **Coverage**: 21.4%
- **Test count**: Unknown (need to count)

### Checkpoints

**After Step 2 (types tests)**:
- Target: 23-24%
- Tests added: ~50

**After Step 3 (discovery tests)**:
- Target: 25-27%
- Tests added: ~50

**After Step 4 (storage tests)**:
- Target: 27-29%
- Tests added: ~50

**After Step 5 (ecosystem tests)**:
- Target: 30%+
- Tests added: ~50

---

## 🚀 Execution Plan

### Session 1 (This Session - 3-4 hours)
1. ✅ Create test plan
2. Review existing patterns
3. Start beardog-types basic tests (health, metrics)
4. Add 25-30 tests
5. Verify coverage improvement

### Session 2 (Next - 3-4 hours)
1. Complete beardog-types tests
2. Start beardog-core discovery tests
3. Add 40-50 tests
4. Check coverage (target: 26-27%)

### Session 3 (Next - 3-4 hours)
1. Complete discovery tests
2. Start storage tests
3. Add 40-50 tests
4. Check coverage (target: 29-30%+)

### Session 4 (Polish - 2 hours)
1. Fill gaps
2. Add edge case tests
3. Document coverage
4. Verify >30%

---

## 🎓 Testing Best Practices

### Do's ✅
1. **Test behavior, not implementation**
2. **One assertion per test** (or closely related assertions)
3. **Clear test names** - describe what's being tested
4. **Arrange-Act-Assert** pattern
5. **Test edge cases** - empty, null, boundary values
6. **Test error paths** - not just happy paths
7. **Use test fixtures** for common setup

### Don'ts ❌
1. **Don't test private functions directly** - test public API
2. **Don't write brittle tests** - avoid testing exact strings/formats
3. **Don't ignore warnings** - fix or justify
4. **Don't skip error cases** - they're critical
5. **Don't duplicate test logic** - use helpers

---

## 🔧 Tools & Commands

### Run Tests
```bash
# All tests
cargo test

# Specific package
cargo test -p beardog-core

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture
```

### Coverage
```bash
# Generate coverage
cargo tarpaulin --out Html --out Json

# Quick coverage check
cargo tarpaulin --packages beardog-core beardog-types
```

### Watch Mode
```bash
# Auto-run tests on changes
cargo watch -x test
```

---

## 📈 Expected Outcomes

### Code Quality
- **Coverage**: 21.4% → 30%+ (+40% relative improvement)
- **Confidence**: Significantly higher in core modules
- **Regression detection**: Much improved
- **Refactoring safety**: Enabled by test coverage

### Project Grade Impact
- **Test Coverage**: F (21.4%) → D+ (30%) [+2 grades]
- **Overall Grade**: B+ (85) → A- (88) [+3 points]

### Development Velocity
- **Refactoring**: Safer and faster
- **Bug detection**: Earlier in development
- **Documentation**: Tests serve as examples
- **Confidence**: Higher for production deployment

---

## 🎯 Success Metrics

| Metric | Current | Target | Stretch |
|--------|---------|--------|---------|
| **Coverage %** | 21.4% | 30% | 35% |
| **Tests Added** | 0 | 125 | 200 |
| **Grade** | B+ (85) | A- (88) | A- (89) |
| **Confidence** | Medium | High | Very High |

---

**Status**: 🟢 Ready to begin  
**Next**: Start with beardog-types health and metrics tests  
**Timeline**: 3-4 sessions to reach 30%+

*"Test coverage is insurance against future bugs."* ✨

