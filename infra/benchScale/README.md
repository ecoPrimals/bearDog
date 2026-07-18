# benchScale — bearDog Roundtrip Validation

Local deployment validation harness for bearDog, aligned with the
wateringHole `DEPLOYMENT_VALIDATION_STANDARD` v1.1.

## Quick Start

```bash
# Full build + validate
./infra/benchScale/validate_roundtrip.sh

# Skip build (binary already exists)
SKIP_BUILD=1 ./infra/benchScale/validate_roundtrip.sh

# Custom port
BEARDOG_PORT=9200 ./infra/benchScale/validate_roundtrip.sh
```

## What It Does

Starts a bearDog TCP server on `127.0.0.1:9100`, exercises all
canonical JSON-RPC method domains via HTTP POST, validates responses,
and reports results. Server lifecycle is fully managed
(start → validate → cleanup).

## Validation Phases

| Phase | Methods | What It Validates |
|-------|---------|-------------------|
| 1 | `health.*` | Health triad (liveness, readiness, check) + legacy aliases |
| 2 | `crypto.ed25519_*` | EdDSA keypair generation, sign, verify roundtrip |
| 3 | `crypto.blake3_hash`, `sha256` | Deterministic hashing |
| 4 | `crypto.hmac_sha256` | HMAC keyed authentication |
| 5 | `crypto.aes256_gcm_*` | AEAD symmetric encrypt/decrypt roundtrip |
| 6 | `crypto.chacha20_poly1305_*` | ChaCha20-Poly1305 AEAD roundtrip |
| 7 | `crypto.x25519_*` | Ephemeral X25519 key exchange |
| 8 | `crypto.hkdf_sha256` | Key derivation |
| 9 | `crypto.argon2id_*` | Password hashing + verification |
| 10 | `crypto.sign`, `crypto.verify` | Semantic aliases dispatch to EdDSA |
| 11 | `beardog.crypto.*` | Cross-primal namespace routing |
| 12 | `primal.info`, `rpc.methods` | Introspection + method discovery |
| 13 | error paths | Unknown method (-32601) |
| 14 | `health.*` (burst) | Rapid-fire liveness probe survival |

## Dependencies

- `curl` (HTTP POST transport)
- `jq` (JSON response validation)
- `ss` or `lsof` (port detection fallback)
- `bash` 4+

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `BEARDOG_BINARY` | (auto-detect) | Path to beardog binary. If unset, checks `target/release`, `target/debug`, then PATH. |
| `BEARDOG_PORT` | `9100` | JSON-RPC TCP port |
| `BEARDOG_BIND` | `127.0.0.1` | Bind address |
| `SKIP_BUILD` | unset | Set to `1` to skip `cargo build --release` |

## Relationship to Canonical benchScale

The canonical benchScale substrate lives at `infra/benchScale/` and
provides multi-node Docker/libvirt labs. This local harness validates a
**single bearDog instance** for pre-deploy gating — the same checks
that `benchscale validate ipc 127.0.0.1:<port>` runs, plus full crypto
method coverage.

For multi-primal composition testing, use primalSpring experiments
against a benchScale Docker lab with the `ecoprimals-tower-2node`
topology.
