<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# AAR: Wave 156k — Genetics & Entropy Live Exploration

**Date**: August 6, 2026
**Author**: bearDog local (eastGate)
**Hardware**: SoloKey v2 (FIDO2), grapheneGate (Pixel 8a), software HSM (eastGate)

---

## Objective

Explore bearDog's genetics, ephemeral entropy, entropy hierarchy, birdsong
encryption, and Dark Forest beacon systems hands-on with live hardware.
Determine what's stable, what's nascent, and where the next evolution
targets are.

---

## What We Found

### Stable and Functional (40 JSON-RPC calls, 0 crashes)

| System | Status | Evidence |
|--------|--------|----------|
| **Lineage key derivation** | Production-ready | Peer-specific keys verified (eastGate↔grapheneGate ≠ eastGate↔iosGate). Blake3-Lineage-KDF, quality 0.8 |
| **Lineage proof roundtrip** | Production-ready | `generate_lineage_proof` → `verify_lineage` → `valid: true` |
| **Device seed derivation** | Production-ready | HKDF-SHA256 with derivation proof. Device-bound, non-reproducible without root seed |
| **Challenge generation** | Production-ready | UUID + 32-byte nonce. Cross-gate auth primitive |
| **Birdsong encrypt/decrypt** | Production-ready | ChaCha20-Poly1305 family-scoped roundtrip. `"hello from eastGate solokey session"` verified |
| **Encrypted beacon generation** | Production-ready | Zero-metadata beacon ID via birdsong |
| **Dark Forest beacon roundtrip** | Production-ready | `beacon.encrypt` → `beacon.try_decrypt`. `"dark forest signal from eastGate"` verified |
| **Entropy mixing** | Functional but flat | SHA3-256 mixing engine works. Quality scoring does not differentiate tiers |

### Nascent (works, but does not yet leverage available hardware)

1. **Entropy quality scoring is flat at 0.4** regardless of which tiers
   (human/machine/cosmic) are provided. The type system supports
   classification but the handler treats all input as a single machine-tier
   fallback.

2. **FIDO2 hardware entropy path exists but is async-only**. The
   `generate_from_hsm_async` method in the entropy orchestrator can XOR
   hardware entropy with OS RNG, but the sync `mix_entropy` handler doesn't
   call it. SoloKey `hmac-secret` is wired but not plumbed into the genetics
   entropy mixer.

3. **Lineage certificate signing** requires `child_public_key` — designed
   for device enrollment but the full ceremony (parent signs child's key)
   isn't exercised end-to-end from a single IPC call.

4. **Challenge-response verification** requires the full nonce +
   family_seed_path — the three-call ceremony (generate → respond → verify)
   is schema-complete but the respond step needs the family seed path
   convention documented.

---

## Entropy Tiering: The Evolution Gap

### Current Model

```
All entropy → OS CSPRNG → quality 0.4 → done
```

### Target Model (from exploration findings)

```
Tier 3 (0.9): Human + Hardware + External  ←── provenance trio braid
Tier 2 (0.7): Hardware HSM + OS RNG        ←── SoloKey hmac-secret, StrongBox
Tier 1 (0.4): OS CSPRNG only               ←── current baseline
Tier 0 (<0.4): Rejected (fail-closed)
```

### Provenance Trio as Entropy Braid

The **provenance trio** pattern provides a concrete Tier 3 entropy model:

1. **Human**: API keys contributed to data sources, SoloKey physical tap,
   biometric gate. Non-fungible, attributable.
2. **Hardware**: SoloKey `hmac-secret`, StrongBox RNG. Physically bound,
   non-reproducible.
3. **External**: Blockchain anchor timestamp, external attestation.
   Non-repudiable.

When braided via SHA3-512 mixing, no single source can reconstruct the
result. This is the foundation of **sunCloud radiant attribution** — the
act of contributing creates a cryptographic receipt that flows through
the ecosystem as proof-of-contribution.

---

## Ledger Anchoring: External Membrane Concept

bearDog's internal crypto provides the **inner membrane** of trust. Blockchain
networks can serve as an **external membrane** — a public, immutable record.

**bearDog's role**: crypto atoms only.
- `crypto.hash_blake3` → state hash
- `crypto.sign_ed25519` → sign the hash
- `genetic.derive_lineage_key` → per-chain key derivation

**Ecosystem's role** (rhizoP / loamSpine):
- Chain selection (ETH, BTC, SOL, Cosmos)
- Transaction construction (chain-specific formats)
- Gas management
- Publishing and verification

The anchoring is **gas-only** — no token accumulation, no DeFi. The chain
is used purely as a timestamped append-only log. bearDog provides signatures;
the ecosystem decides where to publish them.

---

## Specific Evolution Targets

### Phase 1: Quality Score Differentiation (immediate, bearDog-local)

Wire the `genetic.mix_entropy` handler to score differently based on which
tiers are actually provided. No new deps, no ecosystem coordination needed.

### Phase 2: SoloKey Hardware Entropy Path (near-term, bearDog-local)

Connect `generate_from_hsm_async` to the `mix_entropy` handler so SoloKey
`hmac-secret` entropy flows into the mixing engine with proper `hardware_backed`
labeling.

### Phase 3: Attribution Entropy (ecosystem coordination)

Expose `genetic.create_attribution_receipt` for sunCloud radiant attribution.
bearDog hashes the contribution, signs it, timestamps it — downstream
consumers (sunCloud, loamSpine) handle attribution semantics.

### Phase 4: Ledger Anchor Crypto Atoms (ecosystem coordination)

Expose `genetic.anchor_state_hash` that produces a signed state hash suitable
for external chain publishing. rhizoP/loamSpine own chain selection and
transaction construction.

---

## For Upstream

### For overwatch

bearDog genetics/entropy/beacon systems are **stable and functional** across
23 IPC methods. The next expansion target is entropy quality differentiation
(Phase 1) and hardware entropy wiring (Phase 2). Both are bearDog-local work
with no ecosystem dependencies.

### For rhizoP / loamSpine

bearDog can provide signed state hashes (`blake3 + ed25519`) suitable for
ledger anchoring. bearDog does NOT and will NOT hold cryptocurrency, manage
wallets, or construct chain-specific transactions. The ecosystem needs to
define the anchoring convention (which chains, what format, gas strategy)
and bearDog will provide the crypto atoms.

### For sunCloud

Attribution entropy is ready to prototype. bearDog can hash, sign, and
timestamp contribution events. The attribution receipt format and radiant
propagation semantics are sunCloud's domain — bearDog provides the
cryptographic primitives.

### For songBird

Cross-gate entropy exchange (e.g., grapheneGate contributing StrongBox
entropy to eastGate's mixing pool) requires transport. songBird's
JSON-RPC forwarding or tarpc relay is the carrier — bearDog provides
the entropy mixing API and honest labeling.

---

## Hardware Inventory (available for next work)

| Device | Role | Validated |
|--------|------|-----------|
| SoloKey v2 | FIDO2 hardware entropy (hmac-secret), user verification (tap) | Yes (eastGate) |
| grapheneGate (Pixel 8a) | StrongBox HSM, cross-gate peer | Yes (13/13 checks) |
| eastGate (Linux x86_64) | Software HSM, primary development | Yes (production) |
| iosGate (iPhone XS) | Secure Enclave, cross-gate peer | Provisioned (deploy pending Apple cert) |

---

**Conclusion**: The genetics and entropy systems are stable infrastructure
ready for the next evolution push. The provenance trio braid pattern and
ledger anchoring concept define the path from flat quality scoring to a
fully attributable, hardware-backed entropy hierarchy.
