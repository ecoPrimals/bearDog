<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Wave 157k — AEAD Neural API Routing Fix + rootPulse Step Handler

**Date**: Aug 14, 2026 | **Wave**: 157k | **Author**: bearDog (eastGate)

---

## Summary

Two routing gaps identified by westGate provenance trio experiments (14/14 pass, blurb item P2 #10 + P2 #11) closed in a single cascade. bearDog now surfaces all AEAD algorithms through the Neural API translation registry and implements the `auth.sign` step handler for rootPulse graph execution.

---

## Fix 1: AEAD Not Surfaced in Neural API (P2 #11)

### Root Cause

The `crypto.aead.*` semantic method namespace had AES-128-GCM and AES-256-GCM methods (added for TLS 1.2 handshake integration) but was missing ChaCha20-Poly1305. The flat-name equivalents (`crypto.chacha20_poly1305_encrypt/decrypt`) existed and routed correctly, but the dotted semantic names (`crypto.aead.chacha20_poly1305.{encrypt,decrypt}`) were absent from:
- The method list (not announced in `primal.announce`)
- The routing table (calls would return "Unknown crypto method")
- The cost hints table (no latency estimates for the Neural API)

### Changes

| File | Change |
|------|--------|
| `crates/beardog-tunnel/src/.../crypto_handler/method_list.rs` | Added `crypto.aead.chacha20_poly1305.encrypt` and `.decrypt` |
| `crates/beardog-tunnel/src/.../crypto_handler/kex_aead.rs` | Added routing arms for both methods (delegate to existing ChaCha20 handlers) |
| `crates/beardog-tunnel/src/.../handlers/capabilities.rs` | Added cost hints (`cpu: low`, `latency_ms: CRYPTO_MS`) |
| `crates/beardog-tunnel/src/.../crypto_handler_tests.rs` | Updated method count assertion 114 → 116 |

### Validation

- `route_aead_chacha20_poly1305_semantic_roundtrip` test: encrypt → decrypt roundtrip via dotted names
- `semantic_aead_methods_complete` test: asserts all 6 AEAD methods (3 algorithms × encrypt/decrypt) are registered

---

## Fix 2: rootPulse Step Handler Activation (P2 #10)

### Root Cause

The rootPulse trio graphs (`rootpulse_commit`, `rootpulse_harvest`) define a step with `primal = "bearDog"` and `operation = "auth.sign"`. When biomeOS's graph executor reaches this step, it calls bearDog via Neural API `capability.call` with method `auth.sign`. bearDog had no `auth.sign` handler, so the call would fail with "Method not found."

### Design

`auth.sign` is a primal-identity signing operation — it signs data with bearDog's Ed25519 identity key (derived from `FAMILY_SEED`), not with an arbitrary user-provided key. This is semantically distinct from `crypto.sign_ed25519` (which takes a `key_id` parameter).

The handler accepts flexible params from graph step bindings:
- `data` (base64): raw bytes to sign
- `content_hash` (base64): hash to sign (rootpulse_commit step)
- `dag_ref` (string): DAG reference to sign (rootpulse_harvest step)
- `dehydrated_blob` (base64): serialized state blob
- fallback: SHA-256 hash of canonical JSON params

Returns: `{ signature, public_key, algorithm, signer, signed_bytes }`

### Changes

| File | Change |
|------|--------|
| `crates/beardog-tunnel/src/.../handlers/security.rs` | Added `auth.sign` to `methods()` and `handle_auth_sign()` implementation |
| `crates/beardog-tunnel/src/primal_announce.rs` | Added `auth.sign` to `GATE_AUTH_ANNOUNCE_METHODS` |
| `crates/beardog-tunnel/src/.../handlers/security_tests.rs` | 5 new tests (data, content_hash, dag_ref, determinism, missing params) |

### Validation

All 5 tests pass. The signing is deterministic (same identity + same data = same signature). Missing params correctly returns error.

---

## Method Count

| Metric | Before | After |
|--------|--------|-------|
| CryptoHandler methods | 114 | 116 (+2 semantic AEAD) |
| SecurityHandler methods | 18 | 19 (+1 auth.sign) |
| Gate announce methods | 7 | 8 (+1 auth.sign) |
| Total dispatchable | 236 | 239 |
| beardog-tunnel tests | 2,481 | 2,483 |

---

## Upstream Status

- **P2 #10 (rootPulse step handler)**: bearDog done. nestGate, rhizoCrypt, sweetGrass still pending.
- **P2 #11 (Neural API translation registry audit)**: bearDog AEAD done. rhizoCrypt dehydration and nestGate content.put still pending.
