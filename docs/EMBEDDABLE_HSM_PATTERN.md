# Embeddable BearDog HSM Pattern

**Date**: January 8, 2026  
**Status**: ✅ Production Ready  
**For**: biomeOS, Songbird, and other sovereign systems

---

## 🎯 Overview

BearDog is designed to be **embedded** into your application, not run as a standalone server. This document explains how to properly integrate BearDog's HSM capabilities into your primal.

---

## 🔑 Key Concept: `auto_initialize()`

The **CRITICAL** method for HSM integration is:

```rust
use beardog_tunnel::tunnel::hsm::HsmManager;

let hsm = Arc::new(HsmManager::auto_initialize().await?);
```

### What `auto_initialize()` Does

1. ✅ Reads `BEARDOG_HSM_MODE` from environment
2. ✅ Registers the appropriate HSM provider
3. ✅ Handles fallbacks gracefully
4. ✅ Returns a ready-to-use `HsmManager`

### Supported HSM Modes

| Mode | Description | Status |
|------|-------------|--------|
| `software` | Pure Rust software HSM | ✅ Production Ready |
| `hardware` | Hardware HSM (YubiHSM, etc.) | ⏸️  Future (falls back to software) |
| `android` | Android StrongBox | ⏸️  Future (falls back to software) |
| `ios` | iOS Secure Enclave | ⏸️  Future (falls back to software) |

---

## 📦 Software HSM Details

### Pure Rust Implementation

BearDog's software HSM is implemented in **pure Rust** with the following crypto backends:

1. **RustCrypto** (default) - 100% pure Rust
2. **Ring** - Rust + assembly (high performance)
3. **OpenSSL** - Rust wrapper around OpenSSL C library

### Features

- ✅ Zero external dependencies (RustCrypto backend)
- ✅ Embeddable in any Rust application
- ✅ Automatic memory zeroing
- ✅ Audit logging
- ✅ Health monitoring
- ✅ Thread-safe (Arc + RwLock)
- ✅ Async-first design

### Security

- AES-256-GCM encryption at rest
- Constant-time operations
- Secure memory management
- Comprehensive audit logging
- Protection against key extraction

---

## 🚀 Integration Pattern

### Step-by-Step Integration

```rust
use beardog_tunnel::{BeardogBtspProvider, HsmManager};
use beardog_genetics::EcosystemGeneticEngine;
use beardog_tunnel::api::{BearDogApiServer, BearDogApiServerConfig};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Auto-initialize HSM (reads BEARDOG_HSM_MODE env var)
    let hsm = Arc::new(HsmManager::auto_initialize().await?);

    // Step 2: Initialize genetic engine
    let genetics = Arc::new(EcosystemGeneticEngine::new()?);

    // Step 3: Create BTSP provider
    let btsp_provider = Arc::new(
        BeardogBtspProvider::new(hsm, genetics).await?
    );

    // Step 4: Create API server
    let config = BearDogApiServerConfig::default();  // Reads from env
    let server = BearDogApiServer::new(config, btsp_provider).await?;

    // Step 5: Serve
    server.serve().await?;
    Ok(())
}
```

### Environment Variables

```bash
# Required
export BEARDOG_HSM_MODE=software

# Optional (with defaults)
export BEARDOG_HSM_AUTO_INIT=true
export BEARDOG_BIND_ADDR=0.0.0.0:0  # Port 0 = OS auto-select
export BEARDOG_ENABLE_CORS=true

# Optional: Family lineage (for biomeOS genetic siblings)
export BEARDOG_FAMILY_SEED=$(cat /path/to/usb/seed.txt)
```

---

## 🧬 biomeOS Integration

### For Genetic Sibling Towers

biomeOS towers can embed BearDog to enable genetic lineage verification:

```rust
// In your biomeOS tower startup
use beardog_tunnel::{BeardogBtspProvider, HsmManager};
use beardog_genetics::EcosystemGeneticEngine;
use std::sync::Arc;

// 1. Read family seed from USB spore
let family_seed = std::fs::read_to_string("/media/usb/biomeOS/seed.txt")?;
std::env::set_var("BEARDOG_FAMILY_SEED", family_seed);

// 2. Initialize BearDog with software HSM
std::env::set_var("BEARDOG_HSM_MODE", "software");
let hsm = Arc::new(HsmManager::auto_initialize().await?);

// 3. Create genetic engine
let genetics = Arc::new(EcosystemGeneticEngine::new()?);

// 4. Create BTSP provider
let btsp_provider = Arc::new(
    BeardogBtspProvider::new(hsm, genetics).await?
);

// 5. Now you can verify lineage relationships
let is_family = btsp_provider
    .verify_same_family(&peer_lineage_proof)
    .await?;

if is_family {
    println!("✅ Peer is a genetic sibling - auto-trust!");
} else {
    println!("❌ Peer is a stranger - reject!");
}
```

---

## 🔍 Common Mistakes

### ❌ DON'T: Use `HsmManager::new()`

```rust
// ❌ WRONG: No HSM providers registered!
let hsm = Arc::new(HsmManager::new());
```

**Error**: `No HSM providers available`

### ✅ DO: Use `HsmManager::auto_initialize()`

```rust
// ✅ CORRECT: HSM providers auto-registered
let hsm = Arc::new(HsmManager::auto_initialize().await?);
```

---

## 📊 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Your Application                        │
│                      (biomeOS Tower)                        │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            │ embeds
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    BearDog BTSP Provider                    │
│  ┌──────────────────┐  ┌──────────────────┐                │
│  │   HSM Manager    │  │ Genetic Engine   │                │
│  │ (auto_initialize)│  │  (EcosystemGE)   │                │
│  └────────┬─────────┘  └──────────────────┘                │
│           │                                                 │
│           │ registers                                       │
│           ▼                                                 │
│  ┌──────────────────┐                                       │
│  │ Software HSM     │                                       │
│  │ (RustSoftwareHsm)│                                       │
│  │                  │                                       │
│  │ • RustCrypto     │                                       │
│  │ • Ring           │                                       │
│  │ • OpenSSL        │                                       │
│  └──────────────────┘                                       │
└─────────────────────────────────────────────────────────────┘
```

---

## 🧪 Testing

### Unit Tests

```rust
#[tokio::test]
async fn test_hsm_initialization() {
    use std::env;
    env::set_var("BEARDOG_HSM_MODE", "software");
    
    let hsm = HsmManager::auto_initialize().await;
    assert!(hsm.is_ok());
    
    env::remove_var("BEARDOG_HSM_MODE");
}
```

### Integration Tests

See `examples/embeddable_beardog_server.rs` for a complete working example.

---

## 📚 References

- **Example**: `examples/embeddable_beardog_server.rs`
- **HSM Manager**: `crates/beardog-tunnel/src/tunnel/hsm/manager/mod.rs`
- **Software HSM**: `crates/beardog-tunnel/src/tunnel/hsm/software_hsm/`
- **BTSP Provider**: `crates/beardog-tunnel/src/btsp_provider.rs`

---

## 🎓 For biomeOS Team

### Quick Start

1. Add BearDog to your `Cargo.toml`:
```toml
[dependencies]
beardog-tunnel = { path = "../beardog/crates/beardog-tunnel" }
beardog-genetics = { path = "../beardog/crates/beardog-genetics" }
```

2. Set environment variable:
```bash
export BEARDOG_HSM_MODE=software
```

3. Use the integration pattern above in your tower startup code.

4. Test with your genetic siblings:
```bash
# On node-alpha
./start_tower.sh

# On node-beta
./start_tower.sh

# Verify they recognize each other as family
curl http://localhost:9000/api/v1/lineage/same_family \
  -d '{"peer_lineage_proof": "..."}'
```

---

## ✅ Status

- **BearDog Side**: ✅ Complete and production-ready
- **biomeOS Side**: ⏸️  Waiting for integration
- **Blocking Issue**: ❌ RESOLVED (this document)

---

**Last Updated**: January 8, 2026  
**Maintained By**: BearDog Team  
**For Questions**: See `README.md` or `DOCUMENTATION_INDEX.md`

