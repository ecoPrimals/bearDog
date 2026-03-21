# 🔴 CRITICAL: Port-Free Architecture Evolution - IN PROGRESS

**Date**: January 4, 2026  
**Status**: ⚠️ **PARTIALLY COMPLETE** - Needs finishing  
**Blocker**: Dual spore federation on same machine

---

## 🎯 The Problem

**User Feedback**: "beardog can have env port for conditions when it runs standalone. but functionally should always be using songbird. the interactions are dev knowledge we can use to guide the primal self knowledge. review ../songbird and how they solved this issue (i believe they use port 0 and pure ipc?)"

**Root Cause**: BearDog still hardcodes HTTP port `127.0.0.1:9000` instead of:
1. **PRIMARY**: Unix socket IPC (`/tmp/beardog-{family}.sock`)
2. **OPTIONAL**: HTTP (only if `BEARDOG_HTTP_ENABLED=true`)
3. **SMART**: Port 0 (random) to avoid conflicts

---

## ✅ What Songbird Does (CORRECT)

### Architecture
```
Songbird:
  PRIMARY:   Unix socket (/tmp/songbird-{family}.sock)
  SECONDARY: HTTP (optional, for external access)
  DISCOVERY: UDP multicast (239.255.42.99:4242)
  SMART:     Port fallback (bind_with_fallback)
```

### Key Files
- `crates/songbird-orchestrator/src/ipc/unix_socket.rs` - Unix socket IPC server ✅
- `crates/songbird-orchestrator/src/app/http_server.rs` - HTTP (optional) ✅
- Uses `bind_with_fallback()` to avoid port conflicts ✅

---

## 🔧 What Was Started

### 1. Created Unix Socket IPC Server ✅
**File**: `crates/beardog-tunnel/src/unix_socket_ipc.rs` (370 lines)

**Features**:
- JSON-RPC 2.0 protocol
- Methods: `beardog.ping`, `beardog.capabilities`, `beardog.birdsong.encrypt/decrypt`
- Graceful error handling
- Tests included

### 2. Updated lib.rs ✅
**File**: `crates/beardog-tunnel/src/lib.rs`

**Changes**:
- Added `pub mod unix_socket_ipc;`
- Marked HTTP API as "OPTIONAL"

---

## ❌ What Still Needs to Be Done

### 1. Update `beardog-server.rs` (CRITICAL)

**Current** (WRONG):
```rust
// Always starts HTTP server on hardcoded port
let bind_addr = std::env::var("BEARDOG_BIND_ADDR")
    .unwrap_or_else(|_| "127.0.0.1:9000".to_string());

let server = BearDogApiServer::new(config, btsp_provider).await?;
server.serve().await?;  // Blocks on HTTP
```

**Should Be** (CORRECT):
```rust
// 1. ALWAYS start Unix socket IPC (primary)
let socket_path = format!("/tmp/beardog-{}.sock", 
    family_id.as_ref().unwrap_or(&"default".to_string()));

let ipc_server = Arc::new(UnixSocketIpcServer::new(
    socket_path,
    btsp_provider.clone(),
).await?);

// Spawn IPC server (non-blocking)
let ipc_handle = tokio::spawn(async move {
    ipc_server.start().await
});

// 2. OPTIONALLY start HTTP (only if enabled)
let http_enabled = std::env::var("BEARDOG_HTTP_ENABLED")
    .unwrap_or_else(|_| "false".to_string()) == "true";

if http_enabled {
    let bind_addr = std::env::var("BEARDOG_BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:0".to_string());  // Port 0 = random!
    
    let config = BearDogApiServerConfig {
        bind_addr: bind_addr.parse()?,
        enable_cors: true,
        version: "0.15.0".to_string(),
    };
    
    let server = BearDogApiServer::new(config, btsp_provider).await?;
    server.serve().await?;
} else {
    // Wait for IPC server only
    ipc_handle.await??;
}
```

### 2. Add Dependencies

**File**: `crates/beardog-tunnel/Cargo.toml`

**Add**:
```toml
[dependencies]
base64 = "0.21"
chrono = "0.4"
```

### 3. Update Documentation

**Files to Update**:
- `UPSTREAM_HANDOFF_JAN_4_2026.md`
- `ZERO_VENDOR_HARDCODING_COMPLETE.md`
- `README.md`

**Key Points**:
- Unix socket is PRIMARY
- HTTP is OPTIONAL (dev/debug only)
- Port 0 for random assignment
- No port conflicts possible

### 4. Test Dual Instance

**Test Script** (`test-dual-instance.sh`):
```bash
#!/bin/bash

# Test dual BearDog instances on same machine

export BEARDOG_FAMILY_ID="nat0"

# Instance 1
export BEARDOG_NODE_ID="tower1"
./target/release/beardog-server &
PID1=$!

sleep 2

# Instance 2  
export BEARDOG_NODE_ID="tower2"
./target/release/beardog-server &
PID2=$!

sleep 2

# Verify both running
ps -p $PID1 && echo "✅ Instance 1 running"
ps -p $PID2 && echo "✅ Instance 2 running"

# Verify Unix sockets
ls -la /tmp/beardog-nat0-tower1.sock
ls -la /tmp/beardog-nat0-tower2.sock

# Cleanup
kill $PID1 $PID2
```

---

## 📋 TODO List

- [ ] **CRITICAL**: Update `beardog-server.rs` to use Unix socket as primary
- [ ] Add `BEARDOG_HTTP_ENABLED` environment variable
- [ ] Support port 0 (random) for HTTP when enabled
- [ ] Add `base64` and `chrono` dependencies
- [ ] Update all documentation
- [ ] Create `test-dual-instance.sh` script
- [ ] Test dual instance on same machine
- [ ] Verify no port conflicts
- [ ] Update `start-beardog-server.sh`

---

## 🎯 Success Criteria

### Dual Spore Federation (Same Machine)
- [ ] Spore 1 starts: BearDog (Unix socket only)
- [ ] Spore 2 starts: BearDog (Unix socket only)
- [ ] No port conflicts (no HTTP ports used!)
- [ ] Both communicate via Unix sockets
- [ ] Songbird discovers both via UDP multicast
- [ ] BearDog provides encryption for Songbird

### Environment Variables
```bash
# Required
BEARDOG_FAMILY_ID="nat0"
BEARDOG_NODE_ID="tower1"

# Optional (Unix socket path override)
BEARDOG_SOCKET_PATH="/tmp/beardog-custom.sock"

# Optional (HTTP for dev/debug only)
BEARDOG_HTTP_ENABLED="true"
BEARDOG_BIND_ADDR="0.0.0.0:0"  # Port 0 = random
```

---

## 💡 Key Insight

**From User**: "songbird uses udp and makes ports irrelevant. besides they are a massive security risk. and beardog makes sure songbird is secure"

**Architecture**:
1. **Songbird** = Discovery (UDP multicast, no ports)
2. **BearDog** = Security (encrypts Songbird traffic)
3. **Unix sockets** = Local IPC (no ports)
4. **HTTP ports** = Legacy, unnecessary, security risk ❌

---

## 🚀 Next Steps

1. **Finish `beardog-server.rs` evolution** (30 minutes)
2. **Add dependencies** (5 minutes)
3. **Test dual instance** (15 minutes)
4. **Update documentation** (20 minutes)
5. **Notify upstream** (5 minutes)

**Total Time**: ~75 minutes to complete

---

**Status**: ⚠️ **PAUSED** - Unix socket IPC server created, but `beardog-server.rs` still needs evolution to use it as primary interface.

**Blocker**: Cannot run dual spores on same machine until HTTP is made optional and Unix socket is primary.

**Impact**: Once complete, dual spore federation will work perfectly with zero port conflicts!

---

**Remember**: **Ports are the old way. UDP + Unix sockets + BearDog encryption = The future!** 🚀

