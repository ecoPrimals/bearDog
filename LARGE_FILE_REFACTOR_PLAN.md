# Large File Refactoring Plan - January 7, 2026

**Goal**: Refactor 4 files over 1000 lines using SMART, semantic splitting  
**Principle**: Extract by responsibility, not arbitrary line count  
**Status**: Planning phase

---

## Files to Refactor

### 1. btsp_provider.rs (1295 lines) 🔴 HIGH PRIORITY
**Current Size**: 1295 lines  
**Target**: < 1000 lines main file + modules  
**Complexity**: HIGH (core security protocol)

**Responsibilities Identified**:
1. **Metrics** (lines 54-74) → `btsp_provider/metrics.rs`
2. **Type Conversions** (lines 75-200) → `btsp_provider/types.rs`
3. **Contact Exchange** (lines 460-750) → `btsp_provider/contact.rs`
4. **Trust Management** (lines 900-1100) → `btsp_provider/trust.rs`
5. **Core Provider** (remaining) → `btsp_provider/core.rs`

**Extraction Strategy**:
```
crates/beardog-tunnel/src/
├── btsp_provider/
│   ├── mod.rs          (re-exports, ~100 lines)
│   ├── core.rs         (BeardogBtspProvider impl, ~400 lines)
│   ├── metrics.rs      (BtspMetrics, ~50 lines)
│   ├── types.rs        (Internal types, ~150 lines)
│   ├── contact.rs      (Contact exchange, ~300 lines)
│   └── trust.rs        (Trust management, ~250 lines)
└── btsp_provider.rs → REMOVE (replaced by module)
```

---

### 2. hsm/manager/mod.rs (1140 lines) 🟠 MEDIUM PRIORITY
**Current Size**: 1140 lines  
**Target**: < 1000 lines main file + modules  
**Complexity**: VERY HIGH (multiple HSM providers)

**Responsibilities Identified**:
1. **Manager Core** (initialization, lifecycle)
2. **Provider Discovery** (hardware, software, cloud)
3. **Key Operations** (generate, derive, rotate)
4. **Performance** (metrics, caching)
5. **Configuration** (already separate: `config.rs`, `performance.rs`)

**Extraction Strategy**:
```
crates/beardog-tunnel/src/tunnel/hsm/manager/
├── mod.rs          (HsmManager struct, ~300 lines)
├── discovery.rs    (Provider discovery, ~250 lines)
├── key_ops.rs      (Key operations, ~300 lines)
├── lifecycle.rs    (Init, cleanup, ~150 lines)
├── config.rs       (existing)
└── performance.rs  (existing)
```

---

### 3. unix_socket_ipc.rs (1099 lines) 🟠 MEDIUM PRIORITY
**Current Size**: 1099 lines  
**Target**: < 1000 lines main file + modules  
**Complexity**: MEDIUM (protocol handlers)

**Responsibilities Identified**:
1. **Server Core** (socket management, ~200 lines)
2. **Protocol Detection** (lines 240-298, ~60 lines)
3. **tarpc Handler** (lines 300-318, ~20 lines)
4. **JSON-RPC Handler** (lines 320-600, ~280 lines)
5. **HTTP Handler** (lines 602-900, ~300 lines)
6. **Schemas & Types** (lines 45-80, ~40 lines)

**Extraction Strategy**:
```
crates/beardog-tunnel/src/unix_socket_ipc/
├── mod.rs              (Server core, ~250 lines)
├── types.rs            (JsonRpc types, ~100 lines)
├── protocol.rs         (Protocol detection, ~100 lines)
├── handlers/
│   ├── mod.rs          (Handler traits, ~50 lines)
│   ├── tarpc.rs        (tarpc handler, ~100 lines)
│   ├── jsonrpc.rs      (JSON-RPC handler, ~350 lines)
│   └── http.rs         (HTTP handler, ~350 lines)
```

---

### 4. api/trust.rs (1037 lines) 🟡 LOW PRIORITY
**Current Size**: 1037 lines  
**Target**: < 1000 lines (close to limit)  
**Complexity**: MEDIUM (trust evaluation logic)

**Responsibilities Identified**:
1. **Trust Evaluation** (~300 lines)
2. **Peer Management** (~250 lines)
3. **Relationship Tracking** (~250 lines)
4. **API Handlers** (~237 lines)

**Extraction Strategy**:
```
crates/beardog-tunnel/src/api/trust/
├── mod.rs              (API handlers, ~250 lines)
├── evaluation.rs       (Trust evaluation, ~300 lines)
├── peers.rs            (Peer management, ~250 lines)
└── relationships.rs    (Relationship tracking, ~250 lines)
```

---

## Refactoring Principles

### 1. Semantic Boundaries
✅ Extract by **responsibility**, not line count  
✅ Maintain **logical cohesion**  
✅ Minimize **cross-module dependencies**  
❌ DON'T split arbitrarily for line count

### 2. Backward Compatibility
✅ Re-export all public types from `mod.rs`  
✅ Maintain existing API surface  
✅ No breaking changes for consumers  
❌ DON'T change public interfaces

### 3. Test Preservation
✅ Keep existing tests working  
✅ Add module-specific tests  
✅ Maintain test coverage  
❌ DON'T skip test validation

### 4. Documentation
✅ Document module structure in `mod.rs`  
✅ Add module-level docs  
✅ Update architecture docs  
❌ DON'T leave undocumented modules

---

## Implementation Strategy

### Phase 1: btsp_provider.rs (HIGHEST IMPACT)
**Effort**: 2-3 hours  
**Risk**: MEDIUM (core security)  
**Priority**: HIGH

**Steps**:
1. Create `btsp_provider/` directory
2. Extract `metrics.rs` (simple, low risk)
3. Extract `types.rs` (internal types)
4. Extract `contact.rs` (complex, test thoroughly)
5. Extract `trust.rs` (complex, test thoroughly)
6. Create `mod.rs` with re-exports
7. Run full test suite
8. Verify compilation

### Phase 2: unix_socket_ipc.rs (CLEAREST BOUNDARIES)
**Effort**: 2-3 hours  
**Risk**: LOW (clear protocol boundaries)  
**Priority**: MEDIUM

**Steps**:
1. Create `unix_socket_ipc/` directory
2. Extract `types.rs` (simple)
3. Extract `protocol.rs` (detection logic)
4. Extract `handlers/jsonrpc.rs` (self-contained)
5. Extract `handlers/http.rs` (self-contained)
6. Extract `handlers/tarpc.rs` (small)
7. Create `mod.rs` with server core
8. Run IPC tests

### Phase 3: hsm/manager/mod.rs (MOST COMPLEX)
**Effort**: 3-4 hours  
**Risk**: HIGH (multiple HSM providers)  
**Priority**: MEDIUM

**Steps**:
1. Extract `discovery.rs` (provider discovery)
2. Extract `key_ops.rs` (key operations)
3. Extract `lifecycle.rs` (initialization)
4. Keep manager core in `mod.rs`
5. Run HSM tests extensively
6. Validate all HSM providers work

### Phase 4: api/trust.rs (LOWEST PRIORITY)
**Effort**: 1-2 hours  
**Risk**: LOW (barely over limit)  
**Priority**: LOW

**Steps**:
1. Only refactor if time permits
2. Could stay as-is (1037 lines acceptable)
3. If refactored: extract evaluation logic
4. Keep API handlers in main file

---

## Testing Strategy

### For Each Refactoring

1. **Before Refactoring**:
   ```bash
   cargo test --package beardog-tunnel
   cargo build --package beardog-tunnel
   ```

2. **During Refactoring**:
   - Incremental compilation after each extraction
   - Fix any compilation errors immediately
   - Run affected tests

3. **After Refactoring**:
   ```bash
   cargo test --workspace
   cargo build --workspace
   cargo clippy --workspace
   cargo doc --workspace
   ```

4. **Integration Testing**:
   - Run E2E tests
   - Verify Unix socket IPC
   - Test BTSP functionality
   - Validate HSM operations

---

## Risk Mitigation

### High-Risk Refactorings
- `btsp_provider.rs` - Core security protocol
- `hsm/manager/mod.rs` - Multiple HSM providers

**Mitigation**:
1. Create feature branch for each refactoring
2. Extensive testing before merge
3. Review by another developer (if available)
4. Keep rollback plan (git revert ready)

### Low-Risk Refactorings
- `unix_socket_ipc.rs` - Clear protocol boundaries
- `api/trust.rs` - Well-defined API

**Approach**:
1. Can proceed with confidence
2. Standard testing sufficient
3. Merge after basic validation

---

## Success Criteria

### Per-File Success
- ✅ All files < 1000 lines
- ✅ All tests passing
- ✅ No compilation warnings
- ✅ Documentation complete
- ✅ Public API unchanged

### Overall Success
- ✅ All 4 files refactored (or 3 if trust.rs skipped)
- ✅ Test coverage maintained
- ✅ Compilation time same or better
- ✅ Code more maintainable
- ✅ Architecture clearer

---

## Timeline

### Optimistic (8-10 hours)
- btsp_provider.rs: 2 hours
- unix_socket_ipc.rs: 2 hours
- hsm/manager/mod.rs: 3 hours
- api/trust.rs: 1 hour
- Testing & validation: 2 hours

### Realistic (12-15 hours)
- btsp_provider.rs: 3 hours
- unix_socket_ipc.rs: 3 hours
- hsm/manager/mod.rs: 4 hours
- api/trust.rs: 2 hours (optional)
- Testing & validation: 3 hours

### Conservative (15-20 hours)
- Include time for unexpected issues
- Additional testing cycles
- Documentation updates
- Review and refinement

---

## Decision: Start with btsp_provider.rs

**Rationale**:
1. Highest impact (largest file)
2. Clear semantic boundaries
3. Core to beardog functionality
4. Well-tested (good safety net)

**Next Session Plan**:
1. Extract metrics.rs (15 min)
2. Extract types.rs (30 min)
3. Extract contact.rs (60 min)
4. Extract trust.rs (60 min)
5. Create mod.rs (15 min)
6. Test & validate (30 min)

**Total**: ~3 hours for btsp_provider.rs

---

**Created**: January 7, 2026  
**Status**: Ready to execute  
**First Target**: btsp_provider.rs (1295 lines → ~400 lines)

🐻 **Smart refactoring. Semantic boundaries. Zero breaking changes.** 🛡️

