# Comprehensive Evolution Execution Plan - January 24, 2026

**Mission**: Deep debt solutions, modern idiomatic Rust, Pure Rust dependencies, smart refactoring, safe code, agnostic capability-based architecture, primal self-knowledge, isolated mocks

---

## Current State Analysis ✅

### Excellent Foundation
1. ✅ **100% Safe Rust**: `#![deny(unsafe_code)]` in lib.rs - NO unsafe blocks!
2. ✅ **Pure Rust Crypto**: All RustCrypto, zero C dependencies
3. ✅ **Capability-Based Sockets**: Environment-driven, 4-tier fallback
4. ✅ **Semantic Refactoring**: crypto_handlers.rs → 7 domain modules (just completed!)
5. ✅ **Diagnostics Module**: Conditional compilation, zero-cost abstraction

### Areas for Evolution
1. ⚠️ **Mock in Production**: `safe_android_provider.rs` - runtime cfg! checks (should be compile-time)
2. ⚠️ **Large Files**: `btsp_provider.rs` (1,209 lines), HSM manager (1,140 lines)
3. ⚠️ **Some Hardcoding**: Vendor names in discovery patterns
4. ⚠️ **Test Sprawl**: 297 mock references (need audit for test vs production)

---

## Prioritized Execution Plan

### 🔥 PRIORITY 1: Complete Current Hardening (Steps 2-3)
**Rationale**: Already in progress, low effort, high impact

#### Step 2A: Evolve AES-256-GCM Handler (15 min)
- No diagnostic logging found (already clean!)
- ✅ Already using `Zeroizing` wrapper
- ✅ Already has input validation

#### Step 2B: Evolve ChaCha20-Poly1305 Handler (15 min)
- Check for any diagnostic logging
- Verify `Zeroizing` usage
- Add to diagnostics module if needed

#### Step 3: Security Audit (30 min)
- Verify RustCrypto uses `subtle` crate (constant-time)
- Audit all `Zeroizing` usage
- Check error paths for timing leaks
- Document findings

**Total Time**: 1 hour  
**Impact**: Production hardening complete ✅

---

### 🎯 PRIORITY 2: Isolate Mocks to Testing (High Impact)
**Rationale**: Production code should never have mocks

#### Phase 4.1: Audit Mock Usage (1 hour)
- Grep all 297 mock references
- Classify: test vs production
- Identify runtime `cfg!` checks (should be compile-time)

#### Phase 4.2: Evolve Android Mock (2 hours)
**File**: `safe_android_provider.rs`  
**Problem**: Runtime `cfg!(target_os = "android")` with mock signatures
**Solution**: Conditional compilation + feature flags

```rust
// BEFORE (runtime check, mock in production)
if !cfg!(target_os = "android") {
    return Ok(vec![0u8; signature_size]); // MOCK!
}

// AFTER (compile-time, no mock in production)
#[cfg(target_os = "android")]
fn sign_data_impl(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    // Real Android implementation
}

#[cfg(not(target_os = "android"))]
fn sign_data_impl(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    Err(BearDogError::unsupported("Android StrongBox not available on this platform"))
}
```

#### Phase 4.3: Document Mock Policy (30 min)
- Create `MOCK_POLICY.md`
- Define clear boundaries (test vs production)
- Add CI checks to prevent production mocks

**Total Time**: 3.5 hours  
**Impact**: Zero mocks in production ✅

---

### 🏗️ PRIORITY 3: Smart Refactoring (Large Files)
**Rationale**: Maintainability, cognitive load reduction

#### Phase 1.2: Refactor btsp_provider.rs (4 hours)
**Current**: 1,209 lines monolithic  
**Strategy**: Smart semantic refactoring by layer

```
btsp_provider/
├── mod.rs (100 lines) - Public API, re-exports
├── core.rs (300 lines) - Core BTSP logic
├── crypto_operations.rs (250 lines) - Crypto ops
├── trust.rs (200 lines) - Trust evaluation
├── tunnel_lifecycle.rs (250 lines) - Lifecycle mgmt
└── types.rs (109 lines) - Already done!
```

**Benefits**:
- Clear separation of concerns
- Easier testing
- Better parallel development
- Reduced cognitive load

#### Phase 1.3: Refactor HSM Manager (4 hours)
**Current**: 1,140 lines capability-based  
**Strategy**: Refactor by capability domain

```
hsm/manager/
├── mod.rs (80 lines) - Public API
├── capability.rs (200 lines) - Capability detection
├── provider_selection.rs (250 lines) - Provider routing
├── key_operations.rs (300 lines) - Key mgmt
├── entropy.rs (200 lines) - Entropy collection
└── attestation.rs (110 lines) - Attestation
```

**Total Time**: 8 hours  
**Impact**: Better maintainability, clearer architecture ✅

---

### 🔧 PRIORITY 4: Eliminate Hardcoding
**Rationale**: Agnostic, capability-based discovery

#### Phase 3.1: Audit Hardcoded Values (1 hour)
- Vendor names (Consul, etcd, etc.)
- Primal names in code
- Fixed paths
- Magic numbers

#### Phase 3.2: Evolve to Capability Discovery (3 hours)
- Replace vendor checks with capability queries
- Use environment variables
- Runtime discovery patterns
- Document migration

**Total Time**: 4 hours  
**Impact**: Fully agnostic infrastructure ✅

---

### 🧠 PRIORITY 5: Primal Self-Knowledge
**Rationale**: Primals should only know themselves

#### Phase 5.1: Define Self-Knowledge Boundaries (2 hours)
- Audit cross-primal references
- Identify violations
- Create migration plan

#### Phase 5.2: Implement Discovery Patterns (4 hours)
- Replace hardcoded primal names
- Use capability-based discovery
- Neural API integration

**Total Time**: 6 hours  
**Impact**: True primal sovereignty ✅

---

### 📚 PRIORITY 6: Documentation & Testing
**Rationale**: Ensure quality and maintainability

#### Phase 6.1: Comprehensive Testing (8 hours)
- Unit tests for all new modules
- E2E tests for refactored code
- Chaos tests for error paths
- Fault injection tests

#### Phase 6.2: Documentation Evolution (4 hours)
- Update all module docs
- Create architecture guides
- Document evolution history

**Total Time**: 12 hours  
**Impact**: Production-ready confidence ✅

---

## Execution Order (Optimized for Impact)

### Session 1: Hardening Complete (1 hour) ✅
1. ✅ Step 2: AES-256/ChaCha20 handlers
2. ✅ Step 3: Security audit
3. ✅ Document findings

### Session 2: Mock Isolation (3.5 hours)
1. Audit all 297 mock references
2. Evolve Android mock to compile-time
3. Document mock policy

### Session 3: Smart Refactoring Part 1 (4 hours)
1. Refactor btsp_provider.rs by layer
2. Test and validate
3. Document structure

### Session 4: Smart Refactoring Part 2 (4 hours)
1. Refactor HSM manager by capability
2. Test and validate
3. Document structure

### Session 5: Hardcoding Elimination (4 hours)
1. Audit hardcoded values
2. Evolve to capability discovery
3. Test and validate

### Session 6: Self-Knowledge Boundaries (6 hours)
1. Define boundaries
2. Implement discovery patterns
3. Test and validate

### Session 7: Testing & Documentation (12 hours)
1. Comprehensive test coverage
2. Documentation updates
3. Final validation

**Total Estimated Time**: ~35 hours  
**Completion**: All evolution goals achieved ✅

---

## Success Criteria

1. ✅ **Zero Mocks in Production**: All mocks isolated to `#[cfg(test)]`
2. ✅ **Smart Refactoring**: All files < 500 lines, semantic organization
3. ✅ **100% Safe Rust**: No unsafe blocks (already achieved!)
4. ✅ **Pure Rust**: Zero C dependencies (already achieved!)
5. ✅ **Agnostic**: No hardcoded vendor/primal names
6. ✅ **Self-Knowledge**: Primals discover others at runtime
7. ✅ **Comprehensive Tests**: Unit, E2E, chaos, fault
8. ✅ **Modern Idioms**: Latest Rust patterns, zero technical debt

---

## Let's Execute! 🚀

Starting with Session 1: Hardening Complete (in progress)...

