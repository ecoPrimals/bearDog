# Test Modernization: From Sleeps to Event-Driven Concurrent Patterns

**Date**: November 19, 2025  
**Status**: Active Migration  
**Priority**: CRITICAL - Test issues = Production issues

---

## 🎯 Philosophy

**Tests should be:**
- ✅ **Event-driven** (not time-driven)
- ✅ **Fully concurrent** (no serial execution)
- ✅ **Deterministic** (reproducible results)
- ✅ **Fast** (no arbitrary delays)
- ✅ **Robust** (work under load)

**Anti-Pattern**: `sleep()` in tests = Flaky, slow, non-deterministic

---

## 🔴 ANTI-PATTERNS (77 instances found)

###  1. Cache Expiration Testing
```rust
// ❌ BAD: Sleep and hope cache expires
let _ = optimizer.cache_discovery_result(capability, providers).await;
tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
assert!(optimizer.is_cache_expired(&capability).await);
```

```rust
// ✅ GOOD: Mock time control
use tokio::time::{pause, advance, Duration};

pause(); // Pause virtual time
let _ = optimizer.cache_discovery_result(capability, providers).await;
advance(Duration::from_millis(150)).await; // Advance virtual time
assert!(optimizer.is_cache_expired(&capability).await);
```

### 2. Timing/Uptime Testing
```rust
// ❌ BAD: Sleep to allow time to pass
for _ in 0..5 {
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    let uptime = state.start_time.elapsed();
    assert!(uptime >= prev_uptime);
}
```

```rust
// ✅ GOOD: Use actual time measurement
let start = std::time::Instant::now();
let state = core.state.read().await;
let uptime1 = state.start_time.elapsed();

// Do actual work
perform_operations().await;

let uptime2 = state.start_time.elapsed();
assert!(uptime2 > uptime1);
```

### 3. Concurrent Operation Simulation
```rust
// ❌ BAD: Sleep to simulate work
for i in 0..10 {
    tokio::time::sleep(tokio::time::Duration::from_millis(i * 5)).await;
    process_item(i).await;
}
```

```rust
// ✅ GOOD: True concurrent execution
let handles: Vec<_> = (0..10)
    .map(|i| {
        tokio::spawn(async move {
            process_item(i).await
        })
    })
    .collect();

let results: Vec<_> = futures::future::join_all(handles)
    .await
    .into_iter()
    .collect();
```

### 4. Synchronization
```rust
// ❌ BAD: Sleep and hope operation completes
spawn_background_task();
std::thread::sleep(Duration::from_millis(100));
assert!(task_completed());
```

```rust
// ✅ GOOD: Event-driven synchronization
let (tx, rx) = tokio::sync::oneshot::channel();

spawn_background_task(tx);
rx.await.expect("Task should signal completion");
assert!(task_completed());
```

### 5. Performance Testing
```rust
// ❌ BAD: Sleep to measure duration
let start = Instant::now();
tokio::time::sleep(Duration::from_millis(50)).await;
let elapsed = start.elapsed();
optimizer.record_discovery_time(elapsed.as_millis() as u64).await;
```

```rust
// ✅ GOOD: Measure actual operations
let start = Instant::now();
perform_real_discovery_operation().await;
let elapsed = start.elapsed();
optimizer.record_discovery_time(elapsed.as_millis() as u64).await;
```

---

## 🛠️ MODERN PATTERNS

### Pattern 1: Mock Time (tokio::time::pause)
```rust
use tokio::time::{pause, advance, resume, Duration};

#[tokio::test]
async fn test_with_mock_time() {
    pause(); // Freeze virtual time
    
    let start = Instant::now();
    
    // Advance time without actual waiting
    advance(Duration::from_secs(1)).await;
    
    assert_eq!(start.elapsed().as_secs(), 1);
    
    resume(); // Resume normal time
}
```

### Pattern 2: Channel Synchronization
```rust
#[tokio::test]
async fn test_with_channel_sync() {
    let (ready_tx, ready_rx) = tokio::sync::oneshot::channel();
    let (done_tx, done_rx) = tokio::sync::oneshot::channel();
    
    tokio::spawn(async move {
        ready_tx.send(()).unwrap();
        perform_work().await;
        done_tx.send(()).unwrap();
    });
    
    ready_rx.await.unwrap(); // Wait for ready
    // Do assertions
    done_rx.await.unwrap(); // Wait for completion
}
```

### Pattern 3: Barrier Synchronization
```rust
use tokio::sync::Barrier;
use std::sync::Arc;

#[tokio::test]
async fn test_concurrent_start() {
    let barrier = Arc::new(Barrier::new(10));
    let mut handles = vec![];
    
    for i in 0..10 {
        let barrier = Arc::clone(&barrier);
        handles.push(tokio::spawn(async move {
            barrier.wait().await; // All start simultaneously
            perform_work(i).await
        }));
    }
    
    let results = futures::future::join_all(handles).await;
    assert_eq!(results.len(), 10);
}
```

### Pattern 4: Timeout Testing
```rust
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_operation_timeout() {
    let result = timeout(
        Duration::from_millis(100),
        slow_operation()
    ).await;
    
    assert!(result.is_err(), "Should timeout");
}
```

### Pattern 5: WaitGroup Pattern
```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

#[tokio::test]
async fn test_wait_for_all() {
    let sem = Arc::new(Semaphore::new(0));
    let mut handles = vec![];
    
    for i in 0..10 {
        let sem = Arc::clone(&sem);
        handles.push(tokio::spawn(async move {
            perform_work(i).await;
            sem.add_permits(1); // Signal completion
        }));
    }
    
    // Wait for all 10 to complete
    let _ = sem.acquire_many(10).await.unwrap();
}
```

---

## 📋 MIGRATION CHECKLIST

### High Priority (Blocking Production)
- [ ] Cache expiration tests (performance_optimization_tests.rs)
- [ ] Health check tests (health_tests.rs)
- [ ] Concurrency tests (concurrency_tests.rs)
- [ ] Key rotation tests (key_rotation_manager_tests.rs)

### Medium Priority (CI Stability)
- [ ] Authorization tests (authorization_comprehensive_tests.rs)
- [ ] Sovereignty tests (sovereignty_tests/*.rs)
- [ ] Recovery tests (recovery_tests/*.rs)
- [ ] Threat monitoring tests (monitoring_tests.rs)

### Low Priority (Examples/Scripts)
- [ ] Example files (test_ctaphid_init_debug.rs)
- [ ] Benchmark tests (performance_benchmarks_comprehensive_tests.rs)
- [ ] Script tests (pixel8a_simple_hsm_test.rs)

---

## 🎯 SUCCESS CRITERIA

**Before Migration:**
- 77 sleep() calls in tests
- Flaky test potential: HIGH
- Test duration: SLOW (wait time = wasted time)
- Determinism: LOW (timing-dependent)

**After Migration:**
- 0 sleep() calls in critical tests
- Flaky test potential: MINIMAL
- Test duration: FAST (no artificial delays)
- Determinism: HIGH (event-driven)

---

## 🚀 EXECUTION PLAN

### Phase 1: Critical Path (2-3 hours)
1. Fix cache expiration tests (pause/advance)
2. Fix health monitoring tests (channel sync)
3. Fix concurrency tests (barriers/channels)
4. Fix key rotation tests (mock time)

### Phase 2: Test Stability (2-3 hours)
5. Fix authorization tests
6. Fix sovereignty tests
7. Fix recovery tests
8. Fix threat tests

### Phase 3: Cleanup (1-2 hours)
9. Fix remaining test files
10. Document patterns
11. Add linter rules to prevent sleeps
12. Update test guidelines

**Total Time**: 5-8 hours  
**Impact**: Robust, fast, deterministic test suite

---

## 📚 REFERENCES

- [Tokio Time Documentation](https://docs.rs/tokio/latest/tokio/time/)
- [Testing with Mock Time](https://docs.rs/tokio/latest/tokio/time/fn.pause.html)
- [Synchronization Primitives](https://docs.rs/tokio/latest/tokio/sync/)
- BearDog Coding Standards: `BEARDOG_CODING_STANDARDS.md`

---

**Status**: 🔴 ACTIVE MIGRATION  
**Owner**: Development Team  
**Next Step**: Begin Phase 1 fixes

