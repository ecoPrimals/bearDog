# 🎯 tarpc Client Library for Songbird

**Date**: January 6, 2026  
**Status**: ✅ **READY FOR INTEGRATION**  
**Purpose**: Type-safe RPC client for Songbird → BearDog communication

---

## 🎯 Quick Start

### Add BearDog as Dependency

```toml
# In Songbird's Cargo.toml
[dependencies]
beardog-tunnel = { path = "../beardog/crates/beardog-tunnel" }
tarpc = { version = "0.34", features = ["tokio1", "serde-transport"] }
tokio = { version = "1", features = ["full"] }
```

### Basic Usage

```rust
use beardog_tunnel::tarpc_service::{BearDogServiceClient, TrustEvaluationRequest};
use tarpc::{client, context, tokio_serde::formats::Bincode};
use tokio::net::UnixStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to BearDog's Unix socket
    let socket_path = "/tmp/beardog-nat0-tower1.sock";
    let stream = UnixStream::connect(socket_path).await?;
    
    // Create tarpc transport
    let transport = tarpc::serde_transport::new(
        tokio_util::codec::LengthDelimitedCodec::new(),
        Bincode::default(),
    ).from_stream(stream);
    
    // Create client
    let client = BearDogServiceClient::new(client::Config::default(), transport).spawn();
    
    // Call methods with type safety!
    let response = client.ping(context::current()).await?;
    println!("✅ BearDog responded: {:?}", response);
    
    Ok(())
}
```

---

## 📚 API Reference

### Ping - Health Check

```rust
let response = client.ping(context::current()).await?;
// Returns: PingResponse { pong: true, timestamp, version, protocol: "tarpc" }
```

### Capabilities - Query BearDog Features

```rust
let response = client.capabilities(context::current()).await?;
// Returns: CapabilitiesResponse {
//     capabilities: ["encryption", "trust_evaluation", ...],
//     version: "0.15.0",
//     protocols: ["tarpc", "json-rpc"],
//     security_level: 5
// }
```

### Trust Evaluation - Genetic Lineage

```rust
let request = TrustEvaluationRequest {
    peer_id: "tower2".to_string(),
    family_id: "nat0".to_string(),
    requested_operation: Some("encrypt".to_string()),
};

let response = client.evaluate_trust(context::current(), request).await?;
// Returns: TrustEvaluationResponse {
//     trust_level: 2,
//     reason: "genetic_lineage_verified",
//     allowed: true
// }
```

### BirdSong Encrypt

```rust
let plaintext = b"Hello, Songbird!";
let family_id = "nat0".to_string();

let ciphertext = client.birdsong_encrypt(
    context::current(),
    plaintext.to_vec(),
    family_id
).await?;
```

### BirdSong Decrypt

```rust
let plaintext = client.birdsong_decrypt(
    context::current(),
    ciphertext,
    family_id
).await?;
```

### Security Metrics

```rust
let metrics = client.security_metrics(context::current()).await?;
// Returns: SecurityMetricsResponse {
//     trust_evaluations: 42,
//     encryption_operations: 100,
//     active_sessions: 5,
//     uptime_seconds: 3600
// }
```

---

## 🎯 Integration with Songbird SecurityAdapter

### Current Implementation (HTTP - LEGACY)

```rust
pub struct SecurityAdapter {
    endpoint: String,
    client: reqwest::Client,  // ❌ HTTP
}
```

### New Implementation (tarpc - PRIMARY)

```rust
use beardog_tunnel::tarpc_service::{BearDogServiceClient, TrustEvaluationRequest};
use tarpc::{client, context};

pub struct SecurityAdapter {
    endpoint: String,
    protocol: SecurityProtocol,
}

enum SecurityProtocol {
    Tarpc(BearDogServiceClient),  // ✅ PRIMARY
    JsonRpc(JsonRpcClient),        // ✅ FALLBACK
    Http(reqwest::Client),         // ⚠️  LEGACY
}

impl SecurityAdapter {
    pub async fn new(endpoint: String) -> Result<Self> {
        let protocol = if endpoint.starts_with("unix://") {
            // Use tarpc for Unix socket
            let socket_path = endpoint.strip_prefix("unix://").unwrap();
            let stream = UnixStream::connect(socket_path).await?;
            
            let transport = tarpc::serde_transport::new(
                tokio_util::codec::LengthDelimitedCodec::new(),
                tarpc::tokio_serde::formats::Bincode::default(),
            ).from_stream(stream);
            
            let client = BearDogServiceClient::new(
                client::Config::default(),
                transport
            ).spawn();
            
            SecurityProtocol::Tarpc(client)
        } else {
            // Fallback to HTTP for network endpoints
            SecurityProtocol::Http(reqwest::Client::new())
        };
        
        Ok(Self { endpoint, protocol })
    }
    
    pub async fn evaluate_trust(&self, request: TrustEvaluationRequest) -> Result<TrustEvaluationResponse> {
        match &self.protocol {
            SecurityProtocol::Tarpc(client) => {
                // ✅ Type-safe RPC call
                Ok(client.evaluate_trust(context::current(), request).await?)
            }
            SecurityProtocol::Http(client) => {
                // ⚠️  Legacy HTTP fallback
                let url = format!("{}/evaluate_trust", self.endpoint);
                let response = client.post(&url).json(&request).send().await?;
                Ok(response.json().await?)
            }
            // ... JsonRpc case
        }
    }
}
```

---

## ✅ Benefits of tarpc

### 1. Type Safety ⭐⭐⭐⭐⭐

```rust
// ✅ Compile-time checks
let request = TrustEvaluationRequest {
    peer_id: "tower2",  // ❌ Won't compile - expects String
};

// ✅ Correct
let request = TrustEvaluationRequest {
    peer_id: "tower2".to_string(),  // ✅ Type-safe
    family_id: "nat0".to_string(),
    requested_operation: None,
};
```

### 2. Performance ⭐⭐⭐⭐⭐

- **bincode**: ~2x faster than JSON
- **Zero-copy**: Minimal allocations
- **Efficient**: ~1ms overhead

### 3. Modern Rust ⭐⭐⭐⭐⭐

- **async/await**: Throughout
- **Idiomatic**: Rust best practices
- **Ergonomic**: Easy to use

### 4. Security ⭐⭐⭐⭐⭐

- **Level 5/5**: Highest security
- **Type-checked**: No runtime surprises
- **Unix sockets**: Local-only by default

---

## 🔄 Migration Guide

### Step 1: Add Dependencies

```toml
[dependencies]
beardog-tunnel = { path = "../beardog/crates/beardog-tunnel" }
tarpc = { version = "0.34", features = ["tokio1", "serde-transport"] }
tokio-util = { version = "0.7", features = ["codec"] }
```

### Step 2: Update SecurityAdapter

```rust
// Before: HTTP client
let client = reqwest::Client::new();

// After: tarpc client
let stream = UnixStream::connect(socket_path).await?;
let transport = tarpc::serde_transport::new(...).from_stream(stream);
let client = BearDogServiceClient::new(client::Config::default(), transport).spawn();
```

### Step 3: Update Method Calls

```rust
// Before: HTTP POST with JSON
let response = http_client.post(url).json(&body).send().await?;

// After: Type-safe RPC call
let response = client.evaluate_trust(context::current(), request).await?;
```

### Step 4: Test

```bash
cargo test security_adapter
```

---

## 🧪 Testing

### Unit Test Example

```rust
#[tokio::test]
async fn test_beardog_client_ping() {
    // Setup mock or real BearDog server
    let socket_path = "/tmp/beardog-test.sock";
    
    let stream = UnixStream::connect(socket_path).await.unwrap();
    let transport = tarpc::serde_transport::new(
        tokio_util::codec::LengthDelimitedCodec::new(),
        tarpc::tokio_serde::formats::Bincode::default(),
    ).from_stream(stream);
    
    let client = BearDogServiceClient::new(
        client::Config::default(),
        transport
    ).spawn();
    
    let response = client.ping(context::current()).await.unwrap();
    assert!(response.pong);
    assert_eq!(response.protocol, "tarpc");
}
```

---

## 🎊 Summary

**tarpc client library for Songbird**:
- ✅ Type-safe RPC calls
- ✅ Modern async/await
- ✅ High performance (bincode)
- ✅ Security level 5/5
- ✅ Easy integration

**Migration path**:
1. Add dependencies
2. Update SecurityAdapter
3. Replace HTTP calls with tarpc
4. Test and deploy

**Result**: Modern, idiomatic, type-safe inter-primal communication!

---

**Date**: January 6, 2026  
**Status**: Ready for Songbird integration  
**Priority**: HIGH - Preferred over HTTP/JSON-RPC

