# Genetic Lineage Evolution Specification

**Version**: 1.0.0  
**Date**: February 5, 2026  
**Status**: BearDog Implementation Complete

---

## Executive Summary

This specification addresses critical architectural issues identified during cross-network NAT traversal testing:

1. **Lineage Seeds COPIED instead of DERIVED** - Architectural flaw fixed
2. **Device Enrollment Certificates** - New cryptographic protocol
3. **Full Chain Verification** - Trust anchor support

BearDog now provides **11 genetic methods** (up from 8) to support proper lineage derivation and device enrollment.

---

## Problem Statement

### The WRONG Model (Cloning)

```
USB Tower:  .family.seed = 8ff3b864a4bc589a...
Pixel:      .family.seed = 8ff3b864a4bc589a... (IDENTICAL COPY!)
```

**Security Issue**: If ONE device is compromised, ALL devices are compromised because they share the same seed.

### The CORRECT Model (Derivation)

```
Genesis:    root_seed = 8ff3b864a4bc589a... (stored encrypted, rarely accessed)
                        ↓
USB Tower:  device_seed = DERIVE(root_seed, usb_entropy) = a1b2c3d4...
Pixel:      device_seed = DERIVE(root_seed, pixel_entropy) = e5f6g7h8...
```

**Security Properties**:
- Each device has UNIQUE cryptographic material
- Compromising one device doesn't reveal the root seed
- Devices can still prove shared ancestry via certificates

---

## BearDog Methods (11 Total)

### Existing Methods (8)

| Method | Description | Status |
|--------|-------------|--------|
| `genetic.derive_lineage_key` | Derive keys from family lineage | ✅ Works |
| `genetic.derive_lineage_beacon_key` | Beacon encryption key (Dark Forest) | ✅ Works |
| `genetic.mix_entropy` | Three-tier entropy mixing | ✅ Works |
| `genetic.verify_lineage` | Verify genetic relationships | ✅ Basic |
| `genetic.generate_lineage_proof` | Generate lineage proof | ✅ Basic |
| `genetic.generate_challenge` | Dark Forest challenge | ✅ Works |
| `genetic.respond_to_challenge` | Challenge response | ✅ Works |
| `genetic.verify_challenge_response` | Verify response | ✅ Works |

### NEW Methods (3) - February 5, 2026

| Method | Description | Status |
|--------|-------------|--------|
| `genetic.derive_device_seed` | Derive UNIQUE device seed from root | ✅ **NEW** |
| `genetic.sign_lineage_certificate` | Sign device enrollment certificate | ✅ **NEW** |
| `genetic.verify_lineage_certificate` | Verify device certificate | ✅ **NEW** |

---

## API Reference

### genetic.derive_device_seed

Derives a UNIQUE device seed from the family's root genesis seed.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "genetic.derive_device_seed",
  "params": {
    "root_seed": "<base64-encoded, 32 bytes>",
    "device_entropy": "<base64-encoded, >=16 bytes>",
    "device_id": "pixel8a",
    "enrollment_timestamp": 1738713600  // Optional, for reproducibility
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "device_seed": "<base64-encoded, 32 bytes, UNIQUE>",
    "device_id": "pixel8a",
    "kdf": "HKDF-SHA256",
    "domain": "beardog_device_seed_v1",
    "derivation_proof": "<base64-encoded, 32 bytes>"
  },
  "id": 1
}
```

**Key Properties**:
- Uses HKDF-SHA256 with domain separation
- Device entropy includes hardware identifiers, attestation data
- `derivation_proof` allows verifying the seed was derived correctly
- Same inputs = same output (deterministic)
- Different device_entropy = different device_seed (DERIVE, not COPY!)

---

### genetic.sign_lineage_certificate

Signs a lineage certificate for device enrollment.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "genetic.sign_lineage_certificate",
  "params": {
    "parent_seed": "<base64-encoded, 32 bytes>",
    "parent_device_id": "usb-desktop",
    "child_public_key": "<base64-encoded Ed25519 pubkey, 32 bytes>",
    "child_device_id": "pixel8a",
    "family_id": "8ff3b864a4bc589a",
    "expires_at": 1770249600  // Optional Unix timestamp
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "certificate": {
      "version": 1,
      "parent_device_id": "usb-desktop",
      "child_device_id": "pixel8a",
      "child_public_key": "<base64>",
      "family_id": "8ff3b864a4bc589a",
      "issued_at": 1738713600,
      "expires_at": 1770249600,
      "depth": 1,
      "parent_signature": "<base64-encoded Ed25519 signature>",
      "parent_public_key": "<base64-encoded Ed25519 pubkey>"
    },
    "certificate_id": "<hex-encoded SHA256 of certificate>"
  },
  "id": 1
}
```

**Certificate Structure**:
- `version`: Protocol version (currently 1)
- `depth`: Position in lineage tree (1 = direct child of enrolling device)
- `parent_signature`: Ed25519 signature over canonical certificate data
- `parent_public_key`: For independent verification

---

### genetic.verify_lineage_certificate

Verifies a lineage certificate's authenticity and validity.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "genetic.verify_lineage_certificate",
  "params": {
    "certificate": { /* LineageCertificate object */ },
    "expected_family_id": "8ff3b864a4bc589a",  // Optional
    "trust_anchors": [ /* Array of trusted certificates */ ]  // Optional
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
    "details": {
      "signature_valid": true,
      "not_expired": true,
      "family_id_matches": true,
      "chain_verified": true,
      "chain_depth": 1,
      "failure_reason": null
    }
  },
  "id": 1
}
```

**Verification Steps**:
1. Verify Ed25519 signature is cryptographically valid
2. Check certificate is not expired
3. Verify family_id matches expected (if provided)
4. Verify chain up to trust anchors (if provided)

---

## Integration Guide

### biomeOS Device Enrollment Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Genesis   │     │  USB Tower  │     │   Pixel 8a  │
│   Device    │     │  (Parent)   │     │   (Child)   │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                   │
       │  1. Store         │                   │
       │  root_seed        │                   │
       │  (encrypted)      │                   │
       │                   │                   │
       │  2. derive_       │                   │
       │  device_seed      │                   │
       │  ────────────────>│                   │
       │                   │                   │
       │                   │  3. Child         │
       │                   │  generates        │
       │                   │  keypair          │
       │                   │  <─────────────── │
       │                   │                   │
       │                   │  4. sign_lineage_ │
       │                   │  certificate      │
       │                   │  ─────────────────>
       │                   │                   │
       │                   │  5. verify_       │
       │                   │  lineage_         │
       │                   │  certificate      │
       │                   │  <─────────────── │
       │                   │                   │
```

### Step-by-Step

1. **Genesis Device**:
   - Generate and store `root_seed` (encrypted, rarely accessed)
   - Use `genetic.derive_device_seed` for its own device_seed

2. **Parent Device (USB Tower)**:
   - Has its own `device_seed` (derived from root)
   - Receives child's public key (generated by child)
   - Calls `genetic.sign_lineage_certificate`

3. **Child Device (Pixel 8a)**:
   - Generates Ed25519 keypair locally
   - Sends public key to parent
   - Receives signed certificate
   - Calls `genetic.verify_lineage_certificate` to validate
   - Stores certificate for future proof-of-lineage

### biomeOS File Structure Changes

**Current (WRONG)**:
```
~/.biomeos/.family.seed  # COPIED - same on all devices!
```

**New (CORRECT)**:
```
~/.biomeos/.root.seed      # Genesis device only, encrypted
~/.biomeos/.device.seed    # Derived, unique per device
~/.biomeos/.lineage.cert   # Signed certificate proving enrollment
```

---

## Security Considerations

### Forward Secrecy
If a device is compromised:
- Attacker gets `device_seed` (unique to that device)
- Attacker CANNOT recover `root_seed` (HKDF is one-way)
- Other devices remain secure

### Certificate Expiration
- Certificates can have optional `expires_at`
- Recommended: 1 year for normal devices, shorter for high-risk

### Trust Anchors
- Root devices self-certify (depth 0)
- Chain verification ensures path to trusted root
- Support for multiple trust anchors (device recovery)

---

## Performance

| Method | Latency | Algorithm |
|--------|---------|-----------|
| `derive_device_seed` | < 200μs | HKDF-SHA256 |
| `sign_lineage_certificate` | < 500μs | Ed25519 |
| `verify_lineage_certificate` | < 300μs | Ed25519 verify |

---

## Test Coverage

All new methods have comprehensive tests:

- `test_derive_device_seed` - Basic derivation
- `test_derive_device_seed_different_devices` - Unique per device
- `test_derive_device_seed_deterministic` - Reproducible
- `test_sign_and_verify_lineage_certificate` - Full roundtrip
- `test_verify_lineage_certificate_wrong_family` - Family mismatch detection
- `test_verify_lineage_certificate_tampered_signature` - Tamper detection

---

## Migration Plan

### Phase 1: BearDog (Complete)
- [x] `genetic.derive_device_seed`
- [x] `genetic.sign_lineage_certificate`
- [x] `genetic.verify_lineage_certificate`
- [x] Tests
- [x] Documentation

### Phase 2: biomeOS
- [ ] Update device enrollment to use `derive_device_seed`
- [ ] Rename `.family.seed` to `.device.seed`
- [ ] Create `.root.seed` for genesis device only
- [ ] Store `.lineage.cert` for enrolled devices
- [ ] Update meeting protocol to exchange certificates

### Phase 3: Songbird
- [ ] Pass `family_id` to BirdSong integration
- [ ] Add `health` standard method
- [ ] Investigate TLS handshake issue

---

## Related Issues

| Issue | Priority | Status |
|-------|----------|--------|
| Lineage DERIVATION | Critical | ✅ BearDog Complete |
| TLS Handshake | High | ⏳ Songbird Investigation |
| BirdSong family_id | Medium | ⏳ Songbird Fix |
| Songbird `health` | Medium | ⏳ Songbird Implementation |

---

## Changelog

### v1.0.0 (February 5, 2026)
- Initial specification
- BearDog implementation complete (3 new methods)
- Total genetic methods: 8 → 11

---

**Author**: AI Assistant + eastgate  
**Last Updated**: February 5, 2026
