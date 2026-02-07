# Tor Phase 2 Evolution - Pure Rust Protocol

**Created**: February 7, 2026  
**Status**: PLANNING  
**Owner**: BearDog (crypto provider for Songbird's Tor protocol)

---

## Executive Summary

Phase 2 evolves from Tor daemon dependency to **Pure Rust Tor protocol** in Songbird, with BearDog providing ALL cryptographic operations. This achieves true ecoBin compliance - zero external dependencies.

---

## Current State (Phase 1 - Active)

| Component | Status | Location |
|-----------|--------|----------|
| `beardog.crypto.derive_onion_address` | ✅ Complete | `crypto_handlers_hashing.rs` |
| `beardog.crypto.generate_onion_identity` | ✅ Complete | `crypto_handlers_hashing.rs` |
| Ed25519 (identity) | ✅ Complete | `crypto/asymmetric.rs` |
| X25519 (circuits) | ✅ Complete | `crypto/asymmetric.rs` |
| SHA3-256 (checksums) | ✅ Complete | `crypto_handlers_hashing.rs` |
| ChaCha20-Poly1305 (cells) | ✅ Complete | `crypto/symmetric.rs` |
| AES-128-CTR (legacy cells) | ⏳ Deferred | RustCrypto RC conflicts |

**BiomeOS Testing**: Phase 1 validation with Tor daemon

---

## Phase 2 Architecture

### BearDog's Role Expansion

```
┌─────────────────────────────────────────────────────────────────┐
│                 Phase 2: Pure Rust Tor                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Songbird                           BearDog                     │
│  ┌──────────────────────┐          ┌──────────────────────┐    │
│  │ songbird-tor-proto   │          │ Crypto Delegation    │    │
│  │ ├─ Directory fetch   │ ───────▶ │ ├─ Ed25519 signing   │    │
│  │ ├─ Circuit builder   │          │ ├─ X25519 ECDH       │    │
│  │ ├─ Cell processor    │          │ ├─ ChaCha20-Poly1305 │    │
│  │ ├─ Onion listener    │          │ ├─ SHA3/BLAKE3       │    │
│  │ └─ Stream handler    │          │ ├─ HKDF derivation   │    │
│  └──────────────────────┘          │ └─ ntor handshake    │    │
│                                     └──────────────────────┘    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### New BearDog Methods Required

| Method | Purpose | Priority | Effort |
|--------|---------|----------|--------|
| `beardog.crypto.tor_ntor_handshake` | ntor circuit handshake | P0 | 2 days |
| `beardog.crypto.tor_cell_encrypt` | Cell encryption (relay) | P0 | 1 day |
| `beardog.crypto.tor_cell_decrypt` | Cell decryption (relay) | P0 | 1 day |
| `beardog.crypto.tor_kdf` | Tor-specific KDF | P1 | 1 day |
| `beardog.crypto.tor_blind_key` | Blinded key derivation | P1 | 2 days |
| `beardog.crypto.aes_128_ctr_*` | Legacy cell encryption | P2 | Blocked |

---

## Implementation Roadmap

### Milestone 1: ntor Handshake (P0)

The ntor handshake is Tor's modern circuit key exchange protocol.

**Algorithm:**
```
Client:
  x = random_scalar()
  X = x * G (ephemeral public)
  → EXTEND2 cell with X, node_id, key_id

Server:
  y = random_scalar()
  Y = y * G (ephemeral public)
  secret_input = EXP(X, y) || EXP(B, x) || ID || B || X || Y || PROTOID
  KEY_SEED = HMAC(t_key, secret_input)
  verify = HMAC(t_verify, secret_input)
  ← EXTENDED2 cell with Y, verify

Both:
  keys = HKDF(KEY_SEED, ...) → Df, Db, Kf, Kb
```

**BearDog Implementation:**
```rust
/// Handle `beardog.crypto.tor_ntor_handshake`
/// 
/// Performs client or server side of ntor handshake.
/// 
/// # Parameters (client_init)
/// - `node_id`: 20-byte node identity
/// - `key_id`: 32-byte ntor onion key (B)
/// 
/// # Returns
/// - `ephemeral_public`: 32-byte X25519 public (X)
/// - `ephemeral_secret`: 32-byte X25519 secret (x) - for client state
pub async fn handle_tor_ntor_client_init(params: Option<&Value>) -> Result<Value, String>

/// # Parameters (client_finish)
/// - `server_public`: 32-byte Y from server
/// - `verify`: 32-byte HMAC from server
/// - `ephemeral_secret`: client's x
/// - `node_onion_key`: server's B
/// 
/// # Returns
/// - `forward_digest`: Df
/// - `backward_digest`: Db
/// - `forward_key`: Kf
/// - `backward_key`: Kb
pub async fn handle_tor_ntor_client_finish(params: Option<&Value>) -> Result<Value, String>
```

### Milestone 2: Cell Encryption (P0)

Tor cells are 512 bytes with layered encryption.

**Cell Format:**
```
RELAY cell (514 bytes total):
  CircID (2/4 bytes)
  Command (1 byte) = 3 (RELAY)
  Payload (509 bytes):
    Relay command (1 byte)
    Recognized (2 bytes) = 0 if for us
    StreamID (2 bytes)
    Digest (4 bytes) = running hash
    Length (2 bytes)
    Data (498 bytes)
```

**BearDog Implementation:**
```rust
/// Handle `beardog.crypto.tor_cell_encrypt`
/// 
/// Encrypts a relay cell with circuit keys (ChaCha20 or AES-CTR).
/// 
/// # Parameters
/// - `cell_data`: 509-byte relay payload
/// - `key`: 32-byte encryption key (Kf or Kb)
/// - `digest_state`: Running digest state
/// - `cipher`: "chacha20" or "aes128ctr"
/// 
/// # Returns
/// - `encrypted_cell`: Encrypted payload
/// - `digest_state`: Updated digest state
pub async fn handle_tor_cell_encrypt(params: Option<&Value>) -> Result<Value, String>
```

### Milestone 3: Blinded Key Derivation (P1)

For hidden service descriptors, Tor uses key blinding.

**Algorithm:**
```
blinded_key = public_key * hash_to_scalar(...)
```

**BearDog Implementation:**
```rust
/// Handle `beardog.crypto.tor_blind_key`
/// 
/// Derives blinded Ed25519 public key for hidden service descriptors.
/// 
/// # Parameters
/// - `public_key`: 32-byte Ed25519 public key
/// - `blind_param`: Blinding parameter (time period, etc.)
/// 
/// # Returns
/// - `blinded_key`: 32-byte blinded public key
pub async fn handle_tor_blind_key(params: Option<&Value>) -> Result<Value, String>
```

---

## Dependencies

### Current (Phase 1)

```toml
# Already in beardog-tunnel/Cargo.toml
ed25519-dalek = "2.0"
x25519-dalek = { version = "2.0", features = ["static_secrets"] }
chacha20poly1305 = "0.10"
sha3 = "0.10"
data-encoding = "2.5"
```

### Phase 2 Additions

```toml
# May need for ntor
curve25519-dalek = "4"  # Low-level curve ops for ntor

# For AES-128-CTR when RustCrypto stabilizes
# aes = "0.9"
# ctr = "0.10"
```

---

## Testing Strategy

### Unit Tests

1. **ntor Handshake**
   - Known test vectors from Tor spec
   - Client/server interop
   - Invalid inputs handling

2. **Cell Encryption**
   - Round-trip encrypt/decrypt
   - Digest accumulation
   - Cross-cell state

3. **Key Blinding**
   - Known test vectors
   - Time period rotation

### Integration Tests

1. **With Songbird** (when ready)
   - Full circuit establishment
   - Data relay
   - Hidden service publish/connect

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| AES-128-CTR blocked | High | Medium | ChaCha20 covers modern Tor |
| ntor complexity | Medium | High | Use Tor test vectors |
| Spec changes | Low | Medium | Track tor-spec repo |

---

## Success Criteria

- [ ] ntor handshake passes Tor test vectors
- [ ] Cell encryption compatible with Tor relays
- [ ] Key blinding produces valid descriptors
- [ ] All methods < 1ms latency
- [ ] Zero unsafe code
- [ ] 100% Pure Rust

---

## References

- [Tor Protocol Specification](https://spec.torproject.org/tor-spec)
- [ntor Handshake](https://spec.torproject.org/tor-spec/create-created-cells.html#ntor)
- [Onion Service v3 Spec](https://spec.torproject.org/rend-spec-v3)
- [BearDog Crypto API](docs/BEARDOG_RPC_API.md)

---

## Changelog

| Date | Change |
|------|--------|
| Feb 7, 2026 | Initial Phase 2 planning document |
