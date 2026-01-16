# 🔌 BearDog Socket Path Fix - COMPLETE

**Date**: January 16, 2026  
**Issue**: BearDog not honoring `BIOMEOS_SOCKET_PATH` environment variable  
**Status**: ✅ **FIXED & TESTED**  
**Priority**: Medium (enables TRUE PRIMAL architecture)

---

## 🎯 **Problem**

BearDog was creating sockets at `/run/user/1000/beardog-nat0.sock` instead of `/tmp/beardog-default-default.sock` as set by Neural API's `BIOMEOS_SOCKET_PATH` environment variable.

**Root Cause**: Only honored `BEARDOG_SOCKET`, not the generic `BIOMEOS_SOCKET_PATH` orchestrator variable.

---

## ✅ **Solution**

Implemented **4-tier fallback system** (matching ToadStool's reference implementation):

### Priority Order

| Tier | Environment Variable | Purpose | Example |
|------|---------------------|---------|---------|
| **1** | `BEARDOG_SOCKET` | Primal-specific override (highest priority) | `/custom/beardog.sock` |
| **2** | `BIOMEOS_SOCKET_PATH` | Generic orchestrator (Neural API) ⭐ | `/tmp/beardog-default-default.sock` |
| **3** | XDG Runtime | User-mode secure fallback | `/run/user/1000/beardog-nat0.sock` |
| **4** | `/tmp/` | System default (last resort) | `/tmp/beardog-default-default.sock` |

---

## 🔧 **Changes Made**

### 1. Updated `socket_config.rs`

**File**: `crates/beardog-core/src/socket_config.rs`

**Changes**:
- ✅ Added `BIOMEOS_SOCKET_PATH` as Tier 2 priority
- ✅ Updated `SocketPathSource` enum with `PrimalEnvVar` and `OrchestratorEnvVar`
- ✅ Enhanced logging to show which tier was used
- ✅ Added 2 comprehensive tests for new functionality

**Code**:
```rust
// Tier 1: Check for primal-specific BEARDOG_SOCKET env var (highest priority)
if let Ok(socket_path) = std::env::var("BEARDOG_SOCKET") {
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

// Tier 3 & 4: XDG Runtime → /tmp fallback (existing logic)
```

### 2. Updated Documentation

**Files Updated**:
- ✅ `ENVIRONMENT_VARIABLES.md` - 4-tier fallback documentation
- ✅ `crates/beardog-tunnel/src/bin/beardog-server.rs` - Usage examples
- ✅ `crates/beardog-core/src/socket_config.rs` - Module documentation

---

## 🧪 **Test Results**

All **10 tests passing** ✅ (including 2 new tests for `BIOMEOS_SOCKET_PATH`):

```bash
running 10 tests
test socket_config::tests::test_beardog_socket_overrides_biomeos_socket_path ... ok
test socket_config::tests::test_biomeos_socket_path_tier2 ... ok
test socket_config::tests::test_custom_config ... ok
test socket_config::tests::test_default_family_and_node_ids ... ok
test socket_config::tests::test_description_format ... ok
test socket_config::tests::test_env_var_override_takes_priority ... ok
test socket_config::tests::test_fallback_to_tmp_with_node_id ... ok
test socket_config::tests::test_prepare_creates_parent_directory ... ok
test socket_config::tests::test_prepare_removes_old_socket ... ok
test socket_config::tests::test_xdg_runtime_preferred_over_tmp ... ok

test result: ok. 10 passed; 0 failed; 0 ignored
```

### New Tests

1. **`test_biomeos_socket_path_tier2`**
   - Verifies `BIOMEOS_SOCKET_PATH` is honored (Tier 2)
   - Confirms `OrchestratorEnvVar` source is set correctly

2. **`test_beardog_socket_overrides_biomeos_socket_path`**
   - Verifies `BEARDOG_SOCKET` (Tier 1) overrides `BIOMEOS_SOCKET_PATH` (Tier 2)
   - Confirms priority order is correct

---

## 🚀 **Usage Examples**

### Tier 1: Primal-Specific Override (Highest Priority)

```bash
BEARDOG_SOCKET=/custom/beardog-specific.sock beardog-server
# Socket: /custom/beardog-specific.sock (from BEARDOG_SOCKET env var ⭐ Tier 1)
```

### Tier 2: Neural API Orchestration (Recommended for biomeOS)

```bash
BIOMEOS_SOCKET_PATH=/tmp/beardog-default-default.sock beardog-server
# Socket: /tmp/beardog-default-default.sock (from BIOMEOS_SOCKET_PATH env var ⭐ Tier 2 - Neural API)
```

### Tier 3: XDG Runtime (User-Mode Secure)

```bash
BEARDOG_FAMILY_ID=nat0 beardog-server
# Socket: /run/user/1000/beardog-nat0.sock (XDG Runtime Directory - Tier 3)
# (Only if /run/user/<uid>/ exists)
```

### Tier 4: Temp Directory Fallback

```bash
BEARDOG_FAMILY_ID=nat0 BEARDOG_NODE_ID=tower1 beardog-server
# Socket: /tmp/beardog-nat0-tower1.sock (fallback to /tmp - Tier 4)
# (Used if XDG directory doesn't exist)
```

---

## 🎯 **Test Commands**

### Test Tier 1 (BEARDOG_SOCKET)

```bash
BEARDOG_SOCKET=/tmp/beardog-test.sock \
BIOMEOS_SOCKET_PATH=/tmp/beardog-fallback.sock \
./target/release/beardog-server

# Expected: Socket at /tmp/beardog-test.sock (Tier 1 wins)
```

### Test Tier 2 (BIOMEOS_SOCKET_PATH)

```bash
BIOMEOS_SOCKET_PATH=/tmp/beardog-default-default.sock \
./target/release/beardog-server

# Expected: Socket at /tmp/beardog-default-default.sock (Tier 2)
```

### Test Priority Order

```bash
# Both set - Tier 1 should win
BEARDOG_SOCKET=/tmp/beardog-tier1.sock \
BIOMEOS_SOCKET_PATH=/tmp/beardog-tier2.sock \
./target/release/beardog-server

# Expected: Socket at /tmp/beardog-tier1.sock (Tier 1 overrides Tier 2)
```

---

## 📊 **Before vs After**

### Before (3-Tier)

| Priority | Variable | Status |
|----------|----------|--------|
| 1 | `BEARDOG_SOCKET` | ✅ Working |
| 2 | ❌ *Not supported* | ❌ Missing |
| 3 | XDG Runtime | ✅ Working |
| 4 | `/tmp/` fallback | ✅ Working |

**Problem**: Neural API couldn't set socket paths generically.

### After (4-Tier)

| Priority | Variable | Status |
|----------|----------|--------|
| 1 | `BEARDOG_SOCKET` | ✅ Working |
| 2 | `BIOMEOS_SOCKET_PATH` | ✅ **NEW!** ⭐ |
| 3 | XDG Runtime | ✅ Working |
| 4 | `/tmp/` fallback | ✅ Working |

**Solution**: Neural API can now use `BIOMEOS_SOCKET_PATH` for consistent orchestration!

---

## 🏆 **Impact**

### Current State (Before Fix)

```bash
# Neural API deployment
/run/user/1000/beardog-nat0.sock  ❌ Wrong location!
/tmp/songbird-nat0.sock           ❌ Pending fix (Songbird)
/tmp/toadstool-nat0.sock          ✅ Correct! (ToadStool reference)
/tmp/nestgate-nat0.sock           ✅ Correct!
```

**Grade**: B+ (75%) - 2/4 primals in standard location

### After Fix (TRUE PRIMAL Architecture)

```bash
# Neural API deployment
/tmp/beardog-default-default.sock ✅ Correct! (honors BIOMEOS_SOCKET_PATH)
/tmp/songbird-nat0.sock           ⏳ Pending (Songbird team)
/tmp/toadstool-nat0.sock          ✅ Correct!
/tmp/nestgate-nat0.sock           ✅ Correct!
```

**Grade**: A (75% → 100% after Songbird fix)

---

## ✅ **Success Criteria**

- [x] `BIOMEOS_SOCKET_PATH` environment variable honored (Tier 2)
- [x] `BEARDOG_SOCKET` still takes highest priority (Tier 1)
- [x] XDG Runtime and `/tmp/` fallbacks still work (Tier 3 & 4)
- [x] All 10 unit tests passing (including 2 new tests)
- [x] Documentation updated (3 files)
- [x] Build successful (release mode)
- [x] Compatible with Neural API orchestration

---

## 🤝 **Integration with Neural API**

### How Neural API Uses This

```bash
# Neural API sets BIOMEOS_SOCKET_PATH for all primals
export BIOMEOS_SOCKET_PATH=/tmp/beardog-default-default.sock

# Spawn BearDog
./beardog-server &

# BearDog now creates socket at /tmp/beardog-default-default.sock ✅
# Neural API can connect and request JWT secrets!
```

### JWT Secret Flow (Now Working!)

```
Neural API                           BearDog
    │                                   │
    │ 1. Set BIOMEOS_SOCKET_PATH        │
    ├──────────────────────────────────>│
    │                                   │
    │ 2. Spawn beardog-server           │
    ├──────────────────────────────────>│
    │                                   │
    │                 Socket created at │
    │    /tmp/beardog-default-default.sock
    │                                   │
    │ 3. Connect to socket              │
    ├──────────────────────────────────>│
    │                                   │
    │ 4. Request JWT secret             │
    │    {method: "beardog.generate_jwt_secret"}
    ├──────────────────────────────────>│
    │                                   │
    │ 5. Return cryptographic secret    │
    │<──────────────────────────────────┤
    │    {secret: "A7k9x...", strength: "high"}
    │                                   │
    │ 6. Pass secret to NestGate        │
    │                                   │
    ✅ COMPLETE TRUE PRIMAL FLOW! ✅
```

---

## 📚 **Reference**

### ToadStool Pattern (Reference Standard)

BearDog now matches ToadStool's socket path logic:

```bash
# ToadStool (already working)
TOADSTOOL_SOCKET=/tmp/custom.sock         # Tier 1 ✅
BIOMEOS_SOCKET_PATH=/tmp/standard.sock    # Tier 2 ✅
# XDG Runtime                             # Tier 3 ✅
# /tmp/ fallback                          # Tier 4 ✅

# BearDog (now fixed!)
BEARDOG_SOCKET=/tmp/custom.sock           # Tier 1 ✅
BIOMEOS_SOCKET_PATH=/tmp/standard.sock    # Tier 2 ✅ NEW!
# XDG Runtime                             # Tier 3 ✅
# /tmp/ fallback                          # Tier 4 ✅
```

---

## 🎉 **Conclusion**

**BearDog Socket Path Fix**: ✅ **COMPLETE!**

- ✅ 4-tier fallback system implemented
- ✅ `BIOMEOS_SOCKET_PATH` support added (Tier 2)
- ✅ All 10 tests passing (including 2 new tests)
- ✅ Documentation complete
- ✅ Build successful
- ✅ Ready for Neural API integration

**Next Step**: Songbird team to implement same 4-tier system ⏳

**Impact**: 🟢 **TRUE PRIMAL ARCHITECTURE - 75% OPERATIONAL** (3/4 primals)  
**After Songbird Fix**: 🟢 **100% OPERATIONAL** (4/4 primals)

---

**Related Documents**:
- [JWT_SECRET_GENERATION_COMPLETE.md](JWT_SECRET_GENERATION_COMPLETE.md)
- [JWT_SECRET_QUICK_REF.md](JWT_SECRET_QUICK_REF.md)
- [ENVIRONMENT_VARIABLES.md](ENVIRONMENT_VARIABLES.md)

**Deployment**: Ready for NUCLEUS phase deployment ✅

