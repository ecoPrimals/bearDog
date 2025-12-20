# 🚀 BearDog Evolution Execution Plan
**Date**: December 20, 2025  
**Goal**: Deep debt solutions, modern idiomatic Rust, capability-based architecture

---

## ✅ Completed

1. **Formatting Fixed** - cargo fmt run successfully

---

## 🎯 Priority 1: Evolve Production Mocks to Complete Implementations

### Target: `crates/beardog-deploy/src/device.rs`

**Current State**: Mock check_device() returns hardcoded emulator data
**Evolution Target**: Real device discovery via adb/platform APIs
**Impact**: HIGH - enables real mobile deployment

**Files to Evolve**:
1. `crates/beardog-deploy/src/device.rs:97` - check_device() mock → real implementation

---

## 🎯 Priority 2: Migrate Unwrap() to Idiomatic Result (Systematic)

**Approach**: Use `unwrap-migrator` tool on production crates

### Phase 1: Core Infrastructure (High Impact)
1. `beardog-core/src/crypto_service/` - Crypto operations (13 unwraps)
2. `beardog-tunnel/src/tunnel/hsm/` - HSM management (15+ unwraps)
3. `beardog-security/src/` - Security primitives (22+ unwraps)

### Phase 2: Supporting Crates
4. `beardog-auth/` - Authentication flows
5. `beardog-config/` - Configuration loading
6. `beardog-adapters/` - Protocol adapters

**Tool**: `tools/unwrap-migrator/` exists for systematic migration

---

## 🎯 Priority 3: Evolve Hardcoding to Capability-Based Discovery

### Network Ports (337 references)
**Current**: Hardcoded port numbers in constants
**Target**: Runtime port allocation + discovery
**Files**:
- `crates/beardog-types/src/constants/domains/network.rs`
- `crates/beardog-config/src/domains/network_ports.rs`

### Primal References (757 references)
**Status**: ✅ MOSTLY GOOD - Used for capability detection
**Pattern**: Pixel8, StrongBox, YubiKey references are part of capability detection system
**Keep**: Discovery patterns are acceptable
**Evolve**: Any remaining hardcoded peer names

**Tool**: `scripts/hardcoding_eliminator.py` exists for detection

---

## 🎯 Priority 4: Smart Refactoring

### Identify Complex Modules
**Method**: Find files approaching complexity limits (not just size)
**Criteria**:
- Cyclomatic complexity
- Multiple responsibilities
- Tight coupling

**Action**: Analyze and refactor based on domain logic, not arbitrary splits

---

## 🎯 Priority 5: Expand Test Coverage (Strategic)

### Target: 70-76% → 90%

**Focus Areas**:
1. Error paths in crypto service
2. Edge cases in HSM operations
3. Network failure scenarios
4. Concurrent operation coverage
5. Integration points

---

## 🎯 Priority 6: Verify Primal Self-Knowledge Compliance

**Audit**:
1. ✅ `primal_self_knowledge.rs` - Excellent foundation
2. ✅ `primal_capability_adapter.rs` - Runtime discovery implemented
3. ✅ Scripts exist to detect violations
4. Check for any remaining hardcoded primal peer lists

---

## 📊 Metrics to Track

- [ ] Production mocks → 0
- [ ] Unwrap() in prod code: 2649 → <500
- [ ] Hardcoded ports: 337 → 0
- [ ] Test coverage: ~73% → 90%
- [ ] Unsafe code: 0 (maintain)
- [ ] Files >800 lines: Identify and smart-refactor

---

## 🔄 Execution Order

**Week 1 (This Session)**:
1. Evolve device.rs check_device() mock
2. Start unwrap migration (core crates)
3. Analyze hardcoding patterns

**Week 2**:
4. Continue unwrap migration
5. Port hardcoding → capability discovery
6. Expand test coverage

**Week 3**:
7. Smart refactoring of complex modules
8. Final verification
9. Coverage push to 90%

---

**Status**: EXECUTING
**Current Phase**: Priority 1 - Mock Evolution

