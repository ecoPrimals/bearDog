# BearDog

[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)](STATUS.md)
[![Tests](https://img.shields.io/badge/tests-12300+-brightgreen.svg)](STATUS.md)
[![Coverage](https://img.shields.io/badge/coverage-78.6%25-yellow.svg)](STATUS.md)
[![Pure Rust](https://img.shields.io/badge/rust-100%25_pure-orange.svg)](STATUS.md)

**BearDog** is the cryptographic service provider for the ecoPrimals ecosystem -- a **100% Pure Rust** security platform with zero C dependencies.

**Status**: Production Ready | **Crates**: 30 | **Tests**: 12,300+

---

## Overview

BearDog provides secure cryptographic operations for all primals through the **Tower Atomic Pattern**. Each primal delegates crypto to BearDog via JSON-RPC over Unix sockets, keeping a single auditable crypto codebase.

```
┌─────────────┐                    ┌─────────────┐
│  Any Primal │ ←─ JSON-RPC ────→ │  BearDog    │
│ (Protocol)  │    Unix Socket     │  (Crypto)   │
└─────────────┘                    └─────────────┘
     Zero crypto code                 All crypto operations
```

### Key Features

- **100% Pure Rust** -- Zero C dependencies (RustCrypto suite)
- **91+ Crypto Methods** -- Complete JSON-RPC API
- **Tor v3 Support** -- Onion address derivation + ntor handshake + cell crypto
- **Multi-Family Support** -- `--family-id` flag for per-family instances
- **Secret Storage** -- Encrypted secrets with family-scoped keys
- **Relay Authorization** -- Lineage-gated access for relay-assisted coordinated punch
- **Universal IPC** -- Multi-transport, platform-agnostic
- **HSM Integration** -- Hardware, software, mobile backends
- **Dark Forest Beacon** -- Zero metadata leakage discovery

---

## Quick Start

### Prerequisites

```bash
# Rust 1.80+ required (uses std::sync::LazyLock)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build & Test

```bash
git clone <repository>
cd beardog
cargo build --release
cargo test --workspace
```

### Run

```bash
# Default -- auto-detects platform transport
cargo run --release --bin beardog -- server

# With family isolation (multi-family architecture)
./beardog server --family-id alpha

# Custom socket path
./beardog server --socket /custom/path.sock

# TCP transport (Android, Windows, cross-device)
./beardog server --listen 0.0.0.0:9900
```

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
secrets.*      - Store, retrieve, list, delete encrypted secrets
relay.*        - Lineage-gated relay authorization (coordinated punch)
beacon.*       - Dark Forest beacon generation, encryption, meeting exchange
btsp.*         - Secure tunnel configuration
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

- **Separation of Concerns** -- Protocol logic stays in primals, crypto stays in BearDog
- **Single Audit Surface** -- One codebase to review for crypto correctness
- **HSM Abstraction** -- Software, hardware (PKCS#11), or mobile (StrongBox) backends
- **Family Isolation** -- Each family gets its own BearDog instance with independently derived keys

### Multi-Family Architecture

```bash
# Family A gets its own socket and key material
./beardog server --family-id alpha   # beardog-alpha.sock

# Family B is fully isolated
./beardog server --family-id bravo   # beardog-bravo.sock
```

Key material is derived from the family seed. A BearDog instance serving family A never shares keys with family B.

---

## Quality

| Metric | Value |
|--------|-------|
| **Build** | Clean, 0 errors |
| **Tests** | 12,300+ passing (30 crates) |
| **Coverage** | 78.6% line coverage (llvm-cov) |
| **Pure Rust** | 100% -- zero C dependencies |
| **Unsafe Code** | 0 production blocks |
| **Production panics** | 0 -- all `Result<T, E>` |

### Standards

- **Pure Rust** -- No C dependencies anywhere
- **Zero Hardcoding** -- Environment variables and capability discovery
- **Result<T, E>** -- No `unwrap()`/`expect()` in production code
- **< 1000 LOC** -- File size discipline (exceptions justified)
- **std over external** -- `std::sync::LazyLock` over `once_cell`, etc.

---

## Documentation

| Document | Description |
|----------|-------------|
| [STATUS.md](STATUS.md) | Current status and metrics |
| [START_HERE.md](START_HERE.md) | Quick start and onboarding |
| [TOWER_ATOMIC_PATTERN.md](TOWER_ATOMIC_PATTERN.md) | Architecture pattern |
| [ROADMAP.md](ROADMAP.md) | Current priorities and roadmap |
| [ROOT_INDEX.md](ROOT_INDEX.md) | Complete documentation index |

---

## License

See [LICENSE](LICENSE) file.

---

**BearDog**: 100% Pure Rust Cryptographic Service Provider for the ecoPrimals Ecosystem
