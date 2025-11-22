# 🧪 Test Coverage Expansion Plan - November 19, 2025

## Current State

**Coverage**: 35-38%  
**Target**: 90%  
**Gap**: ~55%  
**Priority**: **HIGHEST** for production readiness

---

## 📊 Coverage Analysis

### Well-Covered Modules (>50%)
- ✅ `beardog-types` - Canonical types
- ✅ `beardog-core` - Bootstrap system
- ✅ `beardog-auth` - Authentication flows
- ✅ `beardog-genetics` - Primal evolution

### Under-Covered Modules (<30%)
- ⚠️ `beardog-security` - HSM integration (828 tests, need more coverage)
- ⚠️ `beardog-tunnel` - QUIC transport (needs E2E tests)
- ⚠️ `beardog-monitoring` - Metrics collection
- ⚠️ `beardog-workflows` - Orchestration

### Missing Test Types
- ❌ Multi-component integration tests
- ❌ Network partition simulations
- ❌ Hardware failure scenarios
- ❌ Load and stress tests
- ❌ Security penetration tests

---

## 🎯 Phase 1: Quick Wins (Week 1)

**Target**: 35% → 45% (+10%)  
**Focus**: Low-hanging fruit in existing modules

### beardog-security (Priority 1)
**Current**: 828 tests passing  
**Gaps**: HSM error paths, hardware failure scenarios

**Add**:
1. HSM connection failure tests
2. Key rotation edge cases  
3. Entropy exhaustion scenarios
4. Multi-device failover tests
5. FIDO2 timeout handling

**Estimated**: +15 tests, +3% coverage

### beardog-tunnel (Priority 2)
**Current**: Tests exist but coverage gaps  
**Gaps**: QUIC transport edge cases

**Add**:
1. Network interruption recovery
2. Packet loss handling
3. Connection timeout scenarios
4. TLS handshake failures
5. Large data transfer tests

**Estimated**: +20 tests, +4% coverage

### beardog-monitoring (Priority 3)
**Current**: Basic tests  
**Gaps**: Metric collection edge cases

**Add**:
1. Metric overflow handling
2. Collection failure recovery
3. Storage backend errors
4. Query performance tests
5. Alert threshold tests

**Estimated**: +15 tests, +3% coverage

**Total Phase 1**: +50 tests, +10% coverage = **45%**

---

## 🎯 Phase 2: Integration Tests (Week 2)

**Target**: 45% → 60% (+15%)  
**Focus**: Multi-component interactions

### HSM + Security Integration
**Add**:
1. End-to-end key generation flow
2. Multi-device key rotation
3. Backup and recovery workflows
4. Hardware + software HSM failover
5. Entropy collection pipeline

**Estimated**: +25 tests, +5% coverage

### Network + Discovery Integration
**Add**:
1. Zero-knowledge bootstrap scenarios
2. Multi-primal discovery flows
3. Network partition recovery
4. Service mesh integration
5. Load balancing scenarios

**Estimated**: +30 tests, +6% coverage

### Monitoring + All Components
**Add**:
1. End-to-end observability
2. Distributed tracing scenarios
3. Metric aggregation workflows
4. Alert propagation tests
5. Health check cascades

**Estimated**: +20 tests, +4% coverage

**Total Phase 2**: +75 tests, +15% coverage = **60%**

---

## 🎯 Phase 3: E2E & Chaos (Weeks 3-4)

**Target**: 60% → 75% (+15%)  
**Focus**: Real-world scenarios and failure modes

### E2E User Workflows
**Add**:
1. Complete primal lifecycle (bootstrap → operation → shutdown)
2. Key generation → storage → usage → rotation
3. Multi-primal coordination workflows
4. Configuration migration scenarios
5. Version upgrade paths

**Estimated**: +40 tests, +7% coverage

### Chaos Engineering
**Add**:
1. Random node failures
2. Network partitions
3. Disk full scenarios
4. Memory exhaustion
5. CPU starvation
6. Clock skew issues
7. DNS failures
8. Certificate expiration

**Estimated**: +35 tests, +5% coverage

### Fault Injection
**Add**:
1. Database connection drops
2. API timeout cascades
3. Cache invalidation storms
4. Rate limiting scenarios
5. Circuit breaker activations

**Estimated**: +20 tests, +3% coverage

**Total Phase 3**: +95 tests, +15% coverage = **75%**

---

## 🎯 Phase 4: Performance & Security (Weeks 5-6)

**Target**: 75% → 90% (+15%)  
**Focus**: Non-functional requirements

### Load & Performance Tests
**Add**:
1. Throughput benchmarks (1K, 10K, 100K requests/sec)
2. Latency percentile tests (p50, p95, p99)
3. Memory usage under load
4. Connection pool exhaustion
5. Concurrent operation limits
6. Database query performance
7. Cache hit ratio validation
8. Network bandwidth saturation

**Estimated**: +30 tests, +5% coverage

### Security Tests
**Add**:
1. Authentication bypass attempts
2. Authorization boundary tests
3. Injection attack resistance
4. Cryptographic primitive validation
5. Side-channel attack resistance
6. Timing attack prevention
7. Resource exhaustion DoS
8. Rate limiting effectiveness

**Estimated**: +35 tests, +6% coverage

### Regression Tests
**Add**:
1. Known bug reproduction tests
2. Historical failure scenarios
3. Edge case collections
4. Platform-specific quirks
5. Version compatibility matrix

**Estimated**: +20 tests, +4% coverage

**Total Phase 4**: +85 tests, +15% coverage = **90%**

---

## 📋 Implementation Strategy

### Week 1: Quick Wins (35% → 45%)
**Days 1-2**: beardog-security tests
- HSM error paths
- Key rotation edge cases
- Device failure scenarios

**Days 3-4**: beardog-tunnel tests
- Network interruptions
- Packet loss handling
- Connection timeouts

**Day 5**: beardog-monitoring tests
- Metric collection errors
- Storage failures
- Query performance

**Deliverable**: +50 tests, 45% coverage

### Week 2: Integration (45% → 60%)
**Days 1-2**: HSM + Security integration
- End-to-end workflows
- Multi-device scenarios

**Days 3-4**: Network + Discovery integration
- Bootstrap scenarios
- Service mesh tests

**Day 5**: Monitoring integration
- Observability workflows
- Health check cascades

**Deliverable**: +75 tests, 60% coverage

### Weeks 3-4: E2E & Chaos (60% → 75%)
**Week 3**: E2E workflows
- Complete lifecycle tests
- Multi-primal coordination

**Week 4**: Chaos engineering
- Failure injection
- Recovery validation

**Deliverable**: +95 tests, 75% coverage

### Weeks 5-6: Performance & Security (75% → 90%)
**Week 5**: Load & performance
- Throughput benchmarks
- Latency tests

**Week 6**: Security & regression
- Attack resistance
- Historical bugs

**Deliverable**: +85 tests, 90% coverage

---

## 🛠️ Test Patterns to Use

### 1. Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn key_rotation_preserves_functionality(
        key_size in 256u32..=4096u32,
        rotation_count in 1usize..=100usize,
    ) {
        // Test that key rotation works for any valid parameters
    }
}
```

### 2. Table-Driven Tests
```rust
#[rstest]
#[case::small_data(1024, Duration::from_millis(10))]
#[case::medium_data(1024 * 1024, Duration::from_millis(100))]
#[case::large_data(10 * 1024 * 1024, Duration::from_secs(1))]
fn test_data_transfer(#[case] size: usize, #[case] max_duration: Duration) {
    // Test different data sizes with expected performance
}
```

### 3. Mock Time for Determinism
```rust
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn test_timeout_behavior() {
    // Fast, deterministic timeout testing
    tokio::time::advance(Duration::from_secs(30)).await;
}
```

### 4. Chaos Injection
```rust
#[tokio::test]
async fn test_network_partition_recovery() {
    let chaos = ChaosInjector::new();
    chaos.inject_network_partition(Duration::from_secs(5)).await;
    // Verify recovery
}
```

---

## 📊 Success Metrics

### Coverage Targets
- Week 1: 45% (✅ if within 2%)
- Week 2: 60% (✅ if within 3%)
- Week 4: 75% (✅ if within 4%)
- Week 6: 90% (✅ if within 5%)

### Quality Metrics
- ✅ All tests must pass
- ✅ No flaky tests (run 100x)
- ✅ Fast execution (<5 min full suite)
- ✅ Clear test names and documentation
- ✅ Proper cleanup (no resource leaks)

### Non-Functional
- ✅ Test coverage report generated
- ✅ CI/CD integration complete
- ✅ Failure scenarios documented
- ✅ Performance baselines established

---

## 🚧 Challenges & Mitigations

### Challenge 1: Hardware Testing
**Issue**: Can't test all HSM devices in CI  
**Mitigation**: 
- Mock interfaces for CI
- Hardware tests in separate suite
- Document manual testing procedures

### Challenge 2: Network Conditions
**Issue**: Can't simulate all network conditions  
**Mitigation**:
- Use network emulation tools (toxiproxy, tc)
- Test on real infrastructure periodically
- Document known limitations

### Challenge 3: Time & Resources
**Issue**: Comprehensive testing takes time  
**Mitigation**:
- Prioritize by risk (security > performance)
- Parallelize test execution
- Incremental coverage growth

---

## 📝 Test Documentation Template

```rust
/// Tests [SPECIFIC SCENARIO]
///
/// # Purpose
/// Verify that [SYSTEM BEHAVIOR] when [CONDITIONS]
///
/// # Test Scenario
/// 1. Setup: [INITIAL STATE]
/// 2. Action: [WHAT WE DO]
/// 3. Assert: [EXPECTED OUTCOME]
///
/// # Edge Cases Covered
/// - [EDGE CASE 1]
/// - [EDGE CASE 2]
///
/// # Failure Modes Tested
/// - [FAILURE MODE 1]
/// - [FAILURE MODE 2]
#[tokio::test]
async fn test_specific_scenario() {
    // Test implementation
}
```

---

## 🎯 Next Actions

### Immediate (Today)
1. Review this plan with team
2. Set up test coverage tracking
3. Create test template files
4. Begin Week 1 Day 1 tests

### This Week
1. Implement Phase 1 tests
2. Monitor coverage growth
3. Document discovered issues
4. Adjust plan as needed

### This Month
1. Complete Phases 1-2
2. Reach 60% coverage milestone
3. Begin Phase 3 planning
4. Prepare for chaos testing

---

## 📈 Coverage Tracking

Use this command to track progress:
```bash
cargo llvm-cov --workspace --all-features --html
open target/llvm-cov/html/index.html
```

---

**Plan Created**: November 19, 2025  
**Target Start**: November 20, 2025  
**Target Complete**: December 31, 2025 (6 weeks)  
**Confidence**: 🟢 HIGH (plan is detailed and realistic)

---

*"From 35% to 90% coverage - one test at a time!"* 🧪✅

