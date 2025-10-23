# 🧪 TEST EXPANSION ROADMAP
## Path from 33.77% → 90% Coverage

**Created**: October 21, 2025  
**Current Coverage**: 33.77% (3,689 / 10,932 lines)  
**Target Coverage**: 90% (9,839 / 10,932 lines)  
**Gap**: 6,150 lines  
**Estimated Tests Needed**: ~2,000 tests

---

## 📊 COVERAGE GAP ANALYSIS

### **Current State**
- Covered: 3,689 lines
- Coverable: 10,932 lines
- Uncovered: 7,243 lines (66.26%)
- Test files: 163
- Test markers: ~3,967

### **Target State** (90% Coverage)
- Need to cover: 6,150 more lines (56.23% increase)
- Estimated tests: 1,800-2,200 new tests
- Timeline: 12-15 weeks
- Effort: 500-700 hours

---

## 🎯 PHASE BREAKDOWN

### **Phase 1: Foundation (Weeks 1-4)** 
**Target**: 33.77% → 50% (+16.23%)

**Tests to Add**: 600-800 tests  
**Lines to Cover**: ~1,780 lines  
**Focus Areas**:
1. Core security operations (200 tests)
2. HSM key operations (200 tests)
3. Type conversions (100 tests)
4. Configuration validation (100 tests)

**Daily Target**: 30-40 tests/day

---

### **Phase 2: Integration (Weeks 5-8)**
**Target**: 50% → 70% (+20%)

**Tests to Add**: 800-1,000 tests  
**Lines to Cover**: ~2,186 lines  
**Focus Areas**:
1. Integration scenarios (300 tests)
2. Error path testing (200 tests)
3. Adapter operations (200 tests)
4. Workflow processing (100 tests)

**Daily Target**: 40-50 tests/day

---

### **Phase 3: Comprehensive (Weeks 9-12)**
**Target**: 70% → 90% (+20%)

**Tests to Add**: 600-800 tests  
**Lines to Cover**: ~2,186 lines  
**Focus Areas**:
1. Edge cases (200 tests)
2. E2E scenarios (150 tests)
3. Chaos tests (100 tests)
4. Property-based tests (150 tests)

**Daily Target**: 30-40 tests/day

---

## 📋 CRATE-BY-CRATE PLAN

### **1. beardog-security** (Priority 1)

**Current**: Unknown  
**Target**: 90%  
**Tests to Add**: ~400 tests

**Focus Areas**:
- Key lifecycle operations (100 tests)
- Crypto operations (100 tests)
- Access control (80 tests)
- Authentication flows (80 tests)
- Security primitives (40 tests)

**Files Needing Most Coverage**:
- `src/security/operations.rs`
- `src/crypto/primitives.rs`
- `src/access_control/mod.rs`
- `src/authentication/mod.rs`

---

### **2. beardog-tunnel** (Priority 1)

**Current**: Unknown  
**Target**: 90%  
**Tests to Add**: ~400 tests

**Focus Areas**:
- HSM operations (150 tests)
- Provider management (100 tests)
- Discovery mechanisms (80 tests)
- Failover logic (70 tests)

**Files Needing Most Coverage**:
- `src/tunnel/hsm/manager/mod.rs`
- `src/tunnel/hsm/providers/registry.rs`
- `src/universal_hsm_discovery/mod.rs`
- `src/tunnel/hsm/failover.rs`

---

### **3. beardog-core** (Priority 1)

**Current**: Unknown  
**Target**: 90%  
**Tests to Add**: ~350 tests

**Focus Areas**:
- Zero-knowledge bootstrap (100 tests)
- Capability registry (80 tests)
- Ecosystem coordination (70 tests)
- Lifecycle management (60 tests)
- Monitoring (40 tests)

**Files Needing Most Coverage**:
- `src/zero_knowledge_bootstrap/mod.rs`
- `src/zero_knowledge_bootstrap/capability_registry.rs`
- `src/ecosystem/mod.rs`
- `src/core/system.rs`

---

### **4. beardog-types** (Priority 2)

**Current**: Unknown  
**Target**: 90%  
**Tests to Add**: ~300 tests

**Focus Areas**:
- Canonical type conversions (100 tests)
- Configuration validation (80 tests)
- Health types (60 tests)
- Provider types (60 tests)

**Files Needing Most Coverage**:
- `src/canonical/mod.rs`
- `src/canonical/config/mod.rs`
- `src/production/health.rs`
- `src/hsm/mod.rs`

---

### **5. beardog-adapters** (Priority 2)

**Current**: Unknown  
**Target**: 90%  
**Tests to Add**: ~250 tests

**Focus Areas**:
- Universal adapter (100 tests)
- Vendor adapters (80 tests)
- Capability detection (70 tests)

**Files Needing Most Coverage**:
- `src/universal/mod.rs`
- `src/universal/vendor_adapter.rs`
- `src/universal/capability_discovery.rs`

---

### **6. Other Crates** (Priority 3)

**Tests to Add**: ~300 tests total

- beardog-workflows: 80 tests
- beardog-monitoring: 80 tests
- beardog-auth: 60 tests
- beardog-genetics: 40 tests
- beardog-errors: 40 tests

---

## 🧪 TEST TYPE BREAKDOWN

### **Unit Tests** (60% of new tests)
**Target**: ~1,200 tests

**Focus**:
- Function-level testing
- Single-responsibility verification
- Edge case coverage
- Error path testing

**Example Template**:
```rust
#[test]
fn test_function_name_success() {
    let result = function_under_test(valid_input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_value);
}

#[test]
fn test_function_name_error() {
    let result = function_under_test(invalid_input);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), BearDogError::ExpectedError));
}
```

---

### **Integration Tests** (25% of new tests)
**Target**: ~500 tests

**Focus**:
- Multi-component interactions
- End-to-end flows
- Cross-crate integration
- Real-world scenarios

**Example Template**:
```rust
#[tokio::test]
async fn test_integration_scenario() {
    let component_a = ComponentA::new().await.unwrap();
    let component_b = ComponentB::new().await.unwrap();
    
    let result = component_a.interact_with(component_b).await;
    
    assert!(result.is_ok());
    verify_state(component_a);
    verify_state(component_b);
}
```

---

### **Property-Based Tests** (10% of new tests)
**Target**: ~200 tests

**Focus**:
- Invariant verification
- Randomized input testing
- Mathematical properties
- Roundtrip conversions

**Example Template**:
```rust
#[quickcheck]
fn prop_roundtrip_conversion(input: ArbitraryType) -> bool {
    let converted = input.to_canonical();
    let back = CanonicalType::from(converted);
    back == input
}
```

---

### **E2E Tests** (5% of new tests)
**Target**: ~100 tests

**Focus**:
- Complete user workflows
- Multi-system scenarios
- Real deployment patterns
- Performance under load

**Example Template**:
```rust
#[tokio::test]
async fn test_e2e_user_workflow() {
    // Setup complete system
    let system = setup_test_system().await;
    
    // Execute full workflow
    let result = execute_complete_workflow(system).await;
    
    // Verify end-to-end behavior
    assert!(result.is_ok());
    verify_system_state();
}
```

---

## 📅 WEEKLY TARGETS

| Week | Coverage | Tests Added | Focus |
|------|----------|-------------|-------|
| 1 | 38% | 150 | Security core |
| 2 | 42% | 150 | HSM operations |
| 3 | 46% | 150 | Core bootstrap |
| 4 | 50% | 150 | Type conversions |
| 5 | 55% | 200 | Integration scenarios |
| 6 | 60% | 200 | Error paths |
| 7 | 65% | 200 | Adapter operations |
| 8 | 70% | 200 | Workflow processing |
| 9 | 75% | 200 | Edge cases |
| 10 | 80% | 200 | E2E scenarios |
| 11 | 85% | 200 | Chaos tests |
| 12 | 90% | 200 | Property tests |

---

## 🎯 DAILY WORKFLOW

### **Morning Routine** (3-4 hours)
1. Check coverage report
2. Identify 10-15 uncovered functions
3. Write tests for identified functions
4. Run tests locally
5. Commit passing tests

### **Afternoon Routine** (3-4 hours)
6. Identify integration gaps
7. Write integration tests
8. Run full test suite
9. Fix any failures
10. Update coverage report

### **Daily Goal**: 30-50 new tests, +1-1.5% coverage

---

## 🔧 TOOLS & COMMANDS

### **Generate Coverage Report**
```bash
cargo tarpaulin --output-dir coverage --out Html --out Json
open coverage/tarpaulin-report.html
```

### **Find Uncovered Code**
```bash
# Lines by file
cargo tarpaulin --output-dir coverage --out Json
# Parse JSON for uncovered lines per file
```

### **Run Specific Test Suite**
```bash
# Single crate
cargo test -p beardog-security

# Specific test file
cargo test --test security_operations_tests

# Single test
cargo test test_specific_function
```

### **Check Test Count**
```bash
find crates -name "*.rs" -path "*/tests/*" -exec grep -c "#\[test\]" {} + | awk -F: '{sum+=$2} END {print sum}'
```

---

## 📊 TRACKING & REPORTING

### **Daily Metrics**
- Tests added today: _____
- Coverage change: _____
- Failures encountered: _____
- Time spent: _____

### **Weekly Review**
- Coverage achieved: _____
- Target met? Y/N
- Tests added: _____
- Blockers: _____
- Next week adjustments: _____

### **Monthly Milestones**
- Month 1: 50% coverage (Week 4)
- Month 2: 70% coverage (Week 8)
- Month 3: 90% coverage (Week 12)

---

## 🚧 KNOWN CHALLENGES

### **Challenge 1: Async Testing**
**Issue**: Many functions are async, need tokio runtime  
**Solution**: Use `#[tokio::test]` macro consistently

### **Challenge 2: Mock Dependencies**
**Issue**: Some tests need mocked external services  
**Solution**: Use existing mock infrastructure in `beardog-utils`

### **Challenge 3: HSM Testing**
**Issue**: HSM operations need hardware or simulators  
**Solution**: Use software HSM provider for testing

### **Challenge 4: Integration Complexity**
**Issue**: Integration tests require multiple components  
**Solution**: Build reusable test fixtures

### **Challenge 5: Time Constraints**
**Issue**: Writing 2,000 tests takes significant time  
**Solution**: Prioritize high-value coverage, use generators

---

## ✅ SUCCESS CRITERIA

### **Week 4 Checkpoint**
- [ ] Coverage ≥ 50%
- [ ] 600+ new tests
- [ ] All tests passing
- [ ] No coverage regressions

### **Week 8 Checkpoint**
- [ ] Coverage ≥ 70%
- [ ] 1,400+ new tests
- [ ] E2E infrastructure ready
- [ ] Integration tests comprehensive

### **Week 12 Final**
- [ ] Coverage ≥ 90%
- [ ] 2,000+ new tests
- [ ] All test types covered
- [ ] Production-ready test suite

---

## 🎓 LESSONS & BEST PRACTICES

1. **Write tests before fixing bugs** - Better coverage
2. **Test error paths explicitly** - Often untested
3. **Use property-based testing** - Catches edge cases
4. **Keep tests focused** - One assertion per test
5. **Use descriptive names** - `test_what_when_then` pattern
6. **Avoid test interdependence** - Tests should be isolated
7. **Mock external dependencies** - Fast, reliable tests
8. **Run tests frequently** - Catch issues early
9. **Measure coverage daily** - Track progress
10. **Celebrate milestones** - Maintain motivation

---

**Let's achieve 90% coverage!** 🧪🎯

*Path to Production: 12-15 weeks of focused testing*

