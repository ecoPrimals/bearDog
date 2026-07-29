# AAR — Wave 155j: crypto.sign_ed25519 Direct Key Signing

**Date**: Jul 29, 2026  
**Wave**: 155j  
**Author**: bearDog team (eastGate)  
**Status**: COMPLETE — P1 unblock for Provenance Trio 7/7

---

## Context

The Wave 155i overwatch blurb identified `crypto.sign_ed25519` as the **sole P1 blocker
for Provenance Trio 7/7**, stating that loamSpine `entry.append` calls bearDog to sign
and "gets a health response instead of a signature."

## Investigation

### What We Found

1. **The handler was NOT a health stub.** `handle_sign_ed25519` has been doing real
   Ed25519 signing since at least Wave 34 (deep debt evolution). It calls
   `asymmetric::sign_ed25519()` and returns a valid 64-byte signature. 45 Ed25519
   tests all pass, including sign-verify roundtrip tests.

2. **The real gap**: The handler only supported **key derivation** from a `key_id` param
   (via BLAKE3 KDF from `BEARDOG_MASTER_KEY`). It did NOT accept a `secret_key` parameter
   directly. If loamSpine calls with `{"message": "...", "secret_key": "..."}`, the
   handler ignored `secret_key` and signed with the derived key from
   `"default_signing_key"`, producing a valid signature but with the **wrong key**.

3. **Blurb test count discrepancy**: The blurb showed 11,993 tests; we have 14,019.
   The blurb was working from a stale snapshot (pre-Waves 155c/d/i).

4. **"Health response" likely from stale deployment**: The westGate binary may not
   have been refreshed since the depot was last built. On stale builds, the method may
   not have been registered, causing `MethodNotFound` → fallback that surfaces as a
   non-signature response.

## Changes Made

### `handle_sign_ed25519` — Dual-Mode Signing

The handler now supports two key-sourcing modes:

**Mode 1 — Direct Key** (new): Caller provides `secret_key` (base64-encoded 32-byte
Ed25519 seed). Uses `SigningKey::from_bytes()` directly. Returns `signature`,
`algorithm`, `public_key` (no `key_id` in response).

**Mode 2 — Derived Key** (existing): Caller provides `key_id` + optional `purpose`.
Signing key is deterministically derived from `BEARDOG_MASTER_KEY` via BLAKE3 KDF.
Returns `signature`, `algorithm`, `key_id`, `public_key`.

If `secret_key` is present, `key_id`/`purpose` are ignored.

### loamSpine Integration Contract

loamSpine `entry.append` can now:
```json
{
  "method": "crypto.sign_ed25519",
  "params": {
    "message": "<base64-encoded payload>",
    "secret_key": "<base64-encoded 32-byte Ed25519 seed>"
  }
}
```

Response:
```json
{
  "signature": "<base64-encoded 64-byte Ed25519 signature>",
  "algorithm": "Ed25519",
  "public_key": "<base64-encoded 32-byte public key>"
}
```

All method aliases work: `crypto.sign_ed25519`, `crypto.ed25519.sign`,
`crypto.sign.ed25519`, `crypto.sign` (semantic), `sign_ed25519` (bare),
`beardog.crypto.sign_ed25519`, and `capability.call` with
`{"capability": "crypto", "operation": "sign_ed25519"}`.

### Tests Added (6)

| Test | What It Validates |
|------|-------------------|
| `test_ed25519_sign_with_direct_secret_key` | Direct key mode returns valid signature + public_key, no key_id |
| `test_ed25519_generate_sign_verify_roundtrip` | Full generate→sign(direct)→verify lifecycle |
| `test_ed25519_direct_key_wrong_length_rejected` | 16-byte key → error |
| `test_ed25519_direct_key_invalid_base64_rejected` | Malformed base64 → error |
| `test_ed25519_direct_key_takes_precedence_over_key_id` | secret_key wins when both provided |
| `routes_crypto_sign_ed25519_with_direct_key` | Full capability.call dispatch E2E |

## Metrics

| Metric | Before | After |
|--------|--------|-------|
| Tests | 14,013 | 14,019 |
| Clippy warnings | 0 | 0 |
| Ed25519 tests | 45 | 51 |
| capability.call tests | 11 | 12 |

## Upstream Action Items

### For loamSpine / Provenance Trio

- **bearDog `crypto.sign_ed25519` is LIVE with direct key support.**
- If currently calling with `secret_key` in params — should now work.
- If using `capability.call` — same: `{"capability": "crypto", "operation": "sign_ed25519", "args": {"message": ..., "secret_key": ...}}`.
- **Verify the westGate bearDog binary is refreshed** — the binary may be stale
  (blurb shows 11,993 tests vs our 14,019). Rebuild from `main` and redeploy.

### For overwatch

- bearDog `crypto.sign_ed25519` was never a health stub in current code — the gap
  was **missing direct key acceptance**, which is now resolved.
- The "health response" observation may be from a stale westGate deployment.
- **Provenance Trio 7/7 unblocked** from bearDog's side. loamSpine can now sign
  with caller-provided keys.

### For sporeGate / depot

- westGate Linux depot needs a bearDog binary refresh to pick up Waves 155c through
  155j changes (14,019 tests, ACME Phase 2, Clippy zero, this fix).

## Files Changed

- `crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto/asymmetric.rs` — Dual-mode signing
- `crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto/asymmetric_tests.rs` — 5 new tests
- `crates/beardog-tunnel/src/unix_socket_ipc/handlers/capability_call.rs` — 1 new E2E test
- `STATUS.md` — Updated

---

*Wave 155j — crypto.sign_ed25519 P1 unblock. Direct key signing. 14,019 tests. 0 warnings.*
