# 🚀 Phase 3: Test Coverage Expansion

**Session**: November 19, 2025 (Evening - Continued)  
**Goal**: 35% → 45% coverage  
**Strategy**: Focus on under-covered modules (monitoring, HSM integration, tunnel recovery)

---

## 📊 BASELINE ANALYSIS

### **Test Count by Module**
| Module | Tests | Files | Avg Tests/File | Assessment |
|--------|-------|-------|----------------|------------|
| `beardog-security` | 1,002 | 81 | 12.4 | ✅ Excellent |
| `beardog-tunnel` | 1,315 | 131 | 10.0 | ✅ Excellent |
| `beardog-monitoring` | 113 | 9 | 12.6 | ⚠️ **Need more files** |

### **Coverage Gaps Identified**
1. **beardog-monitoring**: Only 9 test files (vs 81 in security)
2. **HSM error paths**: Hardware failure scenarios
3. **Tunnel recovery**: Connection interruption handling

---

## 🎯 PHASE 3 TARGETS

### **Target 1: Monitoring Error Paths** (+15 tests, +3% coverage)
**Focus**: Error handling, edge cases, failure recovery

**Tests to Add**:
1. ✅ Metric collection failures
2. ✅ Storage backend errors
3. ✅ Metric overflow handling
4. ✅ Concurrent access errors
5. ✅ Memory pressure scenarios
6. ✅ Network partition recovery
7. ✅ Invalid metric format handling
8. ✅ Timestamp ordering violations
9. ✅ Aggregation errors
10. ✅ Export failures
11. ✅ Health check timeouts
12. ✅ Alert trigger failures
13. ✅ Metric retention policy errors
14. ✅ Configuration reload failures
15. ✅ Graceful degradation scenarios

### **Target 2: HSM Integration Error Paths** (+15 tests, +3% coverage)
**Focus**: Hardware failures, device errors, recovery

**Tests to Add**:
1. Device disconnection during operation
2. Timeout during key generation
3. Invalid device response handling
4. Concurrent operation conflicts
5. Device busy/locked scenarios
6. Entropy exhaustion
7. Key rotation during active operations
8. Device firmware mismatch
9. Certificate expiration
10. Attestation failures
11. PIN/biometric failures
12. Device reset during operation
13. Multi-device failover
14. Degraded mode operation
15. Recovery after device reconnection

### **Target 3: Tunnel Recovery Tests** (+20 tests, +4% coverage)
**Focus**: Network interruptions, reconnection, state recovery

**Tests to Add**:
1. Connection drop during transmission
2. Packet loss scenarios
3. Out-of-order packet handling
4. Duplicate packet handling
5. Network partition recovery
6. DNS resolution failures
7. Certificate validation errors
8. TLS renegotiation
9. Timeout and retry logic
10. Exponential backoff verification
11. Circuit breaker integration
12. Keepalive failure handling
13. Bandwidth throttling
14. Connection pool exhaustion
15. State synchronization after reconnect
16. Pending operations on disconnect
17. Message replay prevention
18. Connection migration
19. Graceful shutdown during active transfers
20. Resource cleanup on error

---

## 🎨 TEST PATTERNS TO APPLY

### **Pattern 1: Error Injection**
```rust
#[tokio::test]
async fn test_metric_collection_failure() -> BearDogResult<()> {
    // Create collector with failing backend
    let collector = MetricCollector::with_backend(FailingBackend::new());
    
    // Attempt to collect metric
    let result = collector.collect("test_metric", 42.0).await;
    
    // Verify graceful error handling
    assert!(result.is_err());
    assert_matches!(result.unwrap_err(), BearDogError::Monitoring(_));
    
    // Verify collector remains operational
    assert!(collector.is_healthy());
    Ok(())
}
```

### **Pattern 2: Concurrent Failure**
```rust
#[tokio::test]
async fn test_concurrent_metric_writes() -> BearDogResult<()> {
    let collector = Arc::new(MetricCollector::new());
    
    // Spawn multiple concurrent writers
    let mut handles = vec![];
    for i in 0..100 {
        let collector = Arc::clone(&collector);
        handles.push(tokio::spawn(async move {
            collector.collect(&format!("metric_{}", i), i as f64).await
        }));
    }
    
    // All should complete successfully
    for handle in handles {
        assert!(handle.await.unwrap().is_ok());
    }
    
    Ok(())
}
```

### **Pattern 3: Recovery Validation**
```rust
#[tokio::test]
async fn test_recovery_after_failure() -> BearDogResult<()> {
    let mut collector = MetricCollector::new();
    
    // Inject failure
    collector.inject_failure();
    
    // Attempt operation (should fail)
    assert!(collector.collect("metric", 1.0).await.is_err());
    
    // Trigger recovery
    collector.recover().await?;
    
    // Verify operation succeeds after recovery
    assert!(collector.collect("metric", 2.0).await.is_ok());
    
    Ok(())
}
```

---

## 📈 EXPECTED IMPACT

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| **Overall Coverage** | 35% | 45% | +10% |
| **Monitoring Coverage** | ~25% | ~55% | +30% |
| **HSM Coverage** | ~28% | ~40% | +12% |
| **Tunnel Coverage** | ~29% | ~42% | +13% |
| **Total Tests** | 1,441 | 1,491 | +50 tests |

---

## 🚀 EXECUTION PLAN

### **Step 1**: Create Monitoring Error Tests (30 min)
- File: `monitoring_error_path_comprehensive_tests.rs`
- Tests: 15 error scenarios
- Pattern: Error injection + recovery validation

### **Step 2**: Create HSM Error Tests (30 min)
- File: `hsm_error_path_comprehensive_tests.rs`
- Tests: 15 device failure scenarios
- Pattern: Hardware failure simulation

### **Step 3**: Create Tunnel Recovery Tests (45 min)
- File: `tunnel_recovery_comprehensive_tests.rs`
- Tests: 20 network failure scenarios
- Pattern: Network partition + reconnection

### **Step 4**: Run Coverage Analysis (15 min)
- Execute: `cargo llvm-cov --workspace --lib`
- Verify: Coverage improvement
- Document: Results

**Total Time**: ~2 hours  
**Expected Coverage**: 35% → 45%+

---

**Status**: 🚀 **READY TO BEGIN**  
**Next**: Create monitoring error path tests

