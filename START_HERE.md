# BearDog -- Start Here

BearDog is the cryptographic service provider for the ecoPrimals ecosystem. It provides secure crypto operations for all primals through the Tower Atomic Pattern via JSON-RPC over Unix sockets.

**Status**: Production Ready | **Pure Rust**: 100% | **Tests**: 8,789 passing

---

## Quick Start (5 Minutes)

### 1. Prerequisites

```bash
# Rust 1.80+ (uses std::sync::LazyLock)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# System dependencies (Ubuntu/Debian)
sudo apt-get install build-essential pkg-config
```

### 2. Build & Test

```bash
cargo build --all-features --release
cargo test --workspace
```

### 3. Run BearDog

**Unix Socket (Linux/macOS -- default)**:

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
./beardog server --listen 127.0.0.1:9900
```

**Android (abstract sockets)**:

```bash
BEARDOG_SOCKET=@biomeos_beardog ./beardog server
```

### 4. Test the API

```bash
cargo run --release --example crypto_client
```

---

## Architecture

### Tower Atomic Pattern

BearDog provides **crypto atoms** via JSON-RPC. Other primals (Songbird, Squirrel, NestGate, etc.) delegate all crypto to BearDog rather than implementing their own:

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

See: [TOWER_ATOMIC_PATTERN.md](TOWER_ATOMIC_PATTERN.md)

### Multi-Family Isolation

Each family gets its own BearDog instance with independently derived key material. Family A's keys are never shared with Family B:

```bash
./beardog server --family-id alpha   # Own socket, own keys
./beardog server --family-id bravo   # Fully isolated
```

### JSON-RPC Methods

BearDog exposes 91+ methods organized by domain:

| Namespace | Examples |
|-----------|----------|
| `crypto.*` | `crypto.sign_ed25519`, `crypto.chacha20_poly1305_encrypt`, `crypto.blake3_hash` |
| `tls.*` | `tls.derive_handshake_secrets`, `tls.sign_handshake` |
| `tor.*` | `tor_ntor_client_init`, `tor_cell_encrypt`, `derive_onion_address` |
| `genetic.*` | `genetic.derive_lineage_key`, `genetic.mix_entropy` |
| `secrets.*` | `secrets.store`, `secrets.retrieve`, `secrets.list`, `secrets.delete` |
| `btsp.*` | `btsp.configure_tls`, `btsp.verify_peer` |

Introspection: `discover_capabilities`, `primal.info`, `rpc.methods`

---

## Development

### Standards

- **Pure Rust** -- No C dependencies
- **Zero Hardcoding** -- Use environment variables for configuration
- **Result<T, E>** -- No `unwrap()`/`expect()` in production code
- **Serial Env Tests** -- Use `#[serial_test::serial]` for env var tests
- **< 1000 LOC** -- File size discipline (exceptions justified)

### Workflow

```bash
cargo fmt                                              # Format
cargo clippy --all-targets --all-features -- -D warnings  # Lint
cargo test --workspace                                 # Test
cargo build --all-features --release                   # Build
```

### Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `PRIMAL_NAME` | Primal identity | `beardog` |
| `FAMILY_ID` | Family identifier | (none) |
| `NODE_ID` | Node identifier | (random) |
| `BEARDOG_SOCKET` | Socket path override | auto-detected |
| `UPA_PROVIDER` | UPA provider primal | `songbird` |
| `IPC_SOCKET` | IPC socket override | auto-detected |

---

## Documentation

| Document | Description |
|----------|-------------|
| [README.md](README.md) | Project overview |
| [STATUS.md](STATUS.md) | Canonical status and metrics |
| [TOWER_ATOMIC_PATTERN.md](TOWER_ATOMIC_PATTERN.md) | Architecture pattern |
| [ROADMAP.md](ROADMAP.md) | Current priorities |
| [ROOT_INDEX.md](ROOT_INDEX.md) | Complete documentation index |
| `specs/current/security/` | Security specifications |
| `docs/sessions/` | Session archives |

---

**Last Updated**: February 9, 2026
