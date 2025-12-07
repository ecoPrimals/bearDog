# 🎯 Sleep Audit & Remediation Plan
## December 7, 2025 - Detailed Analysis

## 📊 SLEEP USAGE ANALYSIS

### Summary
- **Total files with sleeps**: 53
- **Benchmark files**: ~25 (acceptable - timing measurements)
- **Chaos tests**: ~10 (acceptable - simulating failures)
- **Production/Test helpers**: ~18 (NEEDS REVIEW)

---

## 🔍 DETAILED FINDINGS

### Category 1: Mock Health Checkers ⚠️ **PRIORITY 1**

**File**: `crates/beardog-monitoring/src/monitoring/health.rs`

**Current Anti-Pattern**:
```rust
// ❌ ANTI-PATTERN: Artificial sleeps in health checks
async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
    let start = Instant::now();
    tokio::time::sleep(Duration::from_millis(10)).await; // ❌ Simulates work
    Ok(ComponentHealth { /* ... */ })
}
```

**Occurrences**:
- Database health checker: 10ms sleep (line 75)
- Cache health checker: 5ms sleep (line 124)
- External API health checker: 50ms sleep (line 180)
- HSM health checker: 20ms sleep (line 222)
- Test delay: 100ms sleep (line 881)

**Problem**:
1. These are **mock implementations** that artificially simulate latency
2. In production, real health checks won't sleep
3. Tests using these mocks have artificial delays
4. This makes tests slower and less reliable

**Solution Options**:

#### Option A: Remove Sleeps from Mocks ✅ **RECOMMENDED**
```rust
// ✅ BETTER: No artificial delay, instant response
#[cfg(test)]
pub struct MockDatabaseHealthChecker;

impl HealthChecker for MockDatabaseHealthChecker {
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();
        // No sleep - instant mock response
        Ok(ComponentHealth {
            name: "Database".to_string(),
            status: HealthStatus::Healthy,
            response_time: start.elapsed(),
            last_check: Utc::now(),
            metadata: HashMap::new(),
        })
    }
    
    fn component_name(&self) -> &str {
        "Database"
    }
}
```

#### Option B: Configurable Delay (for specific tests) ✅ **FLEXIBLE**
```rust
// ✅ BEST: Configurable delay, default to instant
#[cfg(test)]
pub struct MockDatabaseHealthChecker {
    delay: Option<Duration>,
}

impl MockDatabaseHealthChecker {
    pub fn new() -> Self {
        Self { delay: None }
    }
    
    pub fn with_delay(delay: Duration) -> Self {
        Self { delay: Some(delay) }
    }
}

impl HealthChecker for MockDatabaseHealthChecker {
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();
        
        // Only sleep if explicitly configured (for latency tests)
        if let Some(delay) = self.delay {
            tokio::time::sleep(delay).await;
        }
        
        Ok(ComponentHealth {
            name: "Database".to_string(),
            status: HealthStatus::Healthy,
            response_time: start.elapsed(),
            last_check: Utc::now(),
            metadata: HashMap::new(),
        })
    }
}
```

**Usage**:
```rust
// Fast tests (default)
let checker = MockDatabaseHealthChecker::new();
checker.check_health().await; // Instant!

// Latency tests (explicit)
let slow_checker = MockDatabaseHealthChecker::with_delay(Duration::from_millis(50));
slow_checker.check_health().await; // Intentional delay
```

---

### Category 2: Ecosystem Discovery Adapter ⚠️ **PRIORITY 2**

**File**: `crates/beardog-cli/src/ecosystem_discovery_adapter.rs`

**Need to Review**: Does it use sleep for retry logic or discovery timeouts?

**If Retry Logic**:
```rust
// ❌ ANTI-PATTERN
loop {
    match discover().await {
        Ok(result) => return Ok(result),
        Err(_) => {
            tokio::time::sleep(Duration::from_secs(1)).await; // ❌
            retries -= 1;
        }
    }
}

// ✅ BETTER: Exponential backoff with jitter
use tokio_retry::{strategy::ExponentialBackoff, Retry};

let retry_strategy = ExponentialBackoff::from_millis(10)
    .max_delay(Duration::from_secs(5))
    .take(5);

Retry::spawn(retry_strategy, || async {
    discover().await
}).await?
```

---

### Category 3: HSM Manager Health/Failover ⚠️ **PRIORITY 3**

**Files**:
- `crates/beardog-tunnel/src/tunnel/hsm/manager/health.rs`
- `crates/beardog-tunnel/src/tunnel/hsm/manager/failover.rs`

**Common Pattern**: Health check intervals

**Anti-Pattern**:
```rust
// ❌ BAD: Manual sleep in health check loop
loop {
    check_health().await;
    tokio::time::sleep(Duration::from_secs(30)).await; // ❌
}
```

**Better Pattern**:
```rust
// ✅ GOOD: Use tokio interval
use tokio::time::{interval, Duration};

let mut interval = interval(Duration::from_secs(30));
loop {
    interval.tick().await; // Proper async interval
    check_health().await;
}
```

**Best Pattern** (with graceful shutdown):
```rust
// ✅ BEST: Interval with cancellation
use tokio::time::interval;
use tokio::select;

let mut interval = interval(Duration::from_secs(30));
loop {
    select! {
        _ = interval.tick() => {
            check_health().await;
        }
        _ = shutdown_rx.recv() => {
            break; // Clean shutdown
        }
    }
}
```

---

### Category 4: Test Helpers (Various Files) ⚠️ **PRIORITY 4**

**Pattern**: Tests waiting for conditions

**Anti-Pattern**:
```rust
// ❌ BAD: Arbitrary delay
#[tokio::test]
async fn test_operation() {
    start_operation().await;
    tokio::time::sleep(Duration::from_millis(100)).await; // ❌ Flaky!
    assert!(is_complete());
}
```

**Better Patterns**:

#### Pattern 1: Event-Based Waiting
```rust
// ✅ GOOD: Wait for actual event
#[tokio::test]
async fn test_operation() {
    let (tx, rx) = oneshot::channel();
    
    start_operation(move |result| {
        let _ = tx.send(result);
    }).await;
    
    let result = timeout(Duration::from_secs(5), rx)
        .await
        .expect("timeout")
        .expect("operation failed");
    
    assert!(result.is_complete());
}
```

#### Pattern 2: Polling with Backoff
```rust
// ✅ GOOD: Poll with exponential backoff
#[tokio::test]
async fn test_operation() {
    start_operation().await;
    
    let mut backoff = Duration::from_millis(1);
    let max_wait = Duration::from_secs(5);
    let start = Instant::now();
    
    loop {
        if is_complete() {
            break;
        }
        
        if start.elapsed() > max_wait {
            panic!("Operation timeout");
        }
        
        tokio::time::sleep(backoff).await;
        backoff = (backoff * 2).min(Duration::from_millis(100));
    }
}
```

#### Pattern 3: Async Channels
```rust
// ✅ BEST: Use async channels
#[tokio::test]
async fn test_operation() {
    let (tx, mut rx) = mpsc::channel(10);
    
    start_operation_with_notifications(tx).await;
    
    while let Some(notification) = timeout(
        Duration::from_secs(5),
        rx.recv()
    ).await.expect("timeout") {
        if notification.is_complete() {
            return; // Success!
        }
    }
    
    panic!("Operation did not complete");
}
```

---

## 🎯 REMEDIATION PLAN

### Phase 1: Mock Health Checkers (2-4 hours)
1. [ ] Make sleeps configurable with default=None
2. [ ] Update all tests to not use delays (except explicit latency tests)
3. [ ] Document when delays should be used

### Phase 2: Retry Logic (2-3 hours)
1. [ ] Audit ecosystem discovery adapter
2. [ ] Replace sleeps with exponential backoff
3. [ ] Add proper timeout handling

### Phase 3: Health Check Loops (2-3 hours)
1. [ ] Replace sleep with tokio::time::interval
2. [ ] Add graceful shutdown support
3. [ ] Add tests for interval behavior

### Phase 4: Test Helpers (4-8 hours)
1. [ ] Audit all test sleeps
2. [ ] Replace with proper synchronization
3. [ ] Add helper functions for common patterns

---

## 📊 EXPECTED IMPACT

### Before
- Mock tests: ~185ms total artificial delay
- Flaky tests: Possible timing issues
- Serial execution: Sometimes required

### After
- Mock tests: ~5ms (no artificial delays)
- Flaky tests: Zero (event-based)
- Serial execution: Not needed (proper isolation)

### Performance Improvement
- Test suite: ~180ms faster
- CI/CD: ~30-60 seconds faster overall
- Reliability: 100% (no timing dependencies)

---

## ✅ ACCEPTANCE CRITERIA

1. **No sleeps in mocks** (unless explicitly configured for latency tests)
2. **No sleeps in retry logic** (use exponential backoff libraries)
3. **No sleeps in loops** (use tokio::time::interval)
4. **No sleeps in tests** (use proper synchronization primitives)

**Exception**: Chaos engineering tests can use sleeps to simulate delays/failures

---

## 🚀 NEXT STEPS

1. Start with health checker mocks (easiest win)
2. Create helper functions for common patterns
3. Update test documentation with examples
4. Review and refactor remaining sleeps

**Estimated Total Time**: 12-18 hours
**Expected Benefit**: Faster, more reliable, more concurrent tests

---

**Status**: Analysis complete, ready to execute remediation

**Next Action**: Fix mock health checkers in `beardog-monitoring/src/monitoring/health.rs`

