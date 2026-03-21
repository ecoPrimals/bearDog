# 🚀 Quick Start: BearDog Integration Implementation

**Goal**: Make BearDog ready for encrypted compute with Songbird + ToadStool  
**Approach**: Protocol escalation (HTTP → JSON-RPC → tarpc)  
**Status**: Ready to implement

---

## 🎯 **What We're Building**

BearDog will expose crypto operations via **three protocols** (matching Songbird):

```
┌─────────────────────────────────────────────────────────────┐
│  BEARDOG CRYPTO SERVICE (Protocol-Agnostic Core)           │
│                                                             │
│  trait CryptoService {                                      │
│    encrypt(), decrypt(), sign(), verify(), generate_key()  │
│  }                                                          │
└───────────────┬────────────────┬────────────────┬──────────┘
                │                │                │
       ┌────────▼────────┐  ┌───▼─────┐  ┌──────▼───────┐
       │  HTTP/REST      │  │ JSON-RPC│  │    tarpc     │
       │  Port 8080      │  │ Port    │  │  Port 8091   │
       │  Universal      │  │ 8080/rpc│  │ High-perf    │
       └─────────────────┘  └─────────┘  └──────────────┘
                │                │                │
                └────────────────┴────────────────┘
                                 │
                       ┌─────────▼──────────┐
                       │  mDNS Advertisement │
                       │  _beardog._tcp.local│
                       └────────────────────┘
```

**Then Songbird discovers BearDog, negotiates protocol, and upgrades to tarpc (10-100x faster!)**

---

## ⚡ **Critical Path: Days 1-5** (LAN Working)

### **Day 1-2: Core Crypto Trait** 🔴 CRITICAL

**Create**: `crates/beardog-core/src/crypto_service.rs`

```rust
/// Protocol-agnostic crypto operations
#[async_trait]
pub trait CryptoService: Send + Sync {
    async fn encrypt(&self, data: &[u8], algorithm: CryptoAlgorithm, options: EncryptOptions) -> Result<EncryptedData>;
    async fn decrypt(&self, encrypted: &EncryptedData, options: DecryptOptions) -> Result<Vec<u8>>;
    async fn sign(&self, data: &[u8], algorithm: SignatureAlgorithm, options: SignOptions) -> Result<Signature>;
    async fn verify(&self, data: &[u8], signature: &Signature, options: VerifyOptions) -> Result<bool>;
    async fn generate_key(&self, algorithm: KeyAlgorithm, options: KeyGenOptions) -> Result<KeyInfo>;
    async fn get_capabilities(&self) -> Result<ServiceCapabilities>;
}

/// Implementation using beardog-core providers
pub struct BearDogCryptoService {
    crypto_provider: Arc<dyn CryptoProvider>,
    hsm_provider: Option<Arc<dyn HsmProvider>>,
}

impl CryptoService for BearDogCryptoService {
    async fn encrypt(&self, data: &[u8], algorithm: CryptoAlgorithm, options: EncryptOptions) -> Result<EncryptedData> {
        // Use existing beardog-core crypto
        let encrypted = self.crypto_provider.encrypt(data, &algorithm.to_core_algorithm())?;
        Ok(EncryptedData {
            ciphertext: encrypted,
            algorithm,
            metadata: EncryptionMetadata {
                timestamp: SystemTime::now(),
                key_id: options.key_id,
                nonce: self.crypto_provider.get_nonce()?,
            },
        })
    }
    // ... implement other methods
}
```

**Create**: `crates/beardog-types/src/crypto_service.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CryptoAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
    Aes128Gcm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub algorithm: CryptoAlgorithm,
    pub metadata: EncryptionMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionMetadata {
    pub timestamp: SystemTime,
    pub key_id: Option<String>,
    pub nonce: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptOptions {
    pub key_id: Option<String>,
    pub associated_data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptOptions {
    pub key_id: Option<String>,
    pub associated_data: Option<Vec<u8>>,
}

// Similar for Sign/Verify/GenerateKey...
```

**Output**: ✅ Protocol-agnostic crypto trait ready

---

### **Day 3-4: HTTP API** 🔴 CRITICAL

**Modify**: `crates/beardog-api/src/endpoints/crypto.rs`

**REPLACE current mocks with**:

```rust
use beardog_core::crypto_service::CryptoService;

pub struct AppState {
    pub crypto_service: Arc<dyn CryptoService>,
}

/// POST /api/v1/crypto/encrypt
pub async fn encrypt_data(
    State(state): State<Arc<AppState>>,
    Json(request): Json<EncryptRequest>,
) -> Result<Json<EncryptResponse>, BearDogError> {
    // Decode base64
    let data = base64::decode(&request.data)
        .map_err(|e| BearDogError::validation(format!("Invalid base64: {}", e)))?;
    
    // Call crypto service (uses the trait!)
    let encrypted = state.crypto_service
        .encrypt(
            &data,
            request.algorithm,
            EncryptOptions {
                key_id: request.key_id,
                associated_data: request.associated_data.map(|d| base64::decode(&d)).transpose()?,
            },
        )
        .await?;
    
    // Return response
    Ok(Json(EncryptResponse {
        encrypted_data: base64::encode(&encrypted.ciphertext),
        algorithm: encrypted.algorithm,
        metadata: encrypted.metadata,
    }))
}

// Similar for decrypt_data(), sign_data(), verify_signature()
```

**Test**:

```bash
# Encrypt
$ curl http://localhost:8080/api/v1/crypto/encrypt \
    -H "Content-Type: application/json" \
    -d '{"data":"SGVsbG8gV29ybGQ=","algorithm":"Aes256Gcm"}'

# Should return real encrypted data!
```

**Output**: ✅ HTTP API working (mocks replaced)

---

### **Day 5: mDNS Wiring** 🔴 CRITICAL

**Modify**: `crates/beardog-api/src/lib.rs`

```rust
use beardog_core::primal_discovery_mdns::MdnsDiscoveryClient;

pub async fn start_beardog_server(config: BearDogConfig) -> Result<()> {
    // 1. Create crypto service
    let crypto_service = Arc::new(BearDogCryptoService::new(config.crypto)?);
    let app_state = Arc::new(AppState { crypto_service });
    
    // 2. Build HTTP router
    let app = Router::new()
        .nest("/api/v1/crypto", crypto_routes())
        .with_state(app_state);
    
    // 3. Start HTTP server
    let listener = tokio::net::TcpListener::bind("[::]:8080").await?;
    
    // 4. Announce via mDNS (CODE ALREADY EXISTS!)
    let mdns_client = MdnsDiscoveryClient::new()?;
    mdns_client.announce_self(
        "beardog-northgate",  // Your tower name
        vec![
            "crypto.encryption.aes256gcm",
            "crypto.signing.ed25519",
            "hsm.solokeys",
            "genetics.self_enforcing_keys",
            "protocol.http",
        ],
        8080,
        HashMap::from([
            ("version", env!("CARGO_PKG_VERSION")),
            ("hsm_available", "true"),
        ]),
    ).await?;
    
    info!("🐻 BearDog HTTP API on port 8080");
    info!("🐻 BearDog announced on mDNS: _beardog._tcp.local.");
    
    // 5. Serve
    axum::serve(listener, app).await?;
    Ok(())
}
```

**Test**:

```bash
# Discovery
$ avahi-browse -r _beardog._tcp

_beardog._tcp.local.
   hostname = beardog-northgate.local
   address = 192.168.1.100
   port = 8080
   txt = ["version=0.9.0" "hsm_available=true"]
```

**Output**: ✅ BearDog discoverable on LAN, HTTP API working

---

## 🚀 **Extended Path: Days 6-10** (Protocol Escalation)

### **Day 6-7: JSON-RPC API** 🟠 HIGH

**Create**: `crates/beardog-api/src/jsonrpc/mod.rs`

**Add to `Cargo.toml`**:

```toml
jsonrpsee = { version = "0.21", features = ["server", "macros"] }
```

**Implement**:

```rust
use jsonrpsee::server::{Server, ServerBuilder};
use jsonrpsee::proc_macros::rpc;

#[rpc(server)]
pub trait BearDogRpc {
    #[method(name = "beardog.encrypt")]
    async fn encrypt(&self, request: EncryptRequest) -> Result<EncryptResponse, ErrorObject>;
    
    #[method(name = "beardog.decrypt")]
    async fn decrypt(&self, request: DecryptRequest) -> Result<DecryptResponse, ErrorObject>;
    
    // ... other methods
}

pub struct BearDogRpcServer {
    crypto_service: Arc<dyn CryptoService>,
}

#[async_trait]
impl BearDogRpcServer for BearDogRpcServerImpl {
    async fn encrypt(&self, request: EncryptRequest) -> Result<EncryptResponse, ErrorObject> {
        // Same logic as HTTP! Just calls crypto_service.encrypt()
        let data = base64::decode(&request.data)?;
        let encrypted = self.crypto_service.encrypt(&data, request.algorithm, request.options).await?;
        Ok(EncryptResponse { ... })
    }
}

pub async fn start_jsonrpc_server(crypto_service: Arc<dyn CryptoService>, port: u16) -> Result<ServerHandle> {
    let server = ServerBuilder::default().build(format!("[::]:{}") port).await?;
    let rpc_impl = BearDogRpcServerImpl { crypto_service };
    Ok(server.start(rpc_impl.into_rpc())?)
}
```

**Test**:

```bash
$ curl http://localhost:8080/jsonrpc \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"beardog.encrypt","params":{"data":"SGVsbG8=","algorithm":"Aes256Gcm"},"id":1}'
```

**Output**: ✅ JSON-RPC API working

---

### **Day 8-9: tarpc API** 🟠 HIGH

**Create**: `crates/beardog-api/src/tarpc_service.rs`

**Add to `Cargo.toml`**:

```toml
tarpc = { version = "0.34", features = ["tokio1", "serde-transport", "tcp"] }
tokio-serde = { version = "0.9", features = ["bincode"] }
```

**Implement**:

```rust
use tarpc::{server, context};

#[tarpc::service]
pub trait BearDogService {
    async fn encrypt(request: EncryptRequest) -> Result<EncryptResponse, String>;
    async fn decrypt(request: DecryptRequest) -> Result<DecryptResponse, String>;
    // ... other methods
}

#[derive(Clone)]
pub struct BearDogServiceImpl {
    crypto_service: Arc<dyn CryptoService>,
}

#[tarpc::server]
impl BearDogService for BearDogServiceImpl {
    async fn encrypt(self, _: context::Context, request: EncryptRequest) -> Result<EncryptResponse, String> {
        // Same logic again! Just calls crypto_service.encrypt()
        let data = base64::decode(&request.data)?;
        let encrypted = self.crypto_service.encrypt(&data, request.algorithm, request.options).await?;
        Ok(EncryptResponse { ... })
    }
}

pub async fn start_tarpc_server(crypto_service: Arc<dyn CryptoService>, port: u16) -> Result<()> {
    let server_addr = format!("[::]:{}") port).parse()?;
    let service = BearDogServiceImpl { crypto_service };
    
    tarpc::serde_transport::tcp::listen(&server_addr, tokio_serde::formats::Bincode::default)
        .await?
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .map(|channel| channel.execute(service.clone().serve()))
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;
    
    Ok(())
}
```

**Test**:

```rust
// From Rust client
let client = BearDogServiceClient::connect("localhost:8091").await?;
let encrypted = client.encrypt(request).await?;
// ✅ 50μs latency (vs 2ms for HTTP)!
```

**Output**: ✅ tarpc API working (10-100x faster!)

---

### **Day 10: Protocol Discovery** 🟡 MEDIUM

**Create**: `crates/beardog-api/src/endpoints/protocol.rs`

```rust
/// GET /api/protocols/capabilities
pub async fn get_protocol_capabilities(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ProtocolCapabilities>, BearDogError> {
    Ok(Json(ProtocolCapabilities {
        service: "beardog".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        protocols: Protocols {
            http: ProtocolInfo { /* ... */ },
            json_rpc: Some(ProtocolInfo { /* ... */ }),
            tarpc: Some(ProtocolInfo {
                version: "0.34".to_string(),
                endpoints: HashMap::from([("rpc".to_string(), "tarpc://[::]:8091".to_string())]),
                features: vec!["binary".to_string(), "high-performance".to_string()],
                performance: Some(PerformanceInfo {
                    latency_us: 50,
                    throughput_mbps: 1000,
                }),
            }),
        },
        preferred_protocol: "tarpc".to_string(),
        capabilities: state.crypto_service.get_capabilities().await?,
    }))
}

/// POST /api/protocols/negotiate
pub async fn negotiate_protocol(
    State(state): State<Arc<AppState>>,
    Json(request): Json<NegotiateRequest>,
) -> Result<Json<NegotiateResponse>, BearDogError> {
    // Choose best protocol based on client capabilities
    let chosen = if request.client_protocols.contains(&"tarpc") {
        ("tarpc", "tarpc://[::]:8091")
    } else if request.client_protocols.contains(&"json-rpc") {
        ("json-rpc", "http://[::]:8080/jsonrpc")
    } else {
        ("http", "http://[::]:8080/api/v1/crypto")
    };
    
    Ok(Json(NegotiateResponse {
        upgrade_to: chosen.0.to_string(),
        endpoint: chosen.1.to_string(),
        session_token: generate_session_token()?,
    }))
}
```

**Test**:

```bash
$ curl http://localhost:8080/api/protocols/capabilities | jq
{
  "service": "beardog",
  "protocols": {
    "http": {...},
    "json_rpc": {...},
    "tarpc": {
      "version": "0.34",
      "endpoint": "tarpc://[::]:8091",
      "performance": { "latency_us": 50, "throughput_mbps": 1000 }
    }
  },
  "preferred_protocol": "tarpc"
}
```

**Output**: ✅ Protocol discovery working, clients can negotiate upgrades

---

## ✅ **Success Criteria**

### **After Day 5** (Critical Path Complete):

```bash
# 1. mDNS discovery works
$ avahi-browse -r _beardog._tcp
✅ beardog-northgate found at 192.168.1.100:8080

# 2. HTTP API works
$ curl http://beardog-northgate.local:8080/api/v1/crypto/encrypt -d '{...}'
✅ Real encrypted data returned

# 3. Integration ready
✅ Songbird can discover BearDog
✅ Songbird can call BearDog crypto operations
✅ LAN encrypted compute ready!
```

### **After Day 10** (Full Stack Complete):

```bash
# 4. Protocol escalation works
$ curl http://beardog-northgate.local:8080/api/protocols/capabilities
✅ Shows HTTP, JSON-RPC, tarpc available

# 5. JSON-RPC works
$ curl http://beardog-northgate.local:8080/jsonrpc -d '{"jsonrpc":"2.0",...}'
✅ Universal language-agnostic RPC

# 6. tarpc works (from Rust client)
✅ 50μs latency (10-100x faster than HTTP!)

# 7. Songbird auto-upgrades
✅ Discovers via HTTP → negotiates → upgrades to tarpc automatically
```

---

## 🛠️ **Testing During Development**

### **Test Crypto Service** (Day 1-2):

```rust
#[tokio::test]
async fn test_crypto_service() {
    let service = BearDogCryptoService::new(config)?;
    
    // Test encrypt
    let data = b"Hello World";
    let encrypted = service.encrypt(data, CryptoAlgorithm::Aes256Gcm, EncryptOptions::default()).await?;
    
    // Test decrypt
    let decrypted = service.decrypt(&encrypted, DecryptOptions::default()).await?;
    assert_eq!(decrypted, data);
}
```

### **Test HTTP API** (Day 3-4):

```bash
# Manual test
$ curl http://localhost:8080/api/v1/crypto/encrypt \
    -H "Content-Type: application/json" \
    -d '{"data":"SGVsbG8gV29ybGQ=","algorithm":"Aes256Gcm"}' | jq

# Should return real encrypted data
```

### **Test mDNS** (Day 5):

```bash
$ avahi-browse -r _beardog._tcp
# Should show BearDog service
```

---

## 📊 **File Checklist**

### **Must Create**:
- [ ] `crates/beardog-core/src/crypto_service.rs` - Core trait
- [ ] `crates/beardog-types/src/crypto_service.rs` - Types
- [ ] `crates/beardog-api/src/jsonrpc/mod.rs` - JSON-RPC server
- [ ] `crates/beardog-api/src/tarpc_service.rs` - tarpc server
- [ ] `crates/beardog-api/src/endpoints/protocol.rs` - Protocol discovery

### **Must Modify**:
- [ ] `crates/beardog-api/src/endpoints/crypto.rs` - Replace mocks
- [ ] `crates/beardog-api/src/lib.rs` - Wire mDNS, add servers
- [ ] `crates/beardog-api/Cargo.toml` - Add jsonrpsee, tarpc
- [ ] `crates/beardog-core/Cargo.toml` - Export crypto_service

---

## 🎯 **Bottom Line**

**Critical Path** (Days 1-5):
1. Core trait - protocol-agnostic crypto
2. HTTP API - replace mocks with real implementation
3. mDNS wiring - announce on LAN

**After Day 5**: ✅ BearDog ready for LAN integration with Songbird

**Extended Path** (Days 6-10):
4. JSON-RPC API - universal language-agnostic
5. tarpc API - high-performance binary RPC
6. Protocol discovery - capability advertisement

**After Day 10**: ✅ Full protocol escalation, 10-100x performance boost

---

## 📖 **Full Documentation**

See `BEARDOG_INTEGRATION_IMPLEMENTATION_PLAN.md` for complete details, architecture decisions, and integration points.

---

🐻 **Start with Day 1: Core CryptoService trait!**

