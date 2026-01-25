# BearDog Deep Evolution Plan - January 25, 2026
**Mission**: JSON-RPC + tarpc first, Modern Idiomatic Rust, Zero Hardcoding, Safe & Fast

---

## 🎯 EVOLUTION PHILOSOPHY

### Core Principles
1. **JSON-RPC + tarpc First**: All inter-primal communication via standardized RPC
2. **Runtime Discovery Only**: No hardcoded primal knowledge - discover at runtime
3. **Capability-Based**: Find services by what they do, not who they are
4. **Modern Idiomatic Rust**: Latest patterns, zero unsafe where possible
5. **Pure Rust Dependencies**: Eliminate C deps, use Rust alternatives
6. **Smart Refactoring**: Logical module boundaries, not arbitrary splits
7. **Complete Implementations**: Evolve mocks to production code
8. **Fast AND Safe**: No compromise - achieve both

---

## 📋 EXECUTION PHASES

### ✅ Phase 1: CRITICAL FIXES (COMPLETE)
**Status**: ✅ Done  
**Time**: 2 hours

- [x] Fix compilation errors in `primal_discovery.rs`
- [x] Fix compilation errors in `universal_discovery/mod.rs`
- [x] Fix unnested or-patterns (clippy warning)
- [x] Verify beardog-core builds

**Result**: Code now compiles! Ready for evolution.

---

### Phase 2: JSON-RPC + TARPC FIRST (IN PROGRESS)
**Priority**: 🔴 CRITICAL  
**Goal**: Full Songbird IPC integration + tarpc adoption  
**Time**: 2-3 weeks

#### Step 2.1: Songbird IPC Protocol (Week 1)
**Implement**: `/wateringHole/PRIMAL_IPC_PROTOCOL.md`

```rust
// Current: Custom paths
let socket = UnixStream::connect("/tmp/beardog.sock").await?;

// Target: Songbird namespace
let socket = UnixStream::connect("/primal/beardog").await?;
```

**Actions**:
1. [ ] Update all Unix socket paths to `/primal/beardog`
2. [ ] Implement Songbird registration on startup
3. [ ] Add capability declaration (crypto, btsp, ed25519, x25519)
4. [ ] Implement heartbeat mechanism (every 30-60s)
5. [ ] Add capability-based discovery client
6. [ ] Replace direct socket connections with Songbird resolution

**Files to modify**:
- `crates/beardog-tunnel/src/unix_socket_ipc/server.rs`
- `crates/beardog-tunnel/src/main.rs` (startup registration)
- `crates/beardog-core/src/primal_discovery.rs` (use Songbird)
- `crates/beardog-ipc/` (new - Songbird client)

#### Step 2.2: tarpc Integration (Week 2)
**Add**: tarpc for type-safe RPC

```rust
// Current: Manual JSON-RPC
let request = json!({
    "jsonrpc": "2.0",
    "method": "crypto.sign",
    "params": { "data": data },
    "id": 1
});

// Target: tarpc generated stubs
#[tarpc::service]
trait CryptoService {
    async fn sign(data: Vec<u8>) -> Result<Signature, CryptoError>;
}
```

**Actions**:
1. [ ] Add tarpc dependency to workspace
2. [ ] Define tarpc service traits in `beardog-ipc`
3. [ ] Generate client/server stubs
4. [ ] Migrate Unix socket handlers to tarpc
5. [ ] Maintain JSON-RPC compatibility layer
6. [ ] Update tests

#### Step 2.3: Runtime Discovery (Week 3)
**Eliminate**: All hardcoded primal knowledge

**Current violations** (838 IPs, 139 ports):
```rust
// ❌ Hardcoded everywhere
"127.0.0.1:8080"
"localhost:9090"
```

**Target**:
```rust
// ✅ Runtime discovery
let crypto_service = songbird.find_capability("crypto").await?;
let endpoint = crypto_service.endpoint; // Discovered at runtime!
```

**Actions**:
1. [ ] Create `PrimalRegistry` client (queries Songbird)
2. [ ] Replace all hardcoded endpoints with registry lookups
3. [ ] Cache discovered services (TTL: 5 minutes)
4. [ ] Handle service unavailability gracefully
5. [ ] Add discovery tests

---

### Phase 3: HARDCODING ELIMINATION (2-3 weeks)
**Priority**: 🔴 HIGH  
**Goal**: Zero hardcoded values in production  
**Reference**: `specs/current/ZERO_HARDCODING_SPECIFICATION.md`

#### Current State
- **838 IP addresses** across 235 files
- **139 port numbers** across 48 files
- **~40 file paths** hardcoded
- **~45 timeouts** hardcoded

#### Step 3.1: Network Configuration (Week 1)
```rust
// ❌ Current
const API_PORT: u16 = 8080;
let addr = "127.0.0.1:8080".parse()?;

// ✅ Target
use beardog_config::global::BEARDOG_CONFIG;
let port = BEARDOG_CONFIG.network.api.port;
let host = &BEARDOG_CONFIG.network.addresses.api_host;
let addr = format!("{}:{}", host, port).parse()?;
```

**Actions**:
1. [ ] Move all IPs to config (838 instances)
2. [ ] Move all ports to config (139 instances)
3. [ ] Add environment variable overrides
4. [ ] Update default config templates
5. [ ] Add validation

#### Step 3.2: Path Configuration (Week 2)
```rust
// ❌ Current
"/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so"

// ✅ Target - Platform discovery
PathConfig::discover_library_paths()
    .find(|p| p.join("opensc-pkcs11.so").exists())
```

**Actions**:
1. [ ] Implement platform-aware path discovery
2. [ ] Remove hardcoded library paths
3. [ ] Remove hardcoded data directories
4. [ ] Add XDG Base Directory support
5. [ ] Update documentation

#### Step 3.3: Capability-Based Everything (Week 3)
**Instead of**: "Connect to beardog at localhost:8080"  
**Use**: "Find service with 'crypto' capability"

```rust
// ❌ Hardcoded primal knowledge
async fn get_signature(data: &[u8]) -> Result<Signature> {
    let client = connect("beardog", "localhost:8080").await?;
    client.sign(data).await
}

// ✅ Capability-based
async fn get_signature(data: &[u8]) -> Result<Signature> {
    let crypto = discover_capability("crypto").await?;
    crypto.sign(data).await
}
```

---

### Phase 4: SMART FILE REFACTORING (1 week)
**Priority**: 🟡 MEDIUM  
**Goal**: Logical module boundaries, not arbitrary splits

#### Files Over 1000 Lines (9 files)
1. `btsp_provider.rs` (1330 lines) → Split by concern:
   - `btsp_provider/core.rs` (main logic)
   - `btsp_provider/trust.rs` (trust management)
   - `btsp_provider/contact.rs` (contact exchange)
   - `btsp_provider/config.rs` (configuration)

2. `tunnel/hsm/manager/mod.rs` (1140 lines) → Split by responsibility:
   - `manager/core.rs` (HSM manager core)
   - `manager/operation_router.rs` (operation routing)
   - `manager/capability.rs` (capability detection)
   - `manager/health.rs` (health monitoring)

3. `genetic_crypto.rs` (1069 lines) → Split by algorithm:
   - `genetic_crypto/lineage.rs` (lineage tracking)
   - `genetic_crypto/derivation.rs` (key derivation)
   - `genetic_crypto/entropy.rs` (entropy mixing)
   - `genetic_crypto/constraints.rs` (constraint enforcement)

**Principle**: Each module should have ONE clear responsibility

**Actions**:
1. [ ] Analyze each large file for logical boundaries
2. [ ] Create module structure with clear exports
3. [ ] Move code preserving git history
4. [ ] Update imports
5. [ ] Verify tests still pass
6. [ ] Update documentation

---

### Phase 5: UNSAFE CODE EVOLUTION (2 weeks)
**Priority**: 🟡 MEDIUM  
**Goal**: Fast AND safe - no compromise

#### Current Unsafe (163 instances across 71 files)

**Categories**:
1. **FFI (Android/iOS/PKCS#11)** - 60% of unsafe
   - Status: ✅ **ACCEPTABLE** - needed for platform integration
   - Action: Document why unsafe is necessary

2. **SIMD Optimizations** - 30% of unsafe
   - Status: ⚠️ **CAN IMPROVE**
   - Action: Use safe SIMD abstractions where possible

3. **Mock FFI** - 10% of unsafe
   - Status: ⚠️ **SHOULD ELIMINATE**
   - Action: Move to test-only code

#### Step 5.1: Safe SIMD Alternatives (Week 1)
```rust
// ❌ Current: Direct unsafe SIMD
unsafe {
    let a = _mm256_loadu_ps(data.as_ptr());
    let b = _mm256_mul_ps(a, scalar);
    _mm256_storeu_ps(result.as_mut_ptr(), b);
}

// ✅ Target: Safe SIMD via std::simd (nightly) or safer wrappers
use std::simd::*;
let a = f32x8::from_slice(data);
let b = a * f32x8::splat(scalar);
b.copy_to_slice(result);
```

**Actions**:
1. [ ] Audit all SIMD usage
2. [ ] Replace with safe abstractions where possible
3. [ ] Document remaining unsafe SIMD
4. [ ] Add safety comments and invariants
5. [ ] Comprehensive SIMD tests

#### Step 5.2: Mock Elimination (Week 2)
```rust
// ❌ Production mock
#[cfg(not(target_os = "android"))]
pub fn strongbox_init() -> Result<()> {
    // Mock implementation
    Ok(())
}

// ✅ Test-only mock
#[cfg(test)]
mod mocks {
    pub fn strongbox_init() -> Result<()> {
        Ok(())
    }
}

#[cfg(all(not(test), not(target_os = "android")))]
compile_error!("StrongBox only available on Android");
```

**Actions**:
1. [ ] Move all mocks to test modules
2. [ ] Add compile_error for unsupported platforms
3. [ ] Provide runtime capability detection
4. [ ] Update platform detection tests

---

### Phase 6: EXTERNAL DEPENDENCY EVOLUTION (1 week)
**Priority**: 🟢 LOW  
**Goal**: Rust alternatives for remaining C deps

#### Current Dependencies (from Cargo.toml)
```toml
cryptoki = "0.6"  # ⚠️ Has C FFI to PKCS#11
dirs = "5.0"      # ✅ Pure Rust
hickory-resolver = "0.24"  # ✅ Pure Rust (was trust-dns)
```

#### Actions
1. [ ] Audit all dependencies for C linkage
2. [ ] Research pure Rust alternatives
3. [ ] Evaluate performance trade-offs
4. [ ] Implement and test replacements
5. [ ] Document decisions

**Note**: cryptoki is acceptable - it's an interface to hardware HSMs which are inherently C-based.

---

### Phase 7: TEST COVERAGE EXPANSION (2 weeks)
**Priority**: 🔴 HIGH  
**Goal**: 90%+ coverage with llvm-cov

#### Current State
- ⚠️ Cannot verify (was blocked by compilation errors)
- ✅ Extensive test files exist
- Estimated: 60-70% coverage

#### Actions
1. [ ] Run `cargo llvm-cov --html` (NOW POSSIBLE!)
2. [ ] Identify uncovered code paths
3. [ ] Add unit tests for uncovered functions
4. [ ] Add integration tests for uncovered workflows
5. [ ] Add property tests for complex logic
6. [ ] Add chaos/fault injection tests
7. [ ] Document test strategy

**Target**:
- Unit tests: 95%+ coverage
- Integration tests: 85%+ coverage
- E2E tests: Critical paths covered
- Chaos tests: Fault tolerance verified

---

### Phase 8: DOCUMENTATION & VERIFICATION (1 week)
**Priority**: 🟡 MEDIUM  
**Goal**: Document evolution, verify standards compliance

#### Actions
1. [ ] Update all documentation for changes
2. [ ] Create evolution summary document
3. [ ] Update API documentation
4. [ ] Verify all standards compliance:
   - [ ] UniBin/ecoBin ✅ (already compliant)
   - [ ] JSON-RPC/tarpc
   - [ ] Primal IPC Protocol
   - [ ] Zero Hardcoding
   - [ ] Interprimal Interactions
5. [ ] Run full audit again
6. [ ] Create before/after metrics

---

## 📊 SUCCESS METRICS

### Before Evolution (Jan 25, 2026)
```
Compilation:           ❌ BROKEN
JSON-RPC/tarpc:        60% (custom impl)
Hardcoding:            40% eliminated (60% remaining)
File Size Compliance:  92% (9 files over limit)
Unsafe Code:           163 instances
Test Coverage:         ??? (can't verify)
Interprimal Standard:  20% compliant
```

### After Evolution (Target: March 2026)
```
Compilation:           ✅ PASSING
JSON-RPC/tarpc:        100% (full Songbird integration)
Hardcoding:            100% eliminated (zero instances)
File Size Compliance:  100% (all under 1000 lines)
Unsafe Code:           <50 instances (justified FFI only)
Test Coverage:         90%+ (verified with llvm-cov)
Interprimal Standard:  100% compliant
```

---

## 🚀 IMPLEMENTATION STRATEGY

### Week-by-Week Plan

**Week 1-2**: JSON-RPC + Songbird Integration
- Implement IPC protocol
- Add tarpc support
- Runtime discovery foundation

**Week 3-4**: Hardcoding Elimination
- Network configuration
- Path configuration
- Capability-based discovery

**Week 5**: File Refactoring
- Smart module splits
- Preserve git history
- Update documentation

**Week 6-7**: Unsafe Evolution
- Safe SIMD abstractions
- Mock isolation
- Platform detection

**Week 8**: Test Coverage
- Run llvm-cov
- Add missing tests
- Chaos engineering

**Week 9**: Final Verification
- Full audit
- Documentation update
- Metrics comparison

---

## 💡 GUIDING PRINCIPLES

### 1. No Regression
Every change must:
- ✅ Pass all existing tests
- ✅ Maintain or improve performance
- ✅ Not break existing functionality

### 2. Incremental Evolution
- Small, focused commits
- Each commit builds and passes tests
- Easy to review and revert if needed

### 3. Documentation First
- Document the why before the how
- Update docs with code
- Examples for new patterns

### 4. Modern Idiomatic Rust
- Use latest stable Rust features
- Follow Rust API guidelines
- Leverage type system for safety

### 5. Performance Without Compromise
- Fast code that's also safe
- Benchmark critical paths
- Zero-copy where possible
- Profile before optimizing

---

## 🎯 IMMEDIATE NEXT STEPS (This Week)

1. **Run Test Suite** (NOW!)
   ```bash
   cargo test --workspace
   cargo llvm-cov --html
   ```

2. **Start Songbird Integration**
   - Create `beardog-ipc` crate
   - Implement registration client
   - Update socket paths

3. **Begin Hardcoding Audit**
   - Create script to find all IPs/ports
   - Plan config migration
   - Start with network constants

4. **Format & Lint**
   ```bash
   cargo fmt
   cargo clippy --fix --allow-dirty
   ```

---

**Document Status**: ✅ READY TO EXECUTE  
**Next Review**: February 1, 2026  
**Estimated Completion**: March 15, 2026

🐻🐕 **BearDog: Evolving to Modern, Idiomatic, Production-Grade Excellence!** ✨

