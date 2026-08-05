<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Wave 156e Handoff — bearDog Neural API Routing Stub (E1)

**Date**: Aug 5, 2026
**From**: eastGate (bearDog team)
**To**: overwatch, sporeGate team

---

## What Changed

### E1 Debt Item — Neural API Routing Stub

**Root cause**: bearDog already implemented both `capability.register` (legacy) and `primal.announce` (routing table) paths at startup. Two issues prevented full visibility in nestgate.io:

1. **Stale legacy `capability.register` domains** — Only registered 4 domains (`crypto`, `tls_crypto`, `genetic_lineage`, `security`) with 22 generic operations. Missing `auth`, `btsp`, `bonding`, `secrets`, `relay`, `consent`. The `tls_crypto` domain is songBird's responsibility (Wave 155b handoff); `genetic_lineage` naming diverged from runtime `genetics`/`genetic` convention.

2. **TCP socket path mismatch in `primal.announce`** — The CLI server passed `registration_addr` (which correctly resolves to TCP address when in TCP-only mode) to legacy `capability.register`, but always passed the Unix `socket_path` to `send_primal_announce`. TCP-only deployments would register with legacy but not appear in the routing table.

### Fix Details

| File | Change |
|------|--------|
| `beardog-ipc/src/neural_registration.rs` | 4→8 capability domains; crypto operations expanded from 12 generic to 36 canonical names matching runtime handler surface |
| `beardog-cli/src/handlers/server/mod.rs` | `send_primal_announce` now receives `registration_addr` instead of `&socket_path` |

### Domains Now Registered

| Domain | Version | Operations |
|--------|---------|------------|
| `crypto` | 0.9.0 | 36 (signing, AEAD, hash, KDF, key-exchange, ionic bonds, contracts, semantic aliases) |
| `auth` | 1.0.0 | 7 (check, mode, peer_info, issue_ionic, issue_session, verify_ionic, public_key) |
| `btsp` | 2.0.0 | 3 (negotiate, capabilities, server_create_session) |
| `security` | 1.0.0 | 5 (verify_consent, issue_consent_token, evaluate, lineage, generate_jwt_secret) |
| `bonding` | 1.0.0 | 5 (propose, accept, status, terminate, modify_scope) |
| `secrets` | 1.0.0 | 2 (store, retrieve) |
| `relay` | 1.0.0 | 1 (authorize) |
| `consent` | 1.0.0 | 2 (verify, issue) |

### Removed Domains

| Domain | Reason |
|--------|--------|
| `tls_crypto` | songBird's responsibility (Wave 155b handoff) |
| `genetic_lineage` | Naming inconsistency; ops covered under `security.lineage` |

---

## Verification

- **14,019 tests**, 0 failures
- **0 Clippy errors**, pre-existing warnings only
- `cargo check` clean

---

## bearDog Status

| Metric | Value |
|--------|-------|
| P0/P1/P2 | **ZERO** |
| Tests | 14,019 |
| Clippy | 0 errors |
| E1 (Neural API stub) | **SHIPPED** |

---

## For Upstream

- **nestgate.io**: After bearDog redeployment on a NUCLEUS gate, the Neural API routing table should show bearDog with 8 capability domains and full method list via `primal.announce`. If the routing table still doesn't show bearDog, verify the Neural API socket is discoverable at the gate (5-tier discovery: `NEURAL_API_SOCKET` → `BIOMEOS_SOCKET_DIR` → XDG → `/run/user` → temp).
- **Neural API symlink pattern (S9)**: bearDog's 5-tier discovery already handles symlink-based discovery. If S9 standardizes a different pattern, bearDog can adapt without code changes via `NEURAL_API_SOCKET` env var override.
