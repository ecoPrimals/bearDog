# BearDog Roadmap

**Updated**: February 11, 2026
**Status**: Production Ready

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
- Android StrongBox integration (complete)
- HSM abstraction (software, PKCS#11, StrongBox)
- `once_cell` migrated to `std::sync::LazyLock`
- All production `unwrap()`/`expect()` eliminated (zero panic paths)
- 12,300+ tests across 30 crates
- Deep debt evolution (20/20 rounds)
- Doc comments added (328 warnings resolved)
- Smart refactoring of oversized files

### Platform Coverage

| Platform | Transport | Status |
|----------|-----------|--------|
| Linux (x86_64, ARM64) | Unix sockets | Validated |
| macOS (Intel, M-series) | Unix sockets | Validated |
| Android (ARM64) | Abstract sockets + TCP | Ready |
| Windows (x86_64, ARM64) | Named pipes + TCP | Ready |
| iOS (ARM64) | TCP | Ready |

---

## In Progress

### Test Coverage to 90%

Current coverage is 78.6% (up from ~62% baseline). Targeting 90% across all crates.

**Crates above 90%**: beardog-workflows (96.75%), beardog-threat (96.99%), beardog-monitoring (93.05%), beardog-hid (92.86%), beardog-utils (92.28%), beardog-config (92.06%), beardog-security (91.94%), beardog-genetics (90.87%), beardog-adapters (90.20%).

**Crates in progress**: beardog-types (78.60%), beardog-tunnel (75.60%).

### Relay-Assisted Coordinated Punch (BearDog Step 3 — Complete)

BearDog now provides `relay.authorize` for lineage-gated relay authorization. When Songbird's relay server receives an allocation request, it calls BearDog to verify the requester's family membership before forwarding traffic. This is BearDog's role in the relay-assisted coordinated punch protocol that improves symmetric-NAT-to-symmetric-NAT success rates from ~5% to 60-80%.

**What BearDog owns**: `relay.authorize` (identity verification). **What BearDog does NOT own**: UDP sockets, relay forwarding, STUN probes, punch timing (all Songbird).

---

## Future Work

These items are enhancements -- nothing is blocking production use.

### Secret Storage Evolution (when NestGate available)

Current in-memory storage backend evolves to persistent NestGate-backed storage via capability discovery. BearDog discovers NestGate's `storage.store` / `storage.retrieve` at runtime. No code changes needed in BearDog -- the discovery pattern is already implemented.

### Graph Security Phase 2-3 (optional)

- Phase 2: Public key infrastructure (storage/retrieval, trust_db integration)
- Phase 3: Signature verification (Ed25519 helpers, chain of custody validation)

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

**Last Updated**: February 11, 2026
