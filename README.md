# bearDog 🐻🐕

**Pure Rust cryptography for sovereign distributed computing.**

Zero C dependencies. No OpenSSL. No BoringSSL. No ring. Everything is Rust, all the way down.

## What bearDog Does

bearDog provides the cryptographic foundation for the [ecoPrimals](https://github.com/ecoPrimals) distributed scientific computing ecosystem. Every key exchange, every certificate, every signed artifact flows through bearDog.

### Implemented

| Protocol | Implementation | Status |
|---|---|---|
| **TLS 1.3** | Full handshake, 0-RTT, certificate chains | ✅ Production |
| **Ed25519** | Signing, verification, batch verify | ✅ Production |
| **X25519** | Key agreement, ephemeral ECDH | ✅ Production |
| **BLAKE3** | Hashing, keyed MAC, KDF, XOF | ✅ Production |
| **ML-KEM** (Kyber) | Post-quantum key encapsulation | ✅ Production |
| **Tor v3** | Onion address generation, circuit crypto | ✅ Production |
| **ChaCha20-Poly1305** | AEAD encryption | ✅ Production |
| **AES-256-GCM** | AEAD encryption (hardware accelerated) | ✅ Production |

### Design Principles

1. **Zero C dependencies** — No `unsafe` FFI. No system OpenSSL. Auditable Rust from top to bottom.
2. **Post-quantum ready** — ML-KEM (CRYSTALS-Kyber) for key encapsulation alongside classical X25519.
3. **Sovereign identity** — Ed25519 keypairs as the root of trust. No certificate authorities required.
4. **Constant-time** — All secret-dependent operations use constant-time comparisons and arithmetic.
5. **No allocator dependency** — Core primitives work in `no_std` environments.

## Why No C Dependencies?

Most Rust crypto libraries depend on C code through FFI — OpenSSL, BoringSSL, or the `ring` crate (which wraps BoringSSL C/assembly). This means:

- **Supply chain risk**: C code is harder to audit, has different memory safety guarantees
- **Build complexity**: Requires C compiler, platform-specific assembly
- **Reproducibility**: C dependencies introduce build variance across platforms

bearDog eliminates all of this. The entire cryptographic stack compiles with `cargo build` and nothing else. It runs on any platform Rust targets — including SteamOS, embedded Linux, and WASM.

## Part of ecoPrimals

bearDog is one of 16 primals in the [ecoPrimals](https://github.com/ecoPrimals) ecosystem — sovereign distributed scientific computing in Pure Rust:

- **biomeOS** — orchestration kernel
- **songBird** — mesh networking (TLS 1.3 via bearDog)
- **skunkBat** — network defense
- **toadStool** — GPU/NPU/CPU dispatch
- **coralReef** — WGSL shader compilation
- **barraCuda** — WebGPU scientific compute (lattice QCD, molecular dynamics)
- **bingoCube** — entropy and randomness (BLAKE3 via bearDog)
- **nestGate** — content-addressed storage
- **squirrel** — sovereign AI coordination (MCP server)

## Research Papers

bearDog's design is documented in research papers at [sporeprint.primals.eco](https://sporeprint.primals.eco):

- Paper 35: Genetic Lock Entropy Authentication
- Paper 47: Nested Relay Privacy
- Paper 79: ZK-NP Solved
- Paper 87: Anti-Secret Lineage

117 papers total covering WebGPU/WGSL GPU compute, protein structure prediction, lattice QCD, molecular dynamics, pharmacometrics, and bioinformatics — all on commodity hardware.

## License

AGPL-3.0-or-later. If you use bearDog, your users get the source.

## Links

- 📖 [sporePrint](https://sporeprint.primals.eco) — 117 research papers
- 🔭 [HUD](https://hud.primals.eco) — live membrane observation
- 🧬 [ORCID](https://orcid.org/0009-0004-2141-0321) — author profile
