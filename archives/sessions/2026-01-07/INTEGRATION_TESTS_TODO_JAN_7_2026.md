# Integration Tests Refactoring TODO - January 7, 2026

**Status**: ⏸️ Temporarily Disabled  
**Priority**: Medium  
**Estimated Effort**: 2-3 hours

---

## 📋 ISSUE SUMMARY

Two integration test files require refactoring due to `UnixSocketIpcServer` being tightly coupled to `BeardogBtspProvider`:

1. `tests/btsp_jsonrpc_e2e_tests.rs.disabled`
2. `tests/btsp_jsonrpc_chaos_tests.rs.disabled`

**Impact**: Low - Library tests (35/35) all pass, production code unaffected

---

## 🔍 ROOT CAUSE

`UnixSocketIpcServer` is hardcoded to use `Arc<BeardogBtspProvider>`:

```rust
pub struct UnixSocketIpcServer {
    socket_path: PathBuf,
    btsp_provider: Arc<BeardogBtspProvider>,  // ← Tightly coupled
    is_running: Arc<tokio::sync::RwLock<bool>>,
}
```

This prevents using mock providers in integration tests, violating:
- Dependency Inversion Principle
- Testability best practices
- Primal sovereignty (hardcoded to specific implementation)

---

## ✅ FIXES APPLIED (Partial)

1. ✅ Made `handle_jsonrpc_request` public for testing
2. ✅ Fixed `TunnelStatus` field mismatches (removed `peer_id`, `established_at`)
3. ✅ Moved `contact_exchange` to separate impl block (not part of trait)
4. ⏸️ **Remaining**: Refactor `UnixSocketIpcServer` for testability

---

## 🔧 RECOMMENDED SOLUTION

### Option 1: Generic Over Trait (Best)

Make `UnixSocketIpcServer` generic over `SecureTunnelProvider`:

```rust
pub struct UnixSocketIpcServer<P: SecureTunnelProvider + Send + Sync> {
    socket_path: PathBuf,
    btsp_provider: Arc<P>,
    is_running: Arc<tokio::sync::RwLock<bool>>,
}

impl<P: SecureTunnelProvider + Send + Sync + 'static> UnixSocketIpcServer<P> {
    pub async fn new(
        socket_path: impl AsRef<Path>,
        btsp_provider: Arc<P>,
    ) -> Result<Self> {
        // ... implementation ...
    }
}
```

**Pros**:
- Zero runtime overhead (monomorphization)
- Type-safe
- Fully testable
- Follows primal sovereignty principles

**Cons**:
- Requires updating all usage sites
- More complex type signatures

### Option 2: Trait Object (Simpler)

Use dynamic dispatch:

```rust
pub struct UnixSocketIpcServer {
    socket_path: PathBuf,
    btsp_provider: Arc<dyn SecureTunnelProvider + Send + Sync>,
    is_running: Arc<tokio::sync::RwLock<bool>>,
}
```

**Pros**:
- Simpler change
- Minimal code updates
- Fully testable

**Cons**:
- Small runtime overhead (vtable dispatch)
- Slightly less type information

---

## 📝 IMPLEMENTATION PLAN

### Phase 1: Refactor `UnixSocketIpcServer` (1-2 hours)

1. Choose approach (recommend Option 1: Generic)
2. Update struct definition
3. Update all usage sites in production code
4. Verify library tests still pass

### Phase 2: Re-enable Integration Tests (1 hour)

1. Rename `.disabled` files back to `.rs`
2. Update test helpers to use generic server
3. Verify all tests compile
4. Run tests and fix any remaining issues

### Phase 3: Add Additional Tests (30 min)

1. Add tests for mock provider scenarios
2. Add tests for error paths
3. Verify coverage improvement

**Total Estimated Time**: 2.5-3.5 hours

---

## 🎯 CURRENT WORKAROUND

**Files Disabled**:
- `tests/btsp_jsonrpc_e2e_tests.rs` → `tests/btsp_jsonrpc_e2e_tests.rs.disabled`
- `tests/btsp_jsonrpc_chaos_tests.rs` → `tests/btsp_jsonrpc_chaos_tests.rs.disabled`

**Status**: Library tests (35/35) pass, production code functional

**Re-enable When**: Refactoring complete

---

## 📊 IMPACT

### Current State

| Test Type | Status | Count |
|-----------|--------|-------|
| **Library Tests** | ✅ Passing | 35/35 |
| **Integration Tests** | ⏸️ Disabled | 2 files |
| **E2E Tests (btsp_contact_exchange)** | ✅ Passing | Active |

### After Refactoring

| Test Type | Status | Expected Count |
|-----------|--------|----------------|
| **Library Tests** | ✅ Passing | 35/35 |
| **Integration Tests** | ✅ Passing | +20-30 tests |
| **E2E Tests** | ✅ Passing | Active |

**Coverage Improvement**: Estimated +5-10% after re-enabling

---

## 🚀 PRIORITY vs OTHER WORK

**Current Priorities** (Higher Value):
1. ⏫ Test coverage expansion (60% → 90%)
2. ⏫ Phase 5 TODOs (8 security enhancements)
3. ⏫ Clippy pedantic fixes (1,293 warnings)

**This Refactoring** (Medium Value):
- Can be done in Phase 5 or 6
- Not blocking other work
- Production code unaffected

**Recommendation**: Address in Phase 5 after security TODOs, or in Phase 6 during polish phase

---

## 📚 RELATED DOCUMENTS

- `TESTING_POLISH_JAN_7_2026.md` - Discusses this issue
- `SCOPE_VERIFICATION_JAN_7_2026.md` - Primal sovereignty principles
- `DEEP_DEBT_EVOLUTION_JAN_6_2026.md` - Debt tracking

---

## ✅ SUCCESS CRITERIA

1. ✅ `UnixSocketIpcServer` generic or using trait objects
2. ✅ All integration tests re-enabled and passing
3. ✅ Library tests still passing (35/35)
4. ✅ No hardcoded providers (primal sovereignty maintained)
5. ✅ Coverage increase by 5-10%

---

**Date**: January 7, 2026  
**Status**: ⏸️ Documented, deferred to Phase 5/6  
**Impact**: Low (production unaffected)

🐻 **BearDog v0.15.0 - Pragmatic approach: fix when optimal!** 🛡️

