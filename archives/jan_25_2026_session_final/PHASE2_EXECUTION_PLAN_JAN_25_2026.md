# 🚀 Phase 2 Deep Evolution - Execution Plan

**Date**: January 25, 2026  
**Philosophy**: Test issues ARE production issues. Concurrent, robust, modern Rust.  
**Status**: ✅ Executing

---

## 🎯 PRIORITIES (Ordered by Impact)

### 1. **Eliminate Sleep/Serial Tests** (2-3h) 🔴 CRITICAL
**Problem**: 11 serial tests + sleep calls in production = fragility  
**Impact**: Hangs, race conditions, production failures  
**Solution**: Event-driven, concurrent architecture

**Found**:
- 11 `#[serial]` or `single_thread` tests
- ~20 `sleep()` calls in production code (not benchmarks)
- Mutex/RwLock usage that may be overly coarse

**Action**: Evolve to truly concurrent patterns

### 2. **Evolve Production Mocks** (3-4h) 🔴 CRITICAL
**Status**: 90% done, 10% remaining  
**Problem**: Some platform-specific mocks still in production paths  
**Solution**: Complete implementations or proper error handling

**Targets**:
- Android StrongBox: Proper error messages (not mock data)
- iOS Secure Enclave: Complete or remove
- Discovery mocks: Real capability-based runtime discovery

### 3. **Smart Large File Refactoring** (8-12h) 🟡 HIGH
**Problem**: 6 files > 1000 lines = complexity, hard to test  
**Solution**: Domain-driven splitting, not arbitrary

**Targets**:
1. `btsp_provider.rs` (1330 lines) → 5 domain modules
2. `hsm/manager/mod.rs` (1140 lines) → Provider modules
3. `genetic_crypto.rs` (1069 lines) → Operation modules

### 4. **Hardcoding → Capability-Based** (5-8h) 🟡 HIGH
**Problem**: 468 IPs + 172 paths hardcoded  
**Solution**: Runtime discovery, self-knowledge only

**Pattern**:
```rust
// BAD: Hardcoded
let addr = "127.0.0.1:8080";

// GOOD: Discovered
let addr = discover_capability("api").await?;
```

### 5. **Mutex → Lock-Free** (4-6h) 🟢 MEDIUM
**Problem**: Coarse-grained locks = contention  
**Solution**: Fine-grained, lock-free when possible

**Found**: 15 files with Mutex/RwLock in `beardog-core/src`

---

## 🔧 EXECUTION PHASE 1: CONCURRENT & ROBUST (2-4h)

### Task 1.1: Identify Sleep-Based Tests
```rust
// Example problematic test:
#[tokio::test]
async fn test_with_sleep() {
    start_server().await;
    tokio::time::sleep(Duration::from_millis(100)).await;  // ❌ Fragile
    // test logic
}

// Evolve to:
#[tokio::test]
async fn test_event_driven() {
    let (server, ready_rx) = start_server().await;
    ready_rx.await?;  // ✅ Wait for actual readiness signal
    // test logic
}
```

### Task 1.2: Make Tests Fully Concurrent
**Remove**: `#[serial]`, `single_thread` flavor  
**Add**: Proper isolation (unique ports, temp dirs, etc.)

**Pattern**:
```rust
// BAD: Serial to avoid conflicts
#[tokio::test]
#[serial]
async fn test_a() { /* uses port 8080 */ }

// GOOD: Isolated
#[tokio::test]
async fn test_a() {
    let port = allocate_test_port();  // ✅ Dynamic allocation
    /* uses unique port */
}
```

### Task 1.3: Evolve Sleep-Based Production Code
**Target**: `primal_runtime_discovery.rs` - has `std::thread::sleep(150ms)`

**Evolution**:
```rust
// BAD: Polling with sleep
loop {
    if condition { break; }
    std::thread::sleep(Duration::from_millis(150));  // ❌ Wasteful
}

// GOOD: Event-driven
let (tx, rx) = oneshot::channel();
spawn_watcher(tx);
rx.await?;  // ✅ Wakes immediately when ready
```

---

## 🔧 EXECUTION PHASE 2: PRODUCTION MOCKS (3-4h)

### Task 2.1: Android StrongBox Evolution
**File**: `safe_android_provider.rs`  
**Status**: 90% done, needs final polish

**Current**: Returns `vec![0u8; size]` for some operations  
**Target**: Return proper errors with helpful messages

```rust
// BEFORE:
fn sign(&self, _data: &[u8]) -> Vec<u8> {
    vec![0u8; 64]  // ❌ Mock data
}

// AFTER:
fn sign(&self, _data: &[u8]) -> Result<Vec<u8>, Error> {
    Err(Error::unsupported(
        "Android StrongBox requires Android target. \
         Use software HSM or build for Android."
    ))  // ✅ Clear error
}
```

### Task 2.2: Discovery Mock Evolution
**Files**: `universal_discovery/`, `zero_knowledge_bootstrap/`

**Pattern**:
```rust
// BAD: Mock discovery
async fn discover_primal(&self, name: &str) -> String {
    "127.0.0.1:8080".to_string()  // ❌ Hardcoded mock
}

// GOOD: Real discovery
async fn discover_primal(&self, capability: Capability) -> Result<Endpoint> {
    self.query_registry(capability).await  // ✅ Real capability lookup
}
```

---

## 🔧 EXECUTION PHASE 3: SMART REFACTORING (8-12h)

### Task 3.1: btsp_provider.rs (1330 lines)
**Split by domain**:
1. `btsp_provider.rs` (200 lines) - Core coordinator
2. `contact_exchange.rs` (300 lines) - Exchange protocol
3. `session_management.rs` (300 lines) - Session handling
4. `crypto_operations.rs` (300 lines) - Crypto primitives
5. `state_machine.rs` (230 lines) - State transitions

**Benefits**:
- ✅ Each file < 400 lines
- ✅ Clear domain boundaries
- ✅ Easier to test
- ✅ Better concurrency (less lock contention)

### Task 3.2: hsm/manager/mod.rs (1140 lines)
**Split by provider type**:
1. `manager.rs` (200 lines) - Core manager
2. `software_provider.rs` (300 lines) - Software HSM
3. `hardware_provider.rs` (300 lines) - PKCS#11/TPM
4. `cloud_provider.rs` (200 lines) - Cloud KMS
5. `provider_registry.rs` (140 lines) - Provider discovery

---

## 🔧 EXECUTION PHASE 4: HARDCODING ELIMINATION (5-8h)

### Task 4.1: IP/Port Discovery Pattern
**Files**: 468 instances of `127.0.0.1`, `localhost`, hardcoded ports

**Evolution**:
```rust
// BEFORE: Hardcoded everywhere
const SONGBIRD_ADDR: &str = "127.0.0.1:9001";
const BEARDOG_ADDR: &str = "127.0.0.1:9002";

// AFTER: Capability-based discovery
async fn connect_to_primal(capability: Capability) -> Result<Connection> {
    let endpoint = self.capabilities
        .discover(capability)
        .await?;
    Connection::new(endpoint).await
}
```

### Task 4.2: Self-Knowledge Only
**Principle**: Primal only knows itself, discovers others at runtime

**Pattern**:
```rust
// BAD: Knowledge of other primals
const KNOWN_PRIMALS: &[(&str, &str)] = &[
    ("songbird", "/tmp/songbird.sock"),
    ("beardog", "/tmp/beardog.sock"),
];

// GOOD: Self-knowledge + discovery
struct PrimalConfig {
    my_capabilities: Vec<Capability>,  // ✅ What I provide
    my_endpoints: Vec<Endpoint>,       // ✅ Where I listen
    // No knowledge of other primals!
}
```

---

## 📊 SUCCESS METRICS

### Phase 1: Concurrent & Robust
- [ ] 0 sleep() calls in production code
- [ ] 0 serial tests (except chaos)
- [ ] All tests pass concurrently in < 30s

### Phase 2: Production Mocks
- [ ] 0 mock data in production binary
- [ ] All platform-specific code returns proper errors
- [ ] Real implementations or clear "unsupported" errors

### Phase 3: Smart Refactoring
- [ ] 0 files > 1000 lines
- [ ] Each module < 400 lines
- [ ] Clear domain boundaries

### Phase 4: Zero Hardcoding
- [ ] < 10 hardcoded IPs (only test fixtures)
- [ ] 100% runtime discovery for primal-to-primal
- [ ] Self-knowledge only pattern

---

## 🚀 IMMEDIATE NEXT STEPS

1. **START**: Identify and list all serial/sleep tests
2. **EVOLVE**: Make tests concurrent (unique resources)
3. **REMOVE**: Sleep calls in production
4. **COMPLETE**: Android/iOS mock evolution
5. **REFACTOR**: btsp_provider.rs first (highest complexity)

**Philosophy**: Every change makes production more robust, tests faster, code clearer.

**Time Estimate**: 18-28 hours total (Phase 2 work)

