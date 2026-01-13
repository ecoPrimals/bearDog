# 🔥 Deep Debt Evolution Plan - January 13, 2026

**Date**: January 13, 2026  
**Status**: 🚀 **IN PROGRESS**  
**Goal**: Evolve to modern idiomatic Rust with zero technical debt

---

## 🎯 **Objectives**

1. **Test Coverage** - Expand from current to 90% (measure with llvm-cov)
2. **External Dependencies** - Analyze and evolve to Rust alternatives
3. **Large Files** - Smart refactoring (not just splitting)
4. **Unsafe Code** - Evolve to fast AND safe Rust
5. **Hardcoding** - Evolve to agnostic capability-based discovery
6. **Self-Knowledge Only** - Primals discover others at runtime
7. **Mock Evolution** - Isolate to tests, evolve production mocks to real implementations

---

## 📊 **Current State Analysis**

### **Large Files (>1000 lines)**

Found 3 files exceeding limit:

1. **`crates/beardog-tunnel/src/btsp_provider.rs`** (1,191 lines)
   - Needs: Smart modular refactoring
   - Priority: HIGH

2. **`crates/beardog-tunnel/src/tunnel/hsm/manager/mod.rs`** (1,140 lines)
   - Needs: Capability-based module split
   - Priority: HIGH

3. **`crates/beardog-tunnel/src/api/trust.rs`** (1,037 lines)
   - Needs: Domain-driven split
   - Priority: MEDIUM

### **Unsafe Code**

- **Total**: 141 unsafe blocks across 65 files
- **Status**: All currently justified (SIMD, FFI, performance)
- **Goal**: Reduce or better justify each

**Top files with unsafe**:
- `crates/beardog-security/src/simd_crypto.rs` (10 unsafe blocks)
- `crates/beardog-utils/src/simd_safe.rs` (7 unsafe blocks)
- `crates/beardog-utils/src/simd/safe_ops.rs` (7 unsafe blocks)
- `crates/beardog-utils/src/ultimate_safety.rs` (5 unsafe blocks)

**Categories**:
1. **SIMD Operations** (~60%) - Performance critical, need audit
2. **FFI (Android/iOS)** (~25%) - Platform integration, necessary
3. **Zero-Copy** (~10%) - Performance optimization
4. **Other** (~5%) - Needs review

### **Mock Usage**

- **Total**: 928 matches across 128 files
- **In Tests**: ~85% (acceptable)
- **In Production**: ~15% (needs evolution)

**Production Mocks to Evolve**:
- `crates/beardog-tunnel/src/tunnel/hsm/stub_types.rs` (10 mocks) - CRITICAL
- `crates/beardog-workflows/src/workflows/canonical_traits.rs` (31 mocks)
- `crates/beardog-types/src/canonical/providers_unified/zero_cost_registry.rs` (34 mocks)
- `crates/beardog-types/src/canonical/traits/cache.rs` (16 mocks)

### **External Dependencies**

**Current Top-Level Deps**:
- `tokio` v1.47.1 - ✅ Pure Rust, keep
- `serde` v1.0.220 - ✅ Pure Rust, keep
- `tracing` v0.1.41 - ✅ Pure Rust, keep
- `async-trait` v0.1.89 - ✅ Pure Rust, keep
- `ed25519-dalek` v2.2.0 - ✅ Pure Rust crypto, keep
- `chrono` v0.4.42 - ✅ Pure Rust, keep
- `base64` v0.22.1 - ✅ Pure Rust, keep

**Status**: ✅ Already using pure Rust ecosystem (excellent!)

**Potential Optimizations**:
- Consider `time` instead of `chrono` (smaller, faster) - LOW priority
- Consider custom base64 for zero-copy - LOW priority

---

## 🚀 **Execution Plan**

### **Phase 1: Measurement & Analysis** (1-2 hours)

**1.1: Test Coverage Baseline**
```bash
cargo install cargo-llvm-cov
cargo llvm-cov --workspace --html
```
- Establish current coverage %
- Identify untested modules
- Create coverage improvement plan

**1.2: Unsafe Code Audit**
- Categorize all 141 unsafe blocks
- Justify each with safety comment
- Identify candidates for safe alternatives

**1.3: Mock Inventory**
- Separate test mocks from production
- Document production mock purposes
- Plan evolution to real implementations

**1.4: Hardcoding Analysis**
- Find all IP addresses, ports, URLs
- Find all primal name hardcoding
- Plan capability-based alternatives

### **Phase 2: Large File Refactoring** (4-6 hours)

**Priority 1: btsp_provider.rs (1,191 lines)**

Strategy: Domain-driven modular split

```
btsp_provider.rs (1,191 lines)
↓
Split into:
├── btsp/provider.rs (300 lines) - Core provider logic
├── btsp/connection.rs (250 lines) - Connection management
├── btsp/crypto.rs (200 lines) - Crypto operations
├── btsp/discovery.rs (200 lines) - Service discovery
├── btsp/health.rs (150 lines) - Health checking
└── btsp/mod.rs (91 lines) - Module exports
```

**Priority 2: hsm/manager/mod.rs (1,140 lines)**

Strategy: Capability-based split

```
hsm/manager/mod.rs (1,140 lines)
↓
Split into:
├── manager/orchestrator.rs (300 lines) - Main orchestration
├── manager/selection.rs (250 lines) - HSM selection logic
├── manager/lifecycle.rs (200 lines) - Lifecycle management
├── manager/health.rs (150 lines) - Health monitoring
├── manager/discovery.rs (150 lines) - Capability discovery
└── manager/mod.rs (90 lines) - Module exports
```

**Priority 3: api/trust.rs (1,037 lines)**

Strategy: API endpoint split

```
api/trust.rs (1,037 lines)
↓
Split into:
├── trust/verification.rs (300 lines) - Verification logic
├── trust/lineage.rs (250 lines) - Lineage operations
├── trust/consensus.rs (200 lines) - Consensus handling
├── trust/endpoints.rs (200 lines) - HTTP endpoints
└── trust/mod.rs (87 lines) - Module exports
```

### **Phase 3: Unsafe Code Evolution** (6-8 hours)

**Strategy**: Safe abstractions over unsafe primitives

**3.1: SIMD Operations** (~60 unsafe blocks)

Current pattern:
```rust
unsafe {
    _mm256_add_epi32(a, b)
}
```

Evolve to:
```rust
// Safe wrapper with bounds checking
pub fn safe_simd_add(a: &[u8], b: &[u8]) -> Result<Vec<u8>, BearDogError> {
    if a.len() != b.len() {
        return Err(BearDogError::invalid_input("Length mismatch"));
    }
    // Safety: Lengths verified, alignment checked
    unsafe {
        simd_add_unchecked(a, b)
    }
}
```

**3.2: FFI (Android/iOS)** (~35 unsafe blocks)

Current pattern:
```rust
unsafe {
    (*env).CallObjectMethod(...)
}
```

Evolve to:
```rust
// Safe FFI wrapper
pub struct SafeJniEnv<'a>(&'a JNIEnv);

impl SafeJniEnv<'_> {
    pub fn call_object_method(&self, ...) -> Result<JObject, JniError> {
        // Null checks, error handling
        // Safety: Verified JNI environment
        unsafe {
            self.0.CallObjectMethod(...)
        }
    }
}
```

**3.3: Zero-Copy** (~15 unsafe blocks)

Pattern: Already using safe abstractions (excellent!)

Keep current pattern:
```rust
// Already safe!
pub fn zero_copy_slice<T>(data: &[T]) -> &[T] {
    data  // No unsafe needed
}
```

### **Phase 4: Mock Evolution** (4-6 hours)

**4.1: Production Mock Removal**

**Target**: `tunnel/hsm/stub_types.rs` (10 mocks in production)

```rust
// BEFORE: Mock implementation
pub struct StubHsm {
    // Mock data
}

// AFTER: Real implementation with capability discovery
pub struct RealHsm {
    capability_detector: Box<dyn HsmCapabilityDetector>,
    provider: Box<dyn HsmProvider>,
}

impl RealHsm {
    pub async fn new() -> Result<Self, BearDogError> {
        let detector = AutoHsmDetector::new();
        let capabilities = detector.detect().await?;
        let provider = select_best_provider(capabilities).await?;
        Ok(Self { capability_detector: Box::new(detector), provider })
    }
}
```

**4.2: Workflow Traits** (31 mocks)

Strategy: Default implementations instead of mocks

```rust
// BEFORE: Mock trait implementations
impl WorkflowTrait for MockWorkflow { ... }

// AFTER: Default trait with overridable methods
pub trait WorkflowTrait {
    fn execute(&self) -> Result<(), Error> {
        // Default implementation
        self.validate()?;
        self.run()?;
        Ok(())
    }
    
    // Override points
    fn validate(&self) -> Result<(), Error>;
    fn run(&self) -> Result<(), Error>;
}
```

### **Phase 5: Hardcoding Removal** (3-4 hours)

**5.1: Primal Discovery Evolution**

```rust
// BEFORE: Hardcoded primal names
if primal == "songbird" {
    connect_to_songbird()
}

// AFTER: Capability-based discovery
async fn discover_primal_with_capability(
    cap: &str
) -> Result<Vec<PrimalInfo>, BearDogError> {
    let discovery = CapabilityDiscovery::new();
    let primals = discovery
        .discover_by_capability(cap)
        .await?;
    Ok(primals)
}
```

**5.2: Configuration Evolution**

```rust
// BEFORE: Hardcoded defaults
const DEFAULT_PORT: u16 = 8080;

// AFTER: Environment-driven with fallback
pub fn get_port() -> u16 {
    std::env::var("BEARDOG_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or_else(|| {
            // Port 0 = OS assigns
            0
        })
}
```

### **Phase 6: Test Coverage Expansion** (8-10 hours)

**Strategy**: Targeted coverage for untested modules

**6.1: Identify Coverage Gaps**

Run `cargo llvm-cov` and find modules <90% coverage:
- E2E tests
- Error path tests  
- Chaos/fault injection tests
- Property-based tests

**6.2: Add Missing Tests**

Focus on:
- Unsafe code paths (100% coverage required)
- Error handling paths
- Edge cases
- Concurrent scenarios

**6.3: Chaos Testing**

Add chaos tests:
```rust
#[tokio::test]
async fn test_hsm_chaos_recovery() {
    let mut chaos = ChaosEngine::new();
    chaos.inject_random_failures(0.3); // 30% failure rate
    
    let hsm = HsmManager::new().await?;
    
    for _ in 0..100 {
        match hsm.generate_key().await {
            Ok(_) => { /* Should recover */ }
            Err(e) => {
                assert!(hsm.is_healthy().await);
                // System should remain healthy despite errors
            }
        }
    }
}
```

---

## 📋 **Detailed Action Items**

### **Week 1: Measurement & Quick Wins** (10-12 hours)

- [ ] Measure test coverage (llvm-cov)
- [ ] Audit all 141 unsafe blocks
- [ ] Categorize 928 mock usages
- [ ] Refactor btsp_provider.rs (HIGH priority)
- [ ] Remove 10 production mocks from stub_types.rs

### **Week 2: Large File Refactoring** (12-15 hours)

- [ ] Refactor hsm/manager/mod.rs
- [ ] Refactor api/trust.rs
- [ ] Verify all refactored files <1000 lines
- [ ] Ensure zero breaking changes
- [ ] 100% test pass rate maintained

### **Week 3: Unsafe Evolution** (15-18 hours)

- [ ] Create safe SIMD wrappers
- [ ] Audit FFI safety invariants
- [ ] Document all remaining unsafe with safety proofs
- [ ] Add property tests for unsafe code
- [ ] Achieve 100% coverage on unsafe code paths

### **Week 4: Mock & Hardcoding** (12-15 hours)

- [ ] Remove all production mocks
- [ ] Implement real capability discovery
- [ ] Remove hardcoded primal names
- [ ] Environment-driven configuration everywhere
- [ ] Self-knowledge only (runtime discovery)

### **Week 5: Test Coverage** (15-20 hours)

- [ ] Expand E2E tests
- [ ] Add chaos tests
- [ ] Property-based testing
- [ ] Achieve 90% total coverage
- [ ] 100% coverage on critical paths

### **Week 6: Verification & Documentation** (8-10 hours)

- [ ] Final audit
- [ ] Performance benchmarks
- [ ] Update documentation
- [ ] Migration guide
- [ ] Production readiness review

---

## 🎯 **Success Metrics**

### **Code Quality**

- ✅ All files <1000 lines
- ✅ Unsafe code <100 blocks (down from 141)
- ✅ All unsafe blocks documented with safety proofs
- ✅ Zero production mocks
- ✅ Zero hardcoded primal names

### **Test Coverage**

- ✅ 90% total coverage (llvm-cov)
- ✅ 100% coverage on unsafe code
- ✅ 100% coverage on error paths
- ✅ Chaos tests for critical paths

### **Architecture**

- ✅ Pure Rust dependencies (already achieved!)
- ✅ Capability-based discovery everywhere
- ✅ Self-knowledge only
- ✅ Runtime primal discovery

### **Performance**

- ✅ No regressions (benchmarks)
- ✅ Faster where possible (safe SIMD wrappers)
- ✅ Zero-copy maintained

---

## 📚 **Reference Documentation**

### **Patterns to Follow**

1. **Safe Unsafe Wrappers**
   - Bounds checking before unsafe
   - Document safety invariants
   - Property tests for verification

2. **Capability Discovery**
   - No hardcoded names
   - Runtime discovery
   - Fallback mechanisms

3. **Modular Refactoring**
   - Domain-driven splits
   - Clear module boundaries
   - Zero breaking changes

4. **Test Evolution**
   - Measure coverage first
   - Target gaps systematically
   - Chaos for resilience

---

## 🚀 **Getting Started**

**Immediate Next Steps** (Today):

1. **Measure Coverage**:
   ```bash
   cargo install cargo-llvm-cov
   cargo llvm-cov --workspace --html --open
   ```

2. **Start Large File Refactoring**:
   - Begin with `btsp_provider.rs` (highest priority)
   - Create modular structure
   - Migrate code systematically

3. **Audit Unsafe**:
   - List all unsafe blocks
   - Document safety invariants
   - Identify evolution candidates

---

**Status**: 🚀 **READY TO EXECUTE**  
**Estimated Timeline**: 6 weeks (part-time) or 2 weeks (full-time)  
**Risk**: LOW (systematic approach, comprehensive testing)  
**Impact**: HIGH (production-grade codebase, zero debt)

🔥 **Let's evolve to modern idiomatic Rust!**

