<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Environment Variables Reference

**Version**: 0.9.0
**Date**: Jun 2, 2026
**Status**: Common Environment Variables

---

## Overview

BearDog follows **primal sovereignty** principles where all configuration is environment-driven:
- Zero hardcoded IPs, ports, or service names
- Runtime discovery of other primals
- Self-knowledge only (no hardcoded primal names)
- All identity vars are **optional** — standalone mode works without any env vars (per UniBin v1.1 / PRIMAL IPC Protocol v3.1)

---

## Quick Start

### Standalone Mode (no env vars needed)

```bash
./beardog server
```

BearDog starts with ephemeral identity (`standalone-{uuid}`), auto-detected socket path, and software HSM.

### Orchestrated Mode

```bash
export FAMILY_ID=nat0
export NODE_ID=tower1
export FAMILY_SEED=my-secret-seed
./beardog server
```

BearDog starts with family isolation, BTSP enforcement, and deterministic socket path.

---

## Common Variables

### Identity Variables

All identity variables are **optional**. When unset, BearDog runs in standalone mode with ephemeral identity. `BEARDOG_*` prefixed forms take precedence over unprefixed forms.

**Precedence rule:** You only need ONE of `NODE_ID` or `BEARDOG_NODE_ID` (not both). Similarly for `FAMILY_ID` / `BEARDOG_FAMILY_ID`. The `BEARDOG_*` form wins when both are set.

#### `FAMILY_ID` / `BEARDOG_FAMILY_ID`

**Purpose**: Genetic family identifier defining trust boundaries
**Required**: No
**Default**: `"standalone"`
**Precedence**: `BEARDOG_FAMILY_ID` > `FAMILY_ID`
**Used For**:
- Trust evaluation (same family = trusted)
- Socket path construction (family-scoped: `beardog-{family}.sock`)
- Key derivation scope
- BTSP security mode activation (non-default family → production mode)

```bash
export FAMILY_ID=nat0
# or
export BEARDOG_FAMILY_ID=nat0
```

Empty strings are treated as unset.

---

#### `NODE_ID` / `BEARDOG_NODE_ID`

**Purpose**: Unique node identifier within the family
**Required**: No
**Default**: Stable per-process ephemeral `standalone-{uuid}` (generated once, reused for process lifetime)
**Precedence**: `BEARDOG_NODE_ID` > `NODE_ID` > `HOSTNAME` > ephemeral
**Used For**:
- Node identification in IPC routing and contact exchange
- Socket path construction (Tier 5 fallback)
- BTSP session metadata
- Metrics and logging

```bash
export NODE_ID=tower1
# or
export BEARDOG_NODE_ID=tower1
```

Empty strings are treated as unset. When absent, a `tracing::warn` is emitted once and an ephemeral `standalone-{uuid}` is used for the entire process lifetime.

---

#### `FAMILY_SEED` / `BEARDOG_FAMILY_SEED`

**Purpose**: BTSP key material for production mode
**Required**: Only when `FAMILY_ID` is set to a non-default value
**Default**: `.family.seed` file in working directory
**Precedence**: `FAMILY_SEED` > `BEARDOG_FAMILY_SEED` > `.family.seed` file

```bash
export FAMILY_SEED=my-secret-seed
# or
export BEARDOG_FAMILY_SEED=my-secret-seed
```

If `FAMILY_ID` is set but no seed is available, the server refuses to start.

---

#### `PRIMAL_NAME`

**Purpose**: Primal identity name (used in socket paths, logs)
**Required**: No
**Default**: `"beardog"` (compile-time `CARGO_PKG_NAME`)

```bash
export PRIMAL_NAME=beardog
```

---

### BTSP Security Mode

The combination of identity variables determines the security mode:

| `FAMILY_ID` | `FAMILY_SEED` | `BIOMEOS_INSECURE` | Mode |
|-------------|---------------|---------------------|------|
| unset / `"default"` | — | — | **Development** — No BTSP; cleartext JSON-RPC |
| set (e.g. `"alpha"`) | **available** | unset / `false` | **Production** — BTSP handshake enforced |
| set | **available** | `true` / `1` | **FATAL** — Refuses to start (conflicting config) |
| set | **missing** | — | **Startup error** — Set seed or remove `FAMILY_ID` |

---

### HSM Configuration

#### `BEARDOG_HSM_MODE`

**Purpose**: Hardware Security Module mode
**Required**: No
**Default**: `software`
**Options**: `software` | `yubikey` | `tpm` | `android_strongbox` | `ios_secure_enclave`

```bash
export BEARDOG_HSM_MODE=software    # Development (no hardware)
export BEARDOG_HSM_MODE=yubikey     # Production (YubiKey)
export BEARDOG_HSM_MODE=tpm         # Production (TPM 2.0)
```

---

#### `BEARDOG_YUBIKEY_SERIAL`

**Purpose**: Specific YubiKey serial number
**Required**: No
**Default**: Auto-detect first available

```bash
export BEARDOG_YUBIKEY_SERIAL=12345678
```

---

#### `BEARDOG_TPM_DEVICE`

**Purpose**: TPM device path
**Required**: No
**Default**: `/dev/tpm0` or `/dev/tpmrm0`

```bash
export BEARDOG_TPM_DEVICE=/dev/tpmrm0
```

---

### Unix Socket IPC (PRIMARY Transport)

#### `BEARDOG_SOCKET`

**Purpose**: Explicit Unix socket path override (Tier 1 — highest priority)
**Required**: No
**Default**: See 5-tier fallback below

```bash
export BEARDOG_SOCKET=/run/user/1000/beardog-nat0.sock
```

---

#### Socket Path Resolution (5-Tier Fallback)

BearDog uses a 5-tier fallback system for socket paths, compatible with orchestration:

| Tier | Source | Example |
|------|--------|---------|
| **1** | `BEARDOG_SOCKET` env var | `/custom/path/beardog.sock` |
| **2** | `BIOMEOS_SOCKET_PATH` or `BIOMEOS_SOCKET_DIR` | `/tmp/beardog-default-default.sock` |
| **3** | Primal IPC Protocol namespace (`/primal/{name}`) | `/primal/beardog` |
| **4** | XDG runtime dir (`/run/user/<uid>/biomeos/`) | `/run/user/1000/biomeos/beardog-nat0.sock` |
| **5** | Platform temp dir (last resort) | `/tmp/beardog-nat0-tower1.sock` |

In production mode (`FAMILY_ID` set, non-default), Tiers 2–4 produce family-scoped filenames (`beardog-{family}.sock`).

---

### Network Configuration

#### `BEARDOG_BIND_ADDR`

**Purpose**: HTTP/TCP bind address (optional — Unix socket is primary)
**Required**: No
**Default**: `0.0.0.0:0` (OS-assigned random port)
**Special**: Set to empty string to disable HTTP entirely

```bash
export BEARDOG_BIND_ADDR=0.0.0.0:0       # Random port (recommended)
export BEARDOG_BIND_ADDR=127.0.0.1:9000   # Specific port (development)
export BEARDOG_BIND_ADDR=                  # Disable HTTP entirely (production)
```

---

#### `BEARDOG_ENDPOINT`

**Purpose**: Full endpoint URL for service registration
**Required**: No
**Default**: Constructed from `BEARDOG_HOST` and port

```bash
export BEARDOG_ENDPOINT=http://server.example.com:9000
```

---

#### `BEARDOG_HOST`

**Purpose**: Hostname for endpoint construction
**Required**: No
**Default**: `localhost`

```bash
export BEARDOG_HOST=server.example.com
```

---

#### `BEARDOG_API_PORT`

**Purpose**: API port for endpoint construction
**Required**: No
**Default**: `9000`

```bash
export BEARDOG_API_PORT=8443
```

---

#### `BEARDOG_TCP_IPC_PORT`

**Purpose**: TCP IPC fallback port when Unix domain sockets are unavailable (Android, Windows containers, cross-gate deployments)
**Required**: No
**Default**: `9100` (aligned with ecosystem convention — plasmidBin/primalSpring/ironGate all probe BearDog at TCP 9100)

```bash
export BEARDOG_TCP_IPC_PORT=9100
```

---

#### `BEARDOG_METRICS_PORT`

**Purpose**: Prometheus metrics / monitoring HTTP endpoint
**Required**: No
**Default**: `9190` (moved from 9100 to avoid collision with TCP IPC ecosystem convention)

```bash
export BEARDOG_METRICS_PORT=9190
```

---

### Orchestrator Integration

#### `BIOMEOS_SOCKET_PATH`

**Purpose**: Socket path set by orchestrator (Tier 2)
**Required**: No

```bash
export BIOMEOS_SOCKET_PATH=/tmp/beardog-default-default.sock
```

---

#### `BIOMEOS_SOCKET_DIR`

**Purpose**: Socket directory set by orchestrator (Tier 2, joined with filename)
**Required**: No

```bash
export BIOMEOS_SOCKET_DIR=/var/run/biomeos/sockets
```

---

#### `BIOMEOS_INSECURE`

**Purpose**: Skip BTSP authentication (development only)
**Required**: No
**Default**: `false`
**FATAL**: Cannot be set when `FAMILY_ID` is also set (conflicting config)

```bash
export BIOMEOS_INSECURE=1    # Development only — NEVER in production
```

---

### Registry & Discovery

#### `PRIMAL_REGISTRY_SOCKET`

**Purpose**: Primal registry Unix socket path
**Required**: No
**Default**: `/tmp/primal-registry-${FAMILY_ID}.sock`

```bash
export PRIMAL_REGISTRY_SOCKET=/var/run/primal-registry.sock
```

---

#### `BEARDOG_UPA_URL`

**Purpose**: Universal Primal Adapter (UPA) service URL
**Required**: No
**Default**: `https://localhost:8080`

```bash
export BEARDOG_UPA_URL=https://upa.example.com:8080
```

---

### Logging & Monitoring

#### `RUST_LOG`

**Purpose**: Rust logging level
**Required**: No
**Default**: `info`
**Options**: `error` | `warn` | `info` | `debug` | `trace`

```bash
export RUST_LOG=info                                    # Production
export RUST_LOG=debug                                   # Development
export RUST_LOG=beardog=debug,beardog_tunnel=trace      # Module-specific
```

---

#### `BEARDOG_LOG_FORMAT`

**Purpose**: Log output format
**Required**: No
**Default**: `pretty` (human-readable)
**Options**: `pretty` | `json`

```bash
export BEARDOG_LOG_FORMAT=json     # Production (structured)
export BEARDOG_LOG_FORMAT=pretty   # Development (human-readable)
```

---

#### `BEARDOG_LOG_FILE`

**Purpose**: Log file path
**Required**: No
**Default**: stdout

```bash
export BEARDOG_LOG_FILE=/var/log/beardog/beardog.log
```

---

### Security & Trust

#### `BEARDOG_AUTH_MODE`

**Purpose**: MethodGate enforcement mode for JSON-RPC protected methods.
When `enforced`, unauthenticated calls to protected methods are rejected
with `-32001 PERMISSION_DENIED`. When `permissive` (default), violations
are logged but allowed (backward-compatible).

**Required**: No
**Default**: `permissive`
**Options**: `permissive` | `enforced`

```bash
export BEARDOG_AUTH_MODE=enforced   # S4 shadow / production
export BEARDOG_AUTH_MODE=permissive # Development (default)
```

**S4 shadow validation**: ironGate requires `enforced` mode during the
formal 7-day shadow gate so auth parity can be measured.

---

#### `BEARDOG_TRUST_MODE`

**Purpose**: Trust evaluation strictness
**Required**: No
**Default**: `strict`
**Options**: `strict` | `lenient` | `permissive`

```bash
export BEARDOG_TRUST_MODE=strict    # Production
export BEARDOG_TRUST_MODE=lenient   # Testing
```

---

#### `BEARDOG_MTLS_ENABLED`

**Purpose**: Enable mutual TLS for HTTP API
**Required**: No
**Default**: `false`

```bash
export BEARDOG_MTLS_ENABLED=true
export BEARDOG_TLS_CERT_PATH=/etc/beardog/certs/server.crt
export BEARDOG_TLS_KEY_PATH=/etc/beardog/certs/server.key
```

---

### Performance Tuning

#### `BEARDOG_WORKER_THREADS`

**Purpose**: Tokio worker thread count
**Required**: No
**Default**: CPU count

```bash
export BEARDOG_WORKER_THREADS=8
```

---

#### `BEARDOG_CONNECTION_POOL_SIZE`

**Purpose**: Connection pool size
**Required**: No
**Default**: `100`

```bash
export BEARDOG_CONNECTION_POOL_SIZE=200
```

---

#### `BEARDOG_REQUEST_TIMEOUT_SECS`

**Purpose**: Request timeout in seconds
**Required**: No
**Default**: `30`

```bash
export BEARDOG_REQUEST_TIMEOUT_SECS=60
```

---

#### Wave 120 Timeout Variables

Wave 120 introduced granular I/O timeouts for connection lifecycle control:

#### `BEARDOG_READ_TIMEOUT_SECS`

**Purpose**: Read operation timeout in seconds
**Required**: No

```bash
export BEARDOG_READ_TIMEOUT_SECS=30
```

#### `BEARDOG_HANDSHAKE_TIMEOUT_SECS`

**Purpose**: Network handshake timeout in seconds
**Required**: No

```bash
export BEARDOG_HANDSHAKE_TIMEOUT_SECS=10
```

---

### Development & Testing

#### `BEARDOG_DEV_MODE`

**Purpose**: Enable development mode
**Required**: No
**Default**: `false`
**Effect**: Relaxed security, verbose logging

```bash
export BEARDOG_DEV_MODE=true
```

---

#### `BEARDOG_API_DOCS_ENABLED`

**Purpose**: Enable API documentation endpoint
**Required**: No
**Default**: `false`

```bash
export BEARDOG_API_DOCS_ENABLED=true
```

---

#### `BEARDOG_METRICS_ENABLED`

**Purpose**: Enable metrics endpoint
**Required**: No
**Default**: `true`

```bash
export BEARDOG_METRICS_ENABLED=false
```

---

### ACME / Automatic TLS

#### `BEARDOG_TLS_MODE`

**Purpose**: TLS provisioning mode for server startup
**Required**: No
**Default**: `manual`
**Options**: `manual` | `acme`

```bash
export BEARDOG_TLS_MODE=acme
```

#### `BEARDOG_ACME_DIRECTORY`

**Purpose**: ACME directory URL (e.g. Let's Encrypt)
**Required**: When `BEARDOG_TLS_MODE=acme`

```bash
export BEARDOG_ACME_DIRECTORY=https://acme-v02.api.letsencrypt.org/directory
```

#### `BEARDOG_ACME_EMAIL`

**Purpose**: ACME account contact email
**Required**: When `BEARDOG_TLS_MODE=acme`

```bash
export BEARDOG_ACME_EMAIL=admin@example.com
```

#### `BEARDOG_ACME_DOMAINS`

**Purpose**: Comma-separated domain names for certificate issuance
**Required**: When `BEARDOG_TLS_MODE=acme`

```bash
export BEARDOG_ACME_DOMAINS=beardog.example.com,api.beardog.example.com
```

---

## Example Configurations

### Standalone (no env vars)

```bash
./beardog server
# Socket: auto-detected (Tier 3–5)
# Identity: standalone-{uuid}
# HSM: software
# BTSP: disabled (development mode)
```

### Development (single instance)

```bash
export FAMILY_ID=dev
export NODE_ID=dev-1
export BEARDOG_HSM_MODE=software
export BEARDOG_BIND_ADDR=127.0.0.1:9000
export RUST_LOG=debug
export BEARDOG_DEV_MODE=true

./beardog server
```

### Production (primary tower)

```bash
export BEARDOG_FAMILY_ID=prod
export BEARDOG_NODE_ID=tower1
export FAMILY_SEED=my-production-seed
export BEARDOG_HSM_MODE=yubikey
export BEARDOG_BIND_ADDR=
export BEARDOG_TRUST_MODE=strict
export BEARDOG_MTLS_ENABLED=true
export BEARDOG_TLS_CERT_PATH=/etc/beardog/certs/server.crt
export BEARDOG_TLS_KEY_PATH=/etc/beardog/certs/server.key
export RUST_LOG=info
export BEARDOG_LOG_FORMAT=json
export BEARDOG_LOG_FILE=/var/log/beardog/tower1.log

./beardog server
```

### S4 Shadow Validation (ironGate auth gate)

Minimum configuration for bearDog to serve as the S4 auth provider so
ironGate can run the formal 7-day shadow validation.

```bash
# ── Production BTSP on the wire ──
export FAMILY_ID=<your-family>
export FAMILY_SEED=<32+-byte-seed>

# ── Explicit socket for cross-primal discovery ──
export BEARDOG_SOCKET=/run/beardog/beardog.sock

# ── Optional: TCP for cross-host (ironGate on different machine) ──
export BEARDOG_TCP_IPC_PORT=9100

# ── Enforce token auth during shadow period ──
export BEARDOG_AUTH_MODE=enforced

./beardog server
```

ironGate validates by calling:
- `auth.verify_ionic` — Ed25519 token verification
- `auth.public_key` — cache verifying key for offline validation
- `auth.issue_session` — issue tokens with `purpose: "jupyterhub"`
- `auth.peer_info` — peer credential introspection (`SO_PEERCRED`)

### Dual Towers on Same Machine

**Terminal 1**:
```bash
export FAMILY_ID=nat0
export NODE_ID=tower1
./beardog server
# Socket: beardog-nat0.sock (Tier 3/4)
```

**Terminal 2**:
```bash
export FAMILY_ID=nat0
export NODE_ID=tower2
./beardog server
# Socket: beardog-nat0.sock (same family, shared)
```

---

## Summary

| Category | Variables | Required |
|----------|-----------|----------|
| **Identity** | `FAMILY_ID`, `BEARDOG_FAMILY_ID`, `NODE_ID`, `BEARDOG_NODE_ID`, `FAMILY_SEED`, `BEARDOG_FAMILY_SEED`, `PRIMAL_NAME` | None (standalone mode) |
| **HSM** | `BEARDOG_HSM_MODE`, `BEARDOG_YUBIKEY_SERIAL`, `BEARDOG_TPM_DEVICE` | None |
| **Socket** | `BEARDOG_SOCKET`, `BIOMEOS_SOCKET_PATH`, `BIOMEOS_SOCKET_DIR` | None |
| **Network** | `BEARDOG_BIND_ADDR`, `BEARDOG_ENDPOINT`, `BEARDOG_HOST`, `BEARDOG_API_PORT`, `BEARDOG_TCP_IPC_PORT`, `BEARDOG_METRICS_PORT` | None |
| **Security** | `BIOMEOS_INSECURE`, `BEARDOG_AUTH_MODE`, `BEARDOG_TRUST_MODE`, `BEARDOG_MTLS_ENABLED`, `BEARDOG_TLS_CERT_PATH`, `BEARDOG_TLS_KEY_PATH` | None |
| **Registry** | `PRIMAL_REGISTRY_SOCKET`, `BEARDOG_UPA_URL` | None |
| **Logging** | `RUST_LOG`, `BEARDOG_LOG_FORMAT`, `BEARDOG_LOG_FILE` | None |
| **Performance** | `BEARDOG_WORKER_THREADS`, `BEARDOG_CONNECTION_POOL_SIZE`, `BEARDOG_REQUEST_TIMEOUT_SECS`, `BEARDOG_READ_TIMEOUT_SECS`, `BEARDOG_HANDSHAKE_TIMEOUT_SECS` | None |
| **ACME** | `BEARDOG_TLS_MODE`, `BEARDOG_ACME_DIRECTORY`, `BEARDOG_ACME_EMAIL`, `BEARDOG_ACME_DOMAINS` | None |
| **Development** | `BEARDOG_DEV_MODE`, `BEARDOG_API_DOCS_ENABLED`, `BEARDOG_METRICS_ENABLED` | None |

Selected subset — see `crates/beardog-config/src/env_keys.rs` for the full centralized registry (~280+ constants). All variables documented here are optional; standalone mode works with zero env vars.

---

## Discovery Escalation Hierarchy

primalSpring (and springs in general) discover composition members in this order:

| Tier | Method | Requires | Notes |
|------|--------|----------|-------|
| 1 | **Songbird `ipc.resolve`** | Songbird socket reachable | Highest-fidelity routing with cross-gate capability |
| 2 | **biomeOS Neural API** (`capability.discover`) | Neural API running | Capability→socket resolution without filesystem convention |
| 3 | **UDS filesystem convention** | `{primal}-{family}.sock` present | No external service needed |
| 4 | **Socket registry / manifests** | Registry file on disk | Static fallback |
| 5 | **TCP probing** (well-known ports from tolerances) | `BEARDOG_TCP_IPC_PORT` or default 9100 | Last resort; for containers and cross-arch |

Every tier is valid. BearDog is not required to support all tiers. Tier 3 (UDS convention) works out of the box. For TCP fallback (Tier 5), set `BEARDOG_TCP_IPC_PORT=9100` to match ecosystem convention.

---

## Related Documents

- [README.md](../../README.md) — Project overview and identity env var documentation
- [`.env.example`](../../.env.example) — Example environment file
- [STATUS.md](../../STATUS.md) — Current status and metrics
- [ARCHITECTURE.md](../../ARCHITECTURE.md) — System architecture

---

_Last Updated: Jun 2, 2026_
_Version: 0.9.0_
_Status: Production Ready_
