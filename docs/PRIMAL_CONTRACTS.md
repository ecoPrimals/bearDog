# 🔌 BearDog Primal Contracts - JSON-RPC API Specification

**Version**: 1.0.0  
**Date**: May 7, 2026  
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
- Protocol: JSON-RPC 2.0
- Encoding: UTF-8 JSON
- Authentication: Family lineage (genetic)

**Fallback**: TCP (Android/constraints)
- Discovered via: `~/.config/biomeos/beardog.sock` (contains `tcp:IP:PORT`)
- Protocol: Same JSON-RPC 2.0
- Use case: Android devices without Unix socket support

═══════════════════════════════════════════════════════════════════

## 📚 **API CATEGORIES**

BearDog provides **111 JSON-RPC methods** across 12 categories:

### **1. Core Cryptography** (20 methods)
- Signatures: Ed25519, ECDSA (P-256, P-384), RSA
- Key Exchange: X25519, ECDH (P-256, P-384)
- Encryption: ChaCha20-Poly1305, AES-GCM (128, 256)
- Hashing: BLAKE3, SHA-256, SHA-384, SHA-512
- Authentication: HMAC-SHA256

### **2. Genetic Cryptography** (11 methods)
- Lineage key derivation (family-based)
- Beacon key derivation (TRUE Dark Forest)
- Entropy mixing (3-tier: Human/Supervised/Machine)
- Lineage verification + proof generation
- Challenge-response authentication
- Lineage certificate enrollment + verification
- Device enrollment (parent→child derivation)

### **3. TLS/HTTPS Support** (4 methods)
- Handshake secret derivation (HKDF)
- Application secret derivation (HKDF)
- Certificate verification (X.509)
- Handshake signing (Ed25519)

### **4. Tor v3 Onion** (8 methods)
- Onion address derivation from Ed25519
- Identity generation (keypair + .onion)
- ntor handshake (client init, finish, server respond)
- Cell encryption/decryption (ChaCha20)
- HKDF-SHA256 key expansion

### **5. Secret Storage** (4 methods)
- `secrets.store` — Encrypt and store with family-scoped key
- `secrets.retrieve` — Decrypt and return secret
- `secrets.list` — List stored secret names (not values)
- `secrets.delete` — Remove a stored secret

### **6. Beacon (Dark Forest)** (7 methods)
- `beacon.generate` — Generate beacon seed
- `beacon.get_id` — Get public beacon ID
- `beacon.encrypt` — Encrypt with beacon seed
- `beacon.try_decrypt` — Decrypt with our seed
- `beacon.try_decrypt_any` — Decrypt with any known beacon
- `beacon.list_known` — List known beacon IDs
- `beacon.add_known` — Add known beacon from meeting

### **7. Relay Authorization** (1 method)
- `relay.authorize` — Lineage-gated relay authorization

### **8. Federation** (2 methods)
- `federation.verify_family_member` — Verify genetic relationship
- `federation.derive_subfed_key` — Derive sub-federation key

### **9. Password Hashing** (3 methods)
- Argon2id (OWASP recommended, memory-hard)
- PBKDF2-SHA256 (legacy compatibility)
- Constant-time verification

### **10. Security** (6 methods)
- Trust evaluation, JWT verification, graph security

### **11. Introspection** (6 methods)
- `discover_capabilities`, `capabilities`, `get_capabilities`
- `primal.info`, `rpc.methods`, `identity`/`whoami`

### **BTSP (Tunnel)** (6 methods)
- Contact exchange, tunnel establish/encrypt/decrypt/status/close

### **HSM Management** (11 methods)
- Key generation (ephemeral, persistent)
- Key storage (StrongBox on Android)
- Entropy management
- Session management

### **12. Ionic Bond** (8 methods — IonicBondHandler)
- Lifecycle: propose, accept, seal, verify, revoke, list
- Contract signing: `crypto.sign_contract`, `crypto.verify_contract`
- Cross-tower/cross-family trust establishment via Ed25519

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

## 🔗 **IONIC BOND** (IonicBondHandler — 8 methods)

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
| -32000 | HSM error | Hardware security module error |
| -32001 | Lineage error | Genetic verification failed |
| -32002 | Secret not found | Named secret does not exist |
| -32003 | Relay denied | Relay authorization rejected |
| -32004 | Beacon error | Beacon operation failed |

### **Best Practices**

1. **Always check `error` field** before accessing `result`
2. **Handle timeout** (default 30s for crypto operations)
3. **Retry logic** for transient errors (-32603)
4. **Don't retry** authentication failures (-32001)

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

### **Core Cryptography (20 methods)**

1. `crypto.sign_ed25519` - Ed25519 signature (params: `message`, `key_id`, `purpose`)
1a. `crypto.sign` - Semantic alias → `crypto.sign_ed25519`
1b. `crypto.did_from_key` - Derive `did:key` from Ed25519 signing key (params: `key_id`, `purpose`)
2. `crypto.verify_ed25519` - Ed25519 verification (params: `message`, `signature`, `public_key`)
3. `crypto.sign_ecdsa_secp256r1` - ECDSA P-256 signing
4. `crypto.verify_ecdsa_secp256r1` - ECDSA P-256 verification
5. `crypto.sign_ecdsa_secp384r1` - ECDSA P-384 signing
6. `crypto.verify_ecdsa_secp384r1` - ECDSA P-384 verification
7. `crypto.sign_rsa_pkcs1_sha256` - RSA PKCS#1 signing
8. `crypto.verify_rsa_pkcs1_sha256` - RSA PKCS#1 verification
9. `crypto.sign_rsa_pss_sha256` - RSA-PSS signing
10. `crypto.verify_rsa_pss_sha256` - RSA-PSS verification
11. `crypto.x25519_generate_ephemeral` - X25519 keypair generation
12. `crypto.x25519_derive_secret` - X25519 key exchange
13. `crypto.ecdh_p256_generate` - P-256 ECDH keypair
14. `crypto.ecdh_p256_derive` - P-256 key exchange
15. `crypto.chacha20_poly1305_encrypt` - ChaCha20 encryption
16. `crypto.chacha20_poly1305_decrypt` - ChaCha20 decryption
17. `crypto.aes256_gcm_encrypt` - AES-256-GCM encryption
18. `crypto.aes256_gcm_decrypt` - AES-256-GCM decryption
19. `crypto.blake3_hash` - BLAKE3 hashing
20. `crypto.hmac_sha256` - HMAC-SHA256

### **Genetic Cryptography (11 methods)**

21. `genetic.derive_lineage_key` - Lineage-based key derivation
22. `genetic.derive_lineage_beacon_key` - Beacon key (TRUE Dark Forest)
23. `genetic.mix_entropy` - Three-tier entropy mixing
24. `genetic.verify_lineage` - Lineage verification
25. `genetic.generate_lineage_proof` - Proof generation
26. `genetic.generate_challenge` - Challenge generation
27. `genetic.respond_to_challenge` - Challenge response
28. `genetic.verify_challenge_response` - Response verification
29. `genetic.enroll_lineage_certificate` - Lineage certificate enrollment
30. `genetic.verify_lineage_certificate` - Certificate verification
31. `genetic.enroll_device` - Device enrollment (parent→child)

### **TLS/HTTPS (4 methods)**

32. `tls.derive_secrets` - Handshake secret derivation
33. `tls.derive_application_secrets` - Application secret derivation
34. `tls.sign_handshake` - Handshake signing
35. `tls.verify_certificate` - Certificate verification

### **Tor v3 Onion (8 methods)**

36. `tor.derive_onion_address` - Derive .onion from Ed25519
37. `tor.generate_identity` - Generate keypair + .onion address
38. `tor.ntor_client_init` - ntor handshake client init
39. `tor.ntor_server_respond` - ntor handshake server respond
40. `tor.ntor_client_finish` - ntor handshake client finish
41. `tor.encrypt_cell` - Cell encryption (ChaCha20)
42. `tor.decrypt_cell` - Cell decryption (ChaCha20)
43. `tor.hkdf_expand` - HKDF-SHA256 key expansion

### **Secret Storage (4 methods)**

44. `secrets.store` - Encrypt and store secret
45. `secrets.retrieve` - Decrypt and return secret
46. `secrets.list` - List stored secret names
47. `secrets.delete` - Remove a stored secret

### **Dark Forest Beacon (7 methods)**

48. `beacon.generate` - Generate beacon seed
49. `beacon.get_id` - Get public beacon ID
50. `beacon.encrypt` - Encrypt with beacon seed
51. `beacon.try_decrypt` - Decrypt with our seed
52. `beacon.try_decrypt_any` - Decrypt with any known beacon
53. `beacon.list_known` - List known beacon IDs
54. `beacon.add_known` - Add known beacon from meeting

### **Relay Authorization (1 method)**

55. `relay.authorize` - Lineage-gated relay authorization

### **Federation (2 methods)**

56. `federation.verify_family_member` - Verify genetic relationship
57. `federation.derive_subfed_key` - Derive sub-federation key

### **Password Hashing (3 methods)**

58. `crypto.argon2id_hash` - Argon2id password hashing
59. `crypto.argon2id_verify` - Argon2id verification
60. `crypto.pbkdf2_sha256` - PBKDF2 key derivation

### **BTSP Tunnel (6 methods)**

61. `btsp.exchange_contacts` - Exchange contact information
62. `btsp.establish` - Establish encrypted tunnel
63. `btsp.encrypt` - Encrypt tunnel payload
64. `btsp.decrypt` - Decrypt tunnel payload
65. `btsp.status` - Tunnel status check
66. `btsp.close` - Close tunnel

### **Security (6 methods)**

67. `security.evaluate_trust` - Trust evaluation
68. `security.verify_jwt` - JWT verification
69. `security.graph_query` - Security graph query
70. `security.validate_permissions` - Permission validation
71. `security.check_authorization` - Authorization check
72. `security.audit_log` - Audit log entry

### **Introspection (6 methods)**

73. `discover_capabilities` - List all capability categories
74. `capabilities` - Detailed capability information
75. `get_capabilities` - Flat list of all methods
76. `primal.info` - Primal metadata
77. `rpc.methods` - Available RPC methods
78. `identity` / `whoami` - Node identity

### **HSM Management (11 methods)**

79. `hsm.generate_key` - Key generation
80. `hsm.import_key` - Key import
81. `hsm.export_key` - Key export
82. `hsm.delete_key` - Key deletion
83. `hsm.list_keys` - List managed keys
84. `hsm.get_entropy` - Hardware entropy
85. `hsm.create_session` - HSM session creation
86. `hsm.close_session` - HSM session teardown
87. `hsm.sign` - HSM-backed signing
88. `hsm.verify` - HSM-backed verification
89. `hsm.status` - HSM health status

### **Ionic Bond (8 methods — IonicBondHandler)**

90. `crypto.ionic_bond.propose` - Propose a cross-domain bond
91. `crypto.ionic_bond.accept` - Accept a bond proposal (Ed25519 signature)
92. `crypto.ionic_bond.seal` - Cryptographically seal an active bond
93. `crypto.ionic_bond.verify` - Verify bond state and signatures
94. `crypto.ionic_bond.revoke` - Revoke an active or sealed bond
95. `crypto.ionic_bond.list` - List bonds (filterable by domain/state)
96. `crypto.sign_contract` - Sign contract terms with Ed25519 identity
97. `crypto.verify_contract` - Verify contract signature

### **Total**: **111 JSON-RPC methods** (103 CryptoHandler + 8 IonicBondHandler)

═══════════════════════════════════════════════════════════════════

## 🎯 **VERSIONING**

### **API Version**: 1.0.0

**Semantic Versioning**:
- **Major**: Breaking changes to existing methods
- **Minor**: New methods added (backward compatible)
- **Patch**: Bug fixes, performance improvements

**Version Negotiation**:
```json
{
  "jsonrpc": "2.0",
  "method": "system.version",
  "params": {},
  "id": 1
}

Response:
{
  "jsonrpc": "2.0",
  "result": {
    "api_version": "1.0.0",
    "beardog_version": "0.9.0",
    "protocol": "json-rpc-2.0"
  },
  "id": 1
}
```

### **Deprecation Policy**

1. **Announce**: 1 minor version before removal
2. **Warning**: Return warning in response
3. **Remove**: Next major version

═══════════════════════════════════════════════════════════════════

## 🔐 **SECURITY CONSIDERATIONS**

### **Authentication**

BearDog uses **genetic lineage** for authentication:
1. Family members share a `.family.seed` file
2. Challenge-response proves lineage knowledge
3. No passwords, no tokens
4. Perfect forward secrecy

### **Authorization**

- **Family members**: Full access to all methods
- **Non-family**: Cannot connect (Dark Forest)
- **Capabilities**: Self-declared in beacons

### **Transport Security**

- **Unix sockets**: Kernel-enforced access control
- **TCP fallback**: localhost only (127.0.0.1)
- **No TLS wrapper**: Not needed (local IPC)

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

- BearDog source: `primals/bearDog/crates/beardog-tunnel/`
- Crypto handlers: `src/unix_socket_ipc/handlers/crypto/`
- Genetic handlers: `src/unix_socket_ipc/crypto_handlers_genetic.rs`
- Secret handlers: `src/unix_socket_ipc/handlers/secrets.rs`
- Beacon handlers: `src/unix_socket_ipc/handlers/beacon.rs`
- Relay handler: `src/unix_socket_ipc/handlers/relay.rs`
- Capabilities: `src/unix_socket_ipc/handlers/capabilities.rs`
- Handler registry: `src/unix_socket_ipc/handlers/mod.rs`

═══════════════════════════════════════════════════════════════════

**Document Version**: 2.0.0  
**Last Updated**: May 7, 2026  
**Maintainer**: BearDog Security Primal  
**License**: Documented interface (implementation MIT-licensed)

🔌 **Primal Contracts: Enabling Runtime Discovery & Composition**
