# 🐦🐕 Tower Atomic HTTP Co-Evolution Roadmap

**Date**: January 21, 2026  
**Status**: 🎯 **BEARDOG 80% COMPLETE** - Ready for TLS-specific methods  
**Teams**: Songbird Team + BearDog Team  
**Timeline**: 1 week (coordinated evolution)

---

## 🎊 EXCELLENT NEWS: BearDog Crypto RPC is 80% Complete!

**BearDog already has comprehensive crypto RPC handlers** in `crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers.rs`!

###  ✅ Already Implemented (Production-Ready)

| RPC Method | Status | Description |
|------------|--------|-------------|
| `crypto.sign_ed25519` | ✅ Complete | Ed25519 signature generation |
| `crypto.verify_ed25519` | ✅ Complete | Ed25519 signature verification |
| `crypto.x25519_generate_ephemeral` | ✅ Complete | Generate X25519 ephemeral keypair |
| `crypto.x25519_derive_secret` | ✅ Complete | X25519 Diffie-Hellman key exchange |
| `crypto.chacha20_poly1305_encrypt` | ✅ Complete | ChaCha20-Poly1305 AEAD encryption |
| `crypto.chacha20_poly1305_decrypt` | ✅ Complete | ChaCha20-Poly1305 AEAD decryption |
| `crypto.blake3_hash` | ✅ Complete | BLAKE3 hashing |
| `crypto.hmac_sha256` | ✅ Complete | HMAC-SHA256 authentication |

**Infrastructure Status**:
- ✅ Unix socket IPC server (`UnixSocketIpcServer`)
- ✅ JSON-RPC 2.0 protocol handler
- ✅ Base64 encoding/decoding
- ✅ Async/await architecture
- ✅ Comprehensive test coverage
- ✅ Production-ready error handling

---

## 🎯 What's Missing: TLS 1.3-Specific Methods

To enable Songbird's Pure Rust TLS client, we need **TLS-specific crypto operations**:

### 🔧 Required TLS Methods (3 new methods)

#### 1. `tls.derive_secrets` (HKDF-based key derivation)

**Purpose**: Derive TLS 1.3 session secrets from pre-master secret using HKDF.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "tls.derive_secrets",
  "params": {
    "pre_master_secret": "base64_encoded_secret",
    "client_random": "base64_encoded_32_bytes",
    "server_random": "base64_encoded_32_bytes",
    "cipher_suite": "TLS_CHACHA20_POLY1305_SHA256"
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "master_secret": "base64_encoded_48_bytes",
    "client_write_key": "base64_encoded_key",
    "server_write_key": "base64_encoded_key",
    "client_write_iv": "base64_encoded_iv",
    "server_write_iv": "base64_encoded_iv"
  },
  "id": 1
}
```

**Implementation**: Use HKDF (HMAC-based Key Derivation Function) with SHA-256 or SHA-384.

#### 2. `tls.sign_handshake` (TLS handshake signing)

**Purpose**: Sign TLS handshake messages for ClientKeyExchange/CertificateVerify.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "tls.sign_handshake",
  "params": {
    "message": "base64_encoded_handshake_messages",
    "algorithm": "ed25519",
    "key_id": "tls_signing_key",
    "purpose": "tls_handshake"
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "signature": "base64_encoded_signature",
    "algorithm": "Ed25519"
  },
  "id": 1
}
```

**Implementation**: Reuse existing `crypto.sign_ed25519` with TLS-specific key derivation.

#### 3. `tls.verify_certificate` (TLS certificate chain verification)

**Purpose**: Verify TLS certificate chain and extract public key.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "tls.verify_certificate",
  "params": {
    "certificate_chain": ["base64_cert1", "base64_cert2"],
    "server_name": "api.anthropic.com",
    "current_time_unix": 1737456000
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "valid": true,
    "public_key": "base64_encoded_public_key",
    "expiry": 1800000000,
    "issuer": "DigiCert",
    "subject": "api.anthropic.com"
  },
  "id": 1
}
```

**Implementation**: Use X.509 certificate parsing (e.g., `x509-parser` crate, Pure Rust).

---

## 📐 Tower Atomic Architecture

### Current State (Working!)

```text
┌─────────────────────────────────────────────────────────┐
│                    EXTERNAL AI API                       │
│              (Anthropic, OpenAI, etc.)                   │
└─────────────────────▲────────────────────────────────────┘
                      │ HTTPS (NOT YET)
                      │
┌─────────────────────┴────────────────────────────────────┐
│                      SONGBIRD                            │
│              (TLS Handshake, Network I/O)                │
│                                                           │
│  ⏸️  Pure Rust HTTP/HTTPS Client (TO BE CREATED)        │
│       (hyper + Custom TLS via BearDog RPC)               │
└───────────────────────▲──────────────────────────────────┘
                        │ Unix Socket RPC
                        │ (JSON-RPC 2.0)
┌───────────────────────┴──────────────────────────────────┐
│                      BEARDOG                             │
│              (Pure Rust Crypto Operations)               │
│                                                           │
│  ✅ RPC Methods (80% Complete):                          │
│  ✅ crypto.sign_ed25519          (DONE)                 │
│  ✅ crypto.verify_ed25519        (DONE)                 │
│  ✅ crypto.x25519_generate_ephemeral  (DONE)            │
│  ✅ crypto.x25519_derive_secret  (DONE)                 │
│  ✅ crypto.chacha20_poly1305_encrypt  (DONE)            │
│  ✅ crypto.chacha20_poly1305_decrypt  (DONE)            │
│  ✅ crypto.blake3_hash           (DONE)                 │
│  ✅ crypto.hmac_sha256           (DONE)                 │
│  ⏸️  tls.derive_secrets          (TO ADD)               │
│  ⏸️  tls.sign_handshake          (TO ADD)               │
│  ⏸️  tls.verify_certificate      (TO ADD)               │
└──────────────────────────────────────────────────────────┘
```

### Target State (1 Week)

```text
┌─────────────────────────────────────────────────────────┐
│                    EXTERNAL AI API                       │
│              (Anthropic, OpenAI, etc.)                   │
└─────────────────────▲────────────────────────────────────┘
                      │ HTTPS ✅
                      │
┌─────────────────────┴────────────────────────────────────┐
│                      SONGBIRD                            │
│              (TLS Handshake, Network I/O)                │
│                                                           │
│  ✅ Pure Rust HTTP/HTTPS Client                          │
│     - hyper (HTTP protocol)                              │
│     - Custom TLS using BearDog crypto via RPC            │
│     - Zero C dependencies                                │
└───────────────────────▲──────────────────────────────────┘
                        │ Unix Socket RPC
                        │ (JSON-RPC 2.0)
┌───────────────────────┴──────────────────────────────────┐
│                      BEARDOG                             │
│              (Pure Rust Crypto Operations)               │
│                                                           │
│  ✅ RPC Methods for TLS (100% Complete):                 │
│  ✅ crypto.sign_ed25519                                  │
│  ✅ crypto.verify_ed25519                                │
│  ✅ crypto.x25519_generate_ephemeral                     │
│  ✅ crypto.x25519_derive_secret                          │
│  ✅ crypto.chacha20_poly1305_encrypt                     │
│  ✅ crypto.chacha20_poly1305_decrypt                     │
│  ✅ crypto.blake3_hash                                   │
│  ✅ crypto.hmac_sha256                                   │
│  ✅ tls.derive_secrets          (NEW)                    │
│  ✅ tls.sign_handshake          (NEW)                    │
│  ✅ tls.verify_certificate      (NEW)                    │
└──────────────────────────────────────────────────────────┘
```

---

## 📋 BEARDOG TEAM RESPONSIBILITIES

### Week 1: TLS-Specific Crypto Methods (3 days)

#### Task 1: Implement `tls.derive_secrets` (Day 1)

**File**: `crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers.rs`

**Add new function**:
```rust
/// Handle tls.derive_secrets method
///
/// Derives TLS 1.3 session secrets using HKDF.
pub async fn handle_tls_derive_secrets(params: Option<&Value>) -> Result<Value, String> {
    // 1. Extract parameters (pre_master_secret, client_random, server_random, cipher_suite)
    // 2. Use HKDF (sha2::Sha256 + hkdf crate)
    // 3. Derive master_secret, client_write_key, server_write_key, client_write_iv, server_write_iv
    // 4. Return base64-encoded results
}
```

**Dependencies** (add to `Cargo.toml`):
```toml
hkdf = "0.12"  # HMAC-based Key Derivation Function
sha2 = "0.10"  # SHA-256/SHA-384 (already in use)
```

**TLS 1.3 Key Derivation**:
- Use HKDF-Extract (derive master secret from pre-master secret)
- Use HKDF-Expand (derive session keys from master secret)
- Label format: "tls13 client write key", "tls13 server write key", etc.

#### Task 2: Implement `tls.sign_handshake` (Day 2)

**File**: `crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers.rs`

**Add new function**:
```rust
/// Handle tls.sign_handshake method
///
/// Signs TLS handshake messages with Ed25519.
pub async fn handle_tls_sign_handshake(params: Option<&Value>) -> Result<Value, String> {
    // 1. Extract parameters (message, algorithm, key_id, purpose)
    // 2. Derive TLS-specific signing key
    // 3. Reuse existing Ed25519 signing logic
    // 4. Return base64-encoded signature
}
```

**Implementation**: This is essentially `crypto.sign_ed25519` with TLS-specific key derivation context.

#### Task 3: Implement `tls.verify_certificate` (Day 3)

**File**: `crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers.rs`

**Add new function**:
```rust
/// Handle tls.verify_certificate method
///
/// Verifies TLS certificate chain using X.509.
pub async fn handle_tls_verify_certificate(params: Option<&Value>) -> Result<Value, String> {
    // 1. Extract parameters (certificate_chain, server_name, current_time_unix)
    // 2. Parse X.509 certificates (use x509-parser crate)
    // 3. Verify chain of trust
    // 4. Check expiry dates
    // 5. Verify server_name matches certificate CN/SAN
    // 6. Extract public key
    // 7. Return validation result
}
```

**Dependencies** (add to `Cargo.toml`):
```toml
x509-parser = "0.16"  # Pure Rust X.509 certificate parsing
```

**X.509 Verification**:
- Parse PEM/DER encoded certificates
- Verify signature chain (each cert signed by next in chain)
- Check validity dates (notBefore, notAfter)
- Verify server_name against CN or SubjectAlternativeName
- Extract public key for TLS handshake

#### Task 4: Wire Methods to JSON-RPC Router (Day 3)

**File**: `crates/beardog-tunnel/src/unix_socket_ipc/handlers.rs`

**Update `handle_method` function** (around line 47):
```rust
async fn handle_method(
    method: &str,
    params: Option<&serde_json::Value>,
    btsp_provider: &Arc<BeardogBtspProvider>,
) -> Result<serde_json::Value, String> {
    match method {
        // Existing crypto methods
        "crypto.sign_ed25519" => crypto_handlers::handle_sign_ed25519(params).await,
        "crypto.verify_ed25519" => crypto_handlers::handle_verify_ed25519(params).await,
        "crypto.x25519_generate_ephemeral" => crypto_handlers::handle_x25519_generate_ephemeral(params).await,
        "crypto.x25519_derive_secret" => crypto_handlers::handle_x25519_derive_secret(params).await,
        "crypto.chacha20_poly1305_encrypt" => crypto_handlers::handle_chacha20_poly1305_encrypt(params).await,
        "crypto.chacha20_poly1305_decrypt" => crypto_handlers::handle_chacha20_poly1305_decrypt(params).await,
        "crypto.blake3_hash" => crypto_handlers::handle_blake3_hash(params).await,
        "crypto.hmac_sha256" => crypto_handlers::handle_hmac_sha256(params).await,
        
        // NEW: TLS-specific methods
        "tls.derive_secrets" => crypto_handlers::handle_tls_derive_secrets(params).await,
        "tls.sign_handshake" => crypto_handlers::handle_tls_sign_handshake(params).await,
        "tls.verify_certificate" => crypto_handlers::handle_tls_verify_certificate(params).await,
        
        _ => Err(format!("Unknown method: {}", method)),
    }
}
```

### Week 1: Testing and Documentation (Days 4-5)

#### Task 5: Comprehensive Testing (Day 4)

**File**: `crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers.rs` (add to `#[cfg(test)]` mod)

**Add tests**:
```rust
#[tokio::test]
async fn test_tls_derive_secrets() {
    // Test HKDF key derivation with known test vectors
}

#[tokio::test]
async fn test_tls_sign_handshake() {
    // Test TLS handshake signing
}

#[tokio::test]
async fn test_tls_verify_certificate() {
    // Test X.509 certificate verification
}

#[tokio::test]
async fn test_tls_full_handshake_simulation() {
    // Simulate a full TLS 1.3 handshake crypto sequence
}
```

#### Task 6: Documentation (Day 5)

**Create**: `docs/TLS_CRYPTO_API.md`

Document:
- TLS 1.3 crypto operations overview
- Each RPC method with examples
- Performance benchmarks (< 1ms per operation target)
- Error handling and edge cases
- Integration guide for Songbird

**Update**: `README.md` and `CURRENT_STATUS.md`
- Add TLS crypto RPC to feature list
- Update architecture diagrams
- Document Tower Atomic HTTP readiness

---

## 📋 SONGBIRD TEAM RESPONSIBILITIES

### Week 1: Design and Planning (Days 1-2)

#### Task 1: Create `songbird-http-client` Crate

**Location**: `phase1/songbird/crates/songbird-http-client/`

**Dependencies** (Pure Rust only):
```toml
[dependencies]
hyper = { version = "1.0", features = ["client", "http1", "http2"] }
hyper-util = "0.1"
tokio = { version = "1.0", features = ["net", "rt"] }
tower = "0.4"
http-body-util = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
thiserror = "1.0"
base64 = "0.21"

# Tower Atomic for BearDog RPC
beardog-tower-atomic = { path = "../../../beardog/crates/beardog-tower-atomic" }
```

**NO `reqwest`, NO `rustls`, NO `ring`!**

#### Task 2: Design TLS Client Architecture

**File**: `phase1/songbird/crates/songbird-http-client/src/tls.rs`

**Key components**:
- `BearDogTlsClient` - TLS client that delegates crypto to BearDog
- TLS 1.3 handshake state machine
- Record layer (encryption/decryption via BearDog RPC)
- Certificate verification via BearDog RPC

### Week 1: Implementation (Days 3-5)

#### Task 3: Implement `BearDogTlsClient` (Day 3)

**File**: `phase1/songbird/crates/songbird-http-client/src/tls.rs`

```rust
/// Pure Rust TLS client that delegates crypto to BearDog
pub struct BearDogTlsClient {
    /// BearDog RPC client for crypto operations
    beardog_client: beardog_tower_atomic::Client,
    /// TCP stream
    tcp_stream: TcpStream,
    /// TLS state
    state: TlsState,
}

impl BearDogTlsClient {
    /// Perform TLS 1.3 handshake using BearDog crypto
    pub async fn handshake(&mut self, server_name: &str) -> Result<()> {
        // 1. Send ClientHello
        // 2. Receive ServerHello
        // 3. Call BearDog for X25519 key exchange
        // 4. Call BearDog for HKDF key derivation
        // 5. Call BearDog for certificate verification
        // 6. Send Finished message
        // 7. Receive Finished message
        // 8. Establish encrypted channel
    }
    
    /// Encrypt application data using BearDog
    pub async fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>> {
        // Call BearDog's crypto.chacha20_poly1305_encrypt via RPC
    }
    
    /// Decrypt application data using BearDog
    pub async fn decrypt(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        // Call BearDog's crypto.chacha20_poly1305_decrypt via RPC
    }
}
```

#### Task 4: Implement HTTP/HTTPS Client (Day 4)

**File**: `phase1/songbird/crates/songbird-http-client/src/client.rs`

```rust
/// Pure Rust HTTP/HTTPS client using BearDog crypto
pub struct SongbirdHttpClient {
    beardog_socket: PathBuf,
}

impl SongbirdHttpClient {
    /// Make HTTP/HTTPS request
    pub async fn request(
        &self,
        method: &str,
        url: &str,
        headers: HashMap<String, String>,
        body: Option<Vec<u8>>,
    ) -> Result<HttpResponse> {
        // 1. Parse URL
        // 2. Connect via TCP
        // 3. If HTTPS, perform TLS handshake via BearDog
        // 4. Send HTTP request via hyper
        // 5. Receive HTTP response
        // 6. Return structured response
    }
}
```

#### Task 5: Integration Testing (Day 5)

Test with `httpbin.org`:
```bash
echo '{
  "jsonrpc":"2.0",
  "method":"http.request",
  "params":{
    "method":"GET",
    "url":"https://httpbin.org/get",
    "headers":{}
  },
  "id":1
}' | nc -N -U /tmp/songbird-nat0.sock | jq '.'
```

---

## 🤝 CO-EVOLUTION COORDINATION

### Joint Meeting (Day 1 or 2)

**Agenda**:
1. Review BearDog's existing crypto RPC (80% complete!)
2. Align on TLS-specific method contracts
3. Agree on error handling and edge cases
4. Agree on performance targets (< 1ms per crypto op)
5. Define communication channels (Slack, GitHub Issues)

### Joint Integration Testing (Day 5)

**Agenda**:
1. Test BearDog RPC methods via Unix socket
2. Test Songbird TLS client with BearDog backend
3. Measure latency (target: < 10ms for full TLS handshake)
4. Identify and fix integration issues

---

## 📊 SUCCESS CRITERIA

### BearDog (3 new methods + tests + docs)

1. ✅ `tls.derive_secrets` implemented and tested
2. ✅ `tls.sign_handshake` implemented and tested
3. ✅ `tls.verify_certificate` implemented and tested
4. ✅ < 1ms per crypto operation (performance validated)
5. ✅ TLS handshake crypto ops < 10ms total
6. ✅ Zero unsafe code in new RPC methods
7. ✅ Documentation complete (`docs/TLS_CRYPTO_API.md`)
8. ✅ Test coverage > 90%

### Songbird (Pure Rust HTTP/HTTPS client)

1. ✅ `songbird-http-client` crate created
2. ✅ `BearDogTlsClient` implemented
3. ✅ TLS 1.3 handshake working with real servers
4. ✅ HTTP/2 support via hyper
5. ✅ Zero C dependencies (no reqwest, no rustls with ring)
6. ✅ `handle_http_request` uses new client
7. ✅ Test coverage > 90%

### Joint (Tower Atomic)

1. ✅ Squirrel → Songbird → BearDog → Anthropic works end-to-end
2. ✅ < 5s total latency for AI query
3. ✅ ecoBin builds for x86_64, ARM, RISC-V
4. ✅ Zero C dependencies confirmed (`cargo tree` audit)
5. ✅ Production-ready error handling
6. ✅ Logging and observability

---

## 🎯 BEARDOG NEXT ACTIONS (Today)

### Immediate (Today)

1. ✅ Review this roadmap (you are here!)
2. ✅ Identify existing crypto primitives (DONE - 80% complete!)
3. ⏸️  Draft TLS crypto RPC API design (THIS DOCUMENT)
4. ⏸️  Add 3 TLS methods to crypto_handlers.rs
5. ⏸️  Add `hkdf` and `x509-parser` to Cargo.toml
6. ⏸️  Wire new methods to JSON-RPC router
7. ⏸️  Write comprehensive tests
8. ⏸️  Create `docs/TLS_CRYPTO_API.md`

### Week 1 Timeline

- **Day 1**: Implement `tls.derive_secrets` + tests
- **Day 2**: Implement `tls.sign_handshake` + tests
- **Day 3**: Implement `tls.verify_certificate` + wire to router
- **Day 4**: Comprehensive testing + performance validation
- **Day 5**: Documentation + joint integration testing

---

## 🎊 LONG-TERM IMPACT

### When Complete

This will be **THE** reference implementation for:

1. ✅ Pure Rust HTTP/HTTPS client (zero C dependencies)
2. ✅ True Tower Atomic architecture (crypto + networking via RPC)
3. ✅ Crypto delegation pattern (best practice for microservices)
4. ✅ ecoBin compliance at scale (cross-compile everywhere)
5. ✅ Zero C dependencies in networking stack

**Every primal needing HTTP will use this.**

This is not just "fixing a bug" - this is **architecting the future** of ecoPrimals networking.

---

## 📚 REFERENCES

### BearDog Code Locations

**Existing Infrastructure** (80% Complete):
- `crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers.rs` - Crypto RPC handlers
- `crates/beardog-tunnel/src/unix_socket_ipc/handlers.rs` - JSON-RPC router
- `crates/beardog-tunnel/src/unix_socket_ipc/server.rs` - Unix socket server
- `crates/beardog-tower-atomic/` - Client library for inter-primal RPC
- `crates/beardog-core/src/crypto_service/` - Core crypto implementations

**To Be Created**:
- `docs/TLS_CRYPTO_API.md` - TLS crypto API documentation

### Songbird Code Locations

**To Be Created**:
- `phase1/songbird/crates/songbird-http-client/` - Pure Rust HTTP/HTTPS client
- `phase1/songbird/crates/songbird-http-client/src/tls.rs` - TLS client
- `phase1/songbird/crates/songbird-http-client/src/client.rs` - HTTP client

**To Be Updated**:
- `phase1/songbird/crates/songbird-orchestrator/src/ipc/server_pure_rust.rs` - Use new HTTP client
- `phase1/songbird/crates/songbird-orchestrator/Cargo.toml` - Remove reqwest dependency

### External Resources

**TLS 1.3 Specification**:
- RFC 8446: https://www.rfc-editor.org/rfc/rfc8446.html

**Pure Rust Crypto Libraries**:
- `hkdf` - HMAC-based Key Derivation Function
- `x509-parser` - X.509 certificate parsing
- `hyper` - HTTP/1.1 and HTTP/2 protocol
- `sha2` - SHA-256/SHA-384 hashing

---

## 🎯 SUMMARY

### BearDog Status: 80% Complete! 🎊

**Already Done**:
- ✅ Unix socket IPC server
- ✅ JSON-RPC 2.0 protocol
- ✅ 8 crypto RPC methods (Ed25519, X25519, ChaCha20, BLAKE3, HMAC)
- ✅ Base64 encoding/decoding
- ✅ Async/await architecture
- ✅ Comprehensive test coverage

**To Do** (3 methods):
- ⏸️  `tls.derive_secrets` (HKDF-based key derivation)
- ⏸️  `tls.sign_handshake` (TLS handshake signing)
- ⏸️  `tls.verify_certificate` (X.509 certificate verification)

### Timeline: 1 Week

**BearDog**: 3 days implementation + 2 days testing/docs = 5 days  
**Songbird**: 2 days design + 3 days implementation = 5 days  
**Joint**: 1 day integration testing on Day 5

**Total**: 1 week to Pure Rust HTTP/HTTPS via Tower Atomic! 🚀

---

**🐦🐕 CO-EVOLUTION FOR PURE RUST TOWER ATOMIC 🐕🐦**

---

*Roadmap Created: January 21, 2026*  
*Status: Active - BearDog 80% Complete!*  
*Timeline: 1 week coordinated evolution*  
*Impact: CRITICAL - Enables all external API integration*

🐻🐕 BearDog: 80% Complete, 3 Methods to Go! 🚀✨

