<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# AAR — Wave 155m: Deep Debt Sweep — Orphan Purge + Hardcoding Fix

**Date**: July 30, 2026
**Wave**: 155m
**Author**: bearDog team (eastGate)
**Scope**: Orphan module deletion, hardcoded primal name fixes, dead dependency removal

---

## Context

Deep debt sweep targeting: dead code, hardcoded values, orphan modules, and
unused dependencies. Follows the ecoPrimals principle that primal code only has
self-knowledge and discovers other primals at runtime.

---

## What Was Done

### 1. Orphan Module Purge — 94 files deleted

All deleted files were verified to be:
- **Never `mod`-included** from any parent module
- **Compile-unreachable** — not part of any binary, lib, test, or example target
- **No callers** — zero references from wired code

| Cluster | Files | Crates |
|---------|-------|--------|
| `beardog-types` canonical tree | 43 | Types for config domains, security unified, network unified, providers, services, relationships |
| `beardog-tunnel` universal_hsm | 17 | Unwired registry, provider, health, and providers/ tree |
| `beardog-security` types/zero_copy/orchestration | 7 | Orphan type definitions and orchestration stub |
| `beardog-core` discovery/sovereignty/AI | 10 | Superseded mDNS/multi/UPA discovery, sovereignty genesis, AI decision/core splits |
| `beardog-config` timeouts_new splits | 5 | Unwired domain timeout files (ai, database, health, hsm, network) |
| `beardog-tunnel` iOS/software_hsm/Ed448 | 6 | Unwired iOS Secure Enclave prototypes, software HSM orphans, Ed448 stub handler |
| `beardog-auth`/`beardog-discovery` | 2 | Broken ecosystem.rs, orphaned dns_sd.rs |
| Other (auth, config comments) | 4 | Stale comment cleanup in universal_hsm/mod.rs |

### 2. Hardcoded Primal Name Fixes

| File | Before | After |
|------|--------|-------|
| `connection_handlers.rs:136` | `"primal": "bearDog"` | `env_keys::resolve_primal_name()` |
| `primal_self_knowledge.rs:183` | `"beardog-default"` | `env_keys::resolve_primal_name()` |
| `primal_self_knowledge.rs:618` | `"beardog-test"` | `env_keys::resolve_primal_name()` |
| `primal_self_knowledge.rs:619` | `"beardog"` | `env_keys::DEFAULT_PRIMAL_NAME` |

### 3. Dead Dependency Removed

`ed448-goldilocks = "0.9"` removed from workspace `[dependencies]`. The Ed448
handler stub (`crypto_handlers_ed448.rs`) was deleted — re-add both when Phase 3
activates.

### 4. Hygiene

- `PlatformListenerBackend::Placeholder` `#[allow(dead_code)]` now carries
  `reason = "Windows named-pipe listener not yet implemented; ..."`.

---

## What Was NOT Changed (Deliberate)

### `Box<dyn PlatformStream>` pattern

Evaluated for enum dispatch but deferred:
- Both `UnixPlatformStream` and `AndroidPlatformStream` wrap the same type
- `PrefixedStream` naturally needs inner indirection for its prepend-byte pattern
- Vtable cost (one pointer dereference per syscall) is negligible vs kernel overhead
- Pattern is well-documented, correct, and follows the same shape as other Rust async I/O libraries

### Workspace-excluded crates

`beardog-integration`, `beardog-deploy`, `beardog-node-registry`, `beardog-client`,
`beardog-workflows`, `beardog-production` are excluded from the workspace build.
Their orphan files are harmless (never compiled) and preserved for potential
re-enablement.

### Platform FFI dependencies

`ndk-sys`, `security-framework-sys`, `core-foundation-sys` are legitimate
platform FFI for Android StrongBox and iOS Secure Enclave — outside ecoBin
scope by design, target-gated in CI.

---

## Audit Findings for Upstream

### Remaining production stubs (not_yet_available at runtime)

These are correctly fail-closed but represent incomplete features:

| Module | Method | Status |
|--------|--------|--------|
| `beardog-adapters` | `dispatch_capability()` | IPC dispatch not wired |
| `beardog-core` universal_compute_client | `execute_compute_request()` | IPC transport not wired |
| `graph_security/collaboration_service` | All ops | Discovery unavailable |
| `quantum_crypto/kem` | ML-KEM decapsulate | PQC simulation only |
| `quantum_crypto/signatures` | PQC signatures | Simulation only |
| `platform/mod.rs` | Windows `Placeholder` listener | Named pipe not implemented |

These are **not production bugs** — they return structured errors. But callers
should not advertise these capabilities until they are wired.

---

## Test Results

- **14,019 passed**, 0 failed, 131 ignored
- Zero Clippy warnings
- Clean `cargo clean` + fresh rebuild verified
