# 🐻 BearDog Integration Implementation Plan
**Focus**: What BearDog needs to complete for encrypted compute integration  
**Protocol Support**: HTTP → JSON-RPC → tarpc (matching Songbird's escalation system)

**Date**: December 15, 2025  
**Status**: Ready to implement

---

## 🎯 **Overview**

BearDog needs to expose crypto operations via **three protocols** (matching Songbird):

1. **HTTP/REST** - Universal initial connection (Phase 1)
2. **JSON-RPC** - Language-agnostic RPC (Phase 2) 
3. **tarpc** - High-performance binary RPC for Rust clients (Phase 3)

**Songbird Pattern** (already implemented):
```
Client connects via HTTP (port 8080)
  → Discovers capabilities at /api/protocols/capabilities
  → Negotiates protocol upgrade
  → Establishes JSON-RPC (port 8080/jsonrpc) or tarpc (port 8091)
```

**BearDog Will Match This**:
```
Client connects via HTTP (port 8080)
  → Discovers BearDog capabilities
  → Negotiates protocol upgrade
  → Establishes JSON-RPC or tarpc for high-performance crypto
```

---

## 📋 **BearDog Implementation Tasks**

### **PHASE 1: Core Crypto Trait** (Foundation) 🔴 **CRITICAL**

**Goal**: Protocol-agnostic crypto operations

**File**: `crates/beardog-core/src/crypto_service.rs` (NEW)

```rust
/// Protocol-agnostic crypto operations
/// Can be exposed via HTTP, JSON-RPC, tarpc, or any future protocol
#[async_trait]
pub trait CryptoService: Send + Sync {
    /// Encrypt data with specified algorithm
    async fn encrypt(
        &self,
        data: &[u8],
        algorithm: CryptoAlgorithm,
        options: EncryptOptions,
    ) -> Result<EncryptedData>;

    /// Decrypt data
    async fn decrypt(
        &self,
        encrypted: &EncryptedData,
        options: DecryptOptions,
    ) -> Result<Vec<u8>>;

    /// Sign data
    async fn sign(
        &self,
        data: &[u8],
        algorithm: SignatureAlgorithm,
        options: SignOptions,
    ) -> Result<Signature>;

    /// Verify signature
    async fn verify(
        &self,
        data: &[u8],
        signature: &Signature,
        options: VerifyOptions,
    ) -> Result<bool>;

    /// Generate key
    async fn generate_key(
        &self,
        algorithm: KeyAlgorithm,
        options: KeyGenOptions,
    ) -> Result<KeyInfo>;

    /// Get service capabilities
    async fn get_capabilities(&self) -> Result<ServiceCapabilities>;
}

/// Implementation using beardog-core
pub struct BearDogCryptoService {
    crypto_provider: Arc<dyn CryptoProvider>,
    hsm_provider: Option<Arc<dyn HsmProvider>>,
    config: CryptoServiceConfig,
}

impl CryptoService for BearDogCryptoService {
    async fn encrypt(&self, data: &[u8], algorithm: CryptoAlgorithm, options: EncryptOptions) -> Result<EncryptedData> {
        // Use beardog-core crypto provider
        let encrypted = self.crypto_provider.encrypt(data, &algorithm.to_core_algorithm())?;
        
        Ok(EncryptedData {
            ciphertext: encrypted,
            algorithm: algorithm,
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

**Types** (`crates/beardog-types/src/crypto_service.rs` - NEW):

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

// Similar types for Sign/Verify operations...
```

**Estimated Work**: **8-10 hours**

---

### **PHASE 2: HTTP API Endpoints** (Universal Access) 🔴 **CRITICAL**

**Goal**: Expose `CryptoService` via HTTP/REST

**File**: `crates/beardog-api/src/endpoints/crypto.rs` (REPLACE MOCKS)

```rust
use axum::{
    extract::State,
    http::StatusCode,
    routing::{post, get},
    Json, Router,
};
use beardog_core::crypto_service::{CryptoService, BearDogCryptoService};

/// Crypto API routes
pub fn crypto_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/encrypt", post(encrypt_handler))
        .route("/decrypt", post(decrypt_handler))
        .route("/sign", post(sign_handler))
        .route("/verify", post(verify_handler))
        .route("/generate-key", post(generate_key_handler))
        .route("/capabilities", get(get_capabilities_handler))
}

/// Shared state with crypto service
pub struct AppState {
    pub crypto_service: Arc<dyn CryptoService>,
}

/// POST /api/v1/crypto/encrypt
async fn encrypt_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<EncryptRequest>,
) -> Result<Json<EncryptResponse>, BearDogError> {
    // Decode base64 input
    let data = base64::decode(&request.data)
        .map_err(|e| BearDogError::validation(format!("Invalid base64: {}", e)))?;
    
    // Call crypto service (protocol-agnostic!)
    let encrypted = state
        .crypto_service
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

// Similar handlers for decrypt, sign, verify, generate_key...

/// GET /api/v1/crypto/capabilities
async fn get_capabilities_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ServiceCapabilities>, BearDogError> {
    let caps = state.crypto_service.get_capabilities().await?;
    Ok(Json(caps))
}
```

**Request/Response Types** (`crates/beardog-api/src/types.rs`):

```rust
#[derive(Debug, Deserialize)]
pub struct EncryptRequest {
    pub data: String,  // base64-encoded
    pub algorithm: CryptoAlgorithm,
    pub key_id: Option<String>,
    pub associated_data: Option<String>,  // base64-encoded
}

#[derive(Debug, Serialize)]
pub struct EncryptResponse {
    pub encrypted_data: String,  // base64-encoded
    pub algorithm: CryptoAlgorithm,
    pub metadata: EncryptionMetadata,
}

// Similar for Decrypt, Sign, Verify requests/responses...
```

**Estimated Work**: **10-12 hours**

---

### **PHASE 3: JSON-RPC API** (Language-Agnostic RPC) 🟠 **HIGH**

**Goal**: Expose `CryptoService` via JSON-RPC 2.0 (for non-Rust clients)

**File**: `crates/beardog-api/src/jsonrpc/mod.rs` (NEW)

```rust
use jsonrpsee::server::{Server, ServerBuilder};
use jsonrpsee::proc_macros::rpc;
use beardog_core::crypto_service::CryptoService;

/// JSON-RPC 2.0 interface for BearDog crypto operations
#[rpc(server)]
pub trait BearDogRpc {
    /// Encrypt data
    #[method(name = "beardog.encrypt")]
    async fn encrypt(&self, request: EncryptRequest) -> Result<EncryptResponse, ErrorObject>;

    /// Decrypt data
    #[method(name = "beardog.decrypt")]
    async fn decrypt(&self, request: DecryptRequest) -> Result<DecryptResponse, ErrorObject>;

    /// Sign data
    #[method(name = "beardog.sign")]
    async fn sign(&self, request: SignRequest) -> Result<SignResponse, ErrorObject>;

    /// Verify signature
    #[method(name = "beardog.verify")]
    async fn verify(&self, request: VerifyRequest) -> Result<VerifyResponse, ErrorObject>;

    /// Get capabilities
    #[method(name = "beardog.getCapabilities")]
    async fn get_capabilities(&self) -> Result<ServiceCapabilities, ErrorObject>;
}

/// JSON-RPC server implementation
pub struct BearDogRpcServer {
    crypto_service: Arc<dyn CryptoService>,
}

#[async_trait]
impl BearDogRpcServer for BearDogRpcServerImpl {
    async fn encrypt(&self, request: EncryptRequest) -> Result<EncryptResponse, ErrorObject> {
        // Decode base64
        let data = base64::decode(&request.data)
            .map_err(|e| ErrorObject::owned(-32602, format!("Invalid base64: {}", e), None::<()>))?;
        
        // Call crypto service (same trait as HTTP!)
        let encrypted = self.crypto_service
            .encrypt(&data, request.algorithm, request.options.unwrap_or_default())
            .await
            .map_err(|e| ErrorObject::owned(-32603, e.to_string(), None::<()>))?;
        
        Ok(EncryptResponse {
            encrypted_data: base64::encode(&encrypted.ciphertext),
            algorithm: encrypted.algorithm,
            metadata: encrypted.metadata,
        })
    }
    
    // ... implement other methods (same logic as HTTP handlers)
}

/// Start JSON-RPC server
pub async fn start_jsonrpc_server(
    crypto_service: Arc<dyn CryptoService>,
    port: u16,
) -> Result<ServerHandle> {
    let server = ServerBuilder::default()
        .build(format!("[::]:{}") port)
        .await?;
    
    let rpc_impl = BearDogRpcServerImpl { crypto_service };
    let handle = server.start(rpc_impl.into_rpc())?;
    
    info!("🔐 BearDog JSON-RPC server listening on port {}", port);
    Ok(handle)
}
```

**Add to `Cargo.toml`**:

```toml
[dependencies]
jsonrpsee = { version = "0.21", features = ["server", "macros"] }
```

**Estimated Work**: **6-8 hours**

---

### **PHASE 4: tarpc API** (High-Performance Binary RPC) 🟠 **HIGH**

**Goal**: Expose `CryptoService` via tarpc (for Rust-to-Rust high performance)

**File**: `crates/beardog-api/src/tarpc_service.rs` (NEW)

```rust
use tarpc::{server, context};
use beardog_core::crypto_service::CryptoService;

/// tarpc service definition
#[tarpc::service]
pub trait BearDogService {
    /// Encrypt data
    async fn encrypt(request: EncryptRequest) -> Result<EncryptResponse, String>;
    
    /// Decrypt data
    async fn decrypt(request: DecryptRequest) -> Result<DecryptResponse, String>;
    
    /// Sign data
    async fn sign(request: SignRequest) -> Result<SignResponse, String>;
    
    /// Verify signature
    async fn verify(request: VerifyRequest) -> Result<VerifyResponse, String>;
    
    /// Get capabilities
    async fn get_capabilities() -> Result<ServiceCapabilities, String>;
}

/// tarpc server implementation
#[derive(Clone)]
pub struct BearDogServiceImpl {
    crypto_service: Arc<dyn CryptoService>,
}

#[tarpc::server]
impl BearDogService for BearDogServiceImpl {
    async fn encrypt(self, _: context::Context, request: EncryptRequest) -> Result<EncryptResponse, String> {
        // Decode base64
        let data = base64::decode(&request.data)
            .map_err(|e| format!("Invalid base64: {}", e))?;
        
        // Call crypto service (same trait again!)
        let encrypted = self.crypto_service
            .encrypt(&data, request.algorithm, request.options.unwrap_or_default())
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(EncryptResponse {
            encrypted_data: base64::encode(&encrypted.ciphertext),
            algorithm: encrypted.algorithm,
            metadata: encrypted.metadata,
        })
    }
    
    // ... implement other methods (same logic as HTTP/JSON-RPC!)
}

/// Start tarpc server
pub async fn start_tarpc_server(
    crypto_service: Arc<dyn CryptoService>,
    port: u16,
) -> Result<()> {
    let server_addr = format!("[::]:{}") port).parse()?;
    
    let service = BearDogServiceImpl { crypto_service };
    
    tarpc::serde_transport::tcp::listen(&server_addr, tokio_serde::formats::Bincode::default)
        .await?
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = service.clone();
            channel.execute(server.serve())
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;
    
    Ok(())
}
```

**Add to `Cargo.toml`**:

```toml
[dependencies]
tarpc = { version = "0.34", features = ["tokio1", "serde-transport", "tcp"] }
tokio-serde = { version = "0.9", features = ["bincode"] }
```

**Estimated Work**: **8-10 hours**

---

### **PHASE 5: Protocol Discovery** (Capability Advertisement) 🟡 **MEDIUM**

**Goal**: Advertise which protocols BearDog supports

**File**: `crates/beardog-api/src/endpoints/protocol.rs` (NEW)

```rust
/// Protocol capability discovery (matches Songbird pattern)
pub fn protocol_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/capabilities", get(get_protocol_capabilities))
        .route("/negotiate", post(negotiate_protocol))
}

/// GET /api/protocols/capabilities
async fn get_protocol_capabilities(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ProtocolCapabilities>, BearDogError> {
    Ok(Json(ProtocolCapabilities {
        service: "beardog".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        protocols: Protocols {
            http: ProtocolInfo {
                version: "1.1".to_string(),
                endpoints: HashMap::from([
                    ("crypto".to_string(), "http://[::]:8080/api/v1/crypto".to_string()),
                    ("session".to_string(), "http://[::]:8080/api/v1/session".to_string()),
                ]),
                features: vec!["rest".to_string(), "universal".to_string()],
                performance: None,
            },
            json_rpc: Some(ProtocolInfo {
                version: "2.0".to_string(),
                endpoints: HashMap::from([
                    ("rpc".to_string(), "http://[::]:8080/jsonrpc".to_string()),
                ]),
                features: vec!["language-agnostic".to_string(), "simple".to_string()],
                performance: Some(PerformanceInfo {
                    latency_us: 2000,
                    throughput_mbps: 100,
                }),
            }),
            tarpc: Some(ProtocolInfo {
                version: "0.34".to_string(),
                endpoints: HashMap::from([
                    ("rpc".to_string(), "tarpc://[::]:8091".to_string()),
                ]),
                features: vec!["binary".to_string(), "high-performance".to_string()],
                performance: Some(PerformanceInfo {
                    latency_us: 50,      // ~50μs latency
                    throughput_mbps: 1000,  // 1 Gbps+
                }),
            }),
        },
        preferred_protocol: "tarpc".to_string(),  // For Rust clients
        capabilities: state.crypto_service.get_capabilities().await?,
    }))
}

/// POST /api/protocols/negotiate
async fn negotiate_protocol(
    State(state): State<Arc<AppState>>,
    Json(request): Json<NegotiateRequest>,
) -> Result<Json<NegotiateResponse>, BearDogError> {
    // Client says what protocols they support
    let client_protocols = request.client_protocols;
    
    // Prioritize: tarpc (fastest) > json-rpc (universal) > http (fallback)
    let chosen_protocol = if client_protocols.contains(&"tarpc".to_string()) {
        "tarpc"
    } else if client_protocols.contains(&"json-rpc".to_string()) {
        "json-rpc"
    } else {
        "http"
    };
    
    Ok(Json(NegotiateResponse {
        upgrade_to: chosen_protocol.to_string(),
        endpoint: get_endpoint_for_protocol(chosen_protocol),
        session_token: generate_session_token()?,
    }))
}
```

**Estimated Work**: **4-6 hours**

---

### **PHASE 6: mDNS Advertisement** (LAN Discovery) 🟡 **MEDIUM**

**Goal**: Announce BearDog on LAN (code already exists, just wire it up)

**File**: `crates/beardog-api/src/lib.rs` (MODIFY)

```rust
use beardog_core::primal_discovery_mdns::MdnsDiscoveryClient;

/// Start BearDog API server with mDNS announcement
pub async fn start_beardog_server(config: BearDogConfig) -> Result<()> {
    // 1. Create crypto service
    let crypto_service = Arc::new(BearDogCryptoService::new(config.crypto)?);
    
    // 2. Start HTTP server (port 8080)
    let http_server = start_http_server(crypto_service.clone(), 8080);
    
    // 3. Start JSON-RPC server (port 8080/jsonrpc)
    let jsonrpc_server = start_jsonrpc_server(crypto_service.clone(), 8080);
    
    // 4. Start tarpc server (port 8091)
    let tarpc_server = start_tarpc_server(crypto_service.clone(), 8091);
    
    // 5. Announce via mDNS (CODE ALREADY EXISTS!)
    let mdns_client = MdnsDiscoveryClient::new()?;
    mdns_client.announce_self(
        &config.primal_name,  // e.g. "beardog-northgate"
        vec![
            "crypto.encryption.aes256gcm",
            "crypto.signing.ed25519",
            "crypto.hashing.sha256",
            "hsm.solokeys",
            "genetics.self_enforcing_keys",
            "protocol.http",
            "protocol.jsonrpc",
            "protocol.tarpc",
        ],
        8080,  // Primary HTTP port
        HashMap::from([
            ("version".to_string(), env!("CARGO_PKG_VERSION").to_string()),
            ("hsm_available".to_string(), "true".to_string()),
            ("tarpc_port".to_string(), "8091".to_string()),
        ]),
    ).await?;
    
    info!("🐻 BearDog announced on mDNS: _beardog._tcp.local.");
    
    // 6. Run all servers
    tokio::try_join!(http_server, jsonrpc_server, tarpc_server)?;
    
    Ok(())
}
```

**Estimated Work**: **2-3 hours** (mostly wiring existing code)

---

## 📊 **Implementation Summary**

| Phase | Component | Estimated Work | Priority | Protocols Enabled |
|-------|-----------|----------------|----------|-------------------|
| **1** | Core Crypto Trait | 8-10 hours | 🔴 CRITICAL | Foundation for all |
| **2** | HTTP API | 10-12 hours | 🔴 CRITICAL | HTTP/REST |
| **3** | JSON-RPC API | 6-8 hours | 🟠 HIGH | JSON-RPC 2.0 |
| **4** | tarpc API | 8-10 hours | 🟠 HIGH | tarpc (binary RPC) |
| **5** | Protocol Discovery | 4-6 hours | 🟡 MEDIUM | Capability advertisement |
| **6** | mDNS Wiring | 2-3 hours | 🟡 MEDIUM | LAN discovery |

**Total Estimated Work**: **38-49 hours** (~1 week focused work)

**Critical Path**: Phase 1 → Phase 2 → Phase 6 (minimal: 20-25 hours)  
**Full Stack**: All phases (38-49 hours)

---

## 🚀 **Protocol Escalation Flow**

### **Client Discovery & Upgrade** (Songbird calling BearDog)

```rust
// 1. Songbird discovers BearDog via mDNS
let beardog = songbird.discover_service("_beardog._tcp.local.").await?;
// Found: beardog-northgate at 192.168.1.100:8080

// 2. Songbird checks BearDog's protocol capabilities
let caps = http_client
    .get("http://192.168.1.100:8080/api/protocols/capabilities")
    .send()
    .await?;
// Response: { protocols: ["http", "json-rpc", "tarpc"], preferred: "tarpc" }

// 3. Songbird negotiates protocol upgrade
let negotiate = http_client
    .post("http://192.168.1.100:8080/api/protocols/negotiate")
    .json(&NegotiateRequest {
        client_protocols: vec!["http", "tarpc"],
        preferred: "tarpc",
    })
    .send()
    .await?;
// Response: { upgrade_to: "tarpc", endpoint: "tarpc://192.168.1.100:8091", token: "..." }

// 4. Songbird establishes tarpc connection (10-100x faster!)
let tarpc_client = BearDogServiceClient::connect(
    "192.168.1.100:8091",
    negotiate.session_token,
).await?;

// 5. Songbird uses tarpc for high-performance crypto
let encrypted = tarpc_client
    .encrypt(EncryptRequest { ... })
    .await?;
// ~50μs latency vs ~2ms for HTTP!
```

---

## 🎯 **Integration Points**

### **Songbird → BearDog**

**Songbird Side** (separate TODO for Songbird team):
```rust
// Songbird will implement:
pub struct BearDogClient {
    http_client: HttpClient,
    tarpc_client: Option<TarpcClient>,
    endpoint: String,
}

impl BearDogClient {
    // 1. Discover via mDNS
    pub async fn discover() -> Result<Self>;
    
    // 2. Negotiate protocol
    pub async fn negotiate_protocol(&mut self) -> Result<()>;
    
    // 3. Call crypto ops (auto-selects best protocol)
    pub async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
    pub async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
}
```

**BearDog Side** (this document):
```rust
// BearDog exposes:
✅ HTTP API (Phase 2)
✅ JSON-RPC API (Phase 3)
✅ tarpc API (Phase 4)
✅ mDNS advertisement (Phase 6)
✅ Protocol discovery (Phase 5)
```

---

## 💡 **Key Architectural Decisions**

### **1. Trait-Based Abstraction** ✅

**Single `CryptoService` trait** used by all protocols:
- HTTP calls `crypto_service.encrypt()`
- JSON-RPC calls `crypto_service.encrypt()`
- tarpc calls `crypto_service.encrypt()`

**Benefit**: Logic implemented once, exposed three ways

---

### **2. Protocol Parity** ✅

All three protocols expose the **same operations**:
- `encrypt()`, `decrypt()`, `sign()`, `verify()`, `generate_key()`, `get_capabilities()`

**Benefit**: Clients can switch protocols without changing logic

---

### **3. Performance Tiers** ✅

| Protocol | Latency | Throughput | Use Case |
|----------|---------|------------|----------|
| **HTTP** | ~2ms | ~100 Mbps | Initial discovery, universal access |
| **JSON-RPC** | ~1ms | ~500 Mbps | Language-agnostic clients (Python, JS) |
| **tarpc** | ~50μs | ~1 Gbps | Rust-to-Rust high-performance |

**Benefit**: Start universal (HTTP), upgrade to fast (tarpc) automatically

---

### **4. Sovereignty Preserved** ✅

- ❌ Songbird does NOT import `beardog-core` crate
- ✅ Songbird calls BearDog via HTTP/JSON-RPC/tarpc
- ✅ BearDog remains independent
- ✅ Can swap BearDog for another crypto provider

---

## 🛠️ **Recommended Implementation Order**

### **Week 1: Critical Path** (LAN Working)

**Day 1-2**: Phase 1 (Core Crypto Trait)  
**Day 3-4**: Phase 2 (HTTP API - replace mocks)  
**Day 5**: Phase 6 (mDNS wiring)

**Deliverable**: BearDog discoverable on LAN, HTTP API working

---

### **Week 2: Protocol Escalation** (High Performance)

**Day 1-2**: Phase 3 (JSON-RPC API)  
**Day 3-4**: Phase 4 (tarpc API)  
**Day 5**: Phase 5 (Protocol discovery)

**Deliverable**: Full protocol escalation working, 10-100x performance boost for Rust clients

---

### **Week 3: Integration & Testing** (Full Stack)

**Day 1-3**: Songbird integration (Songbird team's work)  
**Day 4**: ToadStool integration (optional)  
**Day 5**: Full stack demo & documentation

**Deliverable**: Complete encrypted compute system

---

## ✅ **Success Criteria**

### **Phase 1-2 Complete** (HTTP API):
```bash
# From any client
$ curl http://beardog-northgate.local:8080/api/v1/crypto/encrypt \
    -H "Content-Type: application/json" \
    -d '{"data":"SGVsbG8=","algorithm":"Aes256Gcm"}'

{"encrypted_data":"...", "algorithm":"Aes256Gcm", "metadata":{...}}
```

### **Phase 3 Complete** (JSON-RPC):
```bash
# From any language (Python, JS, etc.)
$ curl http://beardog-northgate.local:8080/jsonrpc \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"beardog.encrypt","params":{...},"id":1}'

{"jsonrpc":"2.0","result":{...},"id":1}
```

### **Phase 4 Complete** (tarpc):
```rust
// From Rust client (Songbird)
let client = BearDogServiceClient::connect("beardog-northgate.local:8091").await?;
let encrypted = client.encrypt(request).await?;
// ✅ 50μs latency!
```

### **Phase 6 Complete** (mDNS):
```bash
# Discovery
$ avahi-browse -r _beardog._tcp

_beardog._tcp.local.
   hostname = beardog-northgate.local
   address = 192.168.1.100
   port = 8080
   txt = ["version=0.9.0" "hsm_available=true" "tarpc_port=8091"]
```

---

## 📚 **Files to Create/Modify**

### **New Files**:
```
crates/beardog-core/src/crypto_service.rs          # Core trait
crates/beardog-types/src/crypto_service.rs         # Types
crates/beardog-api/src/jsonrpc/mod.rs              # JSON-RPC server
crates/beardog-api/src/tarpc_service.rs            # tarpc server
crates/beardog-api/src/endpoints/protocol.rs       # Protocol discovery
```

### **Modified Files**:
```
crates/beardog-api/src/endpoints/crypto.rs         # Replace mocks with real impl
crates/beardog-api/src/lib.rs                      # Wire mDNS, add routes
crates/beardog-api/Cargo.toml                      # Add jsonrpsee, tarpc
crates/beardog-core/Cargo.toml                     # Update exports
```

---

## 🎯 **Bottom Line**

**What BearDog Needs**:
1. ✅ **Core trait** (8-10 hours) - Protocol-agnostic crypto ops
2. ✅ **HTTP API** (10-12 hours) - Replace mocks with real implementation
3. ✅ **JSON-RPC API** (6-8 hours) - Universal language-agnostic RPC
4. ✅ **tarpc API** (8-10 hours) - High-performance binary RPC
5. ✅ **Protocol discovery** (4-6 hours) - Capability advertisement
6. ✅ **mDNS wiring** (2-3 hours) - LAN discovery (code exists!)

**Total**: ~38-49 hours (~1 week)

**Critical Path** (minimal): Phases 1+2+6 = ~20-25 hours (2-3 days)

**After this**: Songbird can discover BearDog, negotiate protocol, and use tarpc for 10-100x faster crypto operations!

🐻🎵 **BearDog will match Songbird's protocol escalation system perfectly!**

