# Handoff — Wave 155i: ACME Phase 2 Crypto Delegation

**Date**: Jul 29, 2026
**Wave**: 155i
**From**: bearDog code team (eastGate)
**To**: songBird (ACME protocol absorption)

---

## Summary

bearDog now exposes 5 JSON-RPC methods for ACME certificate lifecycle crypto.
songBird can delegate all signing, CSR, and key operations to bearDog via
Tower Atomic IPC. No new dependencies needed on songBird's side beyond the
existing JSON-RPC client.

## New Methods (114 total crypto methods)

| Method | Input | Output |
|--------|-------|--------|
| `crypto.ecdsa_p256_generate_signing_keypair` | `{}` | `private_key`, `public_key`, `public_key_jwk` |
| `crypto.sign_jws_es256` | `signing_input`, `private_key` | `signature` (base64url, 64-byte `r\|\|s`) |
| `crypto.jwk_thumbprint` | `public_key_jwk` or `public_key` | `thumbprint` (base64url SHA-256) |
| `x509.build_csr` | `domains[]`, optional `private_key` | `csr_der`, `private_key`, `public_key` |
| `x509.parse_certificate` | `certificate` (DER b64) or `certificate_pem` | `subject`, `issuer`, `not_before`, `not_after`, `san_entries`, `is_ca` |

## songBird Action Items

1. Port ACME protocol logic from `beardog-acme` (HTTP client, order lifecycle,
   challenge server, renewal daemon)
2. Replace inline `p256::ecdsa::SigningKey` calls with `crypto.ecdsa_p256_generate_signing_keypair`
3. Replace inline JWS signing with `crypto.sign_jws_es256`
4. Replace inline CSR building with `x509.build_csr`
5. Use `x509.parse_certificate` for renewal monitoring
6. Use `crypto.jwk_thumbprint` for HTTP-01 `keyAuthorization` computation

## Deprecation Plan Progress

| Phase | Status |
|-------|--------|
| Phase 1: Feature-gate TLS/ACME | **DONE** (Wave 155b) |
| Phase 2: bearDog exposes crypto IPC | **DONE** (this wave) |
| Phase 3: songBird absorbs ACME | **NEXT** — songBird team |
| Phase 4: bearDog excises `beardog-acme` | After songBird ships |

## Verification

```bash
cargo test -p beardog-tunnel --lib -- acme  # 17 tests pass
cargo clippy -p beardog-tunnel              # 0 warnings
cargo test --workspace                      # 14,013 passed, 0 failed
```
