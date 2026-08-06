# Wave 156i — G64 Cephalization: tarpc Convergence Push

**Date**: August 6, 2026
**Primal**: bearDog
**Wave**: 156i
**Author**: bearDog code team (eastGate)

---

## Summary

Expanded bearDog's tarpc service from **7 → 30 methods**, covering the full crypto domain and auth/ionic token lifecycle. bearDog moves from **"tarpc-wired"** to **near-converged** in the G64 Cephalization table.

## Changes

### tarpc Method Surface (30 methods)

| Category | Methods | Count |
|----------|---------|-------|
| Health | `health_check`, `version` | 2 |
| Hash | `blake3_hash`, `sha256`, `sha384`, `sha512` | 4 |
| MAC | `hmac_sha256`, `blake3_keyed` | 2 |
| KDF | `hkdf_sha256`, `blake3_derive_key`, `argon2id_hash`, `derive_key` | 4 |
| Signing | `sign_ed25519`, `verify_ed25519` | 2 |
| Key Exchange | `x25519_generate_ephemeral`, `x25519_derive_secret` | 2 |
| AEAD | `chacha20_poly1305_{encrypt,decrypt}` | 2 |
| AEAD | `aes256_gcm_{encrypt,decrypt}` | 2 |
| AEAD | `aes128_gcm_{encrypt,decrypt}` | 2 |
| Auth | `auth_issue_ionic`, `auth_verify_ionic`, `auth_public_key` | 3 |
| Auth | `auth_issue_session`, `identity_create` | 2 |
| KDF | `derive_key` (FAMILY_SEED + BLAKE3) | 1 |
| **Total** | | **30** |

### New Types

- `AeadCiphertext` — AEAD encryption result (ciphertext + nonce + tag)
- `X25519Keypair` — ephemeral keypair (secret + public)
- `IonicTokenResult` — issued token + metadata
- `IonicVerifyResult` — verification result with claims or error
- `PublicKeyInfo` — primal public key + DID

### Tests

- `tarpc_e2e_health_roundtrip` — full client→server health check over real UDS
- `tarpc_e2e_crypto_roundtrip` — BLAKE3, SHA, HMAC, HKDF, ChaCha20-Poly1305, AES-256-GCM roundtrips
- `tarpc_e2e_auth_roundtrip` — issue ionic → verify ionic → public key → identity create

## bearDog G64 Status

| Metric | Before | After |
|--------|--------|-------|
| tarpc methods | 7 | **30** |
| Cephalization state | tarpc-wired | **near-converged** |
| Domains covered | crypto (partial) | crypto (full) + auth |
| E2E tarpc tests | 2 | **6** |

## Remaining for full convergence

- `btsp.*` methods (negotiate, capabilities) — requires transport-layer refactoring
- `security.*` methods (consent, evaluate, lineage) — depends on runtime state
- `bonding.*` methods (propose, accept, status) — ionic bond lifecycle
- `secrets.*`, `relay.*`, `consent.*` — lower priority

These are lower-frequency operations where JSON-RPC's flexibility is more valuable than tarpc's speed. The crypto + auth hot-path is where tarpc convergence matters most.

## For Overwatch

bearDog should be reclassified in the G64 table:
- **Old**: "tarpc dep only" (3 primals)
- **Actual**: "tarpc-wired" (since 156h) → **near-converged** (156i, 30 methods)

The blurb's E1 item (Neural API routing stub) was also shipped in 156e.
