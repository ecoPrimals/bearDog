# 🔌 BearDog Tower Atomic

**Tower Atomic** = Unix socket-based JSON-RPC for inter-primal communication

## Purpose

BearDog is a **Pure Rust crypto primal** with ZERO network dependencies.  
When BearDog needs HTTP/TLS, it delegates to Songbird via Tower Atomic.

## Philosophy

- **BearDog**: Crypto only (ed25519, x25519, chacha20, blake3)
- **Songbird**: TLS/HTTP gateway (Pure Rust, 95% complete)
- **Tower Atomic**: Inter-primal glue (Unix sockets, JSON-RPC)

## Example

```rust
use beardog_tower_atomic::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Songbird for HTTP
    let mut songbird = Client::connect("songbird").await?;
    
    // Delegate HTTP to Songbird
    let response = songbird.call("http.get", json!({
        "url": "https://api.example.com/data",
        "headers": { "Authorization": "Bearer ..." }
    })).await?;
    
    println!("Status: {}", response["status"]);
    println!("Body: {}", response["body"]);
    
    Ok(())
}
```

## Architecture

```text
┌─────────────┐                          ┌─────────────┐
│   BearDog   │  Unix Socket JSON-RPC    │  Songbird   │
│   (Crypto)  │ ───────────────────────> │  (TLS/HTTP) │
└─────────────┘                          └─────────────┘
      ↓                                         ↓
  Ed25519, X25519                        HTTPS to external
  ChaCha20, Blake3                       APIs, AI providers
```

## Discovery

Tower Atomic automatically discovers primals via Unix socket paths:

1. `$XDG_RUNTIME_DIR/ecoPrimals/{primal}.sock` (user services)
2. `$HOME/.local/share/ecoPrimals/{primal}.sock` (user-level)
3. `/var/run/ecoPrimals/{primal}.sock` (system services)
4. `/tmp/ecoPrimals/{primal}.sock` (fallback)

## Benefits

### ✅ TRUE Separation of Concerns

- **BearDog**: Crypto ONLY (no HTTP, no TLS, no network)
- **Songbird**: HTTP/TLS ONLY (gateway for external APIs)
- **Tower Atomic**: Communication glue (Unix sockets, fast, secure)

### ✅ 100% Pure Rust

- Zero C dependencies
- Zero ring (via reqwest/rustls)
- Zero unsafe code
- Perfect for ecoBin cross-compilation

### ✅ Ecosystem Consistency

All primals use Tower Atomic:
- biomeOS → BearDog (crypto)
- biomeOS → Songbird (AI, TLS)
- Squirrel → Songbird (AI APIs)
- BearDog → Songbird (HTTP/TLS)
- ToadStool → BearDog (crypto)
- NestGate → BearDog (crypto)

## JSON-RPC 2.0 Protocol

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "http.get",
  "params": {
    "url": "https://api.example.com"
  },
  "id": 1
}
```

### Response (Success)

```json
{
  "jsonrpc": "2.0",
  "result": {
    "status": 200,
    "body": "..."
  },
  "id": 1
}
```

### Response (Error)

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32600,
    "message": "Invalid Request"
  },
  "id": 1
}
```

## License

AGPL-3.0

---

🐻🐕 BearDog: Pure Rust Crypto, Zero Network Dependencies! 🦀✨

*"BearDog is crypto-only. All HTTP delegated to Songbird via Tower Atomic. This is the TRUE PRIMAL way!"*

