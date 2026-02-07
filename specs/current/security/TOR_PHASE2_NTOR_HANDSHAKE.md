# Tor Phase 2: ntor Handshake Specification

**Date**: February 7, 2026  
**Status**: PLANNING  
**Version**: 0.1.0 (Draft)  
**References**: [tor-spec Section 5.1.4](https://spec.torproject.org/tor-spec/create-created-cells.html#ntor)

---

## Overview

The ntor handshake is Tor's modern circuit key exchange protocol, providing:
- Forward secrecy via ephemeral X25519 keys
- Server authentication via static ntor onion key
- Efficient single-round-trip handshake

BearDog will provide ntor crypto operations for Songbird's pure Rust Tor implementation.

---

## Protocol Flow

```
┌──────────┐                              ┌──────────┐
│  Client  │                              │  Server  │
└────┬─────┘                              └────┬─────┘
     │                                         │
     │  1. Generate ephemeral keypair          │
     │     x ← random_scalar()                 │
     │     X = x * G                           │
     │                                         │
     │  ──── EXTEND2(X, node_id, B) ─────────▶ │
     │                                         │
     │                                         │  2. Generate ephemeral keypair
     │                                         │     y ← random_scalar()
     │                                         │     Y = y * G
     │                                         │
     │                                         │  3. Compute shared secrets
     │                                         │     secret_input = EXP(X,y) || EXP(X,b) || ID || B || X || Y || PROTOID
     │                                         │     KEY_SEED = HMAC(t_key, secret_input)
     │                                         │     verify = HMAC(t_verify, secret_input)
     │                                         │
     │  ◀──── EXTENDED2(Y, verify) ─────────── │
     │                                         │
     │  4. Verify and derive keys              │
     │     secret_input = EXP(Y,x) || EXP(B,x) || ID || B || X || Y || PROTOID
     │     KEY_SEED = HMAC(t_key, secret_input)
     │     verify' = HMAC(t_verify, secret_input)
     │     assert verify == verify'
     │     keys = HKDF(KEY_SEED, ...)
     │                                         │
     ▼                                         ▼
```

---

## Constants

```rust
/// Protocol identifier for ntor
const NTOR_PROTOID: &[u8] = b"ntor-curve25519-sha256-1";

/// Key derivation tweak for KEY_SEED
const NTOR_T_KEY: &[u8] = b"ntor-curve25519-sha256-1:key_extract";

/// Key derivation tweak for verify
const NTOR_T_VERIFY: &[u8] = b"ntor-curve25519-sha256-1:verify";

/// Key expansion tweak
const NTOR_T_EXPAND: &[u8] = b"ntor-curve25519-sha256-1:key_expand";

/// MAC tweak
const NTOR_T_MAC: &[u8] = b"ntor-curve25519-sha256-1:mac";
```

---

## BearDog API Design

### Method 1: `beardog.crypto.tor_ntor_client_init`

Initialize client-side ntor handshake.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "beardog.crypto.tor_ntor_client_init",
  "params": {
    "node_id": "<base64: 20-byte node identity hash>",
    "node_onion_key": "<base64: 32-byte X25519 public key B>"
  },
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "ephemeral_public": "<base64: 32-byte X25519 public X>",
    "client_state": "<base64: encrypted client state for finish>"
  },
  "id": 1
}
```

**Implementation Notes:**
- `client_state` contains encrypted `x` (ephemeral secret) + parameters
- State encrypted with session key to prevent exposure

---

### Method 2: `beardog.crypto.tor_ntor_client_finish`

Complete client-side ntor handshake after receiving server response.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "beardog.crypto.tor_ntor_client_finish",
  "params": {
    "client_state": "<base64: from init>",
    "server_public": "<base64: 32-byte Y from server>",
    "server_auth": "<base64: 32-byte verify from server>"
  },
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "valid": true,
    "forward_digest_key": "<base64: Df - 20 bytes>",
    "backward_digest_key": "<base64: Db - 20 bytes>",
    "forward_key": "<base64: Kf - 16/32 bytes>",
    "backward_key": "<base64: Kb - 16/32 bytes>"
  },
  "id": 1
}
```

**Error Response (auth failed):**
```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32001,
    "message": "ntor handshake verification failed"
  },
  "id": 1
}
```

---

### Method 3: `beardog.crypto.tor_ntor_server_respond`

Server-side ntor handshake (for hidden service rendezvous).

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "beardog.crypto.tor_ntor_server_respond",
  "params": {
    "client_public": "<base64: 32-byte X from client>",
    "node_id": "<base64: 20-byte node identity>",
    "onion_secret_key": "<base64: 32-byte b (ntor private key)>",
    "onion_public_key": "<base64: 32-byte B (ntor public key)>"
  },
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "ephemeral_public": "<base64: 32-byte Y>",
    "server_auth": "<base64: 32-byte verify>",
    "forward_digest_key": "<base64: Df>",
    "backward_digest_key": "<base64: Db>",
    "forward_key": "<base64: Kf>",
    "backward_key": "<base64: Kb>"
  },
  "id": 1
}
```

---

## Cryptographic Operations

### Secret Input Construction

```rust
fn build_secret_input(
    exp_xy: &[u8; 32],      // EXP(X, y) or EXP(Y, x)
    exp_xb: &[u8; 32],      // EXP(X, b) or EXP(B, x)
    node_id: &[u8; 20],     // Node identity
    onion_key: &[u8; 32],   // B
    client_pub: &[u8; 32],  // X
    server_pub: &[u8; 32],  // Y
) -> Vec<u8> {
    let mut input = Vec::with_capacity(32 + 32 + 20 + 32 + 32 + 32 + NTOR_PROTOID.len());
    input.extend_from_slice(exp_xy);
    input.extend_from_slice(exp_xb);
    input.extend_from_slice(node_id);
    input.extend_from_slice(onion_key);
    input.extend_from_slice(client_pub);
    input.extend_from_slice(server_pub);
    input.extend_from_slice(NTOR_PROTOID);
    input
}
```

### Key Derivation

```rust
fn derive_ntor_keys(key_seed: &[u8; 32]) -> NtorKeys {
    // Use HKDF-SHA256 with t_expand
    let hkdf = Hkdf::<Sha256>::new(Some(NTOR_T_EXPAND), key_seed);
    
    let mut keys = [0u8; 92]; // Df(20) + Db(20) + Kf(16) + Kb(16) + extra
    hkdf.expand(b"", &mut keys).unwrap();
    
    NtorKeys {
        forward_digest: keys[0..20].try_into().unwrap(),
        backward_digest: keys[20..40].try_into().unwrap(),
        forward_key: keys[40..56].try_into().unwrap(),   // AES-128
        backward_key: keys[56..72].try_into().unwrap(),  // AES-128
    }
}
```

---

## Test Vectors

### Known Test Case

From Tor source code (`src/test/test_ntor_cl.c`):

```
# Node identity (20 bytes, hex)
node_id = 0x00112233445566778899aabbccddeeff00112233

# Node ntor onion key (B, 32 bytes, hex)
node_onion_key = 0x4a2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f9

# Client ephemeral secret (x, 32 bytes, hex)  
client_secret = 0x... (deterministic for test)

# Expected outputs...
```

*(Full test vectors to be extracted from Tor source)*

---

## Security Considerations

### Forward Secrecy

- Ephemeral keys (x, y) MUST be generated fresh per handshake
- Ephemeral secrets MUST be zeroized after key derivation

### Timing Attacks

- All operations should be constant-time
- Use `subtle` crate for comparisons

### State Management

- `client_state` blob is encrypted to prevent ephemeral secret exposure
- Server never stores state (stateless handshake)

---

## Implementation Checklist

- [ ] `tor_ntor_client_init` - Generate ephemeral, return state blob
- [ ] `tor_ntor_client_finish` - Verify server auth, derive keys
- [ ] `tor_ntor_server_respond` - Full server-side response
- [ ] HMAC-SHA256 with Tor tweaks
- [ ] HKDF-SHA256 key expansion
- [ ] Constant-time verification
- [ ] Zeroization of secrets
- [ ] Test vectors from Tor

---

## References

- [tor-spec: ntor handshake](https://spec.torproject.org/tor-spec/create-created-cells.html#ntor)
- [Tor ntor paper](https://www.cypherpunks.ca/~iang/pubs/ntor.pdf)
- [Tor source: ntor implementation](https://gitlab.torproject.org/tpo/core/tor/-/blob/main/src/core/crypto/onion_ntor.c)
