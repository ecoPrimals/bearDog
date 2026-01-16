# 🐦 Songbird Socket Path Fix - Guidance for Songbird Team

**Date**: January 16, 2026  
**From**: BearDog Team  
**To**: Songbird Team  
**Status**: ⏳ **AWAITING IMPLEMENTATION**  
**Priority**: Medium  
**Effort**: ~30-60 minutes (copy BearDog's pattern)

---

## 🎯 **Problem**

Songbird is creating sockets at `/tmp/squirrel-squirrel.sock` instead of `/tmp/songbird-nat0.sock` as expected by Neural API orchestration.

**Root Causes**:
1. Squirrel binary not running in Songbird orchestrator mode
2. Not honoring `SONGBIRD_ORCHESTRATOR_SOCKET` environment variable
3. Not honoring `BIOMEOS_SOCKET_PATH` generic orchestrator variable

---

## ✅ **Solution**

Implement **4-tier fallback system** (same pattern as BearDog and ToadStool):

### Priority Order

| Tier | Environment Variable | Purpose | Example |
|------|---------------------|---------|---------|
| **1** | `SONGBIRD_ORCHESTRATOR_SOCKET` | Primal-specific override (highest priority) | `/custom/songbird.sock` |
| **2** | `BIOMEOS_SOCKET_PATH` | Generic orchestrator (Neural API) ⭐ | `/tmp/songbird-nat0.sock` |
| **3** | XDG Runtime | User-mode secure fallback | `/run/user/1000/songbird-nat0.sock` |
| **4** | `/tmp/` | System default (last resort) | `/tmp/songbird-nat0-tower1.sock` |

---

## 🔧 **Implementation Steps**

### Step 1: Copy BearDog's Socket Config Pattern

**Reference File**: `phase1/beardog/crates/beardog-core/src/socket_config.rs`

Create a similar `socket_config` module for Songbird with this logic:

```rust
pub fn from_env() -> Self {
    let family_id = std::env::var("SONGBIRD_ORCHESTRATOR_FAMILY_ID")
        .or_else(|_| std::env::var("FAMILY_ID"))
        .unwrap_or_else(|_| "default".to_string());

    let node_id = std::env::var("SONGBIRD_ORCHESTRATOR_NODE_ID")
        .or_else(|_| std::env::var("NODE_ID"))
        .unwrap_or_else(|_| "default".to_string());

    // Tier 1: Check for primal-specific SONGBIRD_ORCHESTRATOR_SOCKET env var (highest priority)
    if let Ok(socket_path) = std::env::var("SONGBIRD_ORCHESTRATOR_SOCKET") {
        return Self {
            socket_path: PathBuf::from(socket_path),
            family_id,
            node_id,
            source: SocketPathSource::PrimalEnvVar,
        };
    }

    // Tier 2: Check for generic orchestrator BIOMEOS_SOCKET_PATH
    // This allows Neural API to set a standard path for all primals
    if let Ok(socket_path) = std::env::var("BIOMEOS_SOCKET_PATH") {
        return Self {
            socket_path: PathBuf::from(socket_path),
            family_id,
            node_id,
            source: SocketPathSource::OrchestratorEnvVar,
        };
    }

    // Tier 3: Try XDG Runtime Directory (more secure, per-user)
    if let Some(xdg_path) = Self::try_xdg_runtime(&family_id) {
        return Self {
            socket_path: xdg_path,
            family_id,
            node_id,
            source: SocketPathSource::XdgRuntime,
        };
    }

    // Tier 4: Fallback to /tmp (last resort)
    let tmp_path = format!("/tmp/songbird-{}-{}.sock", family_id, node_id);
    Self {
        socket_path: PathBuf::from(tmp_path),
        family_id,
        node_id,
        source: SocketPathSource::TempDir,
    }
}
```

### Step 2: Update Songbird Orchestrator Binary

Replace hardcoded socket path construction with:

```rust
use songbird_core::socket_config::SocketConfig;

let socket_config = SocketConfig::from_env();

info!("🔌 Socket Path: {}", socket_config.description());
info!("   Family ID: {}", socket_config.family_id());
info!("   Node ID: {}", socket_config.node_id());

// Prepare socket (create parent dir, remove old socket)
socket_config.prepare()?;

// Use socket_config.socket_path() when binding
```

### Step 3: Verify Binary Mode Selection

Ensure `squirrel` binary runs in **Songbird orchestrator mode**, not generic Squirrel mode:

```bash
# Should be Songbird orchestrator mode
./songbird-orchestrator

# NOT generic Squirrel mode
# ./squirrel (wrong!)
```

### Step 4: Add Tests

Copy BearDog's test pattern:

```rust
#[test]
fn test_songbird_socket_tier2() {
    std::env::remove_var("SONGBIRD_ORCHESTRATOR_SOCKET");
    std::env::set_var("BIOMEOS_SOCKET_PATH", "/tmp/songbird-nat0.sock");
    
    let config = SocketConfig::from_env();
    
    assert_eq!(
        config.socket_path_string(),
        "/tmp/songbird-nat0.sock",
        "BIOMEOS_SOCKET_PATH should be honored (Tier 2)"
    );
    assert_eq!(
        config.source(),
        SocketPathSource::OrchestratorEnvVar
    );
    
    std::env::remove_var("BIOMEOS_SOCKET_PATH");
}

#[test]
fn test_songbird_socket_overrides_biomeos() {
    std::env::set_var("SONGBIRD_ORCHESTRATOR_SOCKET", "/custom/songbird.sock");
    std::env::set_var("BIOMEOS_SOCKET_PATH", "/tmp/songbird-generic.sock");
    
    let config = SocketConfig::from_env();
    
    assert_eq!(
        config.socket_path_string(),
        "/custom/songbird.sock",
        "SONGBIRD_ORCHESTRATOR_SOCKET (Tier 1) should override BIOMEOS_SOCKET_PATH (Tier 2)"
    );
    
    std::env::remove_var("SONGBIRD_ORCHESTRATOR_SOCKET");
    std::env::remove_var("BIOMEOS_SOCKET_PATH");
}
```

---

## 🧪 **Test Commands**

### Run Your Existing Tests

You already have 11 test scenarios! Run them:

```bash
cd phase1/squirrel

cargo test --package songbird-orchestrator \
  --test biomeos_socket_env_vars -- --test-threads=1
```

All 11 scenarios should pass after implementing the fix.

### Manual Test

```bash
# Test Tier 1 (SONGBIRD_ORCHESTRATOR_SOCKET)
SONGBIRD_ORCHESTRATOR_SOCKET=/tmp/songbird-test.sock \
SONGBIRD_ORCHESTRATOR_FAMILY_ID=nat0 \
./songbird-orchestrator

# Expected: Socket at /tmp/songbird-test.sock

# Test Tier 2 (BIOMEOS_SOCKET_PATH)
BIOMEOS_SOCKET_PATH=/tmp/songbird-nat0.sock \
SONGBIRD_ORCHESTRATOR_FAMILY_ID=nat0 \
./songbird-orchestrator

# Expected: Socket at /tmp/songbird-nat0.sock
```

---

## 📊 **Before vs After**

### Before (Missing Orchestrator Support)

```bash
# Current behavior (WRONG!)
/tmp/squirrel-squirrel.sock  ❌ Wrong binary mode or hardcoded path

# Neural API can't discover Songbird!
```

### After (TRUE PRIMAL Architecture)

```bash
# With BIOMEOS_SOCKET_PATH (Tier 2)
/tmp/songbird-nat0.sock  ✅ Correct! Neural API can discover

# With SONGBIRD_ORCHESTRATOR_SOCKET (Tier 1)
/custom/songbird.sock  ✅ Custom override works

# With XDG Runtime (Tier 3)
/run/user/1000/songbird-nat0.sock  ✅ Secure user-mode

# With fallback (Tier 4)
/tmp/songbird-nat0-tower1.sock  ✅ Multi-instance support
```

---

## 🎯 **Success Criteria**

After implementation, verify:

- [ ] `SONGBIRD_ORCHESTRATOR_SOCKET` honored (Tier 1)
- [ ] `BIOMEOS_SOCKET_PATH` honored (Tier 2)
- [ ] XDG Runtime fallback works (Tier 3)
- [ ] `/tmp/` fallback works (Tier 4)
- [ ] All 11 existing tests pass
- [ ] Socket created at `/tmp/songbird-nat0.sock` when `BIOMEOS_SOCKET_PATH` is set
- [ ] Compatible with Neural API orchestration

---

## 🤝 **Support**

### Reference Implementations

1. **BearDog** (just fixed!):
   - File: `phase1/beardog/crates/beardog-core/src/socket_config.rs`
   - 10 tests, all passing ✅
   - Document: `BEARDOG_SOCKET_PATH_FIX_JAN_16_2026.md`

2. **ToadStool** (original reference):
   - File: `phase1/toadstool/crates/toadstool-server/src/main.rs` (lines 147-179)
   - Already working correctly ✅

### Questions?

- **BearDog Team**: Check `BEARDOG_SOCKET_PATH_FIX_JAN_16_2026.md` for complete implementation
- **BiomeOS Docs**: See `PRIMAL_HARVEST_COMPLETE_JAN_16_2026.md`
- **Existing Tests**: `phase1/squirrel/crates/songbird-orchestrator/tests/biomeos_socket_env_vars.rs`

---

## 🏆 **Impact**

### Current State (Before Songbird Fix)

```bash
# NUCLEUS deployment
/tmp/beardog-default-default.sock ✅ Fixed! (BearDog)
/tmp/songbird-nat0.sock           ❌ Pending (Songbird)
/tmp/toadstool-nat0.sock          ✅ Working (ToadStool)
/tmp/nestgate-nat0.sock           ✅ Working (NestGate)
```

**Grade**: A- (75%) - 3/4 primals in standard location

### After Songbird Fix (TRUE PRIMAL Complete!)

```bash
# NUCLEUS deployment
/tmp/beardog-default-default.sock ✅ Working (BearDog)
/tmp/songbird-nat0.sock           ✅ Fixed! (Songbird)
/tmp/toadstool-nat0.sock          ✅ Working (ToadStool)
/tmp/nestgate-nat0.sock           ✅ Working (NestGate)
```

**Grade**: A+ (100%) - 4/4 primals in standard location ✅

---

## ⏱️ **Timeline Estimate**

- **Socket Config Module**: 30 minutes (copy BearDog's pattern)
- **Binary Integration**: 15 minutes (replace hardcoded path)
- **Testing**: 15 minutes (run existing 11 tests + manual verification)
- **Total**: ~1 hour for 100% operational NUCLEUS

---

## 🎉 **Let's Get NUCLEUS to 100%!**

BearDog is ready! 🐻 ✅  
ToadStool is ready! 🍄 ✅  
NestGate is ready! 🚪 ✅  
Songbird... **your turn!** 🐦 ⏳

Once Songbird is fixed, we'll have **complete TRUE PRIMAL architecture** with zero hardcoding and pure runtime discovery! 🚀

---

**Priority**: Medium (not urgent - fallbacks working)  
**Effort**: Low (~1 hour - copy existing patterns)  
**Reward**: High (TRUE PRIMAL validation complete!)

🌱🐻🐦🍄🚪 Let's get NUCLEUS to 100%! 🚀

