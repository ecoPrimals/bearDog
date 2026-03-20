# BearDog Quick Start

**Status**: Production Ready | **Tests**: 12,751+ | **Coverage**: 78.6%

---

## Prerequisites

```bash
# Rust 1.80+ required
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Build

```bash
cd /home/eastgate/Development/ecoPrimals/phase1/beardog
cargo build --release
```

---

## Run BearDog Server

### Default (Unix Socket)

```bash
cargo run --release --bin beardog -- server
# Socket: /run/user/$UID/beardog.sock (Linux)
# Socket: /tmp/beardog.sock (macOS)
```

### With Family Isolation

```bash
# Family Alpha
./target/release/beardog server --family-id alpha
# Socket: beardog-alpha.sock

# Family Bravo (fully isolated)
./target/release/beardog server --family-id bravo
# Socket: beardog-bravo.sock
```

### TCP Transport (Android/Windows/Cross-Device)

```bash
./target/release/beardog server --listen 127.0.0.1:9900
```

---

## Test the API

### Using Example Client

```bash
cargo run --release --example crypto_client
```

### Manual JSON-RPC (via socat)

```bash
# Check capabilities
echo '{"jsonrpc":"2.0","method":"discover_capabilities","params":{},"id":1}' | \
  socat - UNIX-CONNECT:/run/user/$UID/beardog.sock

# Get primal info
echo '{"jsonrpc":"2.0","method":"primal.info","params":{},"id":2}' | \
  socat - UNIX-CONNECT:/run/user/$UID/beardog.sock

# BLAKE3 hash
echo '{"jsonrpc":"2.0","method":"crypto.blake3_hash","params":{"data":"dGVzdA=="},"id":3}' | \
  socat - UNIX-CONNECT:/run/user/$UID/beardog.sock
```

---

## Verify

```bash
# Run all tests
cargo test --workspace

# Run clippy (should be 0 errors)
cargo clippy --workspace

# Check format
cargo fmt --all --check
```

---

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `PRIMAL_NAME` | Primal identity | `beardog` |
| `FAMILY_ID` | Family identifier | (none) |
| `BEARDOG_SOCKET` | Socket path override | auto-detected |
| `IPC_SOCKET` | IPC socket override | auto-detected |
| `RUST_LOG` | Log level | `info` |

---

## JSON-RPC Methods (91+)

```
crypto.*       Hash, sign, verify, encrypt, decrypt
tls.*          TLS 1.2/1.3 key derivation
tor.*          Onion identity, ntor handshake
genetic.*      Lineage keys, entropy mixing
secrets.*      Encrypted secret storage
beacon.*       Dark Forest discovery
relay.*        Lineage-gated authorization
```

---

## Next Steps

| Document | Description |
|----------|-------------|
| [README.md](README.md) | Full project overview |
| [STATUS.md](STATUS.md) | Current metrics |
| [TOWER_ATOMIC_PATTERN.md](TOWER_ATOMIC_PATTERN.md) | Architecture |
| [ROADMAP.md](ROADMAP.md) | Priorities |

---

**BearDog**: 100% Pure Rust Cryptographic Service Provider
