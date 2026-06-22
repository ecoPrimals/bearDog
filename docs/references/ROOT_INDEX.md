<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->
<!-- Creative content: CC-BY-SA 4.0 (scyBorg provenance trio) -->

# BearDog Documentation Index

**Last Updated**: Jun 22, 2026

Complete guide to BearDog documentation, organized by purpose.

---

## Getting Started

| Document | Description | Time |
|----------|-------------|------|
| [README.md](../../README.md) | Project overview and features | 5 min |
| [START_HERE.md](../../START_HERE.md) | Quick start and onboarding | 5 min |
| [ROADMAP.md](../../ROADMAP.md) | Current priorities | 5 min |

---

## Architecture & Design

### Core Patterns

| Document | Description |
|----------|-------------|
| [TOWER_ATOMIC_PATTERN.md](TOWER_ATOMIC_PATTERN.md) | How BearDog provides crypto atoms to other primals |
| [ARCHITECTURE.md](../../ARCHITECTURE.md) | Module structure, crate organization, trait hierarchy |
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
| [STATUS.md](../../STATUS.md) | **Canonical status** -- metrics, coverage, platform support |
| [CHANGELOG.md](../../CHANGELOG.md) | Release history and notable changes |

---

## Security Specifications

| Document | Description |
|----------|-------------|
| [TOR_CAPABILITY_SPECIFICATION.md](../../specs/current/security/TOR_CAPABILITY_SPECIFICATION.md) | Tor v3 onion capability spec |
| [TOR_PHASE2_NTOR_HANDSHAKE.md](../../specs/current/security/TOR_PHASE2_NTOR_HANDSHAKE.md) | ntor handshake protocol |
| [TOR_PHASE2_CELL_CRYPTO.md](../../specs/current/security/TOR_PHASE2_CELL_CRYPTO.md) | Relay cell encryption |
| [GENETIC_LINEAGE_EVOLUTION_SPEC.md](../../specs/GENETIC_LINEAGE_EVOLUTION_SPEC.md) | Genetic lineage key derivation |

---

## Operations & Deployment

### Quick Guides

| Document | Description |
|----------|-------------|
| [QUICK_START_SOFTWARE_HSM.md](QUICK_START_SOFTWARE_HSM.md) | Run with software HSM |
| [UNIVERSAL_ADAPTER_QUICK_REF.md](UNIVERSAL_ADAPTER_QUICK_REF.md) | Adapter pattern reference |
| [HOT_PLUG_HSM_DEMO.md](HOT_PLUG_HSM_DEMO.md) | Dynamic HSM detection demo |
| [DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md) | Production deployment guide |

### Configuration

| Resource | Description |
|----------|-------------|
| [ENVIRONMENT_VARIABLES.md](ENVIRONMENT_VARIABLES.md) | Complete env var reference |
| [QUICK_START_ZERO_HARDCODING.md](QUICK_START_ZERO_HARDCODING.md) | Zero hardcoding quick start |
| `configs/` | Configuration templates and examples |

---

## Technical Reference

| Resource | Description |
|----------|-------------|
| [PRIMAL_CONTRACTS.md](../PRIMAL_CONTRACTS.md) | JSON-RPC method contracts |
| [QUICK_REFERENCE_CARD.md](QUICK_REFERENCE_CARD.md) | Quick reference card |
| [README_BIOMEOS_SOCKET.md](README_BIOMEOS_SOCKET.md) | biomeOS socket discovery |
| [TOR_PHASE2_EVOLUTION.md](TOR_PHASE2_EVOLUTION.md) | Tor Phase 2 evolution plan |

---

## Session Archives

Historical session documentation is preserved in `ecoPrimals/archive/` as fossil record.

---

## Reading Paths

**New contributor** (30 min):
README.md -> START_HERE.md -> TOWER_ATOMIC_PATTERN.md

**Architecture deep dive** (2 hours):
TOWER_ATOMIC_PATTERN.md -> ARCHITECTURE.md -> UNIBIN_ECOBIN_EXPLAINED.md -> ENTROPY_HIERARCHY_PRINCIPLE.md

**Integration partner** (1 hour):
TOWER_ATOMIC_PATTERN.md -> PRIMAL_CONTRACTS.md -> START_HERE.md

---

**Last Updated**: Jun 22, 2026
