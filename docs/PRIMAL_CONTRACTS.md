# 🔌 BearDog Primal Contracts - JSON-RPC API Specification

**Version**: 4.0.0  
**Date**: May 28, 2026  
**Status**: Production Stable  
**Protocol**: JSON-RPC 2.0 over Unix Domain Sockets

═══════════════════════════════════════════════════════════════════

## 🎯 **OVERVIEW**

BearDog exposes its cryptographic and genetic capabilities through a JSON-RPC 2.0 interface over Unix domain sockets. This allows any primal to discover and use BearDog's services at runtime without compile-time dependencies.

### **Philosophy: Primal Self-Knowledge**

- **BearDog knows**: Its own capabilities (crypto, genetics, HSM)
- **BearDog discovers**: Other primals via Dark Forest beacons (runtime)
- **No compile-time coupling**: Primals communicate via JSON-RPC

### **Transport**

**Primary**: Unix Domain Sockets
- Path: `/run/user/$UID/biomeos/beardog.sock` (Linux)
- Protocol: JSON-RPC 2.0 over NDJSON (newline-delimited JSON)
- Encoding: UTF-8
- Authentication: Ionic tokens (Ed25519-signed, JH-1) + family lineage (genetic)

**TCP** (Android, Windows, cross-host)
- Discovered via: `~/.config/biomeos/beardog.sock` (contains `tcp:IP:PORT`)
- Protocol: JSON-RPC 2.0 over NDJSON or BTSP encrypted frames
- Default port: 9100 (opt-in via `--port` or `BEARDOG_TCP_IPC_PORT`; 9190 is metrics)

**Named Pipes** (Windows)
- Protocol: Same JSON-RPC 2.0 over NDJSON

═══════════════════════════════════════════════════════════════════

## 📚 **API CATEGORIES**

BearDog provides **226 dispatchable JSON-RPC methods** (217 via `HandlerRegistry` + 9 pre-dispatch gate methods) organized into 18 handler categories, plus 13 route aliases for backward compatibility.

> **SSOT**: Call `rpc.methods` or `capabilities.list` for the live method inventory.

### **1. Core Cryptography** (106 methods — `CryptoHandler`)
- Signatures: Ed25519, ECDSA (P-256, P-384), RSA (PKCS#1, PSS)
- Key Exchange: X25519, ECDH (P-256, P-384)
- Encryption: ChaCha20-Poly1305, AES-GCM (128, 256)
- Hashing: BLAKE3, SHA-256/384/512, SHA-1, SHA3-256
- HMAC: SHA-256/384/512, BLAKE3, HMAC verify
- KDF: HKDF-SHA256, Argon2id, PBKDF2-SHA256, bcrypt, scrypt
- Genetic: lineage derivation, entropy mixing, challenge-response, certificates
- TLS 1.3: secret derivation, handshake signing, finished verify data
- TLS 1.2: PRF derivation (`crypto.kdf.tls12_prf`)
- ECDHE: P-256/P-384 ephemeral generate + compute shared
- AEAD: TLS-style `crypto.aead.aes_{128,256}_gcm.{encrypt,decrypt}`
- Tor v3: onion address, ntor handshake, cell encrypt/decrypt, KDF
- Semantic aliases: `crypto.sign`, `crypto.verify`, `crypto.encrypt`, `crypto.decrypt`, `crypto.hash`, `crypto.hmac`, `crypto.generate_keypair`, `crypto.derive_secret`
- Lineage queries: `lineage.list`, `lineage.verify`, `lineage.get`
- Purpose keys: `crypto.derive_purpose_key`, `crypto.derive_public_key`, `crypto.seed_fingerprint`
- Namespaced: `beardog.crypto.*` (13 methods), `crypto.ed25519.*` (2)

### **2. Ionic Bond** (12 methods — `IonicBondHandler`)
- Lifecycle: `crypto.ionic_bond.propose`, `.accept`, `.seal`, `.verify`, `.verify_proposal`, `.revoke`, `.list`
- Contract signing: `crypto.sign_contract`, `crypto.verify_contract`
- Cross-family contracts: `crypto.contract.propose`, `.countersign`, `.verify`

### **3. BTSP Tunnel** (36 methods — `BtspHandler`)
- Semantic surface: `btsp.contact.exchange`, `btsp.tunnel.{establish,encrypt,decrypt,status,close}`
- Legacy aliases: `btsp.exchange_contacts`, `btsp.establish`, `btsp.encrypt`, `btsp.decrypt`, `btsp.status`, `btsp.close`
- Server surface: `btsp.server.*`, `btsp.configure_tls`, `btsp.verify_peer`, `btsp.negotiate`, `btsp.tunnel_send_http`

### **4. Security** (19 methods — `SecurityHandler`)
- Trust evaluation, consent management, birdsong verification
- JWT: `security.generate_jwt_secret`, `security.jwt_secret`
- Lineage: `security.verify_lineage`, `security.evaluate`, `security.evaluate_trust`
- Namespaced: `beardog.generate_jwt_secret`, `beardog.jwt_secret`, `trust.*`

### **5. Capabilities & Introspection** (10 + 3 methods — `CapabilitiesHandler` + `IntrospectionHandler`)
- Discovery: `capabilities.list`, `capability.list`, `discover_capabilities`, `get_capabilities`
- Identity: `identity.get`, `get_identity`, `primal.capabilities`, `primal.info`
- Introspection: `rpc.methods`, `whoami`, `identity`

### **6. Health** (7 methods — `HealthHandler`)
- `health.liveness`, `health.readiness`, `health.check`
- Legacy: `ping`, `health`, `status`, `check`

### **7. Beacon (Dark Forest)** (7 methods — `BeaconHandler`)
- `beacon.generate`, `beacon.get_id`, `beacon.encrypt`, `beacon.try_decrypt`
- `beacon.try_decrypt_any`, `beacon.list_known`, `beacon.add_known`

### **8. Secret Storage** (4 methods — `SecretsHandler`)
- `secrets.store`, `secrets.retrieve`, `secrets.list`, `secrets.delete`

### **9. Graph Security** (3 methods — `GraphSecurityHandler`)
- `graph.validate_template`, `graph.audit_origin`, `graph.authorize_modification`

### **10. FIDO2** (3 methods — `Fido2Handler`)
- `beardog.fido2.discover`, `beardog.fido2.register`, `beardog.fido2.authenticate`

### **11. Encryption** (2 methods — `EncryptionHandler`)
- `encryption.encrypt`, `encryption.decrypt`

### **12. Federation** (2 methods — `FederationHandler`)
- `federation.verify_family_member`, `federation.derive_subfed_key`

### **13. Relay** (1 method — `RelayHandler`)
- `relay.authorize` — Lineage-gated relay authorization

### **14. Auth & Ionic Token Lifecycle** (7 methods — pre-dispatch gate, JH-0/JH-1/JH-11)
- `auth.check` — caller authentication status (includes validated claims)
- `auth.mode` — enforcement mode (permissive/enforced)
- `auth.peer_info` — peer credential inspection (SO_PEERCRED on Unix)
- `auth.issue_ionic` — issue Ed25519-signed ionic capability token
- `auth.verify_ionic` — verify ionic token, return claims or error
- `auth.issue_session` — issue scoped session token (TTL-aware, `content.*` scope)
- `auth.public_key` — return primal's Ed25519 public key

### **15. Identity** (1 method — pre-dispatch gate)
- `identity.create` — generate ephemeral Ed25519 caller keypair + DID

### **Route Aliases** (13 — remapped before dispatch)
- 8 bare-name crypto aliases: `sign_ed25519`, `verify_ed25519`, `x25519_*`, `chacha20_poly1305_*`, `hmac_sha256`, `blake3_hash`
- 5 bonding aliases: `bonding.propose`, `bonding.accept`, `bonding.status`, `bonding.terminate`, `bonding.modify_scope`

═══════════════════════════════════════════════════════════════════

## 🔐 **CORE CRYPTOGRAPHY**

### **Ed25519 Signature**

**Method**: `crypto.sign_ed25519`  
**Semantic alias**: `crypto.sign` (routes to the same handler)  
**Additional aliases**: `crypto.ed25519.sign`, `beardog.crypto.sign_ed25519`

**Key model**: BearDog derives Ed25519 keys internally from `key_id` + `purpose` via BLAKE3 KDF. Callers never supply private key material. The same `(key_id, purpose)` pair always produces the same keypair.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "crypto.sign",
  "params": {
    "message": "base64_encoded_bytes_to_sign",
    "key_id": "default_signing_key",
    "purpose": "general"
  },
  "id": 1
}
```

| Param | Required | Default | Description |
|-------|----------|---------|-------------|
| `message` | **Yes** | — | Standard Base64 of the raw bytes to sign |
| `key_id` | No | `"default_signing_key"` | Key identifier for deterministic derivation |
| `purpose` | No | `"general"` | Purpose qualifier for key derivation context |

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "signature": "base64_encoded_signature_64_bytes",
    "algorithm": "Ed25519",
    "key_id": "default_signing_key",
    "public_key": "base64_encoded_public_key_32_bytes"
  },
  "id": 1
}
```

**Signing contract**: BearDog signs the decoded bytes of `message` using Ed25519 (RFC 8032). It has no knowledge of what the bytes represent — callers are responsible for domain separation. For cross-primal workflows (e.g. LoamSpine entry signing, RootPulse commit signing), the orchestrator should encode the structured data as bytes and pass them via `message`. BearDog does not accept a `data` or `did` parameter on this method.

**Performance**: ~50-100μs

---

### **DID Key Derivation**

**Method**: `crypto.did_from_key`

Derives a W3C `did:key` identifier from a `BearDog` Ed25519 signing key. Uses the multicodec Ed25519 prefix (`0xed01`) + 32-byte public key, base58btc-encoded with `z` multibase prefix.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "crypto.did_from_key",
  "params": {
    "key_id": "default_signing_key",
    "purpose": "general"
  },
  "id": 1
}
```

| Param | Required | Default | Description |
|-------|----------|---------|-------------|
| `key_id` | No | `"default_signing_key"` | Key identifier for deterministic derivation |
| `purpose` | No | `"general"` | Purpose qualifier for key derivation context |

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "did": "did:key:z6MkhaXg...",
    "public_key": "base64_encoded_public_key_32_bytes",
    "algorithm": "Ed25519",
    "key_id": "default_signing_key"
  },
  "id": 1
}
```

**Cross-primal signing workflow (RP-5 clarification)**:

For workflows like RootPulse commit signing or LoamSpine entry signing:

1. Orchestrator calls `crypto.did_from_key` to obtain the `committer` DID
2. Orchestrator calls `crypto.sign` with entry/commit bytes base64-encoded in `message`
3. Orchestrator passes `signature` and `did` to LoamSpine's `entry.append` / `session.commit`

`BearDog` signs raw bytes and has no knowledge of LoamSpine entry formats. The orchestrator (biomeOS graph or the calling primal) is responsible for serializing structured data into bytes for signing.

---

### **ChaCha20-Poly1305 Encryption**

**Method**: `crypto.chacha20_poly1305_encrypt`

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "crypto.chacha20_poly1305_encrypt",
  "params": {
    "key": "hex_encoded_key_32_bytes",
    "plaintext": "base64_encoded_plaintext"
  },
  "id": 2
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "ciphertext": "base64_encoded_ciphertext",
    "nonce": "base64_encoded_nonce_12_bytes",
    "tag": "base64_encoded_tag_16_bytes"
  },
  "id": 2
}
```

**Performance**: ~500-800μs per 1KB

---

### **BLAKE3 Hashing**

**Method**: `crypto.blake3_hash`

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "crypto.blake3_hash",
  "params": {
    "data": "base64_encoded_data"
  },
  "id": 3
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "hash": "hex_encoded_hash_32_bytes"
  },
  "id": 3
}
```

**Performance**: ~300-500μs per 1KB

═══════════════════════════════════════════════════════════════════

## 🧬 **GENETIC CRYPTOGRAPHY**

### **Derive Lineage Key**

**Method**: `genetic.derive_lineage_key`

**Purpose**: Derive symmetric key from family lineage for encrypted communication

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "genetic.derive_lineage_key",
  "params": {
    "our_family_id": "family",
    "peer_family_id": "peer",
    "context": "session-id-12345",
    "lineage_seed": "base64_encoded_family_seed"
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "key": "base64_encoded_key_32_bytes",
    "method": "Blake3-Lineage-KDF",
    "quality_score": 0.8
  },
  "id": 1
}
```

**Performance**: ~500μs

---

### **Derive Lineage Beacon Key** (TRUE Dark Forest)

**Method**: `genetic.derive_lineage_beacon_key`

**Purpose**: Derive dedicated key for BirdSong beacons (pure noise, zero metadata)

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "genetic.derive_lineage_beacon_key",
  "params": {
    "lineage_seed": "base64_encoded_seed"
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "beacon_key": "hex_encoded_key_64_chars",
    "algorithm": "HKDF-SHA256+ChaCha20-Poly1305",
    "domain": "birdsong_beacon_v1",
    "key_size_bytes": 32,
    "deterministic": true,
    "purpose": "TRUE Dark Forest beacon encryption (zero metadata)"
  },
  "id": 1
}
```

**Security**:
- Domain-separated from other genetic keys
- Deterministic (same lineage = same key)
- All family members derive identical keys
- Enables pure noise beacons (indistinguishable from random)

**Performance**: ~100μs

---

### **Mix Entropy** (Three-Tier Hierarchy)

**Method**: `genetic.mix_entropy`

**Purpose**: Mix entropy from multiple tiers for enhanced security

**Tiers**:
1. **Tier 3**: Human Lived Experience (highest quality)
2. **Tier 2**: Human Supervised Machine
3. **Tier 1**: Store Bought Machine (default)

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "genetic.mix_entropy",
  "params": {
    "tier3_human": "base64_human_entropy",
    "tier2_supervised": "base64_supervised_entropy",
    "tier1_machine": null
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "entropy": "base64_encoded_mixed_32_bytes",
    "quality_score": 0.9,
    "tiers_used": 3
  },
  "id": 1
}
```

**Quality Scores**:
- Tier 1 only: 0.4
- Tier 1+2: 0.6
- Tier 1+2+3: 0.9-1.0

**Performance**: ~200μs

---

### **Generate Challenge** (Dark Forest Authentication)

**Method**: `genetic.generate_challenge`

**Purpose**: Generate cryptographic challenge for lineage verification

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "genetic.generate_challenge",
  "params": {
    "challenger_node_id": "usb_node1",
    "target_family_id": "pixel_tower"
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "nonce": "hex_encoded_nonce_32_bytes",
    "challenge_id": "uuid_v4",
    "challenger": "usb_node1",
    "target": "pixel_tower"
  },
  "id": 1
}
```

**Performance**: ~100μs

---

### **Respond to Challenge**

**Method**: `genetic.respond_to_challenge`

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "genetic.respond_to_challenge",
  "params": {
    "nonce": "hex_encoded_nonce",
    "our_family_seed_path": "/path/to/.family.seed",
    "our_node_id": "pixel_node1"
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "response": "hex_encoded_hmac_sha512",
    "lineage_proof": "base64_encoded_proof",
    "seed_hash_prefix": "hex_16_bytes",
    "responder_node_id": "pixel_node1"
  },
  "id": 1
}
```

**Performance**: ~500μs

═══════════════════════════════════════════════════════════════════

## 🔒 **TLS/HTTPS SUPPORT**

### **Derive Handshake Secrets**

**Method**: `tls.derive_secrets`

**Purpose**: HKDF-based key derivation for TLS 1.3 handshake

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "tls.derive_secrets",
  "params": {
    "shared_secret": "hex_encoded_ecdh_secret",
    "cipher_suite": "TLS_CHACHA20_POLY1305_SHA256",
    "hello_hash": "hex_encoded_sha256"
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "client_handshake_traffic_secret": "hex_encoded_32_bytes",
    "server_handshake_traffic_secret": "hex_encoded_32_bytes"
  },
  "id": 1
}
```

**Cipher Suites Supported**:
- `TLS_CHACHA20_POLY1305_SHA256`
- `TLS_AES_256_GCM_SHA384`
- `TLS_AES_128_GCM_SHA256`

**Performance**: ~200μs

═══════════════════════════════════════════════════════════════════

## 🔑 **PASSWORD HASHING**

### **Argon2id Hash** (OWASP Recommended)

**Method**: `crypto.argon2id_hash`

**Purpose**: Memory-hard password hashing (recommended for new systems)

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "crypto.argon2id_hash",
  "params": {
    "password": "base64_encoded_password",
    "memory_kb": 19456,
    "iterations": 2,
    "parallelism": 1
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "hash": "argon2id$v=19$m=19456,t=2,p=1$...",
    "algorithm": "argon2id",
    "version": 19
  },
  "id": 1
}
```

**Security**:
- Memory-hard (19MB default)
- Resistant to GPU/ASIC attacks
- OWASP recommended parameters

**Performance**: ~100-200ms (intentionally slow)

---

### **Argon2id Verify**

**Method**: `crypto.argon2id_verify`

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "crypto.argon2id_verify",
  "params": {
    "password": "base64_encoded_password",
    "hash": "argon2id$v=19$m=19456,t=2,p=1$..."
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "valid": true
  },
  "id": 1
}
```

**Security**: Constant-time comparison (timing-attack resistant)

═══════════════════════════════════════════════════════════════════

## 🗄️ **SECRET STORAGE**

### **Store Secret**

**Method**: `secrets.store`

**Purpose**: Encrypt and store a named secret using family-scoped key (HKDF-SHA256 + ChaCha20-Poly1305)

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "secrets.store",
  "params": {
    "name": "api_key_prod",
    "value": "base64_encoded_secret_value",
    "metadata": {"service": "payment-gateway"}
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "stored": true,
    "name": "api_key_prod",
    "encrypted_size_bytes": 128
  },
  "id": 1
}
```

**Security**: Per-secret encryption key derived via HKDF-SHA256 from family seed + secret name. Stored as ChaCha20-Poly1305 ciphertext. Only same-family nodes can decrypt.

---

### **Retrieve Secret**

**Method**: `secrets.retrieve`

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "secrets.retrieve",
  "params": {
    "name": "api_key_prod"
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "name": "api_key_prod",
    "value": "base64_encoded_decrypted_value",
    "metadata": {"service": "payment-gateway"}
  },
  "id": 1
}
```

---

### **List Secrets**

**Method**: `secrets.list`

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "secrets.list",
  "params": {},
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "secrets": ["api_key_prod", "db_password", "signing_key"],
    "count": 3
  },
  "id": 1
}
```

**Security**: Returns names only, never values.

---

### **Delete Secret**

**Method**: `secrets.delete`

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "secrets.delete",
  "params": {
    "name": "api_key_prod"
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "deleted": true,
    "name": "api_key_prod"
  },
  "id": 1
}
```

═══════════════════════════════════════════════════════════════════

## 🔦 **DARK FOREST BEACON**

### **Generate Beacon**

**Method**: `beacon.generate`

**Purpose**: Generate a beacon seed for Dark Forest discovery — pure noise, zero metadata leakage

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "beacon.generate",
  "params": {},
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "beacon_id": "hex_encoded_public_beacon_id",
    "generated": true
  },
  "id": 1
}
```

---

### **Encrypt Beacon**

**Method**: `beacon.encrypt`

**Purpose**: Encrypt payload with beacon seed — output indistinguishable from random noise

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "beacon.encrypt",
  "params": {
    "plaintext": "base64_encoded_payload"
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "ciphertext": "base64_encoded_noise",
    "beacon_id": "hex_encoded_beacon_id"
  },
  "id": 1
}
```

**Security**: Ciphertext is pure noise — no headers, no magic bytes, no structure.

---

### **Try Decrypt Beacon**

**Method**: `beacon.try_decrypt`

**Purpose**: Attempt to decrypt beacon with our seed. Returns null if not our beacon.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "beacon.try_decrypt",
  "params": {
    "ciphertext": "base64_encoded_noise"
  },
  "id": 1
}
```

**Response** (success):
```json
{
  "jsonrpc": "2.0",
  "result": {
    "decrypted": true,
    "plaintext": "base64_encoded_payload"
  },
  "id": 1
}
```

**Response** (not our beacon):
```json
{
  "jsonrpc": "2.0",
  "result": {
    "decrypted": false,
    "plaintext": null
  },
  "id": 1
}
```

═══════════════════════════════════════════════════════════════════

## 🔗 **IONIC BOND** (IonicBondHandler — 12 methods)

Cross-tower and cross-family trust establishment via Ed25519. The
propose → accept → seal lifecycle creates cryptographically verifiable
bonds between domains. Contract signing enables programmatic trust for
multi-family deployments (healthSpring, hotSpring dual-tower bonds;
wetSpring cross-spring provenance).

### **Propose Bond**

```json
{
  "jsonrpc": "2.0",
  "method": "crypto.ionic_bond.propose",
  "params": {
    "proposer": "tower_a",
    "target": "tower_b",
    "trust_model": "btsp_enforced",
    "encryption_tier": "aead",
    "allowed_capabilities": ["crypto"],
    "ttl_seconds": 3600
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "proposal_id": "...",
    "terms_hash": "sha256hex...",
    "proposer_signature": "hex..."
  },
  "id": 1
}
```

### **Accept Bond**

```json
{
  "jsonrpc": "2.0",
  "method": "crypto.ionic_bond.accept",
  "params": {
    "proposal_id": "...",
    "acceptor": "tower_b",
    "acceptor_signature": "hex (128 chars, Ed25519 over terms_hash)",
    "acceptor_public_key": "hex (64 chars, Ed25519 public key)"
  },
  "id": 2
}
```

**Response**: `{ "bond": { IonicBond object } }`

### **Seal Bond**

Cryptographically seals an active bond by re-verifying both signatures.
Third step in the propose → accept → seal lifecycle — confirms the bond
is enforcement-ready.

```json
{
  "jsonrpc": "2.0",
  "method": "crypto.ionic_bond.seal",
  "params": {
    "bond_id": "...",
    "sealer": "tower_a"
  },
  "id": 3
}
```

**Response**: `{ "sealed": true, "bond": { IonicBond with state "sealed" } }`

### **Verify Bond**

```json
{
  "jsonrpc": "2.0",
  "method": "crypto.ionic_bond.verify",
  "params": { "bond_id": "..." },
  "id": 4
}
```

**Response**: `{ "valid": true/false, "state": "active"|"sealed"|"revoked", "bond": {...} }`

### **Revoke Bond**

```json
{
  "jsonrpc": "2.0",
  "method": "crypto.ionic_bond.revoke",
  "params": { "bond_id": "...", "revoker": "tower_a" },
  "id": 5
}
```

**Response**: `{ "revoked": true }`

### **List Bonds**

```json
{
  "jsonrpc": "2.0",
  "method": "crypto.ionic_bond.list",
  "params": { "domain": "tower_a", "state": "active" },
  "id": 6
}
```

Both `domain` and `state` are optional filters. **Response**: `{ "bonds": [ IonicBond, ... ] }`

### **Sign Contract**

Signs an arbitrary contract document with BearDog's Ed25519 identity.
Terms are serialized canonically (sorted keys), SHA-256 hashed, then
signed. Enables cross-family trust for dual-tower compositions
(healthSpring, hotSpring) and cross-spring provenance (wetSpring).

```json
{
  "jsonrpc": "2.0",
  "method": "crypto.sign_contract",
  "params": {
    "signer": "hotSpring",
    "terms": {
      "federation": "cern_grid",
      "parties": ["family_a", "family_b"],
      "gpu_lease": { "max_hours": 1000 }
    },
    "context": "gpu_lease"
  },
  "id": 7
}
```

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `signer` | string | yes | Identity of the signing party |
| `terms` | object | yes | Contract terms (canonicalized before hashing) |
| `context` | string | no | Optional scope label (e.g. `"gpu_lease"`, `"ionic_bond"`) |

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "terms_hash": "sha256hex...",
    "signature": "base64 (Ed25519, 64 bytes decoded)",
    "public_key": "base64 (Ed25519, 32 bytes decoded)",
    "signed_at": "2026-05-07T21:00:00Z"
  },
  "id": 7
}
```

### **Verify Contract**

Verifies an Ed25519 signature over a contract terms hash. Any party
can verify without access to the signing key.

```json
{
  "jsonrpc": "2.0",
  "method": "crypto.verify_contract",
  "params": {
    "terms_hash": "sha256hex...",
    "signature": "hex (Ed25519)",
    "public_key": "hex (Ed25519)"
  },
  "id": 8
}
```

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `terms_hash` | string | yes | SHA-256 hex of the canonical contract terms |
| `signature` | string | yes | Ed25519 signature to verify (hex) |
| `public_key` | string | yes | Ed25519 public key of the claimed signer (hex) |

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "valid": true,
    "error": null
  },
  "id": 8
}
```

### **Cross-Spring Signing Workflow**

For healthSpring/hotSpring dual-tower bonds and wetSpring provenance:

1. **Sign**: `crypto.sign_contract` with terms JSON → receive `terms_hash`, `signature`, `public_key`
2. **Distribute**: Share `terms_hash` + `signature` + `public_key` to counterparty via any channel
3. **Verify**: Counterparty calls `crypto.verify_contract` with all three fields
4. **Bond**: Optionally establish an ionic bond with the same parties for ongoing trust enforcement

═══════════════════════════════════════════════════════════════════

## 🔀 **RELAY AUTHORIZATION**

### **Authorize Relay**

**Method**: `relay.authorize`

**Purpose**: Lineage-gated authorization for relay-assisted NAT traversal. BearDog acts as the cryptographic authority — verifies genetic lineage before authorizing relay access.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "relay.authorize",
  "params": {
    "requester_node_id": "pixel_node2",
    "requester_family_id": "pixel_tower_family",
    "lineage_proof": "base64_encoded_blake3_proof"
  },
  "id": 1
}
```

**Response** (authorized — same family):
```json
{
  "jsonrpc": "2.0",
  "result": {
    "authorized": true,
    "masking_level": "transparent",
    "ttl_seconds": 300,
    "reason": "family_member",
    "our_family_id": "pixel_tower_family",
    "provider": "beardog"
  },
  "id": 1
}
```

**Response** (authorized — different family, valid lineage proof):
```json
{
  "jsonrpc": "2.0",
  "result": {
    "authorized": true,
    "masking_level": "standard",
    "ttl_seconds": 120,
    "reason": "lineage_verified",
    "our_family_id": "pixel_tower_family",
    "provider": "beardog"
  },
  "id": 1
}
```

**Response** (denied):
```json
{
  "jsonrpc": "2.0",
  "result": {
    "authorized": false,
    "masking_level": "blocked",
    "ttl_seconds": 0,
    "reason": "lineage_verification_failed",
    "our_family_id": "pixel_tower_family",
    "provider": "beardog"
  },
  "id": 1
}
```

**Authorization Logic**:
1. **Same `family_id`** → authorized, masking `"transparent"`, TTL 300s
2. **Different family + valid Blake3 lineage proof** → authorized, masking `"standard"`, TTL 120s
3. **Different family + invalid/missing proof** → denied, masking `"blocked"`, TTL 0

**Integration**: Songbird's relay server calls this via Neural API → routes to BearDog → BearDog verifies lineage → returns authorization decision. BearDog never touches a network socket.

**Performance**: ~200μs (Blake3 verification)

═══════════════════════════════════════════════════════════════════

## 📡 **DISCOVERY PROTOCOL**

### **How Primals Find BearDog**

**1. Unix Socket Discovery** (Primary):
```bash
# Standard path (XDG-compliant)
/run/user/$UID/biomeos/beardog.sock

# Fallback paths
~/.local/share/biomeos/beardog.sock
/tmp/biomeos.$UID/beardog.sock
```

**2. TCP Fallback Discovery** (Android):
```bash
# Discovery file contains: tcp:127.0.0.1:PORT
~/.config/biomeos/beardog.sock

# Example content:
tcp:127.0.0.1:8765
```

**3. Dark Forest Discovery** (Encrypted):
```
BirdSong beacons (UDP broadcast):
  • Format: Pure noise (indistinguishable from random)
  • Encryption: ChaCha20-Poly1305 with beacon key
  • Only family members can decrypt
  • Contains: node_id, socket_path, capabilities
```

### **Capability Advertisement**

BearDog advertises capabilities via Dark Forest beacons:

```json
{
  "node_id": "beardog_usb_node1",
  "socket_path": "/run/user/1000/biomeos/beardog.sock",
  "capabilities": [
    "crypto.ed25519",
    "crypto.chacha20_poly1305",
    "crypto.blake3",
    "genetic.lineage",
    "genetic.beacon",
    "hsm.strongbox",
    "tls.1.3",
    "secrets.store",
    "secrets.retrieve",
    "secrets.list",
    "secrets.delete",
    "beacon.generate",
    "beacon.encrypt",
    "beacon.try_decrypt",
    "relay.authorize"
  ],
  "timestamp": 1738531200
}
```

═══════════════════════════════════════════════════════════════════

## 🔧 **ERROR HANDLING**

### **Error Response Format**

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32600,
    "message": "Invalid Request",
    "data": {
      "details": "Missing required parameter: 'data'"
    }
  },
  "id": 1
}
```

### **Error Codes**

| Code | Meaning | Description |
|------|---------|-------------|
| -32700 | Parse error | Invalid JSON |
| -32600 | Invalid Request | Missing/invalid params |
| -32601 | Method not found | Unknown method |
| -32602 | Invalid params | Parameter validation failed |
| -32603 | Internal error | Crypto operation failed |
| -32000 | Unauthorized | Caller identity could not be established (invalid/expired token) |
| -32001 | Permission denied | Caller lacks scope for the requested method (`MethodGate` enforcement) |
| -32002 | Not ready | Primal not yet initialized |

### **Best Practices**

1. **Always check `error` field** before accessing `result`
2. **Handle timeout** (default 30s for crypto operations)
3. **Retry logic** for transient errors (-32603)
4. **Don't retry** authentication failures (-32000, -32001)

═══════════════════════════════════════════════════════════════════

## 📊 **PERFORMANCE GUARANTEES**

| Operation Category | Target Latency | Notes |
|-------------------|----------------|-------|
| Ed25519 signing | < 100μs | Fastest signature |
| X25519 key exchange | < 200μs | DH key agreement |
| ChaCha20 encryption | < 1ms | Per 1KB payload |
| BLAKE3 hashing | < 500μs | Per 1KB payload |
| Lineage key derivation | < 500μs | Blake3 KDF |
| Beacon key derivation | < 100μs | HKDF-SHA256 |
| Argon2id hashing | 100-200ms | Intentionally slow (security) |
| TLS handshake | < 1ms | Full HKDF derivation |

### **Scalability**

- **Concurrent connections**: Unlimited (async Tokio)
- **Request queueing**: Automatic via Tokio runtime
- **Backpressure**: TCP flow control
- **Memory**: O(1) per request (streaming)

═══════════════════════════════════════════════════════════════════

## 🧪 **TESTING**

### **Integration Test Example**

```bash
#!/bin/bash
# Test BearDog crypto.blake3_hash

echo '{
  "jsonrpc":"2.0",
  "method":"crypto.blake3_hash",
  "params":{"data":"'$(echo -n "test" | base64)'"},
  "id":1
}' | nc -U /run/user/$(id -u)/biomeos/beardog.sock

# Expected response:
{
  "jsonrpc":"2.0",
  "result":{
    "hash":"4878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215"
  },
  "id":1
}
```

### **Client Library Examples**

**Rust**:
```rust
use tokio::net::UnixStream;
use serde_json::json;

let socket_path = "/run/user/1000/biomeos/beardog.sock";
let mut stream = UnixStream::connect(socket_path).await?;

let request = json!({
    "jsonrpc": "2.0",
    "method": "crypto.blake3_hash",
    "params": {"data": base64::encode(b"test")},
    "id": 1
});

stream.write_all(serde_json::to_vec(&request)?.as_slice()).await?;
// Read response...
```

**Python**:
```python
import socket
import json
import base64

sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
sock.connect('/run/user/1000/biomeos/beardog.sock')

request = {
    "jsonrpc": "2.0",
    "method": "crypto.blake3_hash",
    "params": {"data": base64.b64encode(b"test").decode()},
    "id": 1
}

sock.sendall(json.dumps(request).encode())
response = json.loads(sock.recv(4096).decode())
print(response['result']['hash'])
```

═══════════════════════════════════════════════════════════════════

## 📚 **COMPLETE METHOD INDEX**

> **SSOT**: Call `rpc.methods` or `capabilities.list` at runtime for the live inventory.
> This index documents the primary method surface. Semantic aliases, `beardog.crypto.*`
> namespaced variants, and legacy bare-name aliases are listed in the category breakdown above.

### **Signatures & Key Exchange (14 methods)**

1. `crypto.ed25519_generate_keypair` — Ed25519 keypair generation
2. `crypto.sign_ed25519` — Ed25519 signature
3. `crypto.verify_ed25519` — Ed25519 verification
4. `crypto.sign_ecdsa_secp256r1` — ECDSA P-256 signing
5. `crypto.verify_ecdsa_secp256r1` — ECDSA P-256 verification
6. `crypto.sign_ecdsa_secp384r1` — ECDSA P-384 signing
7. `crypto.verify_ecdsa_secp384r1` — ECDSA P-384 verification
8. `crypto.sign_rsa_pkcs1_sha256` — RSA PKCS#1 signing
9. `crypto.verify_rsa_pkcs1_sha256` — RSA PKCS#1 verification
10. `crypto.sign_rsa_pss_sha256` — RSA-PSS signing
11. `crypto.verify_rsa_pss_sha256` — RSA-PSS verification
12. `crypto.x25519_generate_ephemeral` — X25519 keypair generation
13. `crypto.x25519_derive_secret` — X25519 key exchange
14. `crypto.ecdh_p256_generate` — P-256 ECDH keypair

### **ECDH & ECDHE (6 methods)**

15. `crypto.ecdh_p256_derive` — P-256 key exchange
16. `crypto.ecdh_p384_generate` — P-384 ECDH keypair
17. `crypto.ecdh_p384_derive` — P-384 key exchange
18. `crypto.ecdhe.p256.generate` — TLS ephemeral P-256
19. `crypto.ecdhe.p256.compute_shared` — TLS ephemeral P-256 shared
20. `crypto.ecdhe.p384.generate` — TLS ephemeral P-384

### **Encryption (10 methods)**

21. `crypto.chacha20_poly1305_encrypt` — ChaCha20-Poly1305 encryption
22. `crypto.chacha20_poly1305_decrypt` — ChaCha20-Poly1305 decryption
23. `crypto.aes256_gcm_encrypt` — AES-256-GCM encryption
24. `crypto.aes256_gcm_decrypt` — AES-256-GCM decryption
25. `crypto.aes128_gcm_encrypt` — AES-128-GCM encryption
26. `crypto.aes128_gcm_decrypt` — AES-128-GCM decryption
27. `crypto.aead.aes_128_gcm.encrypt` — TLS-style AEAD encrypt
28. `crypto.aead.aes_128_gcm.decrypt` — TLS-style AEAD decrypt
29. `crypto.aead.aes_256_gcm.encrypt` — TLS-style AEAD encrypt
30. `crypto.aead.aes_256_gcm.decrypt` — TLS-style AEAD decrypt

### **Hashing & HMAC (14 methods)**

31. `crypto.blake3_hash` — BLAKE3 hashing
32. `crypto.sha256` — SHA-256
33. `crypto.sha384` — SHA-384
34. `crypto.sha512` — SHA-512
35. `crypto.sha1` — SHA-1 (legacy compat)
36. `crypto.sha3_256` — SHA3-256
37. `crypto.hmac_sha256` — HMAC-SHA256
38. `crypto.hmac_sha384` — HMAC-SHA384
39. `crypto.hmac_sha512` — HMAC-SHA512
40. `crypto.hmac_blake3` — HMAC-BLAKE3
41. `crypto.hmac_verify` — Constant-time HMAC verification
42. `crypto.hash_for_cipher` — Cipher-appropriate hash
43. `crypto.hkdf_sha256` — HKDF-SHA256 key derivation
44. `crypto.hash` — Semantic alias (routes by algorithm)

### **KDF & Password Hashing (6 methods)**

45. `crypto.argon2id_hash` — Argon2id password hashing
46. `crypto.argon2id_verify` — Argon2id verification
47. `crypto.pbkdf2_sha256` — PBKDF2 key derivation
48. `crypto.bcrypt_hash` — bcrypt hashing
49. `crypto.bcrypt_verify` — bcrypt verification
50. `crypto.scrypt` — scrypt key derivation

### **TLS 1.3 (5 methods)**

51. `tls.derive_secrets` — Handshake secret derivation
52. `tls.derive_handshake_secrets` — Explicit handshake secrets
53. `tls.derive_application_secrets` — Application secret derivation
54. `tls.compute_finished_verify_data` — Finished message verify data
55. `tls.sign_handshake` — Handshake signing (Ed25519)

### **TLS 1.2 (1 method)**

56. `crypto.kdf.tls12_prf` — TLS 1.2 PRF derivation

### **Genetic Cryptography (11 methods)**

57. `genetic.derive_lineage_key` — Lineage-based key derivation
58. `genetic.derive_lineage_beacon_key` — Beacon key (TRUE Dark Forest)
59. `genetic.mix_entropy` — Three-tier entropy mixing
60. `genetic.verify_lineage` — Lineage verification
61. `genetic.generate_lineage_proof` — Proof generation
62. `genetic.generate_challenge` — Challenge generation
63. `genetic.respond_to_challenge` — Challenge response
64. `genetic.verify_challenge_response` — Response verification
65. `genetic.sign_lineage_certificate` — Lineage certificate signing
66. `genetic.verify_lineage_certificate` — Certificate verification
67. `genetic.derive_device_seed` — Device seed derivation (parent→child)

### **Lineage Queries (3 methods)**

68. `lineage.list` — List lineage entries
69. `lineage.verify` — Verify lineage chain
70. `lineage.get` — Get lineage details

### **Tor v3 Onion (8 methods — `beardog.crypto.*` namespace)**

71. `crypto.derive_onion_address` — Derive .onion from Ed25519
72. `beardog.crypto.generate_onion_identity` — Generate keypair + .onion
73. `beardog.crypto.tor_ntor_client_init` — ntor handshake client init
74. `beardog.crypto.tor_ntor_server_respond` — ntor server respond
75. `beardog.crypto.tor_ntor_client_finish` — ntor client finish
76. `beardog.crypto.tor_cell_encrypt` — Cell encryption (ChaCha20)
77. `beardog.crypto.tor_cell_decrypt` — Cell decryption
78. `beardog.crypto.tor_kdf` — HKDF-SHA256 key expansion

### **Purpose Keys & Semantic Aliases (9 methods)**

79. `crypto.derive_purpose_key` — Purpose-scoped key derivation
80. `crypto.derive_public_key` — Derive public key from purpose key
81. `crypto.seed_fingerprint` — Seed fingerprint
82. `crypto.sign_registration` — Signed registration payload
83. `crypto.generate_keypair` — Semantic → ed25519_generate_keypair
84. `crypto.derive_secret` — Semantic → x25519_derive_secret
85. `crypto.public_key` — Get public key
86. `crypto.encrypt` — Semantic → chacha20_poly1305_encrypt
87. `crypto.decrypt` — Semantic → chacha20_poly1305_decrypt

### **Ionic Bond (12 methods — `IonicBondHandler`)**

88. `crypto.ionic_bond.propose` — Propose a cross-domain bond
89. `crypto.ionic_bond.accept` — Accept a bond proposal (Ed25519)
90. `crypto.ionic_bond.seal` — Cryptographically seal an active bond
91. `crypto.ionic_bond.verify` — Verify bond state and signatures
92. `crypto.ionic_bond.verify_proposal` — Verify unsigned proposal
93. `crypto.ionic_bond.revoke` — Revoke an active or sealed bond
94. `crypto.ionic_bond.list` — List bonds (filterable by domain/state)
95. `crypto.sign_contract` — Sign contract terms with Ed25519
96. `crypto.verify_contract` — Verify contract signature
97. `crypto.contract.propose` — Cross-family contract proposal
98. `crypto.contract.countersign` — Countersign cross-family contract
99. `crypto.contract.verify` — Verify cross-family contract

### **Secret Storage (4 methods — `SecretsHandler`)**

100. `secrets.store` — Encrypt and store secret
101. `secrets.retrieve` — Decrypt and return secret
102. `secrets.list` — List stored secret names
103. `secrets.delete` — Remove a stored secret

### **Dark Forest Beacon (7 methods — `BeaconHandler`)**

104. `beacon.generate` — Generate beacon seed
105. `beacon.get_id` — Get public beacon ID
106. `beacon.encrypt` — Encrypt with beacon seed
107. `beacon.try_decrypt` — Decrypt with our seed
108. `beacon.try_decrypt_any` — Decrypt with any known beacon
109. `beacon.list_known` — List known beacon IDs
110. `beacon.add_known` — Add known beacon from meeting

### **Relay (1 method — `RelayHandler`)**

111. `relay.authorize` — Lineage-gated relay authorization

### **Federation (2 methods — `FederationHandler`)**

112. `federation.verify_family_member` — Verify genetic relationship
113. `federation.derive_subfed_key` — Derive sub-federation key

### **Security (19 methods — `SecurityHandler`)**

114. `security.evaluate_trust` — Trust evaluation
115. `security.evaluate` — Security evaluation
116. `security.verify_lineage` — Lineage verification
117. `security.generate_jwt_secret` — JWT secret generation
118. `security.jwt_secret` — Get JWT secret
119. `trust.evaluate` — Trust evaluation (alias)
120. `trust.evaluate_peer` — Peer trust evaluation
Plus 12 additional security/consent/birdsong methods (call `rpc.methods` for full list)

### **Graph Security (3 methods — `GraphSecurityHandler`)**

121. `graph.validate_template` — Validate security graph template
122. `graph.audit_origin` — Audit origin in security graph
123. `graph.authorize_modification` — Authorize graph modification

### **FIDO2 (3 methods — `Fido2Handler`)**

124. `beardog.fido2.discover` — Discover FIDO2 devices
125. `beardog.fido2.register` — Register FIDO2 credential
126. `beardog.fido2.authenticate` — Authenticate with FIDO2

### **Health (7 methods — `HealthHandler`)**

127. `health.liveness` — Liveness probe
128. `health.readiness` — Readiness probe
129. `health.check` — Health check
130. `ping` — Legacy ping
131. `health` — Legacy health
132. `status` — Legacy status
133. `check` — Legacy check

### **Capabilities & Introspection (13 methods — `CapabilitiesHandler` + `IntrospectionHandler`)**

134. `capabilities.list` — List all capabilities
135. `capability.list` — Alias
136. `discover_capabilities` — Discover capability categories
137. `get_capabilities` — Get capability details
138. `identity.get` — Primal identity
139. `get_identity` — Alias
140. `primal.capabilities` — Primal capability manifest
141. `primal.info` — Primal metadata
142. `rpc.methods` — List available RPC methods
143. `whoami` — Node identity
144. `identity` — Node identity alias

### **Encryption (2 methods — `EncryptionHandler`)**

145. `encryption.encrypt` — Generic encrypt
146. `encryption.decrypt` — Generic decrypt

### **Auth Gate (7 methods — pre-dispatch, `MethodGate`)**

147. `auth.check` — Caller authentication status
148. `auth.mode` — Enforcement mode (permissive/enforced)
149. `auth.peer_info` — Peer credential introspection (uid, pid)
150. `auth.issue_ionic` — Issue Ed25519-signed ionic capability token
151. `auth.verify_ionic` — Verify ionic token, return claims
152. `auth.issue_session` — Issue scoped session token (TTL-aware, `content.*` scope)
153. `auth.public_key` — Return primal's Ed25519 public key

### **Identity Gate (1 method — pre-dispatch)**

154. `identity.create` — Generate ephemeral Ed25519 caller keypair + DID

### **BTSP Tunnel (36 methods — `BtspHandler`)**

155–190. `btsp.contact.exchange`, `btsp.tunnel.{establish,encrypt,decrypt,status,close}`, `btsp.server.*`, `btsp.configure_tls`, `btsp.verify_peer`, `btsp.negotiate`, plus legacy aliases. Call `rpc.methods` for the full BTSP surface.

### **Summary**

**217 registry methods** + **9 pre-dispatch gate methods** = **226 dispatchable method names**, plus **13 route aliases** for backward compatibility. The `CryptoHandler` alone registers 108 methods across signatures, encryption, hashing, KDF, TLS, genetic, Tor, and semantic alias surfaces.

═══════════════════════════════════════════════════════════════════

## 🎯 **VERSIONING**

### **API Version**: 1.0.0

**Semantic Versioning**:
- **Major**: Breaking changes to existing methods
- **Minor**: New methods added (backward compatible)
- **Patch**: Bug fixes, performance improvements

**Version Discovery**: Use `primal.info` to retrieve version metadata, or `rpc.methods` for the full method inventory.

### **Deprecation Policy**

1. **Announce**: 1 minor version before removal
2. **Warning**: Return warning in response
3. **Remove**: Next major version

═══════════════════════════════════════════════════════════════════

## 🔐 **SECURITY CONSIDERATIONS**

### **Authentication**

BearDog supports multiple authentication layers (JH-0/JH-1/JH-11):
1. **Genetic lineage**: Family members share a `.family.seed` file; challenge-response proves knowledge
2. **Ionic capability tokens**: Ed25519-signed, scoped, TTL-aware tokens issued via `auth.issue_ionic`
3. **Session tokens**: Scoped session tokens (`content.*`, etc.) via `auth.issue_session`
4. **`MethodGate` enforcement**: `BEARDOG_AUTH_MODE=enforced` requires valid scope for protected methods

### **Authorization**

- **Permissive mode** (default): All methods accessible; tokens carry advisory claims
- **Enforced mode**: Protected methods require valid capability token with matching scope
- **Capabilities**: Self-declared in beacons and `primal.announce` payloads

### **Transport Security**

- **Unix sockets** (primary): Kernel-enforced access control (`SO_PEERCRED` introspection)
- **TCP** (opt-in): Via `--port`/`--listen` or `BEARDOG_TCP_IPC_PORT`; default port 9100
- **BTSP**: Encrypted tunnel transport with Ed25519 mutual authentication

### **Attack Surface**

- **Exposed**: Unix socket (local only)
- **Not exposed**: Network sockets (unless TCP fallback)
- **Mitigation**: Family lineage authentication

═══════════════════════════════════════════════════════════════════

## 📖 **REFERENCES**

### **Standards**

- JSON-RPC 2.0: https://www.jsonrpc.org/specification
- Ed25519: RFC 8032
- X25519: RFC 7748
- ChaCha20-Poly1305: RFC 8439
- BLAKE3: https://github.com/BLAKE3-team/BLAKE3-specs
- Argon2: RFC 9106
- TLS 1.3: RFC 8446

### **Implementation**

All handler paths are relative to `crates/beardog-tunnel/`:

- Crypto handlers: `src/unix_socket_ipc/handlers/crypto_handler/`
- Genetic handlers: `src/unix_socket_ipc/handlers/crypto_handler/genetic.rs`
- Ionic bond: `src/unix_socket_ipc/handlers/ionic_bond/`
- BTSP tunnel: `src/btsp_provider/`
- Security: `src/unix_socket_ipc/handlers/security.rs`
- FIDO2: `src/unix_socket_ipc/handlers/fido2.rs`
- Auth gate: `src/unix_socket_ipc/handlers/method_gate.rs`
- Handler registry: `src/unix_socket_ipc/handlers/mod.rs`

═══════════════════════════════════════════════════════════════════

**Document Version**: 4.0.0  
**Last Updated**: May 28, 2026  
**Maintainer**: BearDog Security Primal  
**License**: Documented interface (implementation MIT-licensed)

🔌 **Primal Contracts: Enabling Runtime Discovery & Composition**
