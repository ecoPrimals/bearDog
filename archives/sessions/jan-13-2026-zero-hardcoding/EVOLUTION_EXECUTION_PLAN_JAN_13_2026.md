# 🚀 BearDog Evolution Execution Plan

**Date**: January 13, 2026  
**Status**: ✅ EXECUTING - Systematic Deep Debt Evolution  
**Philosophy**: Evolve to modern idiomatic Rust with capability-based design

---

## 🎯 EVOLUTION PHILOSOPHY

### Core Principles

1. **Deep Debt Solutions** - Not patches, but architectural improvements
2. **Modern Idiomatic Rust** - Evolve to current best practices
3. **Pure Rust Dependencies** - Analyze and evolve external deps
4. **Smart Refactoring** - Domain-driven, not just splitting
5. **Fast AND Safe** - Evolve unsafe to safe without losing performance
6. **Capability-Based Discovery** - Primals have self-knowledge only
7. **Test Isolation** - Mocks in tests, complete implementations in production

---

## 📋 EXECUTION CHECKLIST

### Phase 1: Foundation (30 minutes) ✅

- [x] Fix reqwest dev-dependency (build passing)
- [ ] Fix 3 clippy must_use errors
- [ ] Verify all tests pass
- [ ] Baseline measurement

### Phase 2: Large File Smart Refactoring (6-8 hours)

**Not just splitting - domain-driven evolution**

#### 1. btsp_provider.rs (1191 lines → Domain Modules)

**Current**: Monolithic BTSP provider  
**Evolution Target**: Capability-based trust domain

**New Structure**:
```
crates/beardog-tunnel/src/btsp_provider/
  ├── mod.rs (200 lines) - Public API, trait definitions
  ├── trust_engine.rs (300 lines) - Trust establishment logic
  ├── contact_manager.rs (250 lines) - Contact exchange
  ├── crypto_operations.rs (250 lines) - Encryption/signing
  ├── capability_discovery.rs (200 lines) - Dynamic capability detection
  └── tests/ - Domain-specific tests
```

**Key Evolution**:
- Separate trust from transport
- Capability discovery (no hardcoding)
- Clean domain boundaries
- Testable in isolation

#### 2. tunnel/hsm/manager/mod.rs (1140 lines → Capability Router)

**Current**: Monolithic HSM manager  
**Evolution Target**: Capability-based HSM routing

**New Structure**:
```
crates/beardog-tunnel/src/tunnel/hsm/manager/
  ├── mod.rs (150 lines) - Public API
  ├── capability_router.rs (300 lines) - Dynamic HSM selection
  ├── lifecycle.rs (250 lines) - HSM lifecycle management
  ├── operations/ - Operation handlers
  │   ├── sign.rs (150 lines)
  │   ├── encrypt.rs (150 lines)
  │   ├── key_derivation.rs (150 lines)
  └── discovery/ - Runtime discovery
      ├── hardware.rs (200 lines) - Hardware HSM detection
      ├── software.rs (150 lines) - Software HSM fallback
```

**Key Evolution**:
- Hot-plug capability detection
- No HSM preference hardcoding
- Automatic best-available selection
- Performance-based routing

#### 3. api/trust.rs (1037 lines → Validation Engine)

**Current**: Mixed trust API  
**Evolution Target**: Trust validation pipeline

**New Structure**:
```
crates/beardog-tunnel/src/api/trust/
  ├── mod.rs (150 lines) - Public API
  ├── validation_engine.rs (300 lines) - Trust validation pipeline
  ├── handlers.rs (250 lines) - HTTP handlers
  ├── types.rs (200 lines) - Trust-specific types
  └── policies/ - Trust policies
      ├── genetic_lineage.rs (150 lines)
      ├── attestation.rs (150 lines)
```

**Key Evolution**:
- Composable validation pipeline
- Policy-based trust decisions
- No hardcoded trust rules
- Runtime policy configuration

### Phase 3: Hardcoding Evolution (10-15 hours)

**Target**: 211 hardcoded values → Capability discovery

#### Network Discovery Evolution

**Current Problems**:
```rust
// ❌ BAD: Hardcoded
let addr = "127.0.0.1:8080";
let endpoint = "http://localhost:3000";
```

**Evolution**:
```rust
// ✅ GOOD: Capability discovery
let service = capabilities.discover_service("beardog-api")
    .await?;
let endpoint = service.preferred_endpoint()?;
```

**Implementation Plan**:

1. **Service Registry** (3 hours)
   - mDNS discovery
   - Environment variable fallback
   - Config file discovery
   - No hardcoded defaults

2. **Dynamic Port Allocation** (2 hours)
   - OS assigns ports
   - Registration in service discovery
   - No port conflicts

3. **Capability Metadata** (3 hours)
   - Primals advertise capabilities
   - Consumers discover at runtime
   - Version negotiation

4. **Migration** (5-7 hours)
   - Systematic replacement
   - 80 network values
   - 40 path values
   - 45 timeout values

### Phase 4: Test Coverage Expansion (20-27 hours)

**Target**: 31% → 90% coverage

#### Auth Subsystem (0% → 80%) - 8-10 hours

**Modules to Test**:
```
crates/beardog-auth/src/
  ├── auth/handlers.rs (0% → 85%)
  ├── auth/types/authorization.rs (0% → 90%)
  ├── auth/types/workflow.rs (0% → 85%)
  ├── auth/types/genetics.rs (0% → 90%)
  └── auth/node_registry.rs (0% → 80%)
```

**Test Categories**:
- Unit tests: Happy path + edge cases
- Integration: Auth workflows end-to-end
- Security: Attack vectors, failures
- Performance: Load testing

#### Integration Tests (6-8 hours)

**Cross-Module Scenarios**:
- Genetic lineage → Auth → Tunneling
- Discovery → Capability → Trust
- HSM detection → Key ops → Audit

#### Edge Cases & Error Paths (4-6 hours)

**Focus Areas**:
- Network failures
- Concurrent access
- Resource exhaustion
- Invalid inputs
- Degraded mode

### Phase 5: Unsafe Code Evolution (8-10 hours)

**Target**: Fast AND safe (not just safe)

#### Analysis Categories

**141 unsafe blocks breakdown**:

1. **SIMD Optimizations** (65 blocks)
   - **Keep if**: Performance critical + tested
   - **Evolve if**: Safe alternative exists with <10% slowdown
   - **Tools**: portable-simd, safe_arch

2. **FFI Boundaries** (35 blocks)
   - **Keep if**: Hardware interface required
   - **Evolve if**: Pure Rust alternative exists
   - **Pattern**: Safe wrapper → unsafe core → panic boundaries

3. **Memory Optimization** (25 blocks)
   - **Keep if**: Zero-copy required + validated
   - **Evolve if**: Can use safe abstractions (bytes, Cow)
   - **Tools**: MaybeUninit, ManuallyDrop patterns

4. **Concurrency** (16 blocks)
   - **Keep if**: Lock-free performance required
   - **Evolve if**: Standard library sufficient
   - **Tools**: crossbeam, parking_lot

**Evolution Process**:
1. Benchmark current unsafe implementation
2. Implement safe alternative
3. Benchmark safe version
4. If <10% slowdown → migrate
5. If >10% → document and keep unsafe

### Phase 6: External Dependency Evolution (4-6 hours)

**Target**: Verify pure Rust, evolve where beneficial

#### Dependency Audit

**Current Dependencies** (from audit):
```toml
tokio = "1.35"           # ✅ Pure Rust
serde = "1.0"            # ✅ Pure Rust
ring = "0.17"            # ⚠️ Has C code (BoringSSL algorithms)
ed25519-dalek = "2.1"    # ✅ Pure Rust
reqwest = "0.12"         # ✅ Pure Rust (with rustls-tls)
rustls = "0.23"          # ✅ Pure Rust
```

**Evolution Opportunities**:

1. **ring → RustCrypto** (where feasible)
   - ring has C code (BoringSSL)
   - RustCrypto is pure Rust
   - **Trade-off**: ring is faster (SIMD), RustCrypto is pure
   - **Decision**: Keep ring, add RustCrypto as alternative

2. **sqlx → pure Rust alternative** (future)
   - Current: Links to PostgreSQL C libs
   - Future: Pure Rust Postgres client
   - **Timeline**: Not blocking

3. **hidapi → rusb + pure Rust HID** (future)
   - Current: C library wrapper
   - Future: Pure Rust HID stack
   - **Timeline**: Phase 2

---

## 🎯 SUCCESS CRITERIA

### Code Quality
- [ ] 0 clippy errors
- [ ] 0 files > 1000 lines
- [ ] 0 hardcoded network values
- [ ] 90%+ test coverage
- [ ] 100% pure Rust in critical path

### Architecture
- [ ] Capability-based discovery throughout
- [ ] Domain-driven module structure
- [ ] Self-knowledge pattern (no primal hardcoding)
- [ ] Runtime configuration only

### Performance
- [ ] <10% regression from unsafe → safe migrations
- [ ] Zero-copy where beneficial
- [ ] SIMD where performance-critical

### Sovereignty
- [ ] No forced dependencies
- [ ] User choice throughout
- [ ] Federation-ready
- [ ] Self-hosted capable

---

## 📊 PROGRESS TRACKING

### Completed ✅
- [x] Build fixing (reqwest dev-dependency)
- [x] Comprehensive audit

### In Progress 🔄
- [ ] Clippy must_use fixes

### Queued 📋
- [ ] Large file smart refactoring
- [ ] Hardcoding evolution
- [ ] Test coverage expansion
- [ ] Unsafe code evolution
- [ ] Dependency analysis

---

## 🚀 EXECUTION ORDER

### Today (3-4 hours)
1. Fix clippy errors (15 min)
2. Start btsp_provider refactoring (2-3 hours)
3. Document progress

### This Week (15-20 hours)
1. Complete all 3 large file refactors (6-8 hours)
2. Network hardcoding evolution (3-5 hours)
3. Auth subsystem tests (6-8 hours)

### Next 2 Weeks (30-40 hours)
1. Complete hardcoding elimination (10-15 hours)
2. Full test coverage (20-27 hours)
3. Unsafe code analysis (8-10 hours)
4. Dependency evolution (4-6 hours)

---

**Status**: 🔥 **EXECUTING**  
**Philosophy**: Evolve, don't patch  
**Timeline**: 2-3 weeks to excellence

🦀🐻🐕 **Modern Idiomatic Rust + Capability-Based Design!**

