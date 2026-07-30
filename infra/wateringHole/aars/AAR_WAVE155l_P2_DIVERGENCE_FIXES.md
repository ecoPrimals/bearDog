<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# AAR — Wave 155l: P2 Divergence Fixes

**Date**: July 30, 2026
**Wave**: 155l
**Author**: bearDog team (eastGate)
**Scope**: Fix 2 P2 divergences found during NUCLEUS deployment (Wave 155k blurb)

---

## Context

Wave 155k NUCLEUS deployment on westGate and blueGate exposed two P2
divergences assigned to bearDog:

1. **Dual-socket footgun** (westGate): `beardog-default.sock` (health) and
   `beardog-{family}.sock` (main) expose different capability counts
2. **`FAMILY_SEED` now required** (blueGate): Reported as a breaking change
   from Wave 155i, though investigation showed it's pre-existing behavior

---

## Findings

### Issue 1: Dual-Socket Footgun — 3 Sub-Problems

**1a. Doc/behavior mismatch**: CLI arg docs said health socket is opt-in
("Default when omitted: no health socket"). In reality, it always spawns on
Unix unless `--bind-mode tcp`.

**Fix**: Updated docs to match actual behavior. The always-on health socket is
correct — cellMembrane and monitoring probes depend on it.

**1b. `--family-id` CLI flag not propagated to env**: The CLI handler logged
the flag but never set `FAMILY_ID`/`BEARDOG_FAMILY_ID` env vars. This meant:
- Socket path: `beardog-{family}.sock` (from CLI flag)
- BTSP mode: Development (env empty → not production)
- Identity: `"standalone"` (from `PrimalIdentity::from_env()`)

Result: family-scoped socket with standalone identity and no BTSP enforcement.

**Fix**: Added `set_var(ENV_FAMILY_ID, ...)` and `set_var(ENV_FAMILY_ID_PREFIXED, ...)`
before `PrimalIdentity::from_env()` is called. Now all components agree.

**1c. Hardcoded symlink suffix in MultiTransportServer**: Used `.sock` instead
of the family-scoped suffix from `SocketConfig::ipc_symlink_filename_suffix()`.
Legacy `modes/server/mod.rs` path had this correct; the newer path did not.

**Fix**: `MultiTransportServer::bind_all_available` now derives the suffix from
`identity.is_standalone()` / `identity.family_id()`.

### Issue 2: `FAMILY_SEED` Precedence Inconsistency

**Not a Wave 155i regression.** The FAMILY_SEED requirement when FAMILY_ID is
set is documented and pre-existing. The actual bug was inconsistent env var
precedence:

| Call site | Before | After |
|-----------|--------|-------|
| `btsp_handshake::load_family_seed` | `FAMILY_SEED` → `BEARDOG_FAMILY_SEED` | `BEARDOG_FAMILY_SEED` → `FAMILY_SEED` |
| `btsp/negotiation::load_family_seed` | `FAMILY_SEED` → `BEARDOG_FAMILY_SEED` | `BEARDOG_FAMILY_SEED` → `FAMILY_SEED` |
| `connection_handlers` | `FAMILY_SEED` → `BEARDOG_FAMILY_SEED` | `BEARDOG_FAMILY_SEED` → `FAMILY_SEED` |
| `secrets.rs` | Already correct | No change |
| `enrollment/config.rs` | Already correct | No change |
| `purpose_key.rs` | Already correct | No change |

EcoPrimals convention: prefixed env (primal-scoped) takes precedence over
unprefixed (ecosystem-wide). 3 of 6 sites had it backwards.

---

## Files Changed

| File | Change |
|------|--------|
| `crates/beardog-cli/src/lib.rs` | Health socket doc: opt-in → always-on |
| `crates/beardog-cli/src/handlers/server/mod.rs` | Propagate `--family-id` to env; un-gate `env_keys` import |
| `crates/beardog-tunnel/src/multi_transport_server.rs` | Family-scoped symlink suffix |
| `crates/beardog-tunnel/src/btsp_handshake/mod.rs` | Seed precedence + doc |
| `crates/beardog-tunnel/src/unix_socket_ipc/handlers/btsp/negotiation.rs` | Seed precedence |
| `crates/beardog-tunnel/src/unix_socket_ipc/connection_handlers.rs` | Seed precedence |
| `docs/references/ENVIRONMENT_VARIABLES.md` | Precedence doc update |
| `STATUS.md` | Wave 155l entry |

---

## Test Results

- **14,019 passed**, 0 failed, 131 ignored
- Zero Clippy warnings
- Clean compile

---

## Upstream Notes

### For overwatch
- Both bearDog P2 divergences from Wave 155k are now closed
- bearDog has **0 P0, 0 P1, 0 P2** items remaining
- The `PRIMAL_BIND_MODE` `tcp_only` silently falling back to `auto` (P2) is
  an **all-primals** issue, not bearDog-specific

### For gate operators
- **westGate**: Redeploy with this build to get correct family-scoped symlinks
  and aligned identity
- **blueGate**: Ensure `BEARDOG_FAMILY_SEED` (or `FAMILY_SEED`) is set when
  `FAMILY_ID` is non-default. This is documented pre-existing behavior, not a
  Wave 155i regression
