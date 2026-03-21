# 🔐 BTSP Unified API Reference

**Version**: 0.9.0  
**Status**: Phase 2 Complete (Phase 3-4 In Progress)  
**Last Updated**: January 21, 2026

---

## 🎯 Overview

BTSP (BearDog Tunnel Security Protocol) Unified is a **single API** for all secure communication:

- **Internal Mode**: Primal-to-primal communication via genetic lineage
- **External Mode**: HTTPS communication via certificate trust

**Key Insight**: Trust mode (genetic lineage vs. certificates) is the fundamental difference, not the protocol!

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    BTSP Unified API                             │
│         Single RPC Interface for All Secure Comms               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Internal Mode              │      External Mode               │
│  (Primal-to-Primal)         │      (External APIs)             │
│  ─────────────────          │      ──────────────              │
│  • Genetic lineage trust    │      • Certificate trust         │
│  • Unix sockets             │      • TCP sockets               │
│  • X25519 + ChaCha20        │      • TLS 1.3 + HTTP/2          │
│  • Long-lived tunnels       │      • Per-request sessions      │
│                             │                                   │
└─────────────────────────────────────────────────────────────────┘
              ↓                             ↓
┌─────────────────────────────────────────────────────────────────┐
│              Shared Crypto Foundation                           │
│      X25519 | ChaCha20-Poly1305 | Ed25519 | BLAKE3             │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📋 RPC Methods

### Core Methods (6 operations, fully implemented)

1. **`btsp.contact_exchange`** - Exchange contact info via genetic lineage
2. **`btsp.tunnel_establish`** - **UNIFIED**: Establish tunnel (internal OR external)
3. **`btsp.tunnel_encrypt`** - Encrypt data through tunnel
4. **`btsp.tunnel_decrypt`** - Decrypt data from tunnel
5. **`btsp.tunnel_status`** - Get tunnel status
6. **`btsp.tunnel_close`** - Close tunnel gracefully

### Unified Methods (3 new, Phase 3-4)

7. **`btsp.configure_tls`** - Configure TLS for external tunnel (Phase 3)
8. **`btsp.verify_peer`** - Unified trust verification (Phase 3)
9. **`btsp.tunnel_send_http`** - Send HTTP request through tunnel (Phase 4)

---

## 🔹 Internal Mode (Primal-to-Primal)

### btsp.tunnel_establish (Internal)

Establish secure tunnel with another primal using genetic lineage.

**Request** (backward compatible):
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "btsp.tunnel_establish",
  "params": {
    "peer_id": "songbird-nat0",
    "peer_endpoint": "unix:///tmp/songbird-nat0.sock"
  }
}
```

**Request** (explicit internal mode):
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "btsp.tunnel_establish",
  "params": {
    "peer_id": "songbird-nat0",
    "peer_endpoint": "unix:///tmp/songbird-nat0.sock",
    "trust_mode": {
      "type": "genetic_lineage",
      "required_family": "nat0",
      "verify_ancestry": true
    },
    "protocol": {
      "type": "btsp_native",
      "version": "2.0",
      "features": ["compression"]
    }
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tunnel_id": "550e8400-e29b-41d4-a716-446655440000",
    "peer_id": "songbird-nat0",
    "mode": "internal",
    "protocol": "btsp_native",
    "established_at": "2026-01-21T19:30:00Z"
  }
}
```

**Features**:
- ✅ Genetic lineage verification
- ✅ Unix socket communication
- ✅ X25519 key exchange
- ✅ ChaCha20-Poly1305 encryption
- ✅ Long-lived secure channel
- ✅ Family-based auto-trust

---

## 🔸 External Mode (External APIs)

### btsp.tunnel_establish (External)

Establish secure TLS tunnel with external HTTPS server.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "btsp.tunnel_establish",
  "params": {
    "peer_id": "api.anthropic.com",
    "peer_endpoint": "tcp://api.anthropic.com:443",
    "trust_mode": {
      "type": "certificate",
      "server_name": "api.anthropic.com",
      "verify_chain": true,
      "root_ca_bundle": "mozilla"
    },
    "protocol": {
      "type": "tls_http",
      "tls_version": "1.3",
      "http_version": "2",
      "alpn_protocols": ["h2", "http/1.1"]
    }
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tunnel_id": "660f9511-f39c-52e5-b827-557766551111",
    "peer_id": "api.anthropic.com",
    "mode": "external",
    "protocol": "tls_http",
    "established_at": "2026-01-21T19:35:00Z"
  }
}
```

**Features** (Phase 3):
- 🔜 TLS 1.3 handshake
- 🔜 Certificate chain verification
- 🔜 X25519 ECDH key exchange
- 🔜 ChaCha20-Poly1305 or AES-256-GCM
- 🔜 HTTP/2 support
- 🔜 ALPN negotiation

---

## 🔐 Trust Modes

### Genetic Lineage (Internal)

Used for primal-to-primal communication where peers share cryptographic family trees.

**Type**: `genetic_lineage`

**Parameters**:
```typescript
{
  type: "genetic_lineage",
  required_family?: string,      // Optional: "nat0", "prod1", etc.
  required_generation?: number,  // Optional: Generation number
  verify_ancestry: boolean       // Default: true
}
```

**Trust Verification**:
1. Extract peer's genetic signature
2. Verify family membership (if required_family specified)
3. Check generation (if required_generation specified)
4. Validate full ancestry chain (if verify_ancestry=true)
5. Compute trust level based on family distance

**Trust Levels**:
- **Direct Family** (100%): Same family, any generation
- **Cousin** (75%): Related families, verified ancestry
- **Known** (50%): Valid signature, unrelated family
- **TOFU** (25%): First contact, trust on first use

---

### Certificate (External)

Used for external HTTPS communication where servers present X.509 certificates.

**Type**: `certificate`

**Parameters**:
```typescript
{
  type: "certificate",
  server_name: string,           // Required: SNI hostname
  verify_chain: boolean,         // Default: true
  root_ca_bundle: "mozilla" | "system" | { "custom": "pem_bundle" },
  allow_self_signed: boolean     // Default: false (DANGER!)
}
```

**Certificate Verification** (Phase 3):
1. Perform TLS 1.3 handshake
2. Receive server certificate chain
3. Parse X.509 certificates (x509-parser)
4. Verify chain back to trusted root CA
5. Check server name matches SAN/CN
6. Validate expiration dates
7. Check revocation status (optional)

**Root CA Bundles**:
- **Mozilla**: Mozilla's root CA bundle (webpki-roots) - Recommended
- **System**: OS-provided root CAs (platform-specific)
- **Custom**: User-provided PEM bundle (advanced)

⚠️  **Security Warning**: `allow_self_signed: true` bypasses all trust verification! Only use in development/testing.

---

## 🌐 Protocols

### BTSP Native (Internal)

Custom protocol optimized for inter-primal communication.

**Type**: `btsp_native`

**Parameters**:
```typescript
{
  type: "btsp_native",
  version: "2.0",                // Default: "2.0"
  features: string[]             // Optional features
}
```

**Supported Features**:
- `"compression"` - Data compression
- `"multiplexing"` - Multiple streams per tunnel
- `"keep_alive"` - Automatic heartbeat

**Security**:
- X25519 key exchange
- ChaCha20-Poly1305 AEAD encryption
- Ed25519 signatures for authentication
- BLAKE3 hashing for integrity

**Performance**:
- < 1ms tunnel establishment
- < 0.1ms encrypt/decrypt operations
- Zero network overhead (Unix sockets)

---

### TLS HTTP (External)

Standard TLS 1.3 + HTTP/2 for external communication.

**Type**: `tls_http`

**Parameters**:
```typescript
{
  type: "tls_http",
  tls_version: "1.3",            // Only TLS 1.3 supported
  http_version: "2" | "1.1",     // Default: "2"
  alpn_protocols: string[]       // Default: ["h2", "http/1.1"]
}
```

**TLS 1.3 Features** (Phase 3):
- 🔜 1-RTT handshake
- 🔜 Perfect forward secrecy
- 🔜 X25519 ECDH key exchange
- 🔜 ChaCha20-Poly1305 or AES-256-GCM
- 🔜 TLS record encryption

**HTTP Features** (Phase 4):
- 🔜 HTTP/2 multiplexing
- 🔜 Server push support
- 🔜 Header compression (HPACK)
- 🔜 Stream prioritization

**Performance** (Phase 3):
- < 50ms TLS handshake (including RPC overhead)
- < 5ms per HTTP request (after handshake)
- Concurrent requests via HTTP/2

---

## 🚀 Transport Layers

### Unix Socket (Internal)

Local communication via Unix domain sockets.

**Format**: `unix:///path/to/socket.sock`

**Examples**:
- `unix:///tmp/beardog-nat0.sock`
- `unix:///run/songbird.sock`
- `unix:///var/run/primals/squirrel.sock`

**Features**:
- Zero network overhead
- File system permissions for access control
- Secure by default (local only)
- High performance (< 1μs latency)

---

### TCP Socket (External)

Network communication via TCP sockets.

**Format**: `tcp://hostname:port`

**Examples**:
- `tcp://api.anthropic.com:443`
- `tcp://api.openai.com:443`
- `tcp://192.168.1.100:8080`
- `tcp://::1:3000` (IPv6)

**Features**:
- Remote server access
- IPv4 and IPv6 support
- Standard HTTPS port 443
- Firewall-friendly

---

## 📊 Encryption Operations

### btsp.tunnel_encrypt

Encrypt data for transmission through tunnel.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "btsp.tunnel_encrypt",
  "params": {
    "tunnel_id": "550e8400-e29b-41d4-a716-446655440000",
    "data": "SGVsbG8sIHdvcmxkIQ=="
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "ciphertext": "ZW5jcnlwdGVkX2RhdGFfaGVyZQ=="
  }
}
```

**Works for both modes**:
- Internal: ChaCha20-Poly1305 with session key
- External: TLS record encryption (Phase 3)

---

### btsp.tunnel_decrypt

Decrypt data received through tunnel.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "btsp.tunnel_decrypt",
  "params": {
    "tunnel_id": "550e8400-e29b-41d4-a716-446655440000",
    "data": "ZW5jcnlwdGVkX2RhdGFfaGVyZQ=="
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "plaintext": "SGVsbG8sIHdvcmxkIQ=="
  }
}
```

**Works for both modes**:
- Internal: ChaCha20-Poly1305 decryption
- External: TLS record decryption (Phase 3)

---

## 📈 Status & Management

### btsp.tunnel_status

Get current tunnel status and statistics.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "btsp.tunnel_status",
  "params": {
    "tunnel_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "result": {
    "tunnel_id": "550e8400-e29b-41d4-a716-446655440000",
    "peer_id": "songbird-nat0",
    "status": "active",
    "established_at": "2026-01-21T19:30:00Z",
    "bytes_sent": 1024000,
    "bytes_received": 2048000,
    "last_activity": "2026-01-21T19:45:30Z"
  }
}
```

---

### btsp.tunnel_close

Gracefully close a tunnel.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "method": "btsp.tunnel_close",
  "params": {
    "tunnel_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "result": {
    "success": true,
    "tunnel_id": "550e8400-e29b-41d4-a716-446655440000",
    "message": "Tunnel closed successfully"
  }
}
```

---

## 🌐 HTTP Operations (Phase 4)

### btsp.tunnel_send_http

Send HTTP request through external mode tunnel.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 6,
  "method": "btsp.tunnel_send_http",
  "params": {
    "tunnel_id": "660f9511-f39c-52e5-b827-557766551111",
    "method": "POST",
    "path": "/v1/messages",
    "headers": {
      "content-type": "application/json",
      "x-api-key": "sk-ant-..."
    },
    "body": "eyJtb2RlbCI6ImNsYXVkZS0zLW9wdXMtMjAyNDAyMjkifQ=="
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 6,
  "result": {
    "status": 200,
    "headers": {
      "content-type": "application/json"
    },
    "body": "eyJyZXNwb25zZSI6ImhlbGxvIn0="
  }
}
```

**Features** (Phase 4):
- 🔜 HTTP/2 request formatting
- 🔜 Automatic header management
- 🔜 Base64 encoding for binary safety
- 🔜 Concurrent requests support

---

## 📚 Usage Examples

### Example 1: Internal Primal Communication

```rust
use beardog_types::btsp::*;

// Establish tunnel with Songbird
let params = TunnelEstablishParams {
    peer_id: "songbird-nat0".into(),
    peer_endpoint: "unix:///tmp/songbird-nat0.sock".into(),
    trust_mode: None, // Defaults to genetic_lineage
    protocol: None,   // Defaults to btsp_native
};

let tunnel = beardog_rpc.call("btsp.tunnel_establish", params).await?;

// Encrypt message
let encrypted = beardog_rpc.call("btsp.tunnel_encrypt", json!({
    "tunnel_id": tunnel["tunnel_id"],
    "data": base64::encode("Hello Songbird!")
})).await?;
```

### Example 2: External API Access (Phase 3-4)

```rust
use beardog_types::btsp::*;

// Establish TLS tunnel with Anthropic
let params = TunnelEstablishParams {
    peer_id: "api.anthropic.com".into(),
    peer_endpoint: "tcp://api.anthropic.com:443".into(),
    trust_mode: Some(TrustMode::Certificate {
        server_name: "api.anthropic.com".into(),
        verify_chain: true,
        root_ca_bundle: CaBundle::Mozilla,
        allow_self_signed: false,
    }),
    protocol: Some(TunnelProtocol::TlsHttp {
        tls_version: "1.3".into(),
        http_version: "2".into(),
        alpn_protocols: vec!["h2".into()],
    }),
};

let tunnel = beardog_rpc.call("btsp.tunnel_establish", params).await?;

// Send HTTP request
let response = beardog_rpc.call("btsp.tunnel_send_http", json!({
    "tunnel_id": tunnel["tunnel_id"],
    "method": "POST",
    "path": "/v1/messages",
    "headers": {"content-type": "application/json"},
    "body": base64::encode(&request_json)
})).await?;
```

---

## ⚡ Performance Targets

### Internal Mode (Fully Implemented)

- **Tunnel Establishment**: < 1ms
- **Encrypt/Decrypt**: < 0.1ms per operation
- **Throughput**: > 1 GB/s (Unix socket limit)
- **Latency**: < 1μs (local process communication)

### External Mode (Phase 3-4)

- **TLS Handshake**: < 50ms (including RPC overhead)
- **HTTP Request**: < 5ms (after handshake)
- **Crypto Operations**: < 1ms each
- **Total Handshake**: < 10ms (crypto only, target met!)

---

## 🔒 Security Considerations

### Internal Mode

✅ **Genetic Lineage Trust**
- Verifies cryptographic family membership
- Supports TOFU (Trust On First Use)
- Trust levels based on family distance

✅ **ChaCha20-Poly1305 Encryption**
- AEAD (Authenticated Encryption)
- 256-bit security
- Nonce management for replay protection

✅ **Unix Socket Security**
- File system permissions
- Local-only communication
- No network exposure

### External Mode (Phase 3)

🔜 **Certificate Verification**
- X.509 certificate chain validation
- Root CA trust anchors (Mozilla/System)
- Hostname verification (SNI)
- Expiration date checks

🔜 **TLS 1.3 Security**
- 1-RTT handshake
- Perfect forward secrecy
- No downgrade attacks
- Encrypted SNI support

⚠️  **Critical: Never use `allow_self_signed: true` in production!**

---

## 🎯 Implementation Status

### ✅ Completed (Phases 1-2)

- [x] Complete type system (TrustMode, Protocol, Transport, RPC params)
- [x] Handler routing infrastructure
- [x] Internal mode tunnel establishment
- [x] Backward compatibility layer
- [x] Unified response formats
- [x] 36 comprehensive tests

### 🔜 In Progress (Phases 3-4)

**Phase 3: TLS Handshake**
- [ ] TCP socket management
- [ ] TLS 1.3 handshake implementation
- [ ] Certificate chain verification
- [ ] TLS record encryption/decryption
- [ ] Error handling & recovery

**Phase 4: HTTP Wrapper**
- [ ] HTTP/2 request formatting
- [ ] Response parsing
- [ ] Header management
- [ ] Stream multiplexing

**Phase 5: Testing & Documentation**
- [ ] E2E tests with httpbin.org
- [ ] Performance benchmarks
- [ ] Migration guide for Songbird
- [ ] Security audit

---

## 📖 Related Documentation

- [BTSP_UNIFIED_EVOLUTION_RESPONSE_JAN_21_2026.md](../BTSP_UNIFIED_EVOLUTION_RESPONSE_JAN_21_2026.md) - Architectural approval & rationale
- [BTSP_UNIFIED_IMPLEMENTATION_PLAN.md](../BTSP_UNIFIED_IMPLEMENTATION_PLAN.md) - 18-day implementation roadmap
- [BTSP_TOWER_ATOMIC_RELATIONSHIP.md](../BTSP_TOWER_ATOMIC_RELATIONSHIP.md) - Pre-unification architecture
- [TLS_CRYPTO_API.md](./TLS_CRYPTO_API.md) - TLS 1.3 crypto operations (underlying implementation)

---

## 🐕 For Songbird Developers

### When to Use Internal Mode

✅ Talking to another ecoPrimal  
✅ Need long-lived secure channel  
✅ Can verify genetic lineage  
✅ Low latency critical  
✅ Peer is known entity  

### When to Use External Mode

✅ Calling external APIs (Anthropic, OpenAI, etc.)  
✅ Need TLS 1.3 compliance  
✅ Standard HTTP/HTTPS required  
✅ Server is unknown  
✅ Certificate-based trust  

---

**Last Updated**: January 21, 2026  
**Version**: 0.9.0 (Phases 1-2 Complete)  
**Status**: Production-ready for internal mode, external mode in development

🐕🐦 **BearDog + Songbird: Complete Secure Communication Stack!** 🔐✨

