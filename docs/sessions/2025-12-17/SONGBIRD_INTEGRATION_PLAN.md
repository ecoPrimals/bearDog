# 🎵🐻 Songbird ↔ BearDog Integration Plan

**Status**: In Progress  
**Priority**: P0 - Critical Path Blocker  
**Timeline**: 1-2 weeks

---

## 🎯 Integration Goals

### Phase 1: Network API (Week 1) - **IN PROGRESS**
- ✅ HTTP API server exists (Axum-based)
- ✅ Crypto endpoints operational (`/api/v1/crypto/*`)
- ✅ Service discovery ready (mDNS)
- 🔨 Add generic encrypt/decrypt endpoints
- 🔨 Add key management API
- 🔨 Document for cross-primal use

### Phase 2: Songbird Client (Week 2)
- 🔨 Songbird discovers BearDog via mDNS
- 🔨 Songbird HTTP client for BearDog API
- 🔨 Graceful fallback if BearDog unavailable
- 🔨 Integration tests

---

## 📋 Current Status Assessment

### ✅ What Already Exists in BearDog

**HTTP API Server** (`crates/beardog-api`):
```rust
// Already implemented!
- Axum-based HTTP server
- Health endpoints: /health, /status
- Crypto endpoints:
  - POST /api/v1/crypto/aes-gcm/encrypt
  - POST /api/v1/crypto/aes-gcm/decrypt  
  - POST /api/v1/crypto/ed25519/sign
  - POST /api/v1/crypto/ed25519/verify
- Capability discovery: /api/v1/capabilities
- JSON-RPC endpoint: /rpc
- CORS support
- Service discovery (mDNS) ready
```

**Configuration** (`crates/beardog-api/src/startup.rs`):
```rust
// Zero hardcoding - runtime discovery!
BearDogApiServer::builder()
    .discover_ports()  // ✅ No hardcoded ports
    .enable_mdns(true) // ✅ Service advertisement
    .build()
```

**Crypto Service** (`crates/beardog-core`):
```rust
// Full cryptographic operations
- AES-256-GCM encryption/decryption
- Ed25519 signing/verification
- Key generation
- Entropy collection
```

---

## 🔨 What Needs to Be Added

### 1. Generic Crypto API Endpoints

**New Endpoints Needed**:
```rust
// Generic endpoints (algorithm-agnostic)
POST /api/v1/encrypt       // Auto-select best algorithm
POST /api/v1/decrypt       // Auto-detect algorithm from payload
POST /api/v1/sign         // Auto-select signing algorithm
POST /api/v1/verify       // Auto-detect signature type

// Key management
POST /api/v1/keys/generate     // Generate new key pair
GET  /api/v1/keys/{id}         // Get public key
POST /api/v1/keys/{id}/derive  // Derive child key
DELETE /api/v1/keys/{id}       // Delete key (secure wipe)

// Entropy
GET  /api/v1/entropy           // Get high-quality entropy
POST /api/v1/entropy/collect   // Trigger entropy collection
```

**Request/Response Types**:
```rust
// Generic encrypt request
{
  "plaintext": "base64-encoded-data",
  "key_id": "optional-key-id",
  "algorithm": "auto" | "aes-256-gcm" | "chacha20-poly1305"
}

// Generic encrypt response
{
  "success": true,
  "data": {
    "ciphertext": "base64-encoded",
    "algorithm": "aes-256-gcm",
    "nonce": "base64-encoded",
    "key_id": "used-key-id"
  },
  "timestamp": "2025-12-17T..."
}
```

### 2. Service Discovery Configuration

**mDNS Advertisement**:
```rust
// Service type: _beardog._tcp.local
// Service properties:
{
  "version": "0.9.0",
  "protocols": ["http", "json-rpc", "tarpc"],
  "capabilities": [
    "encrypt",
    "decrypt",
    "sign",
    "verify",
    "key-management",
    "entropy-generation"
  ],
  "api_version": "v1",
  "endpoints": {
    "http": "http://192.168.1.100:8080",
    "rpc": "http://192.168.1.100:8080/rpc",
    "tarpc": "192.168.1.100:9000"
  }
}
```

### 3. Authentication & Authorization

**For Songbird Integration**:
```rust
// Phase 1: mTLS (mutual TLS)
- Client certificates for authentication
- Per-service authorization rules
- Graceful degradation (works without)

// Phase 2: Token-based (optional)
- JWT tokens for API access
- Scoped permissions
- Token rotation
```

---

## 🎵 Songbird Side Implementation

### Service Discovery Client

**File**: `songbird/crates/songbird-security/src/beardog_client.rs`

```rust
use mdns::Responder;
use reqwest::Client;

pub struct BearDogClient {
    http_client: Client,
    base_url: Option<String>,
    fallback_enabled: bool,
}

impl BearDogClient {
    /// Discover BearDog via mDNS
    pub async fn discover() -> Result<Self, Error> {
        // 1. Search for _beardog._tcp.local
        let services = discover_mdns_services("_beardog._tcp.local").await?;
        
        // 2. Extract HTTP endpoint
        let base_url = services
            .first()
            .and_then(|s| s.properties.get("http_endpoint"))
            .cloned();
        
        Ok(Self {
            http_client: Client::new(),
            base_url,
            fallback_enabled: true,
        })
    }
    
    /// Encrypt data using BearDog
    pub async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        if let Some(url) = &self.base_url {
            // Use BearDog
            self.call_beardog_encrypt(url, data).await
        } else if self.fallback_enabled {
            // Graceful fallback to local crypto
            self.fallback_encrypt(data).await
        } else {
            Err(Error::BearDogUnavailable)
        }
    }
}
```

### Integration with Sovereign Security

**File**: `songbird/crates/songbird-security/src/security_sovereign.rs`

```rust
// Already exists - just needs wiring!
impl SovereignSecurityValidator {
    /// Initialize with BearDog integration
    pub async fn with_beardog() -> Result<Self, Error> {
        let beardog_client = BearDogClient::discover().await?;
        
        Ok(Self {
            beardog: Some(beardog_client),
            fallback_crypto: LocalCryptoProvider::new(),
        })
    }
    
    /// Encrypt with BearDog (or fallback)
    pub async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        if let Some(beardog) = &self.beardog {
            beardog.encrypt(data).await
        } else {
            self.fallback_crypto.encrypt(data)
        }
    }
}
```

---

## 📊 Integration Architecture

```
┌─────────────┐                    ┌─────────────┐
│  Songbird   │                    │   BearDog   │
│             │                    │             │
│ ┌─────────┐ │                    │ ┌─────────┐ │
│ │Discovery│─┼────mDNS query──────▶│ │ mDNS    │ │
│ │ Client  │ │◀───service info────┤ │Advertiser││
│ └─────────┘ │                    │ └─────────┘ │
│             │                    │             │
│ ┌─────────┐ │                    │ ┌─────────┐ │
│ │ HTTP    │─┼───POST /encrypt───▶│ │  Axum   │ │
│ │ Client  │ │◀──encrypted data───┤ │ Server  │ │
│ └─────────┘ │                    │ └─────────┘ │
│             │                    │             │
│ ┌─────────┐ │                    │ ┌─────────┐ │
│ │Fallback │ │  (if BearDog down) │ │  Crypto │ │
│ │ Crypto  │ │                    │ │ Service │ │
│ └─────────┘ │                    │ └─────────┘ │
└─────────────┘                    └─────────────┘
```

---

## 🚀 Implementation Roadmap

### Week 1: BearDog API Expansion

**Day 1-2**: Generic Crypto Endpoints
- [ ] Create `crates/beardog-api/src/endpoints/generic_crypto.rs`
- [ ] Implement `/api/v1/encrypt` and `/api/v1/decrypt`
- [ ] Add algorithm auto-selection logic
- [ ] Write tests

**Day 3-4**: Key Management API
- [ ] Create `crates/beardog-api/src/endpoints/keys.rs`
- [ ] Implement key generation/retrieval endpoints
- [ ] Add secure key storage integration
- [ ] Write tests

**Day 5**: Service Discovery
- [ ] Configure mDNS advertisement with full capabilities
- [ ] Test service discovery locally
- [ ] Document discovery protocol

### Week 2: Songbird Integration

**Day 1-2**: Discovery Client
- [ ] Implement BearDog discovery in Songbird
- [ ] Add HTTP client with retry logic
- [ ] Test across LAN

**Day 3-4**: Sovereign Security Integration
- [ ] Wire BearDog client to security validator
- [ ] Implement graceful fallback
- [ ] Add configuration options

**Day 5**: Testing & Documentation
- [ ] End-to-end integration tests
- [ ] Performance benchmarks
- [ ] Complete documentation
- [ ] Create demo script

---

## 📝 API Documentation for Songbird

### BearDog HTTP API v1

**Base URL**: Discovered via mDNS `_beardog._tcp.local`

**Authentication**: mTLS (Phase 1), JWT tokens (Phase 2)

**Content-Type**: `application/json`

### Endpoints

#### Health Check
```http
GET /health
Response: { "status": "healthy", "version": "0.9.0" }
```

#### Encrypt Data
```http
POST /api/v1/encrypt
Content-Type: application/json

Request:
{
  "plaintext": "base64-encoded-data",
  "algorithm": "auto"
}

Response:
{
  "success": true,
  "data": {
    "ciphertext": "base64-encoded",
    "algorithm": "aes-256-gcm",
    "nonce": "base64-encoded"
  }
}
```

#### Decrypt Data
```http
POST /api/v1/decrypt
Content-Type: application/json

Request:
{
  "ciphertext": "base64-encoded-data",
  "nonce": "base64-encoded",
  "algorithm": "aes-256-gcm"
}

Response:
{
  "success": true,
  "data": {
    "plaintext": "base64-encoded"
  }
}
```

---

## ✅ Success Criteria

### Technical
- [ ] BearDog discoverable via mDNS
- [ ] Generic crypto API functional
- [ ] Key management API working
- [ ] Songbird can call BearDog
- [ ] Graceful fallback operational
- [ ] <100ms latency for local calls
- [ ] All tests passing

### Integration
- [ ] 2-tower mesh with encryption
- [ ] Songbird → BearDog → Songbird flow
- [ ] Error handling robust
- [ ] Documentation complete
- [ ] Demo script working

### Production Readiness
- [ ] TLS/mTLS support
- [ ] Rate limiting configured
- [ ] Monitoring integrated
- [ ] Logs structured
- [ ] Metrics exposed

---

## 🎯 Next Steps

**Immediate** (This Week):
1. ✅ Create this integration plan
2. 🔨 Implement generic crypto endpoints
3. 🔨 Add key management API
4. 🔨 Configure mDNS properly
5. 🔨 Write integration tests

**Short-Term** (Next Week):
1. Implement Songbird discovery client
2. Wire to sovereign security
3. End-to-end testing
4. Document integration
5. Create demo

**Medium-Term** (2-4 Weeks):
1. Add mTLS authentication
2. Distributed key management
3. Three-primal encrypted workflow
4. Complete showcase demos

---

## 📚 References

**BearDog**:
- `crates/beardog-api/` - HTTP API implementation
- `crates/beardog-core/` - Crypto service
- `COMPREHENSIVE_SESSION_COMPLETE_DEC_17_2025.md` - Current status

**Songbird**:
- `songbird/crates/songbird-security/src/security_sovereign.rs` - Security validator
- Gaps report provided by user

**Documentation**:
- This file (integration plan)
- API docs to be generated from code
- Cross-primal integration guide (to be written)

---

**Status**: Plan complete, implementation starting  
**Owner**: BearDog team  
**Reviewers**: Songbird team, ecosystem architects  
**Target**: Week 1 complete by Dec 24, 2025

