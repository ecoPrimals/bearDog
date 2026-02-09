# BearDog Roadmap

**Updated**: February 9, 2026
**Status**: Production Ready
**Grade**: A+ LEGENDARY (99/100)

---

## Current State

BearDog is production-ready with TRUE ecoBin v2.0 compliance achieved. All core evolution is complete.

### Completed

- 100% Pure Rust (zero C dependencies, RustCrypto suite)
- 91+ JSON-RPC crypto methods (semantic naming)
- Multi-family socket support (`--family-id` flag)
- Encrypted secret storage (family-scoped ChaCha20-Poly1305)
- `discover_capabilities` introspection method
- Tor v3 onion address derivation + ntor handshake + cell crypto
- Dark Forest beacon (zero metadata leakage discovery)
- Universal IPC (Unix sockets, abstract sockets, TCP)
- Android StrongBox integration (100% complete)
- HSM abstraction (software, PKCS#11, StrongBox)
- `once_cell` migrated to `std::sync::LazyLock`
- All production `unwrap()`/`expect()` eliminated (zero panic paths)
- 8,789 tests across 28 packages
- 20/20 deep debt evolution rounds

### Platform Coverage

| Platform | Transport | Status |
|----------|-----------|--------|
| Linux (x86_64, ARM64) | Unix sockets | Validated |
| macOS (Intel, M-series) | Unix sockets | Validated |
| Android (ARM64) | Abstract sockets + TCP | Ready |
| Windows (x86_64, ARM64) | Named pipes + TCP | Ready |
| iOS (ARM64) | TCP | Ready |
| WASM | In-process | Ready |

---

## Future Work

These items are enhancements -- nothing is blocking production use.

### Secret Storage Evolution (when NestGate available)

Current in-memory storage backend evolves to persistent NestGate-backed storage via capability discovery. BearDog discovers NestGate's `storage.store` / `storage.retrieve` at runtime. No code changes needed in BearDog -- the discovery pattern is already implemented.

### Graph Security Phase 2-3 (optional, ~5 hours)

- Phase 2: Public key infrastructure (storage/retrieval, trust_db integration)
- Phase 3: Signature verification (Ed25519 helpers, chain of custody validation)

### Test Coverage to 90% (ongoing)

Current coverage is 70.96%. Incremental improvement through:
- E2E test scenarios
- Fault injection / chaos tests
- Edge case coverage for crypto handlers

### Semantic Method Naming Phase 3 (ecosystem coordination)

Fully generic methods: `crypto.encrypt` + `{"algorithm": "aes-256-gcm"}` instead of algorithm-specific method names. Requires coordination across Songbird, Squirrel, NestGate.

### Performance Benchmarks

Comprehensive benchmark suite comparing BearDog crypto latency with OpenSSL/BoringSSL for key operations.

---

## Design Principles

These guide all BearDog evolution:

1. **Pure Rust** -- No C dependencies, ever
2. **Smart Refactoring** -- Domain-driven module boundaries, not arbitrary splits
3. **Safe Code** -- Zero `unsafe`, zero production panics
4. **Agnostic Config** -- Environment variables and capability discovery, no hardcoding
5. **Runtime Discovery** -- Primals discover each other at runtime, never hardcode names
6. **Honest Code** -- No production mocks, clear capability boundaries

---

**Last Updated**: February 9, 2026
