# AAR — Wave 155i: ACME Phase 2 Crypto Delegation Surface

**Date**: Jul 29, 2026
**Wave**: 155i
**Gate**: G6 (songBird ACME dependency)
**Status**: SHIPPED — 5 new JSON-RPC methods, 17 new tests

---

## Objective

Expose the crypto operations that songBird needs to delegate ACME certificate
lifecycle management to bearDog as the trust foundation. songBird owns the
ACME protocol (HTTP-01 challenges, renewal scheduling, TLS termination);
bearDog owns the cryptographic primitives (key generation, signing, CSR building).

## What We Shipped

### 5 New JSON-RPC Methods

| Method | Purpose | RFC |
|--------|---------|-----|
| `crypto.ecdsa_p256_generate_signing_keypair` | Persistent P-256 signing key with JWK output | — |
| `crypto.sign_jws_es256` | JWS ES256 signing (raw 64-byte `r\|\|s`, not DER) | RFC 7515 |
| `crypto.jwk_thumbprint` | JWK thumbprint via SHA-256 | RFC 7638 |
| `x509.build_csr` | PKCS#10 CSR with SubjectAltName | RFC 2986 |
| `x509.parse_certificate` | X.509 metadata extraction (expiry, SANs, issuer) | RFC 5280 |

### Architecture

```
songBird (ACME protocol)          bearDog (crypto via JSON-RPC)
────────────────────────          ─────────────────────────────
ACME directory/orders       →→→   crypto.ecdsa_p256_generate_signing_keypair
HTTP-01 challenge server    →→→   crypto.jwk_thumbprint (keyAuthorization)
JWS request signing         →→→   crypto.sign_jws_es256
Certificate finalization    →→→   x509.build_csr
Renewal monitoring          →→→   x509.parse_certificate (expiry check)
```

### Files Changed

| File | Change |
|------|--------|
| `crates/beardog-tunnel/Cargo.toml` | Added `x509-cert` dependency |
| `crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto/acme.rs` | **NEW** — 5 handlers + 17 tests |
| `crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto/mod.rs` | Wired `acme` module + re-exports |
| `crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto_handler/acme_ops.rs` | **NEW** — dispatch router |
| `crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto_handler/mod.rs` | Added `acme_ops` module |
| `crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto_handler/router.rs` | Added `acme_ops` to dispatch chain |
| `crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto_handler/method_list.rs` | Registered 5 new methods |
| `crates/beardog-tunnel/src/unix_socket_ipc/handlers/capabilities.rs` | Added cost estimates |
| `crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto_handler_tests.rs` | Updated method count 109→114 |

### Key Design Decisions

1. **JWS format**: `crypto.sign_jws_es256` returns raw 64-byte `(r||s)` with
   base64url encoding — ACME requires this, not ASN.1 DER.

2. **JWK in keygen**: `crypto.ecdsa_p256_generate_signing_keypair` returns the
   public key as both SEC1 and JWK — songBird needs the JWK for ACME account
   registration headers.

3. **CSR with existing key**: `x509.build_csr` accepts an optional `private_key`
   parameter so songBird can reuse a previously generated key.

4. **`x509.parse_certificate` behind `tls-x509`**: Uses the same `x509-parser`
   dependency as `tls.verify_certificate`. Always available in default build.

### Test Coverage

- 17 new tests covering all 5 handlers + routing
- Total workspace: 14,013 passed, 0 failed
- Clippy: 0 warnings

## songBird Integration Guide

### ACME Account Registration

```json
// 1. Generate ACME account key
{"method": "crypto.ecdsa_p256_generate_signing_keypair"}
// → {"private_key": "...", "public_key_jwk": {"kty":"EC","crv":"P-256",...}}

// 2. Compute thumbprint for HTTP-01
{"method": "crypto.jwk_thumbprint", "params": {"public_key_jwk": {...}}}
// → {"thumbprint": "base64url_sha256"}

// 3. Sign ACME requests (JWS ES256)
{"method": "crypto.sign_jws_es256", "params": {
  "signing_input": "base64_protected.payload",
  "private_key": "base64_account_key"
}}
// → {"signature": "base64url_64_byte_r_s"}
```

### Certificate Issuance

```json
// 4. Build CSR for domains
{"method": "x509.build_csr", "params": {
  "domains": ["example.com", "www.example.com"]
}}
// → {"csr_der": "base64_pkcs10", "private_key": "...", ...}

// 5. Monitor cert expiry
{"method": "x509.parse_certificate", "params": {
  "certificate": "base64_der_cert"
}}
// → {"not_after": 1714003200, "san_entries": [...], ...}
```

## What bearDog Does NOT Own

- ACME protocol HTTP calls (`reqwest`, directory, orders, challenges)
- HTTP-01 challenge TCP server
- Renewal scheduling / daemon
- TLS termination (`rustls`, `ServerConfig`)
- Certificate PEM filesystem storage

These remain in `beardog-acme` (deprecated, feature-gated) and will be ported
to songBird per the Wave 155b 3-phase deprecation plan.
