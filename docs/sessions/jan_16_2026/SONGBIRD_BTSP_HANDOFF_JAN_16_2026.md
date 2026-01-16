# 🐦 Songbird BTSP Integration - Unix Socket Migration

**Date**: January 16, 2026  
**From**: BearDog Team  
**To**: Songbird Team  
**Priority**: Medium (BTSP working, just needs migration)  
**Timeline**: 2-4 hours

---

## 🎯 TL;DR

**BTSP is already on Unix sockets!** You just need to:
1. Remove HTTP client code (`reqwest` calls to BearDog)
2. Add Unix socket client (example below)
3. Test and deploy

---

## 📦 What Changed?

### Before (HTTP - DEPRECATED)

```rust
use reqwest::Client;

let client = Client::new();
let response = client
    .post("http://localhost:9000/btsp/tunnel/establish")
    .json(&establish_request)
    .send()
    .await?;

let tunnel_handle: TunnelHandle = response.json().await?;
```

### After (Unix Socket - CURRENT)

```rust
use tokio::net::UnixStream;
use serde_json::json;

let socket_path = std::env::var("BEARDOG_SOCKET")
    .unwrap_or("/tmp/beardog-default-default.sock".to_string());

let mut stream = UnixStream::connect(&socket_path).await?;

let request = json!({
    "jsonrpc": "2.0",
    "method": "btsp.tunnel_establish",
    "params": { "peer": peer_endpoint },
    "id": 1
});

// Send + receive (helper function recommended)
let response = send_jsonrpc_request(&mut stream, request).await?;
let tunnel_handle: TunnelHandle = serde_json::from_value(response["result"].clone())?;
```

---

## 🔧 Implementation Guide

### Step 1: Add BTSP Client Module

Create `songbird/crates/songbird-orchestrator/src/btsp_client.rs`:

```rust
use anyhow::Result;
use serde_json::json;
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

/// BTSP Client for communicating with BearDog via Unix socket
pub struct BtspClient {
    socket_path: PathBuf,
}

impl BtspClient {
    /// Create new BTSP client with socket discovery
    pub fn new() -> Self {
        let socket_path = std::env::var("BEARDOG_SOCKET")
            .or_else(|_| std::env::var("BIOMEOS_SOCKET_PATH"))
            .or_else(|_| {
                // Try XDG runtime
                std::env::var("XDG_RUNTIME_DIR").map(|dir| {
                    let family_id = std::env::var("BEARDOG_FAMILY_ID")
                        .or_else(|_| std::env::var("FAMILY_ID"))
                        .unwrap_or_else(|_| "default".to_string());
                    format!("{}/beardog-{}.sock", dir, family_id)
                })
            })
            .unwrap_or_else(|_| "/tmp/beardog-default-default.sock".to_string());

        Self {
            socket_path: PathBuf::from(socket_path),
        }
    }

    /// Establish a BTSP tunnel with a peer
    pub async fn establish_tunnel(&self, peer: PeerEndpoint) -> Result<TunnelHandle> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_establish",
            "params": { "peer": peer },
            "id": 1
        });

        let response = self.send_request(request).await?;
        serde_json::from_value(response["result"].clone())
            .map_err(|e| anyhow::anyhow!("Failed to parse tunnel handle: {}", e))
    }

    /// Encrypt data through a tunnel
    pub async fn tunnel_encrypt(
        &self,
        tunnel: &TunnelHandle,
        data: &[u8],
        direction: Direction,
    ) -> Result<Vec<u8>> {
        let data_b64 = base64::engine::general_purpose::STANDARD.encode(data);

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_encrypt",
            "params": {
                "tunnel": tunnel,
                "direction": direction,
                "data": data_b64
            },
            "id": 2
        });

        let response = self.send_request(request).await?;
        let ciphertext_b64 = response["result"]["ciphertext"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing ciphertext in response"))?;

        base64::engine::general_purpose::STANDARD
            .decode(ciphertext_b64)
            .map_err(|e| anyhow::anyhow!("Failed to decode ciphertext: {}", e))
    }

    /// Decrypt data from a tunnel
    pub async fn tunnel_decrypt(
        &self,
        tunnel: &TunnelHandle,
        data: &[u8],
    ) -> Result<Vec<u8>> {
        let data_b64 = base64::engine::general_purpose::STANDARD.encode(data);

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_decrypt",
            "params": {
                "tunnel": tunnel,
                "data": data_b64
            },
            "id": 3
        });

        let response = self.send_request(request).await?;
        let plaintext_b64 = response["result"]["plaintext"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing plaintext in response"))?;

        base64::engine::general_purpose::STANDARD
            .decode(plaintext_b64)
            .map_err(|e| anyhow::anyhow!("Failed to decode plaintext: {}", e))
    }

    /// Get tunnel status
    pub async fn tunnel_status(&self, tunnel: &TunnelHandle) -> Result<TunnelStatus> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_status",
            "params": { "tunnel": tunnel },
            "id": 4
        });

        let response = self.send_request(request).await?;
        serde_json::from_value(response["result"].clone())
            .map_err(|e| anyhow::anyhow!("Failed to parse tunnel status: {}", e))
    }

    /// Close a tunnel
    pub async fn tunnel_close(&self, tunnel: &TunnelHandle) -> Result<()> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_close",
            "params": { "tunnel": tunnel },
            "id": 5
        });

        self.send_request(request).await?;
        Ok(())
    }

    /// Send JSON-RPC request and receive response
    async fn send_request(&self, request: serde_json::Value) -> Result<serde_json::Value> {
        let mut stream = UnixStream::connect(&self.socket_path).await?;

        // Send request
        let request_bytes = serde_json::to_vec(&request)?;
        stream.write_all(&request_bytes).await?;
        stream.write_all(b"\n").await?; // JSON-RPC delimiter

        // Read response
        let mut buffer = Vec::new();
        let mut reader = BufReader::new(&mut stream);
        reader.read_until(b'\n', &mut buffer).await?;

        let response: serde_json::Value = serde_json::from_slice(&buffer)?;

        // Check for JSON-RPC error
        if let Some(error) = response.get("error") {
            return Err(anyhow::anyhow!(
                "JSON-RPC error: {}",
                error.get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error")
            ));
        }

        Ok(response)
    }
}

// Type aliases (use BearDog's types)
use beardog_capabilities::traits::PeerEndpoint;
use beardog_tunnel::btsp_provider::{Direction, TunnelHandle, TunnelStatus};
```

---

### Step 2: Update Your Code

**Replace HTTP calls**:

```rust
// OLD (HTTP)
let client = reqwest::Client::new();
let response = client.post(...).await?;

// NEW (Unix socket)
let btsp = BtspClient::new();
let tunnel = btsp.establish_tunnel(peer).await?;
```

---

### Step 3: Environment Variables

**For deployment** (Neural API should set these):

```bash
# Primary socket path (highest priority)
BEARDOG_SOCKET=/tmp/beardog-default-default.sock

# Fallback (biomeOS orchestrator)
BIOMEOS_SOCKET_PATH=/tmp/beardog-orchestrator.sock

# Family ID (for socket discovery)
BEARDOG_FAMILY_ID=nat0
FAMILY_ID=nat0
```

---

## 📋 BTSP Methods Available

| Method | Purpose | Params |
|--------|---------|--------|
| `btsp.tunnel_establish` | Create secure tunnel | `peer` (PeerEndpoint) |
| `btsp.tunnel_encrypt` | Encrypt data | `tunnel`, `data`, `direction` |
| `btsp.tunnel_decrypt` | Decrypt data | `tunnel`, `data` |
| `btsp.tunnel_status` | Get status | `tunnel` |
| `btsp.tunnel_close` | Close tunnel | `tunnel` |
| `btsp.contact_exchange` | Discover peer | `target_peer_id`, `lineage`, `max_hops` |

**All use JSON-RPC 2.0** ✅  
**All async** ✅  
**All production-ready** ✅

---

## 🧪 Testing

### Quick Test (netcat)

```bash
echo '{"jsonrpc":"2.0","method":"ping","id":1}' | nc -U /tmp/beardog-default-default.sock
```

**Expected**:
```json
{
    "jsonrpc": "2.0",
    "result": {
        "status": "healthy",
        "primal": "beardog",
        "version": "0.9.0"
    },
    "id": 1
}
```

---

### Integration Test

```rust
#[tokio::test]
async fn test_btsp_establish_tunnel() {
    let btsp = BtspClient::new();
    
    let peer = PeerEndpoint {
        id: "test-peer".to_string(),
        endpoint: "10.0.1.100:9000".to_string(),
        public_key: Some("test-key".to_string()),
        capabilities: vec!["federation".to_string()],
    };

    let tunnel = btsp.establish_tunnel(peer).await.unwrap();
    
    assert!(tunnel.id.starts_with("btsp_"));
    assert_eq!(tunnel.peer_id, "test-peer");
}
```

---

## ✅ Migration Checklist

- [ ] Copy `BtspClient` code into your codebase
- [ ] Find all HTTP calls to BearDog BTSP (`reqwest::Client`)
- [ ] Replace with `BtspClient` calls
- [ ] Add environment variables (`BEARDOG_SOCKET`)
- [ ] Test Unix socket connection
- [ ] Run integration tests
- [ ] Deploy and verify

**Estimated Time**: 2-4 hours

---

## 🎯 Why This is Better

**Before (HTTP)**:
- ❌ HTTP overhead for local communication
- ❌ More dependencies (reqwest, hyper)
- ❌ Slower (TCP vs Unix socket)
- ❌ Not aligned with Concentrated Gap strategy

**After (Unix Socket)**:
- ✅ Zero HTTP for inter-primal communication
- ✅ Faster (Unix sockets)
- ✅ Simpler (JSON-RPC)
- ✅ **Concentrated Gap**: Songbird = single HTTP gateway

---

## 🤝 Support

**Questions?** Check:
- `BTSP_EVOLUTION_COMPLETE_JAN_16_2026.md` - Full evolution details
- `crates/beardog-tunnel/src/unix_socket_ipc/handlers.rs` - Implementation

**Need Help?** Reach out in wateringHole/ 🌱

---

🌱🐦🐻 **BTSP Unix Socket Migration - Simple and Fast!** 🐻🐦🌱

**Created**: January 16, 2026  
**Purpose**: Songbird BTSP integration guide  
**Timeline**: 2-4 hours  
**Result**: Concentrated Gap strategy complete!

