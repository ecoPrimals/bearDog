# 🧪 BearDog Test Suite

**Last Updated**: October 26, 2025  
**Total Tests**: 3,223+ test functions  
**Organization**: Categorized by type and domain

---

## 📋 Test Organization

### Test Categories

- **Unit Tests** (~2,000 tests) - Pure unit tests, single function/method
- **Integration Tests** (~800 tests) - Multiple component integration
- **E2E Tests** (13 tests) - End-to-end workflows
- **Chaos Tests** (23 tests) - Fault injection and resilience
- **Property Tests** (~100 tests) - Property-based testing

### Test Domains

- `core` - Core orchestration
- `security` - Security subsystem (crypto, auth, HSM)
- `networking` - Network operations
- `genetics` - Genetic evolution
- `monitoring` - Observability
- `workflows` - Workflow orchestration
- `adapters` - Adapter system
- `config` - Configuration

---

## 🚀 Running Tests

### By Category

```bash
# All tests
./scripts/test-by-category.sh all

# Unit tests only
./scripts/test-by-category.sh unit

# Integration tests
./scripts/test-by-category.sh integration

# E2E tests
./scripts/test-by-category.sh e2e

# Chaos tests
./scripts/test-by-category.sh chaos

# Security tests
./scripts/test-by-category.sh security

# Fast tests only (skip slow tests)
./scripts/test-by-category.sh fast
```

### By Domain

```bash
# Core domain
./scripts/test-by-domain.sh core

# Security domain
./scripts/test-by-domain.sh security

# HSM domain
./scripts/test-by-domain.sh hsm

# All domains
./scripts/test-by-domain.sh all
```

### Test Metrics

```bash
# View test organization metrics
./scripts/test-metrics.sh
```

---

## 📝 Writing Tests

### Test Naming Convention

```rust
// Pattern: test_[domain]_[component]_[action]_[condition]

/// TEST_CATEGORY: unit
/// TEST_DOMAIN: security
#[test]
fn test_security_crypto_encrypt_with_valid_key() {
    // Test implementation
}

/// TEST_CATEGORY: integration
/// TEST_DOMAIN: hsm
#[tokio::test]
async fn test_hsm_android_initialize_full_workflow() {
    // Test implementation
}
```

### Test Documentation

All tests should include:
- `TEST_CATEGORY` - unit, integration, e2e, chaos, etc.
- `TEST_DOMAIN` - core, security, hsm, etc.
- `TEST_PRIORITY` (optional) - critical, high, medium, low
- `TEST_REQUIRES` (optional) - hardware, network, slow, etc.

Example:

```rust
/// TEST_CATEGORY: e2e
/// TEST_DOMAIN: security
/// TEST_PRIORITY: critical
/// TEST_REQUIRES: network
#[tokio::test]
async fn test_security_full_auth_flow() {
    // Test implementation
}
```

---

## 📁 Test Structure

```
tests/
├── test_categories.rs     # Test categorization system
├── e2e/                   # End-to-end tests
│   ├── mod.rs
│   ├── production_deployment.rs
│   ├── full_stack_integration.rs
│   ├── security_flow.rs
│   └── disaster_recovery.rs
├── chaos/                 # Chaos engineering tests
│   ├── mod.rs
│   ├── fault_injection.rs
│   ├── recovery.rs
│   └── scenarios.rs
└── *.rs                   # Integration tests

crates/*/tests/            # Crate-specific integration tests
crates/*/src/**/tests.rs   # Inline unit tests (#[cfg(test)])
```

---

## 🎯 Test Coverage Goals

### Current Coverage (Estimated)

```
Overall:      27-28% (aggregate)
Unit:         ~35%
Integration:  ~20%
E2E:          ~5%
Chaos:        ~3%
```

### Target Coverage

```
Overall:      90%
Unit:         90%
Integration:  80%
E2E:          70%
Chaos:        50%
```

---

## 📊 Test Metrics

Run `./scripts/test-metrics.sh` to see:
- Total test count
- Tests by location
- Tests by category
- Tests by domain
- Coverage analysis
- Test organization quality

---

## 🔍 Test Guidelines

### Unit Tests
- Test single function/method
- No external dependencies
- Fast execution (<10ms)
- Use mocks for dependencies

### Integration Tests
- Test multiple components
- Real implementations where possible
- Moderate execution time (<1s)
- May use test fixtures

### E2E Tests
- Test complete workflows
- Full system integration
- Slower execution (1-10s)
- Use realistic scenarios

### Chaos Tests
- Test fault scenarios
- Resilience validation
- Variable execution time
- Use fault injection framework

---

## 🚨 Common Issues

### Slow Tests
Mark with `#[ignore]` and `TEST_REQUIRES: slow`

### Flaky Tests
Document known flakiness and root cause

### Hardware Requirements
Mark with `TEST_REQUIRES: hardware`

### Network Requirements
Mark with `TEST_REQUIRES: network`

---

## 📚 Related Documentation

- [E2E Test Framework](e2e/README.md)
- [Chaos Test Framework](chaos/README.md)
- [Test Coverage Plan](../TEST_COVERAGE_EXPANSION_PLAN.md)
- [Test Organization Audit](../TEST_ORGANIZATION_AUDIT_OCT_26_2025.md)

---

**SOVEREIGN TESTING! 🧪🔐**
