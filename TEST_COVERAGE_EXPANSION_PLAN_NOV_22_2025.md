# 🎯 Test Coverage Expansion Plan

**Date**: November 22, 2025  
**Current Coverage**: 78%  
**Target Coverage**: 85%  
**Gap**: 7% (~100 tests needed)

---

## 📊 Coverage Analysis

### Modules with Lowest Coverage

Based on codebase analysis:

| Module | Current Est. | Target | Priority | Tests Needed |
|--------|--------------|--------|----------|--------------|
| **beardog-core/discovery/** | ~60% | 85% | 🔴 HIGH | ~15 tests |
| **beardog-core/ecosystem/** | ~65% | 85% | 🔴 HIGH | ~15 tests |
| **beardog-core/ai/** | ~55% | 80% | 🔴 HIGH | ~20 tests |
| **beardog-adapters/universal/** | ~68% | 85% | 🟠 MEDIUM | ~20 tests |
| **beardog-core/universal_discovery/** | ~70% | 85% | 🟠 MEDIUM | ~15 tests |
| **beardog-core/ecosystem_integration/** | ~72% | 85% | 🟡 LOW | ~10 tests |

---

## 🎯 High-Value Test Opportunities

### 1. AI Module (beardog-core/ai/) - 20 tests

**Current State**: Many tests are placeholders with "PHASE-2" comments

**Opportunities**:
```rust
// Currently:
#[test]
fn test_decision_with_high_confidence() {
    // PHASE-2(AI-Testing): Add real high confidence decision test
}

// Should add:
- Real decision engine tests
- Configuration validation tests
- Integration with hybrid intelligence
- Error handling for AI failures
- Fallback mechanism tests
```

**Files to enhance**:
- `ai/tests/hybrid_intelligence_comprehensive_tests.rs`
- `ai/hybrid_intelligence/core/integration.rs`
- `ai/hybrid_intelligence/types/`

**Expected Impact**: +15-20% local coverage

---

### 2. Discovery Module (beardog-core/discovery/) - 15 tests

**Current State**: Basic coverage, missing edge cases

**Opportunities**:
- Infant discovery edge cases
- Network timeout handling
- Discovery protocol error paths
- HSM vendor discovery scenarios
- Concurrent discovery requests

**Files to test**:
- `discovery/infant_discovery.rs`
- `discovery/universal_infant_discovery.rs`
- `discovery/vendor_agnostic_hsm.rs`

**Expected Impact**: +12-15% local coverage

---

### 3. Ecosystem Module (beardog-core/ecosystem/) - 15 tests

**Current State**: Integration tests exist, unit tests sparse

**Opportunities**:
- Primal interface edge cases
- Service registration failures
- Quantum discovery scenarios
- Self-discovery error paths
- Adaptive sovereignty learning

**Files to test**:
- `ecosystem/primal_interface/`
- `ecosystem/service_registration.rs`
- `ecosystem/quantum_discovery.rs`
- `ecosystem/self_discovery.rs`

**Expected Impact**: +10-12% local coverage

---

### 4. Universal Adapters (beardog-adapters/universal/) - 20 tests

**Current State**: Good structure, could expand scenarios

**Opportunities**:
- Capability-based adapter error paths
- Vendor adapter failover scenarios
- Connection management edge cases
- Execution timeout handling
- Discovery failure recovery

**Files to test**:
- `universal/capability_based_adapter/discovery.rs`
- `universal/capability_based_adapter/connection.rs`
- `universal/capability_based_adapter/execution.rs`
- `universal/vendor_adapter/`

**Expected Impact**: +8-10% local coverage

---

### 5. Universal Discovery (beardog-core/universal_discovery/) - 15 tests

**Current State**: Protocol tests exist, missing failure scenarios

**Opportunities**:
- Health check edge cases
- Load balancing algorithm tests
- Network discovery failures
- Registry synchronization tests
- Protocol upgrade scenarios

**Files to test**:
- `universal_discovery/health.rs`
- `universal_discovery/load_balancing.rs`
- `universal_discovery/network.rs`
- `universal_discovery/registry.rs`

**Expected Impact**: +8-10% local coverage

---

## 🚀 Implementation Strategy

### Phase 1: Quick Wins (Day 1-2)
**Target**: +3% coverage (~30 tests)

1. **Fill in PHASE-2 placeholders** in AI tests (10 tests)
2. **Add discovery edge cases** (10 tests)
3. **Add ecosystem error paths** (10 tests)

### Phase 2: Medium Effort (Day 3-5)
**Target**: +3% coverage (~40 tests)

4. **Expand adapter scenarios** (20 tests)
5. **Add universal discovery tests** (15 tests)
6. **Add integration tests** (5 tests)

### Phase 3: Polish (Day 6-7)
**Target**: +1% coverage (~30 tests)

7. **Add property-based tests** (10 tests)
8. **Add concurrency tests** (10 tests)
9. **Add performance regression tests** (10 tests)

**Total**: 7% coverage improvement, ~100 tests

---

## 📋 Test Templates

### Template 1: Error Path Test
```rust
#[tokio::test]
async fn test_operation_with_timeout() -> Result<()> {
    let component = Component::new(Config::default());
    
    // Simulate timeout condition
    let result = tokio::time::timeout(
        Duration::from_millis(1),
        component.slow_operation()
    ).await;
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TimeoutError));
    Ok(())
}
```

### Template 2: Edge Case Test
```rust
#[test]
fn test_with_invalid_input() {
    let component = Component::new(Config::default());
    
    let result = component.process(InvalidInput);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code(), "INVALID_INPUT");
}
```

### Template 3: Integration Test
```rust
#[tokio::test]
async fn test_end_to_end_workflow() -> Result<()> {
    let system = TestSystem::setup().await?;
    
    // Execute workflow
    let result = system
        .discover()
        .await?
        .connect()
        .await?
        .execute()
        .await?;
    
    assert_eq!(result.status, Success);
    Ok(())
}
```

---

## 🎯 Success Metrics

### Coverage Targets
- **Current**: 78%
- **Phase 1**: 81% (+3%)
- **Phase 2**: 84% (+6%)
- **Phase 3**: 85% (+7%)

### Quality Metrics
- All tests pass ✅
- No new clippy warnings ✅
- All tests documented ✅
- Edge cases covered ✅

---

## 📊 Tracking Progress

### Tests Added by Module
```
beardog-core/ai/                 [ ] 0/20
beardog-core/discovery/          [ ] 0/15
beardog-core/ecosystem/          [ ] 0/15
beardog-adapters/universal/      [ ] 0/20
beardog-core/universal_discovery/[ ] 0/15
Other improvements              [ ] 0/15

TOTAL: 0/100 tests (0%)
```

### Coverage by Phase
```
Phase 1 (Quick Wins):     [ ] 0/30 tests
Phase 2 (Medium Effort):  [ ] 0/40 tests
Phase 3 (Polish):         [ ] 0/30 tests
```

---

## 🚀 Execution Plan

### Step 1: Setup
- [x] Analyze current coverage
- [x] Identify low-coverage modules
- [x] Create test plan
- [ ] Set up coverage tracking

### Step 2: Phase 1 (This Session)
- [ ] Add 10 AI tests (PHASE-2 placeholders)
- [ ] Add 10 discovery edge case tests
- [ ] Add 10 ecosystem error path tests
- [ ] Verify +3% coverage

### Step 3: Phase 2 (Next Session)
- [ ] Add 20 adapter scenario tests
- [ ] Add 15 universal discovery tests
- [ ] Add 5 integration tests
- [ ] Verify +3% coverage (6% total)

### Step 4: Phase 3 (Final Session)
- [ ] Add property-based tests
- [ ] Add concurrency tests
- [ ] Add performance tests
- [ ] Verify 85% coverage achieved

---

## 🎯 Next Action

**START**: Add AI module tests (PHASE-2 placeholders)
- File: `crates/beardog-core/src/ai/tests/hybrid_intelligence_comprehensive_tests.rs`
- Target: 10 new tests
- Time: 30-45 minutes
- Coverage gain: ~1%

**Command**:
```bash
# Start with AI tests
cargo test --package beardog-core --test hybrid_intelligence_comprehensive_tests
```

---

**Created**: November 22, 2025  
**Status**: Ready to execute  
**Priority**: Phase 1 - Quick Wins

🐻 **Let's boost that coverage!**

