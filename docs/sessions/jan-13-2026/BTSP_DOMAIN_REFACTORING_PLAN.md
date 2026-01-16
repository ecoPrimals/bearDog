# 🏗️ BTSP Provider Domain Refactoring Plan

**Date**: January 13, 2026  
**Current**: `btsp_provider.rs` (1191 lines)  
**Target**: Domain-driven module structure  
**Philosophy**: Capability-based, not just line-count splitting

---

## 📊 CURRENT ANALYSIS

### File Structure (btsp_provider.rs)

**Lines 1-120**: Module setup, traits, re-exports  
**Lines 121-230**: Trust management, tunnel state  
**Lines 231-500**: `BeardogBtspProvider` struct + new()  
**Lines 501-900**: SecureTunnelProvider implementation  
**Lines 901-1050**: Legacy BtspProvider implementation  
**Lines 1051-1191**: Helper methods, conversions

### Existing Submodules (Already Good!)
- ✅ `contact.rs` - Contact exchange logic
- ✅ `metrics.rs` - Metrics and monitoring  
- ✅ `trust.rs` - Trust decisions
- ✅ `types.rs` - Type definitions

### What's in the Main File?
1. **Provider Implementation** - The main struct
2. **Tunnel Lifecycle** - Creation, management
3. **Capability Interface** - SecureTunnelProvider trait impl
4. **Legacy Interface** - BtspProvider trait impl (deprecated)
5. **Helper Functions** - Conversions, utilities

---

## 🎯 REFACTORING STRATEGY

### NOT Simple Splitting!

❌ **Bad Approach**: Just split into 3-4 files by line count  
✅ **Good Approach**: Extract coherent domains with clear responsibilities

### Domain-Driven Design

The file implements these domains:
1. **Provider Core** - Public API, initialization
2. **Tunnel Lifecycle** - Creation, management, state
3. **Capability Implementation** - SecureTunnelProvider interface
4. **Crypto Operations** - Encryption, key derivation
5. **Legacy Support** - Deprecated trait (will be removed)

---

## 📁 PROPOSED STRUCTURE

```
crates/beardog-tunnel/src/btsp_provider/
├── mod.rs (200 lines)
│   ├── Public API
│   ├── BeardogBtspProvider struct
│   ├── Module coordination
│   └── Re-exports
│
├── core.rs (250 lines)  
│   ├── Provider initialization
│   ├── Configuration
│   ├── State management
│   └── Core logic
│
├── tunnel_lifecycle.rs (300 lines)
│   ├── Tunnel struct
│   ├── Creation/destruction
│   ├── State transitions
│   └── Session key management
│
├── capability_impl.rs (250 lines)
│   ├── SecureTunnelProvider trait impl
│   ├── Establish tunnel
│   ├── Encrypt/decrypt
│   └── Status checking
│
├── crypto_operations.rs (200 lines)
│   ├── Genetic key derivation
│   ├── Session key generation
│   ├── Encryption/decryption
│   └── HSM integration
│
├── legacy.rs (150 lines) [DEPRECATED]
│   ├── BtspProvider trait impl
│   ├── Backward compatibility
│   └── Will be removed in v0.11.0
│
├── contact.rs (existing)
├── metrics.rs (existing)
├── trust.rs (existing)
└── types.rs (existing)
```

---

## 🔑 KEY IMPROVEMENTS

### 1. Capability-Based Design

**Current**: Mixed concerns in single file  
**Evolution**: Clear separation of capability implementation from infrastructure

```rust
// mod.rs - Public API
pub struct BeardogBtspProvider {
    core: ProviderCore,
    lifecycle: TunnelLifecycleManager,
    crypto: CryptoOperations,
}

// capability_impl.rs - Implements the trait
#[async_trait]
impl SecureTunnelProvider for BeardogBtspProvider {
    async fn establish_tunnel(&self, peer: &PeerEndpoint) 
        -> Result<TunnelHandle, BearDogError> 
    {
        // Delegates to lifecycle manager
        self.lifecycle.establish(peer, &self.crypto, &self.core).await
    }
}
```

### 2. Testability

Each domain can be tested in isolation:
- `core_tests.rs` - Provider initialization
- `lifecycle_tests.rs` - Tunnel management
- `capability_tests.rs` - Interface compliance
- `crypto_tests.rs` - Encryption operations

### 3. No Hardcoding

**Current**: Some hardcoded assumptions  
**Evolution**: All dependencies injected

```rust
// Before (implicit HSM)
impl BeardogBtspProvider {
    pub fn new() -> Self {
        let hsm = HsmManager::new();  // Implicit creation
        // ...
    }
}

// After (dependency injection)
impl BeardogBtspProvider {
    pub fn with_hsm(hsm: Arc<HsmManager>) -> Self {
        Self {
            core: ProviderCore::new(hsm),
            // ...
        }
    }
}
```

---

## 🚀 MIGRATION PLAN

### Phase 1: Create Module Structure (30 min)

1. Create directory: `crates/beardog-tunnel/src/btsp_provider/`
2. Move existing modules:
   ```bash
   mv crates/beardog-tunnel/src/btsp_provider/contact.rs \
      crates/beardog-tunnel/src/btsp_provider/
   # (already there as submodules)
   ```
3. Create new module files:
   - `core.rs`
   - `tunnel_lifecycle.rs`
   - `capability_impl.rs`
   - `crypto_operations.rs`
   - `legacy.rs`

### Phase 2: Extract Domains (1.5 hours)

#### Step 1: Extract Tunnel Lifecycle (30 min)
Move `Tunnel` struct and methods to `tunnel_lifecycle.rs`:
- Lines 133-228 (Tunnel struct + impl)
- Related helper methods

#### Step 2: Extract Crypto Operations (30 min)
Move crypto-specific code to `crypto_operations.rs`:
- Key derivation
- Encryption/decryption helpers
- HSM integration points

#### Step 3: Extract Capability Implementation (30 min)
Move `SecureTunnelProvider` impl to `capability_impl.rs`:
- Lines 501-900
- Clean interface implementation

### Phase 3: Create Provider Core (30 min)

Extract provider state and initialization:
- `BeardogBtspProvider` struct
- Constructor
- Configuration
- State management

### Phase 4: Update mod.rs (30 min)

Create clean public API:
```rust
// mod.rs
mod core;
mod tunnel_lifecycle;
mod capability_impl;
mod crypto_operations;
mod legacy;

// Existing
pub mod contact;
pub mod metrics;
pub mod trust;
pub mod types;

pub use contact::ContactInfo;
pub use metrics::BtspMetrics;
pub use types::{Direction, PeerInfo, SecurityContext, TrustLevel};

// Main provider
pub use core::BeardogBtspProvider;

// Capability trait implementation is in capability_impl.rs
// but trait is from beardog_capabilities
```

### Phase 5: Update Imports (30 min)

Update all files that import from `btsp_provider.rs`:
```bash
# Find all imports
rg "use.*btsp_provider" --type rust
```

Update to new module structure:
```rust
// Before
use crate::btsp_provider::BeardogBtspProvider;

// After (same - public API unchanged!)
use crate::btsp_provider::BeardogBtspProvider;
```

### Phase 6: Run Tests (15 min)

```bash
cargo test --lib
cargo test --package beardog-tunnel
```

---

## ✅ SUCCESS CRITERIA

### Code Quality
- [ ] All modules < 300 lines
- [ ] Clear domain boundaries
- [ ] No circular dependencies
- [ ] Public API unchanged

### Architecture
- [ ] Dependency injection used
- [ ] No hardcoded assumptions
- [ ] Testable in isolation
- [ ] Clear separation of concerns

### Testing
- [ ] All existing tests pass
- [ ] No test changes needed (API compatible)
- [ ] Can add domain-specific tests easily

---

## 🎯 ESTIMATED TIME

**Total**: 2.5 - 3 hours

- Phase 1: 30 min (structure)
- Phase 2: 1.5 hours (extraction)
- Phase 3: 30 min (core)
- Phase 4: 30 min (mod.rs)
- Phase 5: 30 min (imports)
- Phase 6: 15 min (tests)

---

## 📊 BEFORE/AFTER COMPARISON

### Before
```
btsp_provider.rs (1191 lines)
├── Everything mixed
├── Hard to navigate
├── Hard to test parts
└── Large cognitive load
```

### After
```
btsp_provider/
├── mod.rs (200) - Public API
├── core.rs (250) - Provider implementation
├── tunnel_lifecycle.rs (300) - Tunnel management  
├── capability_impl.rs (250) - Trait implementation
├── crypto_operations.rs (200) - Crypto helpers
├── legacy.rs (150) - Deprecated (will remove)
├── contact.rs (existing)
├── metrics.rs (existing)
├── trust.rs (existing)
└── types.rs (existing)

Total: ~1,500 lines (with new tests)
Largest file: 300 lines ✅
Clear boundaries: ✅
Testable: ✅
```

---

## 🚀 EXECUTION

Ready to proceed with refactoring following this plan!

**Status**: 📋 **PLAN COMPLETE**  
**Next**: 🔨 **EXECUTE MIGRATION**

🐻🐕 **Smart Domain-Driven Refactoring!**

