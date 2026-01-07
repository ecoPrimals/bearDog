# 📐 Large File Refactoring Plan - January 6, 2026

**Date**: January 6, 2026  
**Status**: 📋 **PLANNED** - Smart refactoring strategy  
**Goal**: Logical cohesion over arbitrary limits

---

## 📊 Current State

### Files >1000 Lines

| File | Lines | Status | Priority |
|------|-------|--------|----------|
| `tunnel/hsm/manager/mod.rs` | 1140 | 📋 Planned | Medium |
| `btsp_provider.rs` | 1051 | 📋 Planned | Medium |
| `api/trust.rs` | 1037 | 📋 Planned | Medium |
| `config/domains/discovery_unified.rs` | 992 | ✅ OK | Low |

**Total**: 4 files, 4220 lines

---

## 🎯 Philosophy: Smart Refactoring

### ❌ BAD: Arbitrary Splitting

```
// ❌ DON'T split just to meet a line limit
btsp_provider_part1.rs  // 500 lines
btsp_provider_part2.rs  // 500 lines
btsp_provider_part3.rs  // 51 lines
```

**Problems**:
- Arbitrary boundaries
- No logical cohesion
- Difficult to navigate
- Unclear ownership

---

### ✅ GOOD: Logical Module Extraction

```
// ✅ DO extract by logical boundaries
btsp_provider/
  mod.rs          // Public API + initialization
  birdsong.rs     // BirdSong encryption logic
  tunnel.rs       // Tunnel management
  session.rs      // Session lifecycle
  metrics.rs      // Metrics collection
```

**Benefits**:
- Clear responsibilities
- Easy to find code
- Natural test boundaries
- Maintainable long-term

---

## 📋 REFACTORING PLANS

### 1. `btsp_provider.rs` (1051 lines)

**Analysis**: Single large file implementing secure tunnel provider

**Proposed Structure**:
```
btsp_provider/
├── mod.rs              (~200 lines)  # Public API, initialization
├── birdsong.rs         (~250 lines)  # BirdSong encryption/decryption
├── tunnel.rs           (~250 lines)  # Tunnel establishment/management
├── session.rs          (~200 lines)  # Session lifecycle
└── metrics.rs          (~150 lines)  # Metrics collection
```

**Natural Boundaries**:
- **BirdSong operations**: Encryption, decryption, lineage
- **Tunnel management**: Establish, terminate, list
- **Session lifecycle**: Create, validate, expire
- **Metrics**: Collection, aggregation

**Priority**: **Medium** - File is well-organized but could benefit from logical separation

---

### 2. `tunnel/hsm/manager/mod.rs` (1140 lines)

**Analysis**: HSM manager with multiple concerns

**Proposed Structure**:
```
tunnel/hsm/manager/
├── mod.rs              (~200 lines)  # Public API
├── initialization.rs   (~200 lines)  # HSM initialization
├── operations.rs       (~250 lines)  # Key gen, sign, verify
├── capability.rs       (~200 lines)  # Capability detection
├── lifecycle.rs        (~200 lines)  # Health, rotation
└── metrics.rs          (~100 lines)  # Metrics
```

**Natural Boundaries**:
- **Initialization**: Auto-detect, configure, initialize HSM
- **Operations**: Cryptographic operations (keygen, sign, verify, encrypt, decrypt)
- **Capability**: Detect HSM capabilities and features
- **Lifecycle**: Health checks, key rotation, cleanup
- **Metrics**: Performance and usage metrics

**Priority**: **Medium** - Complex file that would benefit from separation

---

### 3. `api/trust.rs` (1037 lines)

**Analysis**: Trust API endpoints

**Proposed Structure**:
```
api/trust/
├── mod.rs              (~150 lines)  # Public API, routing
├── evaluation.rs       (~300 lines)  # Trust evaluation logic
├── lineage.rs          (~250 lines)  # Genetic lineage handling
├── verification.rs     (~250 lines)  # Verification operations
└── types.rs            (~100 lines)  # Request/response types
```

**Natural Boundaries**:
- **Evaluation**: Trust level calculation, policy enforcement
- **Lineage**: Genetic lineage queries and validation
- **Verification**: Signature and attestation verification
- **Types**: Shared types for requests/responses

**Priority**: **Medium** - API-focused file that could be clearer

---

### 4. `config/domains/discovery_unified.rs` (992 lines)

**Status**: ✅ **NO REFACTORING NEEDED** (under 1000 lines)

**Priority**: **Low** - File is just under the threshold and well-organized

---

## 🎯 REFACTORING STRATEGY

### Phase 1: Analysis (DONE)

- ✅ Identify large files
- ✅ Analyze logical boundaries
- ✅ Plan module structure

---

### Phase 2: Refactoring (PLANNED)

For each file:

1. **Create module directory**
   ```bash
   mkdir -p crates/beardog-tunnel/src/btsp_provider
   ```

2. **Extract logical modules**
   - Move code by logical boundaries
   - Preserve all tests
   - Update imports

3. **Create `mod.rs` with public API**
   - Re-export public types
   - Maintain backward compatibility
   - Add module documentation

4. **Update imports throughout codebase**
   ```rust
   // Old
   use crate::btsp_provider::SomeType;
   
   // New (same - re-exported)
   use crate::btsp_provider::SomeType;
   ```

5. **Run tests**
   ```bash
   cargo test --workspace
   ```

6. **Verify no regressions**

---

### Phase 3: Validation

- [ ] All tests pass
- [ ] No clippy warnings introduced
- [ ] Documentation builds
- [ ] Public API unchanged
- [ ] Import paths work

---

## 📊 ESTIMATED EFFORT

| File | Complexity | Estimated Time | Risk |
|------|------------|----------------|------|
| `btsp_provider.rs` | Medium | 2-3 hours | Low |
| `hsm/manager/mod.rs` | High | 3-4 hours | Medium |
| `api/trust.rs` | Medium | 2-3 hours | Low |

**Total**: 7-10 hours for complete refactoring

---

## ⚠️ RISKS & MITIGATION

### Risk 1: Breaking Public API

**Mitigation**: Use `pub use` re-exports in `mod.rs` to maintain all public paths

```rust
// mod.rs
pub use birdsong::*;
pub use tunnel::*;
// ... all public types re-exported
```

---

### Risk 2: Import Path Changes

**Mitigation**: Keep module name same, use re-exports

```rust
// Before: crates/.../btsp_provider.rs
// After: crates/.../btsp_provider/mod.rs
// Result: Same import paths work!
```

---

### Risk 3: Test Breakage

**Mitigation**: 
- Extract tests with their code
- Run tests after each module extraction
- Use `#[path]` attribute if needed

---

## ✅ SUCCESS CRITERIA

- [ ] All 4 large files under 500 lines (top-level `mod.rs`)
- [ ] Logical module boundaries
- [ ] All tests pass
- [ ] No public API changes
- [ ] Improved maintainability
- [ ] Clear module documentation

---

## 📚 REFERENCE: Rust Module Best Practices

### ✅ Good Module Structure

```rust
// mod.rs - Public API
pub mod birdsong;  // Public submodule
pub mod tunnel;    // Public submodule
mod session;       // Private submodule

// Re-export for convenience
pub use birdsong::BirdSongManager;
pub use tunnel::TunnelHandle;
```

### ✅ Good File Organization

```
provider/
├── mod.rs          # Public API, re-exports
├── birdsong.rs     # One logical concern
├── tunnel.rs       # Another logical concern
└── internal/       # Private implementation details
    ├── mod.rs
    └── session.rs
```

---

## 🎯 RECOMMENDATION

### Immediate

**Hold refactoring** until higher-priority tasks complete:
- E2E tests
- Coverage analysis
- Production deployment

### Rationale

1. Files are large but **well-organized**
2. No **urgent maintainability issues**
3. Refactoring is **low ROI** compared to:
   - Completing E2E test suite
   - Achieving 90% coverage
   - Deploying to production

### Future (Next Sprint)

When bandwidth allows:
1. Start with `btsp_provider.rs` (cleanest boundaries)
2. Then `api/trust.rs` (API-focused)
3. Finally `hsm/manager/mod.rs` (most complex)

---

## 📊 PRIORITY ASSESSMENT

| Task | Impact | Effort | Priority |
|------|--------|--------|----------|
| E2E Tests | High | Medium | 🔴 **HIGH** |
| Coverage (90%) | High | High | 🔴 **HIGH** |
| Deploy to Prod | High | Low | 🔴 **HIGH** |
| Refactor Files | Low | High | 🟡 **MEDIUM** |

---

## 🎊 CONCLUSION

**Current Status**: 4 files >1000 lines, all well-organized  
**Immediate Action**: ⏸️ **DEFERRED** (low priority)  
**Future Action**: ✅ **PLANNED** (smart refactoring strategy ready)

**Rationale**: Files are maintainable as-is. Focus on higher-impact work (testing, deployment) before refactoring.

---

_Plan Date: January 6, 2026_  
_Status: Ready for execution when prioritized_  
_Estimated Effort: 7-10 hours_

