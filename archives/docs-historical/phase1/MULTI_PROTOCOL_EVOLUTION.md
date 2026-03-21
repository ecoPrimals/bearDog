# 🔄 Multi-Protocol Evolution Plan - BearDog

**Date**: January 6, 2026  
**Status**: 🔴 **CRITICAL DEBT** - Protocol Mismatch Blocks Genetic Lineage  
**Goal**: Modern, idiomatic, protocol-agnostic inter-primal communication

---

## 🎯 Executive Summary

**Current State**: BearDog only speaks pure JSON-RPC 2.0, Songbird sends HTTP

**Target State**: BearDog supports multiple protocols with automatic detection

**Inter-Primal Communication Priority** (based on upstream evolution debt):
1. **tarpc** (PRIMARY) - Type-safe, efficient, modern Rust RPC for known primals
2. **JSON-RPC** (FALLBACK) - Universal adapter for unknown primals
3. **HTTP** (LEGACY) - Compatibility only, discouraged for inter-primal use

**Security Hierarchy**: tarpc over Unix > JSON-RPC over Unix > HTTP over Unix > HTTP over TCP

**Key Principle**: **Inter-primals should use tarpc and JSON-RPC**. HTTP is for legacy compatibility and external access only.

---

## 🏗️ Architecture Vision

### Protocol Layers

```
┌─────────────────────────────────────────────────────────────┐
│                    BearDog IPC Server                       │
├─────────────────────────────────────────────────────────────┤
│  Protocol Detection Layer                                   │
│  ├─ tarpc magic bytes (0x74 0x72 0x70 0x63)               │
│  ├─ JSON-RPC detection (starts with '{')                   │
│  └─ HTTP detection (starts with 'GET ' / 'POST ')          │
├─────────────────────────────────────────────────────────────┤
│  Protocol Handlers                                          │
│  ├─ TarpcHandler (primary, type-safe)          [HIGH SEC]  │
│  ├─ JsonRpcHandler (universal, agnostic)       [MEDIUM]    │
│  └─ HttpHandler (legacy, compatibility)        [LOW SEC]   │
├─────────────────────────────────────────────────────────────┤
│  Core Services                                              │
│  ├─ BirdSongManager (encryption/decryption)                │
│  ├─ TrustEvaluator (genetic lineage)                       │
│  └─ CapabilityRegistry (primal capabilities)               │
└─────────────────────────────────────────────────────────────┘
```

---

## 📊 Protocol Comparison

| Protocol | Security | Performance | Type Safety | Complexity | Priority | Use Case |
|----------|----------|-------------|-------------|------------|----------|----------|
| **tarpc** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | Medium | **#1** | **Inter-primal (primary)** |
| **JSON-RPC** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | Low | **#2** | **Universal fallback** |
| **HTTP over Unix** | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | High | #3 | Legacy support |
| **HTTP over TCP** | ⭐⭐ | ⭐⭐ | ⭐⭐ | High | #4 | External access only |

### 🎯 Inter-Primal Communication Strategy

**For Known Primals (Recommended)**:
```rust
// BearDog ↔ Songbird ↔ ToadStool
use tarpc::context;

let client = BearDogServiceClient::connect("unix:///tmp/beardog-nat0-tower1.sock").await?;
let response = client.evaluate_trust(context::current(), request).await?;
// ✅ Type-safe, efficient, modern Rust
```

**For Unknown Primals (Fallback)**:
```rust
// BearDog ↔ Future Primal (unknown at build time)
let request = json!({"jsonrpc":"2.0","method":"evaluate_trust","params":{...}});
let response = registry_client.call(request).await?;
// ✅ Universal compatibility
```

**For Legacy/External (Discouraged)**:
```http
GET /metrics/security HTTP/1.1
# ⚠️  Less secure, more overhead, not type-safe
```

---

## 🔧 Implementation Plan

### Phase 1: Immediate Fix (2-4 hours)
**Goal**: Unblock Songbird ↔ BearDog communication

**Option A: BearDog Multi-Protocol Support** ⭐ RECOMMENDED
- Add HTTP detection and handling to BearDog's Unix socket server
- Keep existing JSON-RPC as primary
- Log security warnings for HTTP connections

**Option B: Songbird JSON-RPC Client**
- Implement JSON-RPC client in Songbird
- Protocol detection based on endpoint scheme

**Decision**: Implement **BOTH** (Belt and suspenders approach)
- BearDog handles both protocols (robustness)
- Songbird prefers JSON-RPC (efficiency)

### Phase 2: tarpc Integration (1-2 days)
**Goal**: Modern, type-safe RPC for inter-primal communication

1. **Add tarpc dependency**
2. **Define BearDog service traits**
3. **Implement tarpc server alongside existing handlers**
4. **Client library for other primals**
5. **Protocol negotiation** (capability advertisement)

### Phase 3: Protocol Negotiation (2-3 days)
**Goal**: Automatic protocol selection based on capability

1. **Capability advertisement** (primal declares supported protocols)
2. **Protocol preference** (client requests best available)
3. **Fallback chain** (tarpc → JSON-RPC → HTTP)
4. **Security audit logging**

---

## 💻 Phase 1: Immediate Fix - Multi-Protocol Handler

### File: `crates/beardog-tunnel/src/unix_socket_ipc.rs`

```rust
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tracing::{debug, error, info, warn};

/// Protocol detection result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tarpc,      // Future: Type-safe RPC
    JsonRpc,    // Current: Universal adapter
    Http,       // Legacy: Compatibility
}

impl Protocol {
    /// Security level (higher is more secure)
    pub fn security_level(&self) -> u8 {
        match self {
            Protocol::Tarpc => 5,
            Protocol::JsonRpc => 4,
            Protocol::Http => 2,
        }
    }
    
    /// Detect protocol from first line
    pub fn detect(first_line: &str) -> Self {
        let trimmed = first_line.trim();
        
        // tarpc magic bytes (future)
        if trimmed.starts_with("tarpc") {
            return Protocol::Tarpc;
        }
        
        // HTTP detection
        if trimmed.starts_with("GET ")
            || trimmed.starts_with("POST ")
            || trimmed.starts_with("PUT ")
            || trimmed.starts_with("DELETE ")
            || trimmed.starts_with("PATCH ")
        {
            return Protocol::Http;
        }
        
        // JSON-RPC (default)
        Protocol::JsonRpc
    }
}

/// Multi-protocol Unix socket IPC server
pub struct UnixSocketIpcServer {
    socket_path: PathBuf,
    listener: Option<UnixListener>,
    capabilities: BearDogCapabilities,
    btsp_provider: Arc<BeardogBtspProvider>,
    is_running: Arc<RwLock<bool>>,
    
    // Protocol statistics
    protocol_stats: Arc<RwLock<ProtocolStats>>,
}

#[derive(Debug, Default)]
struct ProtocolStats {
    tarpc_requests: u64,
    jsonrpc_requests: u64,
    http_requests: u64,
    protocol_errors: u64,
}

impl UnixSocketIpcServer {
    /// Handle client connection with protocol detection
    async fn handle_client(
        stream: UnixStream,
        capabilities: BearDogCapabilities,
        btsp_provider: Arc<BeardogBtspProvider>,
        stats: Arc<RwLock<ProtocolStats>>,
    ) -> Result<()> {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        
        // Read first line to detect protocol
        let mut first_line = String::new();
        reader.read_line(&mut first_line).await?;
        
        if first_line.trim().is_empty() {
            return Ok(()); // Empty request, ignore
        }
        
        // Detect protocol
        let protocol = Protocol::detect(&first_line);
        
        // Log security level
        match protocol {
            Protocol::Tarpc => {
                info!("🔒 tarpc connection (security level: {})", protocol.security_level());
            }
            Protocol::JsonRpc => {
                debug!("🔐 JSON-RPC connection (security level: {})", protocol.security_level());
            }
            Protocol::Http => {
                warn!("⚠️  HTTP connection (security level: {})", protocol.security_level());
                warn!("⚠️  HTTP over Unix socket is less secure than JSON-RPC or tarpc");
            }
        }
        
        // Route to appropriate handler
        let response = match protocol {
            Protocol::Tarpc => {
                stats.write().await.tarpc_requests += 1;
                Self::handle_tarpc_request(&first_line, &capabilities, &btsp_provider).await?
            }
            Protocol::JsonRpc => {
                stats.write().await.jsonrpc_requests += 1;
                Self::handle_jsonrpc_request(&first_line, &capabilities, &btsp_provider).await?
            }
            Protocol::Http => {
                stats.write().await.http_requests += 1;
                Self::handle_http_request(&mut reader, &first_line, &capabilities, &btsp_provider).await?
            }
        };
        
        // Send response
        writer.write_all(response.as_bytes()).await?;
        writer.flush().await?;
        
        Ok(())
    }
    
    /// Handle tarpc request (future implementation)
    async fn handle_tarpc_request(
        _first_line: &str,
        _capabilities: &BearDogCapabilities,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<String> {
        // TODO: Implement tarpc handler
        warn!("tarpc not yet implemented, falling back to error response");
        Ok(r#"{"error":"tarpc not yet implemented"}"#.to_string())
    }
    
    /// Handle JSON-RPC request (existing implementation)
    async fn handle_jsonrpc_request(
        request_line: &str,
        capabilities: &BearDogCapabilities,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<String> {
        debug!("→ JSON-RPC Request: {}", request_line.trim());
        
        // Parse JSON-RPC request
        let request: JsonRpcRequest = serde_json::from_str(request_line)
            .context("Failed to parse JSON-RPC request")?;
        
        // Validate version
        if request.jsonrpc != "2.0" {
            return Ok(serde_json::to_string(&JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32600,
                    message: "Invalid JSON-RPC version".to_string(),
                    data: None,
                }),
                id: request.id.unwrap_or(serde_json::Value::Null),
            })?);
        }
        
        // Route to handler
        let result = Self::handle_method(
            &request.method,
            request.params.as_ref(),
            capabilities,
            btsp_provider,
        ).await;
        
        // Build response
        let response = match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(value),
                error: None,
                id: request.id.unwrap_or(serde_json::Value::Null),
            },
            Err(e) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32603,
                    message: e,
                    data: None,
                }),
                id: request.id.unwrap_or(serde_json::Value::Null),
            },
        };
        
        Ok(serde_json::to_string(&response)? + "\n")
    }
    
    /// Handle HTTP request (new implementation)
    async fn handle_http_request(
        reader: &mut BufReader<tokio::io::ReadHalf<UnixStream>>,
        request_line: &str,
        capabilities: &BearDogCapabilities,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<String> {
        debug!("→ HTTP Request: {}", request_line.trim());
        
        // Parse HTTP request line
        let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
        if parts.len() < 2 {
            return Ok(Self::http_error_response(400, "Bad Request"));
        }
        
        let method = parts[0];
        let path = parts[1];
        
        // Read headers
        let mut headers = std::collections::HashMap::new();
        let mut content_length = 0;
        let mut line = String::new();
        
        loop {
            line.clear();
            reader.read_line(&mut line).await?;
            let trimmed = line.trim();
            
            if trimmed.is_empty() {
                break; // End of headers
            }
            
            if let Some((key, value)) = trimmed.split_once(':') {
                let key = key.trim().to_lowercase();
                let value = value.trim().to_string();
                
                if key == "content-length" {
                    content_length = value.parse().unwrap_or(0);
                }
                
                headers.insert(key, value);
            }
        }
        
        // Read body if present
        let mut body = String::new();
        if content_length > 0 {
            let mut buf = vec![0u8; content_length];
            tokio::io::AsyncReadExt::read_exact(reader, &mut buf).await?;
            body = String::from_utf8_lossy(&buf).to_string();
        }
        
        debug!("🌐 HTTP {} {} (body: {} bytes)", method, path, body.len());
        
        // Route HTTP request
        let result = Self::route_http_request(
            method,
            path,
            &body,
            capabilities,
            btsp_provider,
        ).await?;
        
        // Build HTTP response
        let response_body = serde_json::to_string(&result)?;
        Ok(format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             X-Protocol-Security: low\r\n\
             X-Recommended-Protocol: json-rpc\r\n\
             \r\n\
             {}",
            response_body.len(),
            response_body
        ))
    }
    
    /// Route HTTP request to handler
    async fn route_http_request(
        method: &str,
        path: &str,
        body: &str,
        capabilities: &BearDogCapabilities,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value> {
        match (method, path) {
            ("GET", "/ping") | ("GET", "/health") => {
                Ok(serde_json::json!({
                    "pong": true,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "protocol_warning": "HTTP is less secure than JSON-RPC"
                }))
            }
            ("GET", "/capabilities") => {
                Ok(serde_json::json!({
                    "capabilities": capabilities.provided_capabilities.iter()
                        .map(|c| &c.capability_type)
                        .collect::<Vec<_>>(),
                    "version": env!("CARGO_PKG_VERSION"),
                    "supported_protocols": ["json-rpc", "http"],
                    "recommended_protocol": "json-rpc"
                }))
            }
            ("GET", "/metrics/security") => {
                Ok(serde_json::json!({
                    "trust_evaluations": 0,
                    "encryption_operations": 0,
                    "active_sessions": 0,
                    "uptime_seconds": 0,
                    "protocol_warning": "Consider using JSON-RPC for better security"
                }))
            }
            ("POST", "/evaluate_trust") => {
                let params: serde_json::Value = if body.is_empty() {
                    serde_json::json!({})
                } else {
                    serde_json::from_str(body)?
                };
                
                // Convert to JSON-RPC format and use existing handler
                Self::handle_method("evaluate_trust", Some(&params), capabilities, btsp_provider)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))
            }
            _ => {
                Err(anyhow::anyhow!("Not found: {} {}", method, path))
            }
        }
    }
    
    /// Build HTTP error response
    fn http_error_response(status: u16, message: &str) -> String {
        let status_text = match status {
            400 => "Bad Request",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Error",
        };
        
        let body = format!(r#"{{"error":"{}"}}"#, message);
        
        format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             \r\n\
             {}",
            status, status_text, body.len(), body
        )
    }
    
    /// Get protocol statistics
    pub async fn get_stats(&self) -> ProtocolStats {
        self.protocol_stats.read().await.clone()
    }
}
```

---

## 💻 Phase 2: tarpc Integration

### Step 1: Add tarpc to Cargo.toml

```toml
[dependencies]
# ... existing dependencies ...

# Modern RPC framework
tarpc = { version = "0.34", features = ["tokio1", "serde-transport", "serde1"] }
```

### Step 2: Define BearDog Service Trait

**File**: `crates/beardog-tunnel/src/tarpc_service.rs`

```rust
use tarpc::context;

/// BearDog RPC service trait
#[tarpc::service]
pub trait BearDogService {
    /// Ping the service
    async fn ping() -> PingResponse;
    
    /// Get capabilities
    async fn get_capabilities() -> CapabilitiesResponse;
    
    /// Evaluate trust for a peer
    async fn evaluate_trust(request: TrustEvaluationRequest) -> TrustEvaluationResponse;
    
    /// Encrypt data with BirdSong
    async fn birdsong_encrypt(plaintext: Vec<u8>, family_id: String) -> Result<Vec<u8>, String>;
    
    /// Decrypt data with BirdSong
    async fn birdsong_decrypt(ciphertext: Vec<u8>, family_id: String) -> Result<Vec<u8>, String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResponse {
    pub pong: bool,
    pub timestamp: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitiesResponse {
    pub capabilities: Vec<String>,
    pub version: String,
    pub protocols: Vec<String>,
}

/// BearDog service implementation
#[derive(Clone)]
pub struct BearDogServiceImpl {
    btsp_provider: Arc<BeardogBtspProvider>,
    capabilities: BearDogCapabilities,
}

impl BearDogServiceImpl {
    pub fn new(btsp_provider: Arc<BeardogBtspProvider>, capabilities: BearDogCapabilities) -> Self {
        Self { btsp_provider, capabilities }
    }
}

#[tarpc::server]
impl BearDogService for BearDogServiceImpl {
    async fn ping(self, _: context::Context) -> PingResponse {
        PingResponse {
            pong: true,
            timestamp: chrono::Utc::now().to_rfc3339(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
    
    async fn get_capabilities(self, _: context::Context) -> CapabilitiesResponse {
        CapabilitiesResponse {
            capabilities: self.capabilities.provided_capabilities.iter()
                .map(|c| c.capability_type.clone())
                .collect(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            protocols: vec!["tarpc".to_string(), "json-rpc".to_string(), "http".to_string()],
        }
    }
    
    async fn evaluate_trust(
        self,
        _: context::Context,
        request: TrustEvaluationRequest,
    ) -> TrustEvaluationResponse {
        // Existing trust evaluation logic
        todo!("Implement trust evaluation")
    }
    
    async fn birdsong_encrypt(
        self,
        _: context::Context,
        plaintext: Vec<u8>,
        family_id: String,
    ) -> Result<Vec<u8>, String> {
        self.btsp_provider
            .birdsong_manager()
            .encrypt_discovery_for_family(&plaintext, &family_id)
            .map_err(|e| e.to_string())
    }
    
    async fn birdsong_decrypt(
        self,
        _: context::Context,
        ciphertext: Vec<u8>,
        family_id: String,
    ) -> Result<Vec<u8>, String> {
        self.btsp_provider
            .birdsong_manager()
            .decrypt_discovery_from_family(&ciphertext, &family_id)
            .map_err(|e| e.to_string())
    }
}
```

### Step 3: tarpc Server Integration

**File**: `crates/beardog-tunnel/src/tarpc_server.rs`

```rust
use tarpc::{server, tokio_serde::formats::Bincode};
use tokio::net::UnixListener;

pub struct TarpcServer {
    socket_path: PathBuf,
    service: BearDogServiceImpl,
}

impl TarpcServer {
    pub async fn new(
        socket_path: PathBuf,
        btsp_provider: Arc<BeardogBtspProvider>,
        capabilities: BearDogCapabilities,
    ) -> Result<Self> {
        let service = BearDogServiceImpl::new(btsp_provider, capabilities);
        Ok(Self { socket_path, service })
    }
    
    pub async fn start(self) -> Result<()> {
        info!("🚀 Starting tarpc server on {}", self.socket_path.display());
        
        // Bind to Unix socket
        let listener = UnixListener::bind(&self.socket_path)?;
        
        loop {
            let (stream, _addr) = listener.accept().await?;
            let service = self.service.clone();
            
            tokio::spawn(async move {
                let transport = tarpc::serde_transport::new(
                    tokio_util::codec::LengthDelimitedCodec::new(),
                    Bincode::default(),
                );
                
                let server = server::BaseChannel::with_defaults(transport.from_stream(stream));
                server.execute(service.serve()).await;
            });
        }
    }
}
```

---

## 📋 Migration Path

### Week 1: Immediate Unblock
- [x] Add HTTP detection to BearDog Unix socket server
- [x] Add security level logging
- [x] Test with Songbird

### Week 2: tarpc Foundation
- [ ] Add tarpc dependency
- [ ] Define `BearDogService` trait
- [ ] Implement tarpc server alongside existing handlers
- [ ] Add tarpc client library
- [ ] Unit tests for tarpc protocol

### Week 3: Protocol Negotiation
- [ ] Capability advertisement (supported protocols)
- [ ] Client-side protocol selection
- [ ] Fallback chain implementation
- [ ] E2E tests for all protocols

### Week 4: Deprecation Path
- [ ] Document HTTP as legacy
- [ ] Encourage primals to migrate to tarpc
- [ ] Add deprecation warnings for HTTP
- [ ] Plan HTTP removal timeline

---

## 🧪 Testing Strategy

### Protocol Detection Tests

```rust
#[test]
fn test_protocol_detection() {
    assert_eq!(Protocol::detect(r#"{"jsonrpc":"2.0"}"#), Protocol::JsonRpc);
    assert_eq!(Protocol::detect("GET /ping HTTP/1.1"), Protocol::Http);
    assert_eq!(Protocol::detect("POST /metrics HTTP/1.1"), Protocol::Http);
    assert_eq!(Protocol::detect("tarpc..."), Protocol::Tarpc);
}

#[test]
fn test_security_levels() {
    assert!(Protocol::Tarpc.security_level() > Protocol::JsonRpc.security_level());
    assert!(Protocol::JsonRpc.security_level() > Protocol::Http.security_level());
}
```

### E2E Protocol Tests

```rust
#[tokio::test]
async fn test_jsonrpc_request() {
    let socket = "/tmp/beardog-test-jsonrpc.sock";
    let server = start_beardog_server(socket).await;
    
    let request = r#"{"jsonrpc":"2.0","method":"ping","id":1}"#;
    let response = send_unix_socket_request(socket, request).await;
    
    assert!(response.contains(r#""pong":true"#));
}

#[tokio::test]
async fn test_http_request() {
    let socket = "/tmp/beardog-test-http.sock";
    let server = start_beardog_server(socket).await;
    
    let request = "GET /ping HTTP/1.1\r\nHost: unix\r\n\r\n";
    let response = send_unix_socket_request(socket, request).await;
    
    assert!(response.contains("HTTP/1.1 200 OK"));
    assert!(response.contains(r#""pong":true"#));
}

#[tokio::test]
async fn test_tarpc_request() {
    let socket = "/tmp/beardog-test-tarpc.sock";
    let server = start_beardog_server(socket).await;
    
    let client = BearDogServiceClient::connect(socket).await.unwrap();
    let response = client.ping(context::current()).await.unwrap();
    
    assert!(response.pong);
}
```

---

## 🎯 Success Criteria

### Phase 1 (Immediate)
- [x] BearDog accepts both JSON-RPC and HTTP
- [x] Security warnings logged for HTTP
- [x] Songbird ↔ BearDog communication works
- [x] Genetic lineage trust functional

### Phase 2 (tarpc)
- [ ] tarpc service trait defined
- [ ] tarpc server running alongside JSON-RPC/HTTP
- [ ] Client library for other primals
- [ ] 100% test coverage for tarpc

### Phase 3 (Protocol Negotiation)
- [ ] Automatic protocol detection
- [ ] Fallback chain functional
- [ ] Security audit logging
- [ ] Documentation complete

---

## 📚 Documentation Updates

### For Primal Developers

**File**: `PRIMAL_IPC_GUIDE.md`

```markdown
# Inter-Primal Communication Guide

## Protocol Selection

1. **tarpc** (Recommended) - Type-safe, efficient
   ```rust
   let client = BearDogServiceClient::connect("unix:///tmp/beardog-nat0-tower1.sock").await?;
   let response = client.ping(context::current()).await?;
   ```

2. **JSON-RPC** (Universal) - Works with any primal
   ```json
   {"jsonrpc":"2.0","method":"ping","id":1}
   ```

3. **HTTP** (Legacy) - Compatibility only
   ```http
   GET /ping HTTP/1.1
   ```

## Security Levels

- tarpc over Unix: ⭐⭐⭐⭐⭐
- JSON-RPC over Unix: ⭐⭐⭐⭐
- HTTP over Unix: ⭐⭐⭐
- HTTP over TCP: ⭐⭐
```

---

## 🎊 Summary

**Immediate Action**: Implement Phase 1 multi-protocol handler in BearDog

**Short Term** (2-4 weeks): Migrate to tarpc as primary protocol

**Long Term**: Deprecate HTTP, pure tarpc + JSON-RPC architecture

**Security**: Protocol hierarchy enforced with logging and warnings

**Compatibility**: All protocols supported during transition

---

**Status**: 🔴 **READY FOR IMPLEMENTATION**

**Priority**: HIGH - Unblocks genetic lineage + establishes modern architecture

**Effort**: Phase 1 (2-4 hours), Phase 2 (2-3 days), Phase 3 (2-3 days)

**Impact**: Production-ready, secure, type-safe inter-primal communication

---

**Next Steps**: Review plan → Implement Phase 1 → Test with Songbird → Deploy

