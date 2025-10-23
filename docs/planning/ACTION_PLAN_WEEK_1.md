# 🎯 BEARDOG ACTION PLAN - WEEK 1
## Immediate Actions Based on Oct 21, 2025 Audit

**Goal**: Start the path to 90% test coverage and fix critical issues  
**Timeline**: October 21-27, 2025 (Week 1)

---

## 📊 CURRENT STATE

- **Test Coverage**: 33.77% (need 90%)
- **Clippy Warnings**: 7 (excellent!)
- **Unwraps**: 1,241 total (~500-600 in production)
- **TODOs**: 93 (low, well-managed)
- **Grade**: B+ (85/100)

---

## 🎯 WEEK 1 GOALS

### **Goal 1: Test Coverage 33.77% → 38%** (+4-5%)
- Add ~200-250 tests
- Focus on core security paths
- Effort: 40-50 hours

### **Goal 2: Fix Top 20 Critical Unwraps**
- Security module unwraps
- HSM operation unwraps
- Core path unwraps
- Effort: 8-10 hours

### **Goal 3: Remove Top 20 Hardcoded Values**
- Network configuration
- Port assignments
- Service endpoints
- Effort: 6-8 hours

### **Goal 4: Documentation**
- Document top 10 public APIs
- Add missing `# Errors` sections
- Effort: 4-6 hours

**Total Effort**: 58-74 hours (full work week)

---

## 📋 DAILY BREAKDOWN

### **Monday: Test Expansion Foundation** (8-10 hours)

**Morning (4-5 hours)**:
1. Review test infrastructure (what's there, what's missing)
2. Identify 50 untested functions in `beardog-security`
3. Create test templates for security tests
4. Write 20 new security tests

**Afternoon (4-5 hours)**:
5. Identify 50 untested functions in `beardog-tunnel`
6. Create test templates for HSM tests
7. Write 20 new HSM tests

**End of Day Target**: 40 new tests, +1.5% coverage

---

### **Tuesday: Continue Test Expansion** (8-10 hours)

**Morning (4-5 hours)**:
1. Write 30 new tests for `beardog-core`
2. Focus on zero-knowledge bootstrap paths
3. Test discovery mechanisms

**Afternoon (4-5 hours)**:
4. Write 30 new tests for `beardog-types`
5. Test canonical type conversions
6. Test configuration validation

**End of Day Target**: 60 more tests (100 total), +3% coverage

---

### **Wednesday: Critical Unwraps** (8-10 hours)

**Morning (4-5 hours)**:
1. Scan for unwraps in security-critical paths
2. Create list of top 20 critical unwraps
3. Convert 10 unwraps to proper error handling

**Files to Focus On**:
- `crates/beardog-security/src/` (priority 1)
- `crates/beardog-tunnel/src/tunnel/hsm/` (priority 2)
- `crates/beardog-core/src/core/` (priority 3)

**Afternoon (4-5 hours)**:
4. Convert remaining 10 unwraps
5. Add tests for new error paths
6. Verify error propagation

**End of Day Target**: 20 unwraps fixed

---

### **Thursday: Hardcoding Removal** (8-10 hours)

**Morning (4-5 hours)**:
1. Audit `crates/beardog-types/src/constants/domains/network.rs`
2. Identify top 20 hardcoded values for removal
3. Convert 10 constants to environment variables
4. Add environment variable fallbacks

**Afternoon (4-5 hours)**:
5. Convert remaining 10 hardcoded values
6. Test environment variable loading
7. Document configuration options

**End of Day Target**: 20 hardcoded values removed

---

### **Friday: More Tests + Documentation** (8-10 hours)

**Morning (4-5 hours)**:
1. Write 50 more tests across all crates
2. Focus on integration scenarios
3. Test error paths

**Afternoon (4-5 hours)**:
4. Document top 10 public APIs
5. Add missing `# Errors` sections
6. Add code examples

**End of Day Target**: 50 more tests (150 total), 10 APIs documented

---

### **Weekend: Optional Momentum** (8-12 hours)

**Saturday (4-6 hours)**:
1. Write 50 more property-based tests
2. Add chaos test scenarios
3. Improve E2E test coverage

**Sunday (4-6 hours)**:
4. Write 50 more integration tests
5. Polish documentation
6. Run full test suite

**End of Weekend Target**: 100 more tests (250 total), +5% coverage

---

## 🎯 WEEK 1 TARGETS

| Metric | Start | Target | Stretch |
|--------|-------|--------|---------|
| Test Coverage | 33.77% | 38% | 40% |
| New Tests | 0 | 150 | 250 |
| Unwraps Fixed | 0 | 20 | 30 |
| Hardcoded Removed | 0 | 20 | 30 |
| APIs Documented | 0 | 10 | 15 |
| Grade | B+ (85%) | B+ (86%) | A- (88%) |

---

## 📝 TOP 20 CRITICAL UNWRAPS TO FIX

### **Priority 1: Security Module** (10 unwraps)

1. `crates/beardog-security/src/tests/hsm_operations_comprehensive_tests.rs` (2 unwraps)
2. `crates/beardog-security/src/tests/key_lifecycle_tests.rs` (12 unwraps - check if in prod code)
3. `crates/beardog-security/src/tests/crypto_edge_cases_extended_tests.rs` (20 unwraps - check)
4. `crates/beardog-security/src/lib.rs` (2 unwraps)
5. `crates/beardog-security/src/standalone.rs` (5 unwraps)

### **Priority 2: HSM Module** (10 unwraps)

6. `crates/beardog-tunnel/src/tunnel/hsm/software_hsm/types.rs` (18 unwraps)
7. `crates/beardog-tunnel/src/tunnel/hsm/unified_provider.rs` (19 unwraps)
8. `crates/beardog-tunnel/src/tunnel/hsm/providers/registry.rs` (16 unwraps)
9. `crates/beardog-tunnel/src/tunnel/hsm/providers/software.rs` (13 unwraps)
10. `crates/beardog-tunnel/src/tunnel/hsm/manager/capability.rs` (12 unwraps)

**Command to Verify**:
```bash
grep -n "\.unwrap()\|\.expect(" crates/beardog-security/src/standalone.rs
grep -n "\.unwrap()\|\.expect(" crates/beardog-tunnel/src/tunnel/hsm/unified_provider.rs
```

---

## 🔧 TOP 20 HARDCODED VALUES TO REMOVE

### **Priority 1: Network Configuration** (10 values)

Located in: `crates/beardog-types/src/constants/domains/network.rs`

1. `DEFAULT_HTTP_PORT: u16 = 8080` → `env::var("BEARDOG_HTTP_PORT")`
2. `DEFAULT_HTTPS_PORT: u16 = 8443` → `env::var("BEARDOG_HTTPS_PORT")`
3. `DEFAULT_POSTGRES_PORT: u16 = 5432` → `env::var("DATABASE_PORT")`
4. `DEFAULT_GRAFANA_PORT: u16 = 3000` → `env::var("GRAFANA_PORT")`
5. `DEFAULT_API_BIND: &str = "0.0.0.0:8080"` → `env::var("API_BIND_ADDRESS")`
6. Connection timeout constants (make configurable)
7. Retry interval constants (make configurable)
8. Max connection counts (make configurable)
9. Buffer size constants (make configurable)
10. Thread pool sizes (make configurable)

### **Priority 2: Service Endpoints** (10 values)

11. Discovery endpoint URLs (use service discovery)
12. Compute endpoint URLs (use service discovery)
13. Storage endpoint URLs (use service discovery)
14. Monitoring endpoint URLs (use configuration)
15. Metrics collection URLs (use configuration)
16. Health check endpoints (use configuration)
17. Default database URLs (use environment)
18. Default cache URLs (use environment)
19. Default queue URLs (use environment)
20. Default logging endpoints (use configuration)

**Implementation Pattern**:
```rust
pub fn default_http_port() -> u16 {
    std::env::var("BEARDOG_HTTP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080) // Fallback only
}
```

---

## 📚 TOP 10 APIS TO DOCUMENT

### **Priority 1: Core Public APIs** (5 APIs)

1. `beardog_core::BearDogCore` - Main entry point
2. `beardog_security::SecurityOperations` - Security interface
3. `beardog_tunnel::HsmManager` - HSM management
4. `beardog_types::canonical::Config` - Configuration types
5. `beardog_adapters::UniversalAdapter` - Adapter interface

### **Priority 2: Key Traits** (5 APIs)

6. `beardog_traits::Provider` - Provider trait
7. `beardog_traits::Workflow` - Workflow trait
8. `beardog_errors::BearDogError` - Error type
9. `beardog_types::canonical::Health` - Health types
10. `beardog_monitoring::HealthChecker` - Health checking

**Documentation Template**:
```rust
/// Brief one-line description.
///
/// Longer description with context and usage.
///
/// # Examples
///
/// ```
/// // Example usage
/// ```
///
/// # Errors
///
/// Returns an error if...
///
/// # Panics
///
/// Panics if... (if applicable)
pub fn example_function() -> Result<(), BearDogError> {
    // implementation
}
```

---

## ✅ SUCCESS CRITERIA

### **Must Have** (Week 1 Complete):
- ✅ 150+ new tests written
- ✅ Test coverage ≥ 38%
- ✅ 20 critical unwraps fixed
- ✅ 20 hardcoded values removed
- ✅ 10 APIs documented

### **Nice to Have** (Stretch Goals):
- 🎯 250 new tests written
- 🎯 Test coverage ≥ 40%
- 🎯 30 unwraps fixed
- 🎯 30 hardcoded values removed
- 🎯 15 APIs documented

### **Quality Gates**:
- ✅ All tests pass (100% pass rate)
- ✅ Clippy warnings ≤ 10
- ✅ Build remains clean
- ✅ No regressions in existing tests

---

## 📊 TRACKING PROGRESS

Run these commands daily to track progress:

```bash
# Test Coverage
cargo tarpaulin --output-dir coverage --out Json
cat coverage/tarpaulin-report.json | grep '"coverage"'

# Test Count
find crates -name "*test*.rs" | xargs grep -c "#\[test\]" | awk -F: '{sum+=$2} END {print sum}'

# Unwraps
grep -r "\.unwrap()\|\.expect(" crates/ | grep -v test | wc -l

# Hardcoded IPs
grep -ri "127\.0\.0\.1|localhost|:8080|:3000" crates/ | grep -v test | wc -l

# Clippy
cargo clippy --all-targets --all-features 2>&1 | grep -c "^warning:"

# Documented APIs
cargo doc --no-deps 2>&1 | grep -c "warning"
```

---

## 🎯 END OF WEEK 1 REVIEW

**Questions to Answer**:
1. Did we hit 38% coverage?
2. Did we fix 20 unwraps?
3. Did we remove 20 hardcoded values?
4. Did we document 10 APIs?
5. Are all tests still passing?
6. What blocked us?
7. What do we adjust for Week 2?

---

**Let's build to production excellence!** 🐻🔐

*Created: October 21, 2025*

