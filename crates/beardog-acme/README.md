<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# beardog-acme

ACME (RFC 8555) certificate lifecycle client for `BearDog`'s TLS sovereignty.

## Purpose

Enables automated certificate issuance and renewal, replacing Cloudflare's
managed TLS with self-sovereign certificate management via Let's Encrypt
or any ACME-compatible CA.

## Architecture

- **Account management**: Ed25519 account key generation, JWS signing
- **HTTP-01 challenge solver**: Lightweight server on port 80
- **Certificate storage**: PEM persistence at `$BEARDOG_DATA_DIR/acme/`
- **Renewal daemon**: 12h check cycle, 30-day-before-expiry threshold

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `BEARDOG_ACME_DIRECTORY` | Let's Encrypt production | ACME directory URL |
| `BEARDOG_ACME_DOMAINS` | (required) | Comma-separated domain list |
| `BEARDOG_ACME_EMAIL` | (optional) | Contact email for CA notifications |
| `BEARDOG_ACME_CHALLENGE_PORT` | 80 | Port for HTTP-01 challenge server |
| `BEARDOG_ACME_RENEWAL_DAYS` | 30 | Days before expiry to trigger renewal |
| `BEARDOG_DATA_DIR` | `~/.beardog` | Root data directory |

## Status

- **Phase 2**: Core ACME flow — account, HTTP-01, order, storage
- **Phase 3** (current): Renewal daemon, cert expiry parsing, order finalization, cert download
