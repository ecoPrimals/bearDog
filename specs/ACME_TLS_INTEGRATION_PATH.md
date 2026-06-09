<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# ACME Client Integration Path — Stadial TLS Shadow Cutover

**Date**: May 17, 2026 (design); implemented Wave 107–112 (May 19–24, 2026)
**Status**: Implemented — `crates/beardog-acme/` ships full RFC 8555 client
**Owner**: bearDog (crypto spine)
**Stadial Pairing**: cellMembrane/projectNUCLEUS (TLS termination)

---

## Context

BearDog already terminates TLS via `rustls` (H2-10 sovereignty, Wave 100).
Certificates are loaded from PEM files via `BEARDOG_TLS_CERT_PATH` /
`BEARDOG_TLS_KEY_PATH`. This works for static deployments but does not cover
automated certificate issuance, renewal, or the stadial shadow cutover where
BearDog replaces Cloudflare as the TLS termination point.

This document specifies the integration path for ACME (RFC 8555) certificate
lifecycle management inside BearDog. It is intentionally **design-only** — no
code is required at this stage.

---

## Architecture

### Current State

```
                     ┌────────────────────┐
  Client ──TLS──►    │  rustls acceptor   │  ◄── static PEM (env vars)
                     │  (beardog-tunnel)  │
                     └────────────────────┘
```

### Stadial Target

```
                     ┌────────────────────┐
  Client ──TLS──►    │  rustls acceptor   │  ◄── live cert (ACME resolver)
                     │  (beardog-tunnel)  │
                     └─────────┬──────────┘
                               │ cert reload signal
                     ┌─────────▼──────────┐
                     │  ACME client task   │  ◄── Let's Encrypt / ZeroSSL
                     │  (beardog-acme)     │
                     └─────────┬──────────┘
                               │ challenge solver
                     ┌─────────▼──────────┐
                     │  HTTP-01 / TLS-ALPN │  (port 80/443)
                     │  or DNS-01 via API  │
                     └────────────────────┘
```

---

## Design Decisions

### 1. Crate Location

New crate: `crates/beardog-acme/`

Dependencies (all pure Rust, no C):
- `rustls` (already in workspace — shared with beardog-tunnel)
- `p256` + `x509-cert` — CSR generation (pure Rust, RustCrypto; replaced `rcgen`)
- `reqwest` (rustls backend) — ACME directory + order HTTP calls
- `base64ct` — JWS encoding (already in workspace)

The crate must not introduce `ring` outside the existing `rustls` wrapper
allowance in `deny.toml`.

### 2. Challenge Types

| Challenge | Use Case | BearDog Role |
|-----------|----------|-------------|
| HTTP-01 | Standard web servers | bearDog serves `/.well-known/acme-challenge/` on port 80 |
| TLS-ALPN-01 | Port-443-only deployments | bearDog handles ALPN `acme-tls/1` in rustls config |
| DNS-01 | Wildcard certs, no inbound ports | bearDog calls DNS API (provider-agnostic trait) |

**Default**: HTTP-01 (simplest, no DNS API credentials needed).
TLS-ALPN-01 preferred for stadial deployments where port 80 is unavailable.

### 3. Certificate Storage

Certificates and account keys persist to the filesystem:
- `$BEARDOG_DATA_DIR/acme/account.json` — ACME account key (Ed25519)
- `$BEARDOG_DATA_DIR/acme/certs/<domain>/fullchain.pem`
- `$BEARDOG_DATA_DIR/acme/certs/<domain>/privkey.pem`

On startup, BearDog loads existing certs. If expired or missing, the ACME
client triggers issuance.

### 4. Hot Reload

`rustls::ServerConfig` supports `Arc`-wrapped certificate resolvers. BearDog's
TLS acceptor already wraps `ServerConfig` in `Arc`. The ACME client:

1. Obtains a new certificate
2. Writes PEM to disk
3. Builds a new `ServerConfig` with the fresh cert
4. Atomically swaps the `Arc<ServerConfig>` via `ArcSwap` or `RwLock`

No server restart required. Active connections continue on the old cert;
new connections use the new cert.

### 5. Renewal Strategy

- Check cert expiry on startup and every 12 hours (tokio interval)
- Renew at 30 days before expiry (Let's Encrypt default: 90-day certs)
- On renewal failure: log warning, retry with exponential backoff
- If cert expires entirely: fall back to `BEARDOG_TLS_CERT_PATH` static cert
  if configured, otherwise refuse TLS connections with clear error

### 6. Shadow Cutover Mode

For the stadial transition away from Cloudflare:

1. **Shadow mode**: BearDog serves TLS alongside Cloudflare. DNS points to
   Cloudflare; BearDog's ACME domain uses a subdomain or alternate record.
   Both endpoints serve identical content. Monitoring compares responses.

2. **Cutover**: DNS switches from Cloudflare to BearDog's IP. BearDog's ACME
   cert is already valid and renewed. No downtime.

3. **Rollback**: DNS switches back to Cloudflare. BearDog continues renewing
   its cert in the background (no wasted state).

Environment variable: `BEARDOG_TLS_MODE=acme|static|shadow`
- `static` (default, current behavior): load from `BEARDOG_TLS_CERT_PATH`
- `acme`: full ACME lifecycle, no static fallback
- `shadow`: ACME lifecycle + static cert as fallback during validation

---

## IPC Surface (future)

When built, the ACME client exposes these JSON-RPC methods for
cellMembrane/projectNUCLEUS orchestration:

```
acme.status        → { "domain", "expires_at", "issuer", "mode" }
acme.trigger_renew → { "order_url", "status" }
acme.list_certs    → { "certs": [...], "count" }
```

These are **not registered yet** — they will be added when the crate is built.

---

## Security Considerations

- ACME account key is Ed25519 (no RSA key generation needed)
- Account key stored on-disk with filesystem permissions (0600)
- Optional: account key stored in BearDog's encrypted secret storage
  (`secrets.store` IPC) for family-scoped protection
- All ACME HTTP calls use `rustls` (no `native-tls`, no `openssl`)
- Challenge tokens are ephemeral and scoped to the validation period
- DNS-01 credentials (if used) are env-var-configured, never logged

---

## Dependencies on Other Primals

| Primal | Role | Required? |
|--------|------|-----------|
| cellMembrane | DNS record management for cutover | No (manual DNS is fine) |
| projectNUCLEUS | Orchestrates shadow→cutover transition | No (can be manual) |
| nestGate | Persistent cert storage across restarts | No (filesystem default) |

BearDog operates independently. Downstream primals enrich the cutover
workflow but are not gated dependencies.

---

## Implementation Phases

### Phase 1 (stadial entry): Design complete, documented (this document)

### Phase 2 (stadial active): Build `beardog-acme` crate
- ACME directory discovery
- Account registration (Ed25519 JWK)
- HTTP-01 challenge solver
- Certificate issuance and PEM persistence
- Hot reload via `Arc` swap

### Phase 3 (stadial mature): Shadow cutover tooling
- `acme.status` / `acme.trigger_renew` IPC methods
- TLS-ALPN-01 challenge support
- DNS-01 provider trait (Cloudflare, Route53, manual)
- Monitoring integration with health triad

---

## References

- [RFC 8555 — ACME](https://www.rfc-editor.org/rfc/rfc8555)
- [Let's Encrypt documentation](https://letsencrypt.org/docs/)
- Wave 100 handoff: `BEARDOG_V090_WAVE100_TLS_RATELIMIT_SOVEREIGNTY_HANDOFF_MAY10_2026.md`
- `DEPLOYMENT_VALIDATION_STANDARD.md` (wateringHole)
- `SOVEREIGNTY_STANDARDS.md` (wateringHole)
