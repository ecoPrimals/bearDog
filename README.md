<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# BearDog

[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)](STATUS.md)
[![Clippy](https://img.shields.io/badge/clippy-0_warnings-brightgreen.svg)](STATUS.md)
[![Pure Rust](https://img.shields.io/badge/rust-100%25_pure-orange.svg)](STATUS.md)
[![Edition](https://img.shields.io/badge/edition-2024-blue.svg)](Cargo.toml)
[![License](https://img.shields.io/badge/license-AGPL--3.0--or--later-blue.svg)](LICENSE)

**BearDog** is the cryptographic service provider for the ecoPrimals ecosystem — a **100% Pure Rust** security platform with zero C dependencies.

**Status**: Production Ready | **Edition**: 2024 | **MSRV**: 1.93.0 | **Crates**: 25 | **JSON-RPC Methods**: 229 | **Tests**: 13,911+ | **Coverage**: 90.51% | **Last Updated**: July 22, 2026

---

## Overview

BearDog provides secure cryptographic operations for all primals through the **Tower Atomic Pattern**. Each primal delegates crypto to BearDog via **JSON-RPC 2.0 over NDJSON** (newline-delimited JSON), keeping a single auditable crypto codebase. Every request and response is a single JSON object terminated by `\n`.

```
┌─────────────┐                    ┌─────────────┐
│  Any Primal │ ←─ JSON-RPC ────→ │  BearDog    │
│ (Protocol)  │  NDJSON framing    │  (Crypto)   │
└─────────────┘                    └─────────────┘
     Zero crypto code                 All crypto operations
```

### Key Features

- **100% Pure Rust** — Zero C dependencies (RustCrypto suite, postcard, mdns-sd for LAN discovery)
- **Rust 2024 Edition** — Modern idioms, MSRV 1.93.0
- **Fully Concurrent** — Dependency injection architecture, no global mutable state
- **229 JSON-RPC Methods** — Complete crypto API (BTSP handshake-as-a-service, ionic bond lifecycle, contract signing, lineage queries, consent gate, FIDO2/CTAP2 hardware authentication, cross-gate trust exchange)
- **Tor v3 Support** — Onion address derivation + ntor handshake + cell crypto
- **Multi-Family Support** — `--family-id` flag for per-family instances
- **Secret Storage** — Encrypted secrets with family-scoped keys + `CredentialStore` trait (in-memory, file-vault backends)
- **Relay Authorization** — Lineage-gated access for relay-assisted coordinated punch
- **Universal IPC** — Multi-transport, platform-agnostic
- **HSM Integration** — Software, Windows DPAPI, Linux SecretService, Android StrongBox backends
- **Dark Forest Beacon** — Zero metadata leakage discovery

---

## Quick Start

### Prerequisites

```bash
# Rust 1.93+ (edition 2024, pinned via rust-toolchain.toml)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build & Test

```bash
git clone git@github.com:ecoPrimals/bearDog.git
cd bearDog
cargo build --release
cargo test --workspace
```

### Run

```bash
# Default — auto-detects platform transport
cargo run --release --bin beardog -- server

# With family isolation (multi-family architecture)
./beardog server --family-id alpha

# Custom socket path
./beardog server --socket /custom/path.sock

# TCP transport (Android, Windows, cross-device) — opt-in via --port/--listen/BEARDOG_TCP_IPC_PORT
./beardog server --listen 0.0.0.0:9100
```

TCP is opt-in via `--port`/`--listen`/`BEARDOG_TCP_IPC_PORT`. Without those, BearDog runs UDS-only.

---

## Platform Support

| Platform | Transport | Status |
|----------|-----------|--------|
| Linux | Unix sockets | Production |
| macOS | Unix sockets | Production |
| Android | Abstract sockets + TCP | Production |
| Windows | Named pipes + TCP | Ready |
| iOS | TCP | Ready |

BearDog auto-detects the platform and binds appropriate transports.

**Wire format**: All transports use **NDJSON** (newline-delimited JSON-RPC 2.0). Each request must be a single line terminated by `\n`. Responses are likewise newline-terminated. Idle connections without a newline are timed out after 30 seconds.

---

## Cryptographic Capabilities

### Algorithms

| Category | Algorithms |
|----------|-----------|
| **Signatures** | Ed25519, ECDSA (P-256/P-384), RSA |
| **Key Exchange** | X25519, ECDHE (P-256/P-384) |
| **AEAD** | ChaCha20-Poly1305, AES-128/256-GCM |
| **Hashing** | BLAKE3, SHA-256/384/512, SHA3-256 |
| **MAC** | HMAC-SHA256/384/512, HMAC-BLAKE3 |
| **KDF** | HKDF, TLS 1.2/1.3 PRF, PBKDF2, Argon2id |
| **Passwords** | Argon2id, bcrypt, scrypt |
| **Tor** | Onion address, ntor handshake, cell crypto |
| **Secrets** | Encrypted storage with family-scoped keys |

### JSON-RPC Method Categories

```
crypto.*       - Hash, sign, verify, encrypt, decrypt, key exchange
tls.*          - TLS 1.2/1.3 key derivation and handshake
tor.*          - Onion identity, ntor, cell crypto
genetic.*      - Lineage keys, beacon, challenge-response
secrets.*      - Store, retrieve, list, delete encrypted secrets (CredentialStore backends)
relay.*        - Lineage-gated relay authorization (coordinated punch)
beacon.*       - Dark Forest beacon generation, encryption, meeting exchange
btsp.*         - Secure tunnel configuration (Phase 1–3: handshake, key exchange, encrypted framing with ChaCha20-Poly1305)
```

### Introspection

```
discover_capabilities  - List all capabilities
primal.info            - Primal identity and metadata
rpc.methods            - List all available methods
```

---

## Architecture

### Tower Atomic Pattern

All primals delegate cryptographic operations to BearDog via JSON-RPC:

- **Separation of Concerns** — Protocol logic stays in primals, crypto stays in BearDog
- **Single Audit Surface** — One codebase to review for crypto correctness
- **HSM Abstraction** — Software, hardware (PKCS#11), or mobile (StrongBox) backends
- **Family Isolation** — Each family gets its own BearDog instance with independently derived keys

### Multi-Family Architecture

```bash
# Family A gets its own socket and key material
./beardog server --family-id alpha   # beardog-alpha.sock

# Family B is fully isolated
./beardog server --family-id bravo   # beardog-bravo.sock
```

Key material is derived from the family seed. A BearDog instance serving family A never shares keys with family B.

### BTSP Security Modes

BearDog enforces **BTSP (BearDog Transport Security Protocol)** based on environment configuration:

| `FAMILY_ID` | `FAMILY_SEED` | Mode | Behavior |
|-------------|---------------|------|----------|
| unset / `"default"` | — | **Development** | No BTSP; cleartext JSON-RPC accepted on all transports |
| set (e.g. `"alpha"`) | **required** | **Production** | BTSP handshake enforced; first-byte `{` bypasses for local composition |
| set | **missing** | **Startup error** | Server refuses to start — set seed or remove `FAMILY_ID` |

**Family seed resolution** (checked in order):

1. `FAMILY_SEED` env var (raw bytes)
2. `BEARDOG_FAMILY_SEED` env var (raw bytes)
3. `.family.seed` file in the working directory

```bash
# Development mode (no BTSP)
./beardog server

# Production mode (BTSP enforced, cleartext bypass for JSON-RPC)
FAMILY_ID=alpha FAMILY_SEED=my-secret-seed ./beardog server
```

### Cleartext JSON-RPC (BTSP bypass)

In production mode, BearDog auto-detects cleartext JSON-RPC on both UDS and TCP: if the **first byte** of a connection is `{` (0x7B), the connection is treated as cleartext NDJSON without a BTSP handshake. This allows other primals and springs to call methods like `crypto.hash`, `health.liveness`, and `capabilities.list` without implementing BTSP.

```bash
# Works in both development and production modes
echo '{"jsonrpc":"2.0","method":"crypto.hash","params":{"data":"aGVsbG8="},"id":1}' | nc -U beardog.sock
```

The `capabilities.list` response includes a `transport_security` block with `cleartext_available: true/false` so clients can discover BTSP requirements programmatically.

### Identity Environment Variables

BearDog resolves identity from environment variables for orchestrated deployments. All are optional — standalone mode works without any env vars.

| Variable | Fallback | Purpose |
|----------|----------|---------|
| `FAMILY_ID` / `BEARDOG_FAMILY_ID` | `"standalone"` | Family isolation — determines socket naming (`beardog-{family}.sock`) and key derivation scope |
| `FAMILY_SEED` / `BEARDOG_FAMILY_SEED` | `.family.seed` file | BTSP key material — required when `FAMILY_ID` is set |
| `NODE_ID` / `BEARDOG_NODE_ID` | `standalone-{uuid}` | Node identity within a family — used in IPC routing, contact exchange, and BTSP session metadata |

**Resolution behavior:**
- `BEARDOG_*` prefixed vars take precedence, then unprefixed (`NODE_ID`, `FAMILY_ID`)
- Empty strings are treated as unset
- When `NODE_ID` is absent, a stable per-process ephemeral id (`standalone-{uuid}`) is generated once and reused for the process lifetime
- Standalone mode is fully operational for local-only usage — primals MUST NOT hard-fail on missing identity vars (per UniBin v1.1 / PRIMAL IPC Protocol v3.1)

```bash
# Orchestrated deployment (recommended for multi-primal hosts)
export FAMILY_ID=nat0
export NODE_ID=tower1
export FAMILY_SEED=my-secret-seed
./beardog server

# Standalone mode (no env vars needed)
./beardog server --port 9000
```

**Important:** You only need ONE of `NODE_ID` or `BEARDOG_NODE_ID` (not both). Similarly for `FAMILY_ID` / `BEARDOG_FAMILY_ID`. The `BEARDOG_*` prefixed form takes precedence when both are set.

**For launcher scripts:** auto-set `NODE_ID` to `$(hostname)` if not already set. See `start_primal.sh` in primalSpring for reference.

---

## Quality

| Metric | Value |
|--------|-------|
| **Build** | Clean, 0 errors |
| **Clippy** | 0 warnings (pedantic + nursery + all cast lints warn + doc_markdown warn + missing_errors_doc warn + unwrap/expect warn) |
| **Missing Docs** | 0 warnings (including `# Errors` sections on all `Result` functions) |
| **Pure Rust** | 100% — zero C dependencies |
| **Unsafe Code** | 0 production blocks (`forbid(unsafe_code)` workspace-wide) |
| **Format** | `cargo fmt` clean |
| **TODO/FIXME** | 0 |
| **Files > 800 LOC** | 0 (production code) |
| **Rust files** | 1,952 |
| **Tests** | 13,911+ (concurrent; 35 `#[serial]` in `beardog-production`) |
| **Coverage** | 90.51% line (llvm-cov workspace, target 90% met) |
| **Serial Tests** | 35 (`beardog-production` shared `AtomicBool` state) |
| **cargo deny** | All 4 checks pass (advisories, bans, licenses, sources) |
| **License** | AGPL-3.0-or-later (SPDX headers on all .rs files) |

### Standards

- **Edition 2024** — Modern Rust with latest language features (MSRV 1.93.0)
- **Pure Rust** — No C dependencies anywhere (ecoBin compliant)
- **Dependency Injection** — Config flows through parameters, `Default` is pure (no I/O), `from_env()` at boundaries only
- **Zero Hardcoding** — Environment variables and capability-based discovery
- **Result<T, E>** — Zero `.unwrap()` in production; `#[expect(clippy::expect_used, reason = "...")]` for justified invariants; `unwrap_used`/`expect_used` warn at workspace level
- **Concurrent Tests** — 35 `#[serial]` isolated to `beardog-production` (shared `AtomicBool`); all others concurrent, zero sleeps in non-chaos tests
- **< 800 LOC** — File size discipline across all production .rs files
- **Workspace Lints** — Centralized clippy pedantic + nursery + all cast lints + `doc_markdown` + `missing_errors_doc` at warn
- **SPDX headers** — Every `.rs` file has `// SPDX-License-Identifier: AGPL-3.0-or-later`
- **`rust-toolchain.toml`** — Pinned toolchain with cross-compile targets

---

## Documentation

| Document | Description |
|----------|-------------|
| [STATUS.md](STATUS.md) | Current status and metrics |
| [START_HERE.md](START_HERE.md) | Quick start and onboarding |
| [ROADMAP.md](ROADMAP.md) | Current priorities and roadmap |
| [ARCHITECTURE.md](ARCHITECTURE.md) | System architecture |
| [SECURITY.md](SECURITY.md) | Security model |
| [CHANGELOG.md](CHANGELOG.md) | Release and wave notes |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Contribution guidelines |
| [CONTEXT.md](CONTEXT.md) | Ecosystem and project context |

---

## License

AGPL-3.0-or-later — See [LICENSE](LICENSE) file.

---

## Part of ecoPrimals

This repo is part of the [ecoPrimals](https://github.com/ecoPrimals) sovereign
computing ecosystem — a collection of pure Rust binaries that coordinate via
JSON-RPC, capability-based routing, and zero compile-time coupling.

See [wateringHole](https://github.com/ecoPrimals/wateringHole) for ecosystem
documentation, standards, and the primal registry.
