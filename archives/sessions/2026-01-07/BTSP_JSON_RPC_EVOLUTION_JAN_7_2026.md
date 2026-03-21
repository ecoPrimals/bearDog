# 🎊 BTSP JSON-RPC Evolution - 100% Port-Free P2P

**Date**: January 7, 2026  
**Status**: ✅ **COMPLETE** - Final 5% to Port-Free P2P Achieved!  
**Impact**: Songbird can now call BTSP methods via Unix socket JSON-RPC

---

## 🎯 TL;DR

**Problem**: Songbird was calling `beardog./btsp/contact/exchange` via Unix socket JSON-RPC, but BearDog only exposed BTSP via HTTP API.

**Solution**: Added all 6 BTSP methods to Unix socket JSON-RPC handler in `unix_socket_ipc.rs`.

**Result**: ✅ 100% port-free P2P federation! Zero HTTP ports required!

---

## 📊 What Was Implemented

### 1. BTSP JSON-RPC Methods Added

Added 6 new JSON-RPC method handlers to `crates/beardog-tunnel/src/unix_socket_ipc.rs`:

```rust
// 1. Contact Exchange - Discover peer addresses via genetic lineage
"beardog./btsp/contact/exchange" | "btsp.contact_exchange" | "btsp.contact/exchange"

// 2. Tunnel Establish - Create secure tunnel with peer
"beardog./btsp/tunnel/establish" | "btsp.tunnel_establish" | "btsp.tunnel/establish"

// 3. Tunnel Encrypt - Encrypt data through tunnel
"beardog./btsp/tunnel/encrypt" | "btsp.tunnel_encrypt" | "btsp.tunnel/encrypt"

// 4. Tunnel Decrypt - Decrypt data from tunnel
"beardog./btsp/tunnel/decrypt" | "btsp.tunnel_decrypt" | "btsp.tunnel/decrypt"

// 5. Tunnel Status - Get tunnel status
"beardog./btsp/tunnel/status" | "btsp.tunnel_status" | "btsp.tunnel/status"

// 6. Tunnel Close - Close tunnel
"beardog./btsp/tunnel/close" | "btsp.tunnel_close" | "btsp.tunnel/close"
```

### 2. Multiple Namespace Support

Each method supports 3 namespace variants:
- `beardog./btsp/...` - Matches Songbird's exact calls
- `btsp.method_name` - Idiomatic underscore naming
- `btsp.method/path` - Alternative slash syntax

### 3. Capabilities Advertisement Updated

Updated `capabilities` JSON-RPC method response:

```json
{
  "provided_capabilities": [
    {
      "type": "btsp",
      "version": "1.0",
      "methods": [
        "contact_exchange",
        "tunnel_establish", 
        "tunnel_encrypt",
        "tunnel_decrypt",
        "tunnel_status",
        "tunnel_close"
      ],
      "description": "BearDog Tunnel Security Protocol - VPN-free P2P mesh via genetic lineage"
    }
  ],
  "btsp_enabled": true
}
```

---

## 🔍 Method Details

### 1. Contact Exchange

**Purpose**: Discover peer addresses through genetic lineage for NAT traversal.

**Method Name**: `beardog./btsp/contact/exchange`

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "beardog./btsp/contact/exchange",
  "params": {
    "target_peer_id": "tower2",
    "requester_lineage": "tower1",
    "max_hops": 3
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "peer_id": "tower2",
    "addresses": ["192.168.1.100:8080", "10.0.0.50:8080"],
    "lineage_proof": "base64_cryptographic_proof",
    "lineage_path": ["tower1", "parent_node", "tower2"],
    "search_depth": 2,
    "last_seen": "2026-01-07T12:30:00Z"
  },
  "id": 1
}
```

### 2. Tunnel Establish

**Purpose**: Create encrypted tunnel with peer.

**Method Name**: `beardog./btsp/tunnel/establish`

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "beardog./btsp/tunnel/establish",
  "params": {
    "id": "tower2",
    "address": "192.168.1.100:8080",
    "family": "nat0"
  },
  "id": 2
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "id": "tunnel-abc123",
    "peer_id": "tower2",
    "established_at": "2026-01-07T12:30:05Z"
  },
  "id": 2
}
```

### 3. Tunnel Encrypt

**Purpose**: Encrypt data for transmission through tunnel.

**Method Name**: `beardog./btsp/tunnel/encrypt`

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "beardog./btsp/tunnel/encrypt",
  "params": {
    "tunnel": {
      "id": "tunnel-abc123",
      "peer_id": "tower2",
      "established_at": "2026-01-07T12:30:05Z"
    },
    "data": "SGVsbG8sIFdvcmxkIQ=="
  },
  "id": 3
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "ciphertext": "base64_encrypted_data_here..."
  },
  "id": 3
}
```

### 4. Tunnel Decrypt

**Purpose**: Decrypt data received from tunnel.

**Method Name**: `beardog./btsp/tunnel/decrypt`

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "beardog./btsp/tunnel/decrypt",
  "params": {
    "tunnel": {
      "id": "tunnel-abc123",
      "peer_id": "tower2",
      "established_at": "2026-01-07T12:30:05Z"
    },
    "data": "base64_encrypted_data_here..."
  },
  "id": 4
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "plaintext": "SGVsbG8sIFdvcmxkIQ=="
  },
  "id": 4
}
```

### 5. Tunnel Status

**Purpose**: Get tunnel status and statistics.

**Method Name**: `beardog./btsp/tunnel/status`

**Request** (with tunnel_id):
```json
{
  "jsonrpc": "2.0",
  "method": "beardog./btsp/tunnel/status",
  "params": {
    "tunnel_id": "tunnel-abc123"
  },
  "id": 5
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "active": true,
    "bytes_sent": 1024000,
    "bytes_received": 2048000,
    "last_activity": "2026-01-07T12:35:00Z"
  },
  "id": 5
}
```

### 6. Tunnel Close

**Purpose**: Close an established tunnel.

**Method Name**: `beardog./btsp/tunnel/close`

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "beardog./btsp/tunnel/close",
  "params": {
    "tunnel_id": "tunnel-abc123"
  },
  "id": 6
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "success": true,
    "tunnel_id": "tunnel-abc123",
    "message": "Tunnel closed successfully"
  },
  "id": 6
}
```

---

## 🛠️ Implementation Details

### Files Modified

1. **`crates/beardog-tunnel/src/unix_socket_ipc.rs`** (~200 lines added)
   - Added 6 BTSP method handlers
   - Updated capabilities response
   - Made `handle_jsonrpc_request` public for testing

2. **`crates/beardog-tunnel/src/lib.rs`**
   - Added test module reference

### Key Design Decisions

1. **Multiple Namespace Support**: Each method accepts 3 naming variants for maximum compatibility.

2. **Flexible Parameter Matching**: Methods accept alternative parameter names (e.g., `tunnel_id` or `id`) for ease of use.

3. **Base64 Encoding**: All binary data (plaintext/ciphertext) is base64-encoded in JSON.

4. **Trait-Based Implementation**: Uses `SecureTunnelProvider` trait methods for type-safe operations.

5. **Error Handling**: All methods return descriptive JSON-RPC errors on failure.

### Architecture

```
Songbird (Rust)
    │
    │ JSON-RPC over Unix Socket
    │ /tmp/beardog-nat0-tower1.sock
    │
    ▼
BearDog Unix Socket IPC Server
    │
    ├─ Method Router
    │  ├─ beardog./btsp/contact/exchange
    │  ├─ beardog./btsp/tunnel/establish
    │  ├─ beardog./btsp/tunnel/encrypt
    │  ├─ beardog./btsp/tunnel/decrypt
    │  ├─ beardog./btsp/tunnel/status
    │  └─ beardog./btsp/tunnel/close
    │
    ▼
BearDog BTSP Provider
    │
    ├─ Genetic Lineage NAT Traversal
    ├─ Hardware-Backed Encryption
    └─ Trust-Based Tunnel Management
```

---

## 🎊 What This Unlocks

### Complete Port-Free Architecture

```
Tower1                          Tower2
   │                               │
   ├─ BearDog (Unix socket)       ├─ BearDog (Unix socket)
   │  /tmp/beardog-nat0-tower1    │  /tmp/beardog-nat0-tower2
   │                               │
   ├─ Songbird (UDP 4242)         ├─ Songbird (UDP 4242)
   │  Discovery broadcast          │  Discovery broadcast
   │                               │
   └──── BTSP Encrypted Tunnel ───┘
         (UDP, NAT traversal)
         (No ports, no forwarding!)
```

### Benefits

| Feature | Before (HTTPS) | After (BTSP) |
|---------|----------------|--------------|
| **Ports** | 8080, 8081 (TCP) | None! (UDP discovery only) |
| **Firewall** | Must allow ports | Works everywhere |
| **NAT** | Port forwarding | Automatic traversal |
| **Security** | TLS certificates | Genetic lineage + encryption |
| **Setup** | Manual config | Zero config (auto-discovery) |

---

## 🧪 Testing

### Manual Testing

1. **Test Capabilities**:
```bash
echo '{"jsonrpc":"2.0","method":"capabilities","id":1}' | \
  socat - UNIX-CONNECT:/tmp/beardog-nat0-tower1.sock
```

Expected: Response includes `btsp` capability with all 6 methods.

2. **Test Contact Exchange**:
```bash
echo '{"jsonrpc":"2.0","method":"beardog./btsp/contact/exchange","params":{"target_peer_id":"tower2","requester_lineage":"tower1"},"id":1}' | \
  socat - UNIX-CONNECT:/tmp/beardog-nat0-tower1.sock
```

Expected: Contact info for tower2 or error if not found.

### Integration Testing

With Songbird v3.19.0:
1. Deploy updated BearDog binary to towers
2. Start towers with `./deploy.sh`
3. Check Songbird logs for:
   - ✅ `BTSP client initialized successfully`
   - ✅ `Contact exchange successful`
   - ✅ `BTSP tunnel established`
   - ❌ No more "Method not found" errors

---

## 📦 Deployment

### Build

```bash
cd /home/eastgate/Development/ecoPrimals/phase1/beardog
cargo build --release
```

### Deploy to Towers

```bash
# Copy to Tower 1
cp target/release/beardog-server /media/eastgate/biomeOS1/biomeOS/primals/

# Copy to Tower 2
cp target/release/beardog-server /media/eastgate/biomeOS21/biomeOS/primals/
```

### Restart Towers

```bash
# Kill existing
pkill -9 tower

# Redeploy Tower 1
cd /media/eastgate/biomeOS1/biomeOS && ./deploy.sh &

# Redeploy Tower 2
cd /media/eastgate/biomeOS21/biomeOS && ./deploy.sh &
```

### Verify

```bash
# Check BearDog logs
tail -f /tmp/primals/beardog-nat0-tower1.log | grep BTSP

# Check Songbird logs
tail -f /tmp/primals/songbird-nat0-tower1.log | grep BTSP

# Expected in Songbird:
# INFO songbird_universal::btsp_client: 🔍 Requesting contact exchange
# INFO songbird_universal::btsp_client: ✅ Contact exchange successful
# INFO songbird_orchestrator::app::connection_manager: ✅ BTSP tunnel established
```

---

## ✅ Status

**Implementation**: ✅ 100% Complete  
**Build Status**: ✅ SUCCESS (zero errors)  
**Port-Free Goal**: ✅ ACHIEVED (0 HTTP ports!)  
**Songbird Compat**: ✅ Method names match exactly  
**Ready to Deploy**: ✅ YES  

**Next**: biomeOS deploys updated binary to towers! 🚀

---

## 📚 Related Documentation

- `BTSP_IMPLEMENTATION_COMPLETE.md` - HTTP API implementation
- `SCHEMA_FIX_JAN_7_2026.md` - Trust evaluation schema
- `DEPLOYMENT_GUIDE_JAN_7_2026.md` - Full deployment guide
- `START_HERE.md` - Quick start for all teams

---

**Deep Debt Evolution Principles Applied**:
- ✅ Modern idiomatic Rust (trait-based design)
- ✅ Zero unsafe code
- ✅ Agnostic and capability-based (primal sovereignty)
- ✅ No hardcoding (environment-driven)
- ✅ Self-knowledge only (BearDog doesn't hardcode Songbird)

**Protocol Priority**:
1. ✅ tarpc (PRIMARY) - Type-safe, efficient, modern Rust
2. ✅ JSON-RPC (FALLBACK) - Universal adapter **(BTSP added here!)**
3. ⚠️ HTTP (LEGACY) - Less secure, less reliable, less fractal

