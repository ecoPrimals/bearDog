<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# BearDog — Start Here

BearDog is the cryptographic service provider for the ecoPrimals ecosystem. It provides secure crypto operations for all primals through the Tower Atomic Pattern via JSON-RPC 2.0 over NDJSON (newline-delimited JSON).

**Status**: Production Ready | **Pure Rust**: 100% | **Edition**: 2024

---

## Quick Start (5 Minutes)

### 1. Prerequisites

```bash
# Rust 1.93+ (edition 2024, pinned via rust-toolchain.toml)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Build & Test

```bash
cargo build --release
cargo test --workspace
```

### 3. Run BearDog

**Unix Socket (Linux/macOS — default)**:

```bash
cargo run --release --bin beardog -- server
```

**With family isolation (multi-family architecture)**:

```bash
./beardog server --family-id alpha    # Creates beardog-alpha.sock
./beardog server --family-id bravo    # Creates beardog-bravo.sock
```

**TCP (Android/Windows/cross-device)**:

```bash
./beardog server --listen 127.0.0.1:9100
```

**Android (abstract sockets)**:

```bash
BEARDOG_SOCKET=@biomeos_beardog ./beardog server
```

### 4. Test the API

```bash
cargo run --release --example api_demo
```

---

## Architecture

### Tower Atomic Pattern

BearDog provides **crypto atoms** via JSON-RPC. Other ecosystem primals delegate all crypto to BearDog rather than implementing their own:

```
┌─────────────┐                    ┌─────────────┐
│  Any Primal │ ←─ JSON-RPC ────→ │  BearDog    │
│ (Protocol)  │    Unix Socket     │  (Crypto)   │
└─────────────┘                    └─────────────┘
     Zero crypto code                 All crypto operations
```

**Benefits**:
- Primals remain 100% Pure Rust with no crypto dependencies
- Single auditable crypto codebase
- HSM abstraction (software, hardware, mobile)

### Multi-Family Isolation

Each family gets its own BearDog instance with independently derived key material. Family A's keys are never shared with Family B:

```bash
./beardog server --family-id alpha   # Own socket, own keys
./beardog server --family-id bravo   # Fully isolated
```

### JSON-RPC Methods

BearDog exposes 127 methods organized by domain:

| Namespace | Examples |
|-----------|----------|
| `crypto.*` | `crypto.sign_ed25519`, `crypto.verify_ed25519`, `crypto.chacha20_poly1305_encrypt`, `crypto.blake3_hash` |
| `tls.*` | `tls.derive_handshake_secrets`, `tls.sign_handshake` |
| `tor.*` | `tor_ntor_client_init`, `tor_cell_encrypt`, `derive_onion_address` |
| `genetic.*` | `genetic.derive_lineage_key`, `genetic.mix_entropy` |
| `secrets.*` | `secrets.store`, `secrets.retrieve`, `secrets.list`, `secrets.delete` |
| `btsp.*` | `btsp.configure_tls`, `btsp.verify_peer` |
| `quantum.*` | `quantum.generate_kem_keypair`, `quantum.sign`, `quantum.verify` |

Introspection: `discover_capabilities`, `primal.info`, `rpc.methods`

---

## Development

### Standards

- **Edition 2024** — Rust 2024 with MSRV 1.93.0 (`rust-toolchain.toml` pinned)
- **Pure Rust** — No C dependencies
- **Dependency Injection** — Pure `Default` (no I/O), `from_env()` at boundaries, `from_env_provider()` in tests
- **Zero Hardcoding** — Config flows through parameters, capability-based discovery
- **Self-Knowledge Only** — Primals discover peers at runtime, never hardcode other primal names
- **Result<T, E>** — Zero `unwrap()` in production; `expect()` only on documented invariants
- **Concurrent Tests** — 35 `#[serial]` isolated to `beardog-production`; all others concurrent, zero sleeps in non-chaos tests
- **< 800 LOC** — File size discipline (production code)
- **Constant-Time** — Use `subtle` crate for secret comparisons

### Workflow

```bash
cargo fmt --all                      # Format
cargo clippy --workspace             # Lint (0 warnings expected)
cargo test --workspace               # Test
cargo build --release                # Build
```

### Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `PRIMAL_NAME` | Primal identity | `beardog` |
| `FAMILY_ID` | Family identifier | (none) |
| `NODE_ID` | Node identifier | (random) |
| `BEARDOG_SOCKET` | Socket path override | auto-detected |
| `BEARDOG_PORT` | Listening port | OS-assigned |
| `BEARDOG_LISTEN_ADDR` | Full listen address | (none) |

---

## Quality Metrics

| Metric | Value |
|--------|-------|
| Clippy | 0 warnings (pedantic + nursery + cast + unwrap/expect warn) |
| Missing Docs | 0 |
| Unsafe | `forbid(unsafe_code)` workspace-wide |
| Pure Rust | 100% |
| Tests | 14,980+ (concurrent; 35 `#[serial]` in `beardog-production`) |
| Coverage | 90.51% line (llvm-cov) |
| `#[serial]` | 35 (`beardog-production` shared `AtomicBool`) |
| Files > 800 LOC | 0 (production) |

---

## Documentation

| Document | Description |
|----------|-------------|
| [README.md](README.md) | Project overview |
| [STATUS.md](STATUS.md) | Current status and metrics |
| [ROADMAP.md](ROADMAP.md) | Current priorities |
| [ARCHITECTURE.md](ARCHITECTURE.md) | System architecture |
| [SECURITY.md](SECURITY.md) | Security model |

---

**Last Updated**: May 27, 2026
