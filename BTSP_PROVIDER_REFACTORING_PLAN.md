# 🔧 btsp_provider.rs Smart Refactoring Plan

**File**: `crates/beardog-tunnel/src/btsp_provider.rs`  
**Current Size**: 1,330 lines  
**Target**: 5 domain modules (~200-300 lines each)  
**Approach**: Domain-driven, not arbitrary

---

## 📊 DOMAIN ANALYSIS

### Current Structure (From File Analysis):
1. **Lines 1-100**: Imports, types, re-exports
2. **Lines 100-400**: Core SecureTunnelProviderImpl struct + state
3. **Lines 400-700**: Contact exchange protocol
4. **Lines 700-900**: Tunnel management (establish, encrypt, decrypt)
5. **Lines 900-1100**: Session management & lifecycle
6. **Lines 1100-1330**: Metrics, cleanup, tests

### Identified Domains:
1. **Core Coordinator** - Provider impl, routing, orchestration
2. **Contact Exchange** - TOFU, contact sharing, verification
3. **Session Management** - Lifecycle, state, reconnection
4. **Crypto Operations** - Encrypt, decrypt, key derivation
5. **Metrics & Monitoring** - Telemetry, health checks

---

## 🎯 REFACTORING PLAN

### New Module Structure:
```
btsp_provider/
├── mod.rs                    (~200 lines) - Public API & coordinator
├── contact_exchange.rs       (~300 lines) - TOFU contact protocol
├── session.rs                (~280 lines) - Session lifecycle
├── crypto.rs                 (~300 lines) - Encryption operations
└── metrics.rs                (~250 lines) - Telemetry & monitoring
```

### Domain Boundaries:

#### 1. `mod.rs` (Core Coordinator)
**Responsibility**: Public API, trait implementation, routing
**Contains**:
- `SecureTunnelProviderImpl` struct
- `SecureTunnelProvider` trait impl
- High-level coordination logic
- Re-exports from sub-modules

**Lines**: ~200
**Dependencies**: All sub-modules

#### 2. `contact_exchange.rs` (Contact Protocol)
**Responsibility**: Contact discovery, TOFU, verification
**Contains**:
- `ContactExchanger` struct
- TOFU (Trust On First Use) logic
- Contact serialization/deserialization
- BirdSong contact hints

**Lines**: ~300
**Key Functions**:
- `exchange_contacts_with_peer()`
- `verify_contact_signature()`
- `store_trusted_contact()`

#### 3. `session.rs` (Session Management)
**Responsibility**: Tunnel session lifecycle
**Contains**:
- `SessionManager` struct
- Session state machine
- Reconnection logic
- Keepalive handling

**Lines**: ~280
**Key Functions**:
- `create_session()`
- `handle_session_lifecycle()`
- `reconnect_session()`

#### 4. `crypto.rs` (Crypto Operations)
**Responsibility**: All cryptographic operations
**Contains**:
- `CryptoEngine` struct
- Encryption/decryption
- Key derivation (genetic crypto)
- HSM integration

**Lines**: ~300
**Key Functions**:
- `encrypt_tunnel_data()`
- `decrypt_tunnel_data()`
- `derive_session_keys()`

#### 5. `metrics.rs` (Already exists as sub-module!)
**Status**: ✅ Already separated
**Contains**: BtspMetrics, telemetry
**Action**: Keep as-is, ensure clean interface

---

## 🔨 EXECUTION STEPS

### Step 1: Create Module Directory (5 min)
```bash
mkdir -p crates/beardog-tunnel/src/btsp_provider
```

### Step 2: Extract Contact Exchange (30 min)
- Create `contact_exchange.rs`
- Move all contact-related functions
- Define `ContactExchanger` struct with clear state
- Test builds

### Step 3: Extract Session Management (30 min)
- Create `session.rs`
- Move session lifecycle functions
- Define `SessionManager` with state machine
- Test builds

### Step 4: Extract Crypto Operations (30 min)
- Create `crypto.rs`
- Move encryption/decryption functions
- Define `CryptoEngine` with HSM integration
- Test builds

### Step 5: Create Coordinator (30 min)
- Rename `btsp_provider.rs` → `btsp_provider/mod.rs`
- Keep only public API + coordination
- Wire sub-modules together
- Test builds

### Step 6: Verify & Test (15 min)
- Run full test suite
- Verify no functionality changes
- Check build time impact
- Update documentation

**Total Time**: ~2.5 hours

---

## 📋 SUCCESS CRITERIA

- [ ] All files < 400 lines
- [ ] Clear domain boundaries
- [ ] No functionality changes
- [ ] All tests pass
- [ ] Build time same or better
- [ ] Documentation updated

---

## 🎯 BENEFITS

### Modularity
- ✅ Each module has single responsibility
- ✅ Easy to test in isolation
- ✅ Clear dependencies

### Maintainability
- ✅ Easier to navigate (~300 lines vs 1330)
- ✅ Changes localized to relevant domain
- ✅ Less risk of unintended coupling

### Concurrency
- ✅ Can refine locking per domain
- ✅ Crypto engine can be lock-free
- ✅ Session manager can use fine-grained locks

### Testing
- ✅ Unit test each domain separately
- ✅ Mock interfaces between domains
- ✅ Integration tests at coordinator level

---

## 🚀 READY TO EXECUTE

**Next**: Start with Step 1 (create module directory)

**Philosophy**: Domain-driven, not arbitrary. Each module represents a clear business domain with well-defined responsibilities.

