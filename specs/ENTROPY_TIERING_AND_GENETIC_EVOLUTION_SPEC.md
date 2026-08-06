<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Entropy Tiering & Genetic Evolution Specification

**Version**: 2.0.0
**Date**: August 6, 2026
**Status**: Exploration Complete — Evolution Targets Identified
**Depends**: `ENTROPY_HIERARCHY_PRINCIPLE.md` v1.0, `GENETIC_LINEAGE_EVOLUTION_SPEC.md` v1.0

---

## Executive Summary

BearDog's genetics, entropy hierarchy, and beacon systems are **stable and
functional** across the full IPC surface (11 genetic methods, 3 birdsong
methods, 9 beacon methods). Live exploration on eastGate confirmed all
roundtrips: lineage key derivation, proof generation/verification, birdsong
encrypt/decrypt, beacon encrypt/decrypt, device seed derivation, and
three-tier entropy mixing.

These systems are **nascent** — they execute correctly but do not yet
differentiate entropy quality based on actual source provenance. This spec
defines the evolution path from flat quality scoring to a real tiered entropy
model that leverages the hardware we now have validated: SoloKey v2 (FIDO2),
grapheneGate StrongBox, and software HSM.

---

## Current State (Wave 156k Exploration)

### What Works

| System | Methods | Roundtrip Verified |
|--------|---------|-------------------|
| Lineage key derivation | `genetic.derive_lineage_key` | Peer-specific keys (eastGate↔grapheneGate ≠ eastGate↔iosGate) |
| Dark Forest beacon keys | `genetic.derive_lineage_beacon_key` | HKDF-SHA256 + ChaCha20-Poly1305, deterministic |
| Lineage proofs | `genetic.generate_lineage_proof` → `genetic.verify_lineage` | HMAC roundtrip verified |
| Challenge-response | `genetic.generate_challenge` | UUID + 32-byte nonce |
| Device seeds | `genetic.derive_device_seed` | HKDF-SHA256 with derivation proof |
| Birdsong encryption | `birdsong.encrypt` → `birdsong.decrypt` | ChaCha20-Poly1305, family-scoped |
| Encrypted beacons | `birdsong.generate_encrypted_beacon` | Zero-metadata beacon ID |
| Dark Forest beacon | `beacon.generate` → `beacon.encrypt` → `beacon.try_decrypt` | Full roundtrip |
| Entropy mixing | `genetic.mix_entropy` | SHA3-256 mixing engine functional |

### What's Flat

The entropy mixing handler reports `quality_score: 0.4` and `tiers_used: 1`
regardless of which tiers are provided. All three named tiers (human, machine,
cosmic) are accepted but mixed uniformly through a single OS RNG fallback path.
The type system supports classification but the runtime does not yet score
differently based on source provenance.

---

## Evolution Target: Real Entropy Tiering

### Tier Model

```
Tier 3  ┌─────────────────────────────────────────────────┐  quality ≥ 0.9
        │  BRAIDED: Human + Hardware + External           │
        │  Sources: SoloKey hmac-secret tap + OS RNG +    │
        │           external attestation (provenance trio) │
        │  Useful for: Genesis seeds, root key material   │
        └─────────────────────────────────────────────────┘

Tier 2  ┌─────────────────────────────────────────────────┐  quality ≥ 0.7
        │  HARDWARE-BACKED: Device HSM + OS RNG           │
        │  Sources: StrongBox/SecureEnclave RNG, FIDO2    │
        │           hardware entropy, OS CSPRNG           │
        │  Useful for: Session keys, enrollment seeds     │
        └─────────────────────────────────────────────────┘

Tier 1  ┌─────────────────────────────────────────────────┐  quality ≥ 0.4
        │  MACHINE: OS CSPRNG only                        │
        │  Sources: /dev/urandom, rand crate OsRng        │
        │  Useful for: Ephemeral nonces, request IDs      │
        └─────────────────────────────────────────────────┘

Tier 0  ┌─────────────────────────────────────────────────┐  quality < 0.4
        │  DEGRADED: insufficient entropy                 │
        │  REJECT — fail-closed                           │
        └─────────────────────────────────────────────────┘
```

### Provenance Trio Braiding

The **provenance trio** pattern (from sunCloud radiant attribution) provides a
concrete example of Tier 3 entropy:

1. **Human contribution**: A human provides API keys, credentials, or
   physical interaction (SoloKey tap, biometric gate). This is non-fungible
   and attributable to a specific person.

2. **Hardware attestation**: The SoloKey's `hmac-secret` extension or
   StrongBox's hardware RNG produces entropy that is physically bound to
   a specific device. This is non-reproducible without physical possession.

3. **External witness**: A blockchain anchor, time-stamped ledger entry, or
   external service attestation provides a third-party proof that the entropy
   event occurred at a specific time. This is non-repudiable.

When all three are present and mixed, the resulting entropy is **braided** —
no single source can reconstruct it, and the provenance chain is auditable
end-to-end.

```
Human (API key / SoloKey tap)  ──┐
                                 ├──→  SHA3-512 braid  ──→  Tier 3 seed
Hardware (hmac-secret / RNG)   ──┤                          quality: 0.95
                                 │                          provenance: auditable
External (chain anchor / TS)   ──┘                          non-fungible: yes
```

---

## Evolution Target: Attribution Entropy

### sunCloud Radiant Attribution

When a human provides API keys to data sources, that act is itself an entropy
event — it represents a human decision, bound to a specific identity, at a
specific time. BearDog can capture this as **attribution entropy**:

```rust
pub struct AttributionEntropy {
    /// What the human provided (hashed, never stored raw)
    pub contribution_hash: [u8; 32],
    /// When the contribution was made
    pub timestamp: DateTime<Utc>,
    /// Which device witnessed the contribution
    pub witness_device: DeviceId,
    /// Quality: always Tier 3 (human + hardware + timestamped)
    pub quality: f64,
}
```

This kicks off the **sunCloud radiant attribution** pipeline: the act of
contributing data sources creates a cryptographic receipt that flows through
the ecosystem as proof-of-contribution. BearDog doesn't need to understand
the data — it provides the crypto atoms (hashing, signing, timestamping)
that make attribution verifiable.

---

## Evolution Target: Ledger Anchoring as External Membrane

### Concept

BearDog's internal crypto (BLAKE3, Ed25519, ChaCha20-Poly1305) provides the
**inner membrane** of trust. Blockchain anchoring provides an **external
membrane** — a public, immutable record that the internal state existed at
a given time.

rhizoP and loamSpine can leverage cryptocurrency networks as **ledger marks**:

```
bearDog internal crypto  ──→  Ed25519 signature of state hash
                              │
loamSpine / rhizoP       ──→  Publish signature + hash to chain(s)
                              │
                              ├── Ethereum (gas-only, OP_RETURN equivalent)
                              ├── Bitcoin (OP_RETURN, ~$0.50)
                              ├── Solana (memo program, sub-cent)
                              └── Cosmos (IBC memo)
```

The anchoring is **gas-only** — no token accumulation, no DeFi, no smart
contract complexity. The chain is used purely as a **timestamped append-only
log** that anyone can verify independently. bearDog provides the signature;
the ecosystem decides where to anchor it.

### What bearDog Owns

- `crypto.hash_blake3` — state hash
- `crypto.sign_ed25519` — sign the hash
- `genetic.derive_lineage_key` — per-chain key derivation
- `beacon.encrypt` — optional encrypted metadata

### What bearDog Does NOT Own

- Chain selection (rhizoP / loamSpine decision)
- Transaction construction (chain-specific)
- Gas management (wallet concern)
- Token economics (not bearDog's domain)

bearDog provides **crypto atoms**. The ecosystem provides **chain strategy**.

---

## Implementation Phases

### Phase 1: Quality Score Differentiation (near-term)

Wire the `mix_entropy` handler to return different `quality_score` values
based on which tiers were actually provided:

| Tiers Provided | quality_score | tiers_used |
|---------------|---------------|------------|
| None (fallback) | 0.4 | 1 |
| Machine only | 0.4 | 1 |
| Machine + Human | 0.7 | 2 |
| Machine + Cosmic | 0.6 | 2 |
| Machine + Human + Cosmic | 0.9 | 3 |

### Phase 2: Hardware Entropy Source (SoloKey hmac-secret)

Wire `generate_from_hsm_async` path through the entropy mixer so that
SoloKey-sourced entropy is labeled `hardware_backed: true` and scored
at Tier 2 (0.7+).

### Phase 3: Provenance Tracking

Add `EntropyProvenance` struct to entropy generation results that records
which sources contributed, their individual quality scores, and whether
hardware attestation was present.

### Phase 4: Attribution Entropy + Ledger Anchoring

Expose `genetic.create_attribution_receipt` and
`genetic.anchor_state_hash` methods that produce crypto atoms suitable
for external chain publishing by rhizoP/loamSpine.

---

## Dependencies

| Component | Owner | Role |
|-----------|-------|------|
| bearDog | local | Crypto atoms, entropy mixing, provenance tracking |
| SoloKey v2 | hardware (eastGate) | `hmac-secret` hardware entropy |
| grapheneGate | hardware (Pixel 8a) | StrongBox hardware RNG |
| rhizoP | upstream primal | Chain selection, transaction construction |
| loamSpine | upstream primal | Ledger mark strategy, anchor publishing |
| sunCloud | upstream system | Radiant attribution pipeline |
| songBird | upstream primal | Transport for cross-gate entropy exchange |

---

## Non-Goals

- bearDog does NOT hold cryptocurrency or manage wallets
- bearDog does NOT select which chain to anchor on
- bearDog does NOT store raw API keys or credentials
- bearDog does NOT implement chain-specific transaction formats
- bearDog provides crypto atoms; ecosystem provides strategy

---

**Status**: Spec complete. Phase 1 (quality score differentiation) is the
immediate evolution target. Phases 2-4 depend on ecosystem coordination.
