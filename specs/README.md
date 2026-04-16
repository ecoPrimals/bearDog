# BearDog Specifications

**Last Updated**: April 15, 2026
**Version**: 0.9.0
**Status**: Production Ready
**Crates**: 29 | **Tests**: 14,787+ passing | **Coverage**: 90.51% (llvm-cov line coverage)

---

## Quick Status

| Metric | Status | Details |
|--------|--------|---------|
| Build | Clean | Zero errors, minimal warnings |
| Tests | 14,787+ passing | 29 crates, 100% pass rate |
| Coverage | 90.51% (llvm-cov) | 9 crates above 90% target |
| Memory Safety | ZERO unsafe | 100% safe Rust |
| Pure Rust | 100% | Zero C dependencies (RustCrypto suite) |
| Production panics | 0 | All production paths use `Result<T, E>` |
| Clippy | Clean | Pedantic-level compliance |
| Hardcoding | 0 | All config via env vars + capability discovery |

---

## Specification Structure

### Architecture (`current/architecture/`)

Core system design and type system specifications.

| Spec | Status | Description |
|------|--------|-------------|
| [BEARDOG_SCOPE_AND_BOUNDARIES](current/architecture/BEARDOG_SCOPE_AND_BOUNDARIES.md) | Current | Definitive scope definition |
| [CANONICAL_TYPE_SYSTEM_SPECIFICATION](current/architecture/CANONICAL_TYPE_SYSTEM_SPECIFICATION.md) | Current | Type system architecture |
| [ECOSYSTEM_SEPARATION_OF_CONCERNS](current/architecture/ECOSYSTEM_SEPARATION_OF_CONCERNS.md) | Current | BearDog vs other primals |
| [CAPABILITY_BASED_PRIMAL_INTERACTION](current/architecture/CAPABILITY_BASED_PRIMAL_INTERACTION.md) | Current | Runtime discovery pattern |
| [PRIMAL_SOVEREIGNTY_ARCHITECTURE](current/architecture/PRIMAL_SOVEREIGNTY_ARCHITECTURE.md) | Current | Self-knowledge principle |

### Security (`current/security/`)

Cryptographic specifications and security protocols.

| Spec | Status | Description |
|------|--------|-------------|
| [TOR_CAPABILITY_SPECIFICATION](current/security/TOR_CAPABILITY_SPECIFICATION.md) | Current | Tor v3 onion crypto (8 RPC methods) |
| [TOR_PHASE2_NTOR_HANDSHAKE](current/security/TOR_PHASE2_NTOR_HANDSHAKE.md) | Planning | ntor handshake protocol |
| [TOR_PHASE2_CELL_CRYPTO](current/security/TOR_PHASE2_CELL_CRYPTO.md) | Planning | Relay cell encryption |
| [ENTROPY_SECURITY_SPECIFICATION](current/security/ENTROPY_SECURITY_SPECIFICATION.md) | Current | Entropy hierarchy |
| [UNIVERSAL_HSM_SPECIFICATION](current/security/UNIVERSAL_HSM_SPECIFICATION.md) | Current | Hardware security modules |
| [UNIVERSAL_CRYPTO_PROVIDER_ARCHITECTURE](current/security/UNIVERSAL_CRYPTO_PROVIDER_ARCHITECTURE.md) | Current | Crypto provider pattern |

### Integration (`current/integration/`)

Cross-primal coordination and ecosystem patterns.

| Spec | Status | Description |
|------|--------|-------------|
| [BEARDOG_ECOSYSTEM_SECURITY_INTEGRATION](current/integration/BEARDOG_ECOSYSTEM_SECURITY_INTEGRATION.md) | Current | BearDog as security primal |
| [SONGBIRD_INTEGRATION_SPECIFICATION](current/integration/SONGBIRD_INTEGRATION_SPECIFICATION.md) | Current | Mesh/transport primal integration |
| [UNIVERSAL_ADAPTER_SPECIFICATION](current/integration/UNIVERSAL_ADAPTER_SPECIFICATION.md) | Current | Multi-provider adapter pattern |
| [SONGBIRD_BEARDOG_VPN_FREE_ARCHITECTURE](current/integration/SONGBIRD_BEARDOG_VPN_FREE_ARCHITECTURE.md) | Current | VPN-free P2P via genetic lineage |

### Production (`current/production/`)

Deployment, monitoring, and operational specifications.

| Spec | Status | Description |
|------|--------|-------------|
| [PRODUCTION_READINESS_SPECIFICATION](current/production/PRODUCTION_READINESS_SPECIFICATION.md) | Current | Production requirements |
| [CONFIGURATION_MANAGEMENT](current/production/CONFIGURATION_MANAGEMENT.md) | Current | Config hierarchy |
| [DISASTER_RECOVERY_RESILIENCE](current/production/DISASTER_RECOVERY_RESILIENCE.md) | Current | DR and resilience |

### Testing (`current/testing/`)

| Spec | Status | Description |
|------|--------|-------------|
| [TESTING_STRATEGY_TOWER_PIXEL8](current/testing/TESTING_STRATEGY_TOWER_PIXEL8.md) | Current | Multi-device test strategy |

---

## Key Design Specs (Root Level)

| Spec | Status | Description |
|------|--------|-------------|
| [GENETIC_LINEAGE_EVOLUTION_SPEC](GENETIC_LINEAGE_EVOLUTION_SPEC.md) | Current | Lineage derivation, enrollment certs |
| [UNIFIED_CONFIGURATION_ARCHITECTURE](UNIFIED_CONFIGURATION_ARCHITECTURE.md) | Current | Canonical config system |

---

## Cross-primal and cross-team documentation

Cross-team and ecosystem-wide specifications do **not** live under this primal repository. Per [primal sovereignty](current/architecture/PRIMAL_SOVEREIGNTY_ARCHITECTURE.md), shared standards and coordination docs belong in **wateringHole**, not in BearDog.

Older cross-team drafts that previously appeared under paths like `otherTeams/` were **archived to `ecoPrimals/fossilRecord`** when boundaries were clarified. Use wateringHole for current shared specs; use fossilRecord only for historical traceability.

---

## BearDog's Scope

### BearDog IS:

- **Cryptographic Service Provider** — All crypto operations for the ecosystem
- **Identity Authority** — Lineage verification, relay authorization, family gating
- **Secret Storage** — Encrypted secrets with family-scoped keys
- **HSM Abstraction** — Software, hardware (PKCS#11), mobile (StrongBox)

### BearDog IS NOT:

- Network transport (mesh / transport primal)
- Persistent storage (storage primal)
- Compute orchestration (ToadStool)
- AI execution (Squirrel)
- OS/container management (biomeOS)

**Principle**: BearDog answers "WHO are you?" and "CAN you do this crypto?". It never touches a socket, stores data persistently, or runs workloads.

---

## Current JSON-RPC Methods (100)

| Category | Methods | Examples |
|----------|---------|---------|
| `crypto.*` | 20 | `sign_ed25519`, `chacha20_poly1305_encrypt`, `blake3_hash` |
| `tls.*` | 4 | `derive_handshake_secrets`, `derive_application_secrets` |
| `tor.*` | 8 | `derive_onion_address`, `tor_ntor_client_init`, `tor_cell_encrypt` |
| `genetic.*` | 11 | `verify_lineage`, `generate_lineage_proof`, `derive_beacon_key` |
| `secrets.*` | 4 | `store`, `retrieve`, `list`, `delete` |
| `beacon.*` | 7 | `generate`, `encrypt`, `try_decrypt`, `list_known` |
| `relay.*` | 1 | `authorize` (lineage-gated relay authorization) |
| `federation.*` | 2 | `verify_family_member`, `derive_subfed_key` |
| `btsp.*` | 6 | `contact_exchange`, `tunnel_establish`, `tunnel_encrypt` |
| `security.*` | 6 | `evaluate`, `verify_jwt`, `generate_jwt_secret` |
| Introspection | 6 | `discover_capabilities`, `primal.info`, `rpc.methods` |

---

## Reading Order

**New to BearDog** (30 min):
1. This README (5 min)
2. [BEARDOG_SCOPE_AND_BOUNDARIES](current/architecture/BEARDOG_SCOPE_AND_BOUNDARIES.md) (10 min)
3. [ECOSYSTEM_SEPARATION_OF_CONCERNS](current/architecture/ECOSYSTEM_SEPARATION_OF_CONCERNS.md) (10 min)
4. [GENETIC_LINEAGE_EVOLUTION_SPEC](GENETIC_LINEAGE_EVOLUTION_SPEC.md) (5 min)

**Security deep dive** (1 hour):
TOR_CAPABILITY_SPECIFICATION → ENTROPY_SECURITY → UNIVERSAL_HSM → UNIVERSAL_CRYPTO_PROVIDER

**Integration partner** (1 hour):
BEARDOG_ECOSYSTEM_SECURITY_INTEGRATION → [SONGBIRD_INTEGRATION_SPECIFICATION](current/integration/SONGBIRD_INTEGRATION_SPECIFICATION.md) → UNIVERSAL_ADAPTER

---

## Historical Notes

Specs from 2025 (October-December) are preserved for historical reference. Many metrics from that era (4% coverage, 22 crates, 435 tests) are now vastly outdated. Current metrics are in [STATUS.md](../STATUS.md): **14,787+ tests** and **90.51% line coverage** (llvm-cov workspace, April 2026).

Historical experiment write-ups from the cryptographic foundation phase are **not** kept in this repo; they are preserved under **`ecoPrimals/fossilRecord`** for archival reference.

---

**Last Updated**: April 15, 2026
