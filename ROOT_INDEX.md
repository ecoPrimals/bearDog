# BearDog Documentation Index

**Last Updated**: February 9, 2026

Complete guide to BearDog documentation, organized by purpose.

---

## Getting Started

| Document | Description | Time |
|----------|-------------|------|
| [README.md](README.md) | Project overview and features | 5 min |
| [START_HERE.md](START_HERE.md) | Quick start and onboarding | 5 min |
| [DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md) | Production deployment | 10 min |
| [ROADMAP.md](ROADMAP.md) | Current priorities | 5 min |

---

## Architecture & Design

### Core Patterns

| Document | Description |
|----------|-------------|
| [TOWER_ATOMIC_PATTERN.md](TOWER_ATOMIC_PATTERN.md) | How BearDog provides crypto atoms to other primals |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Module structure, crate organization, data flow |
| [UNIBIN_ECOBIN_EXPLAINED.md](UNIBIN_ECOBIN_EXPLAINED.md) | Binary architecture standards (UniBin, ecoBin v2.0) |

### Principles & Standards

| Document | Description |
|----------|-------------|
| [MOCK_ISOLATION_POLICY.md](MOCK_ISOLATION_POLICY.md) | Mocks only in tests, production code honesty |
| [ENTROPY_HIERARCHY_PRINCIPLE.md](ENTROPY_HIERARCHY_PRINCIPLE.md) | HSM hierarchy (software, hardware, cloud) |

---

## Status & Quality

| Document | Description |
|----------|-------------|
| [STATUS.md](STATUS.md) | **Canonical status** -- metrics, evolution history, quality |
| [CURRENT_STATUS.md](CURRENT_STATUS.md) | Redirect to STATUS.md (backward compat) |

---

## Security Specifications

| Document | Description |
|----------|-------------|
| [specs/current/security/TOR_CAPABILITY_SPECIFICATION.md](specs/current/security/TOR_CAPABILITY_SPECIFICATION.md) | Tor v3 onion capability spec |
| [specs/current/security/TOR_PHASE2_NTOR_HANDSHAKE.md](specs/current/security/TOR_PHASE2_NTOR_HANDSHAKE.md) | ntor handshake protocol |
| [specs/current/security/TOR_PHASE2_CELL_CRYPTO.md](specs/current/security/TOR_PHASE2_CELL_CRYPTO.md) | Relay cell encryption |
| [specs/GENETIC_LINEAGE_EVOLUTION_SPEC.md](specs/GENETIC_LINEAGE_EVOLUTION_SPEC.md) | Genetic lineage key derivation |

---

## Operations & Deployment

### Quick Guides

| Document | Description |
|----------|-------------|
| [QUICK_START_SOFTWARE_HSM.md](QUICK_START_SOFTWARE_HSM.md) | Run with software HSM |
| [UNIVERSAL_ADAPTER_QUICK_REF.md](UNIVERSAL_ADAPTER_QUICK_REF.md) | Adapter pattern reference |
| [HOT_PLUG_HSM_DEMO.md](HOT_PLUG_HSM_DEMO.md) | Dynamic HSM detection demo |

### Configuration

| Resource | Description |
|----------|-------------|
| [ENVIRONMENT_VARIABLES.md](ENVIRONMENT_VARIABLES.md) | Complete env var reference |
| `configs/` | Configuration templates and examples |

---

## Technical Reference

| Resource | Description |
|----------|-------------|
| [docs/DEPENDENCY_RATIONALE.md](docs/DEPENDENCY_RATIONALE.md) | Why each dependency exists |
| [docs/RUSTCRYPTO_ANALYSIS.md](docs/RUSTCRYPTO_ANALYSIS.md) | Pure Rust crypto stack analysis |
| [docs/PRIMAL_CONTRACTS.md](docs/PRIMAL_CONTRACTS.md) | JSON-RPC method contracts |

---

## Session Archives

Historical session documentation is in `docs/sessions/2026-01-30/` (40+ documents covering platform evolution, deep debt execution, StrongBox refactor, and genomeBin implementation).

Older archives are in `archives/` organized by date and topic.

---

## Reading Paths

**New contributor** (30 min):
README.md -> START_HERE.md -> TOWER_ATOMIC_PATTERN.md

**Architecture deep dive** (2 hours):
TOWER_ATOMIC_PATTERN.md -> ARCHITECTURE.md -> UNIBIN_ECOBIN_EXPLAINED.md -> ENTROPY_HIERARCHY_PRINCIPLE.md

**Integration partner** (1 hour):
TOWER_ATOMIC_PATTERN.md -> docs/PRIMAL_CONTRACTS.md -> START_HERE.md

---

**Last Updated**: February 9, 2026
