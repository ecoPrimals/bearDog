# Tor Phase 2: Cell Encryption Specification

**Date**: February 7, 2026  
**Status**: PLANNING  
**Version**: 0.1.0 (Draft)  
**References**: [tor-spec Section 6](https://spec.torproject.org/tor-spec/relay-cells.html)

---

## Overview

Tor relay cells use layered encryption - each hop in a circuit adds/removes a layer. BearDog provides the crypto primitives; Songbird manages the circuit state.

---

## Cell Structure

### Fixed Cell (514 bytes)

```
┌──────────────────────────────────────────────────────────────┐
│ CircID (4 bytes) │ Command (1 byte) │ Payload (509 bytes)   │
└──────────────────────────────────────────────────────────────┘
```

### Relay Payload (509 bytes)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Relay   │ Recognized │ StreamID │ Digest  │ Length  │ Data      │ Padding  │
│ Command │ (2 bytes)  │ (2 bytes)│ (4 bytes│ (2 bytes│ (variable)│ (to 509) │
│ (1 byte)│ = 0 if us  │          │ running)│         │           │          │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Encryption Algorithms

### Modern: ChaCha20 (Counter Mode)

Used for circuits negotiated with ChaCha20 support.

```rust
/// ChaCha20 cell encryption (counter mode, NO authentication tag)
/// 
/// Note: Tor cells are authenticated by the running digest, not AEAD
fn chacha20_cell_crypt(
    key: &[u8; 32],
    nonce: &[u8; 12],  // Usually derived from counter
    data: &mut [u8],
) {
    use chacha20::ChaCha20;
    use chacha20::cipher::{KeyIvInit, StreamCipher};
    
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    cipher.apply_keystream(data);
}
```

### Legacy: AES-128-CTR

Used for most current circuits (until ChaCha20 adoption increases).

```rust
/// AES-128-CTR cell encryption
/// 
/// Note: Requires RustCrypto aes/ctr crates (currently RC)
fn aes128_ctr_cell_crypt(
    key: &[u8; 16],
    counter: &mut [u8; 16],  // Modified in place
    data: &mut [u8],
) {
    // Implementation pending RustCrypto stable
}
```

---

## BearDog API Design

### Method 1: `beardog.crypto.tor_cell_encrypt`

Encrypt a relay cell payload for one circuit hop.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "beardog.crypto.tor_cell_encrypt",
  "params": {
    "payload": "<base64: 509-byte relay payload>",
    "key": "<base64: 16 or 32-byte key (Kf or Kb)>",
    "digest_state": "<base64: running digest state>",
    "cipher": "chacha20"
  },
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "encrypted_payload": "<base64: 509 bytes>",
    "digest_state": "<base64: updated digest state>"
  },
  "id": 1
}
```

---

### Method 2: `beardog.crypto.tor_cell_decrypt`

Decrypt a relay cell payload from one circuit hop.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "beardog.crypto.tor_cell_decrypt",
  "params": {
    "encrypted_payload": "<base64: 509-byte encrypted>",
    "key": "<base64: 16 or 32-byte key>",
    "digest_state": "<base64: running digest state>",
    "cipher": "chacha20"
  },
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "payload": "<base64: 509-byte decrypted>",
    "digest_state": "<base64: updated digest state>",
    "recognized": true,
    "digest_valid": true
  },
  "id": 1
}
```

---

### Method 3: `beardog.crypto.tor_digest_init`

Initialize running digest state for a circuit direction.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "beardog.crypto.tor_digest_init",
  "params": {
    "key": "<base64: Df or Db from ntor>"
  },
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "digest_state": "<base64: initialized state>"
  },
  "id": 1
}
```

---

## Running Digest

Tor uses a running SHA-1 (or SHA3 for modern) digest to authenticate cells.

### Digest Update Process

```rust
/// Update running digest with cell data
fn update_running_digest(state: &mut Sha1, cell_data: &[u8; 509]) {
    // The digest field in the cell is set to 0 before hashing
    let mut temp = *cell_data;
    temp[5..9].copy_from_slice(&[0, 0, 0, 0]);  // Zero digest field
    
    state.update(&temp);
}

/// Verify cell digest
fn verify_cell_digest(state: &Sha1, cell_data: &[u8; 509]) -> bool {
    let expected = &state.clone().finalize()[0..4];
    let actual = &cell_data[5..9];
    
    // Constant-time comparison
    subtle::ConstantTimeEq::ct_eq(expected, actual).into()
}
```

---

## Circuit Crypto State

Each circuit direction maintains:

```rust
struct CircuitCryptoState {
    /// Encryption/decryption key
    key: [u8; 32],  // Or 16 for AES-128
    
    /// Running digest state
    digest: Sha1State,  // Or Sha3State for modern
    
    /// Cipher type
    cipher: CellCipher,
    
    /// Nonce/counter state (for ChaCha20)
    nonce_counter: u64,
}

enum CellCipher {
    Aes128Ctr,
    ChaCha20,
}
```

---

## Layered Encryption

For a 3-hop circuit (guard → middle → exit):

### Sending (Client → Exit)

```
1. Build relay cell with plaintext data
2. Set recognized = 0, compute & set digest
3. Encrypt with Kf_exit (innermost layer)
4. Encrypt with Kf_middle
5. Encrypt with Kf_guard (outermost layer)
6. Send to guard
```

### Receiving (Exit → Client)

```
1. Receive from guard
2. Decrypt with Kb_guard
3. Decrypt with Kb_middle
4. Decrypt with Kb_exit
5. Check recognized == 0 and digest valid
6. Extract plaintext data
```

---

## Implementation Checklist

- [ ] `tor_cell_encrypt` - Encrypt single hop
- [ ] `tor_cell_decrypt` - Decrypt single hop
- [ ] `tor_digest_init` - Initialize digest state
- [ ] `tor_digest_update` - Update without full cell (for partial ops)
- [ ] SHA-1 running digest (legacy compatibility)
- [ ] SHA3 running digest (modern)
- [ ] ChaCha20 counter mode
- [ ] AES-128-CTR (when available)
- [ ] Recognized field check
- [ ] Digest verification
- [ ] Test vectors from Tor

---

## Test Vectors

### Known Cell Encryption Test

```
# From Tor test suite
key = 0x...
plaintext = 0x... (509 bytes)
expected_ciphertext = 0x...
```

*(Full test vectors to be extracted from Tor source)*

---

## Performance Targets

| Operation | Target | Notes |
|-----------|--------|-------|
| Cell encrypt | < 10μs | Single hop |
| Cell decrypt | < 10μs | Single hop |
| Digest update | < 1μs | Per cell |
| Full 3-hop encrypt | < 50μs | All layers |

---

## References

- [tor-spec: Relay Cells](https://spec.torproject.org/tor-spec/relay-cells.html)
- [tor-spec: Encryption](https://spec.torproject.org/tor-spec/relay-cells.html#encryption)
- [Tor source: relay_crypt.c](https://gitlab.torproject.org/tpo/core/tor/-/blob/main/src/core/crypto/relay_crypto.c)
