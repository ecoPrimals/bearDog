# Context — BearDog

## What This Is

BearDog is the **sovereign genetic cryptography primal** for the ecoPrimals ecosystem: a 100% Pure Rust cryptographic service provider that implements lineage-aware identity, entropy, and hardware-backed key operations. Other primals delegate crypto to BearDog via the Tower Atomic pattern (JSON-RPC and related IPC), so there is a single auditable crypto surface across the stack.

## Role in the Ecosystem

BearDog answers “who is this node?” and “perform this crypto operation safely” for Songbird, NestGate, biomeOS, and the rest of the primals. It does not own mesh routing, persistent vault storage, or general compute orchestration; it supplies **crypto atoms, secrets, HSM access, and genetic/lineage semantics** that those components consume at runtime through capability discovery.

## Technical Facts

- **Language:** 100% Rust (edition 2024), zero C dependencies in application code
- **License:** AGPL-3.0-only (SPDX on sources)
- **Version:** 0.9.0
- **Workspace:** 30 crates (`Cargo.toml` workspace)
- **Rust sources:** 2,000+ `.rs` files
- **MSRV:** 1.93.0 (`rust-toolchain.toml`)
- **Tests:** 15,100+ passing (0 failed)
- **Coverage:** 90.05% line (llvm-cov, workspace)
- **Unsafe:** 0 production blocks (`forbid(unsafe_code)` workspace-wide)
- **IPC:** JSON-RPC 2.0 over NDJSON via Unix sockets / TCP / named pipes (platform-dependent); tarpc optional behind feature gate

## Key Capabilities

- **Protocols:** JSON-RPC 2.0 over NDJSON (91+ methods); tarpc optional behind feature gate
- **Cryptography:** Ed25519, X25519, ChaCha20-Poly1305, BLAKE3 (plus TLS, Tor, post-quantum, and broader RustCrypto suite as exposed by handlers)
- **Hardware / identity:** HSM abstraction (software, PKCS#11, StrongBox, etc.), **FIDO2** / HID device discovery (`beardog-hid`)

Method domains include `crypto.*`, `tls.*`, `tor.*`, `genetic.*`, `secrets.*`, `beacon.*`, `relay.*`, `btsp.*`, `quantum.*`, and introspection (`discover_capabilities`, `primal.info`, `rpc.methods`). See [README.md](README.md) and [STATUS.md](STATUS.md) for detail.

## What This Does NOT Do

BearDog is not a network mesh (that is Songbird), not durable encrypted storage (NestGate), not a GPU or host orchestrator (ToadStool/coralReef), and not an AI runtime (Squirrel). It exposes crypto and identity services over IPC; it does not replace those domains.

## Build and Test

```bash
git clone <repository-url>
cd beardog
cargo build --release
cargo test --workspace
```

Run the server (example): `cargo run --release --bin beardog -- server`. Pin Rust via `rust-toolchain.toml` (MSRV 1.93.0).

## Related Repositories

- [wateringHole](https://github.com/ecoPrimals/wateringHole) — ecosystem standards, primal registry, public-surface conventions (`PUBLIC_SURFACE_STANDARD.md`)
- [ecoPrimals](https://github.com/ecoPrimals) — organization root for sovereign computing primals

## Design Philosophy

BearDog is evolved under strict compiler and lint constraints: dependency injection, no production panics, capability-based discovery instead of hardcoded peers, and tests that run concurrently without serial gates. Complexity is pushed to **runtime coordination** between primals, not compile-time coupling.

For the full public on-ramp, architecture, and security model, see [README.md](README.md), [ARCHITECTURE.md](ARCHITECTURE.md), and [SECURITY.md](SECURITY.md).
