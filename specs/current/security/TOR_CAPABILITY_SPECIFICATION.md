# Tor Capability Specification

**Date**: February 7, 2026  
**Status**: Phase 1 Active  
**Version**: 1.0.0

---

## Executive Summary

BearDog provides cryptographic primitives for Tor v3 onion services, enabling Songbird to implement privacy-preserving network capabilities. This document specifies BearDog's role in the Tor integration.

---

## Architecture

### BearDog's Role

BearDog is the **cryptographic engine** for Tor operations:

```
┌─────────────────────────────────────────────────────────────────┐
│                    Tor Crypto Flow                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐     ┌──────────────┐     ┌──────────────┐    │
│  │   Songbird   │────▶│   BearDog    │────▶│  Tor Layer   │    │
│  │ (protocol)   │     │ (crypto)     │     │ (daemon/pure)│    │
│  └──────────────┘     └──────────────┘     └──────────────┘    │
│         │                    │                    │             │
│         │                    │                    │             │
│         ▼                    ▼                    ▼             │
│   Request crypto      Provide keys          Route traffic      │
│   operations          Sign/verify           Hidden service     │
│                       Encrypt/decrypt                          │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Deployment Phases

| Phase | Component | BearDog Provides |
|-------|-----------|------------------|
| **Phase 1** | Tor Daemon | Ed25519 identity keys, signature verification |
| **Phase 2** | Pure Rust Tor | All crypto (cells, circuits, handshakes) |

---

## Supported Crypto Operations

### 1. Onion Identity (Ed25519)

Tor v3 onion addresses are derived from Ed25519 public keys.

**Methods:**
- `beardog.crypto.ed25519_generate_keypair` - Generate identity keypair
- `beardog.crypto.sign_ed25519` - Sign with identity key
- `beardog.crypto.verify_ed25519` - Verify identity signatures

**Tor v3 Onion Address Derivation:**
```
onion_address = base32(public_key || checksum || version).onion

where:
  checksum = sha3_256(".onion checksum" || public_key || version)[0:2]
  version = 0x03 (Tor v3)
```

### 2. Key Exchange (X25519)

Circuit key establishment uses X25519 ECDH.

**Methods:**
- `beardog.crypto.x25519_generate_ephemeral` - Generate ephemeral keypair
- `beardog.crypto.x25519_derive_secret` - ECDH shared secret

### 3. Cell Encryption (ChaCha20-Poly1305)

Modern Tor supports ChaCha20-Poly1305 for cell encryption.

**Methods:**
- `beardog.crypto.chacha20_poly1305_encrypt` - Encrypt cell data
- `beardog.crypto.chacha20_poly1305_decrypt` - Decrypt cell data

### 4. Hashing (SHA3-256)

Onion address derivation uses SHA3-256.

**Methods:**
- `beardog.crypto.sha3_256` - Hash for onion address checksum

### 5. Additional Operations

**Methods:**
- `beardog.crypto.hmac_sha256` - HKDF key derivation
- `beardog.crypto.blake3_hash` - General purpose hashing

---

## API Reference

### Namespace

All Tor-related crypto uses the `beardog.crypto.*` namespace to distinguish from general crypto operations.

### Onion Address Derivation

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "beardog.crypto.derive_onion_address",
  "params": {
    "public_key": "<base64-encoded 32-byte Ed25519 public key>"
  },
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "onion_address": "ve3lahyh7ktngjkvjdirsgfkmgsi6qcqfzrjrjkq3bffiie2n6qmdwid.onion",
    "public_key": "<base64>",
    "version": 3,
    "checksum": "<hex>"
  },
  "id": 1
}
```

### Generate Onion Identity

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "beardog.crypto.generate_onion_identity",
  "params": {
    "purpose": "hidden_service"
  },
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "public_key": "<base64-encoded 32-byte public key>",
    "secret_key": "<base64-encoded 64-byte secret key>",
    "onion_address": "<56-char>.onion",
    "version": 3
  },
  "id": 1
}
```

---

## Crypto Primitives Status

| Primitive | BearDog Status | Tor Usage |
|-----------|----------------|-----------|
| Ed25519 | ✅ Implemented | Identity keys, signing |
| X25519 | ✅ Implemented | Circuit handshakes |
| ChaCha20-Poly1305 | ✅ Implemented | Cell encryption (modern) |
| SHA3-256 | ✅ Implemented | Onion address checksum |
| HMAC-SHA256 | ✅ Implemented | KDF operations |
| AES-128-CTR | ⏳ Deferred | Cell encryption (legacy) |

### AES-128-CTR Status

AES-128-CTR is deferred due to RustCrypto RC version conflicts. Modern Tor also supports ChaCha20-Poly1305 which is fully implemented. For legacy Tor nodes, AES-128-CTR will be added when RustCrypto stabilizes.

---

## Integration with Songbird

### Phase 1: Tor Daemon

Songbird manages the Tor daemon while BearDog provides identity keys:

```
Songbird                           BearDog
   │                                  │
   │─── generate_onion_identity ─────▶│
   │◀── {public_key, secret_key} ─────│
   │                                  │
   │─── sign_ed25519(descriptor) ────▶│
   │◀── {signature} ──────────────────│
   │                                  │
   ▼                                  │
Tor Daemon ◀─── publish descriptor ───┘
```

### Phase 2: Pure Rust Tor

Songbird implements Tor protocol, delegating ALL crypto to BearDog:

```
Songbird Tor Protocol              BearDog
   │                                  │
   │─── x25519_generate_ephemeral ───▶│  (circuit handshake)
   │◀── {public_key, secret_key} ─────│
   │                                  │
   │─── x25519_derive_secret ────────▶│  (shared secret)
   │◀── {shared_secret} ──────────────│
   │                                  │
   │─── chacha20_poly1305_encrypt ───▶│  (cell encryption)
   │◀── {ciphertext} ─────────────────│
   │                                  │
   ▼                                  │
Tor Network ◀─── encrypted cells ─────┘
```

---

## Security Considerations

### Key Storage

- Identity keys SHOULD be stored in HSM when available
- Keys MUST be protected by BearDog's genetic lineage
- Keys SHOULD NOT be exported outside the primal ecosystem

### Cryptographic Agility

BearDog supports multiple algorithms to handle:
- Modern Tor (ChaCha20-Poly1305)
- Legacy Tor (AES-128-CTR, when implemented)
- Future Tor (post-quantum, via quantum_crypto module)

### Dark Forest Integration

Tor capability integrates with Dark Forest privacy model:
- Onion addresses are family-encrypted in beacons
- Only family members can decrypt .onion endpoints
- Non-family sees opaque encrypted blob

---

## Testing

### Unit Tests

```rust
#[test]
fn test_onion_address_derivation() {
    // Generate Ed25519 keypair
    let keypair = ed25519_generate_keypair();
    
    // Derive onion address
    let onion = derive_onion_address(&keypair.public_key);
    
    // Verify format: 56 chars + ".onion"
    assert!(onion.ends_with(".onion"));
    assert_eq!(onion.len(), 62); // 56 + 6
    
    // Verify base32 encoding (lowercase)
    let addr_part = &onion[..56];
    assert!(addr_part.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
}
```

### Integration Tests

```bash
# Test onion identity generation
beardog-cli crypto generate-onion-identity

# Test signature with onion key
beardog-cli crypto sign-ed25519 --key-id onion_identity --message "test"
```

---

## Future Work

### Phase 2 Requirements

When Songbird implements pure Rust Tor, BearDog will need:

1. **ntor Handshake Support**
   - Specific HKDF construction for ntor
   - May require new method: `beardog.crypto.ntor_handshake`

2. **Cell Digest Computation**
   - Running digest for RELAY cells
   - SHA1 (legacy) or SHA3 (modern)

3. **Blinded Key Derivation**
   - For hidden service descriptors
   - Ed25519 key blinding per spec

### Timeline

| Milestone | Target | Dependencies |
|-----------|--------|--------------|
| Phase 1 Validation | Feb 2026 | Tor daemon installed |
| AES-128-CTR | Q2 2026 | RustCrypto stable |
| Pure Rust Tor | Q3 2026 | Songbird protocol impl |
| Post-Quantum Tor | 2027 | Tor spec updates |

---

## References

- [Tor Protocol Specification](https://spec.torproject.org/tor-spec)
- [Onion Service v3 Specification](https://spec.torproject.org/rend-spec-v3)
- [BearDog Crypto API](../../docs/BEARDOG_RPC_API.md)
- [Songbird Integration](../otherTeams/SONGBIRD_INTEGRATION.md)

---

## Changelog

### v1.0.0 (February 7, 2026)
- Initial specification
- Phase 1 crypto primitives documented
- beardog.crypto.* namespace defined
