<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Handoff — Wave 155l: P2 Divergence Fixes

**Date**: July 30, 2026
**From**: bearDog team (eastGate)
**To**: overwatch, gate operators

---

## What Shipped

Two P2 divergences from the Wave 155k ecosystem blurb have been fixed:

### 1. Dual-Socket Footgun — RESOLVED

| Sub-problem | Fix |
|-------------|-----|
| Health socket doc said "opt-in" but always spawns | Doc updated to match behavior |
| `--family-id` CLI flag not propagated to env | Now sets `FAMILY_ID` + `BEARDOG_FAMILY_ID` before identity/BTSP resolution |
| Capability symlinks used hardcoded `.sock` suffix | Now uses family-scoped suffix from identity |

**Impact**: On westGate/strandGate, `--family-id alpha` now produces:
- Main socket: `beardog-alpha.sock` (full registry, BTSP enforced)
- Health socket: `beardog-default.sock` (alive/version only, plaintext)
- Capability symlinks: `crypto-alpha.sock`, `identity-alpha.sock`, etc.

### 2. `FAMILY_SEED` Precedence — NORMALIZED

All load sites now check `BEARDOG_FAMILY_SEED` first, then `FAMILY_SEED`,
matching ecoPrimals convention (primal-scoped overrides ecosystem-wide).

Previously 3 of 6 sites had the order reversed. This could cause surprising
behavior when both env vars were set with different values.

---

## bearDog Status

| Metric | Value |
|--------|-------|
| Version | 0.9.0 |
| Tests | 14,019 |
| Clippy | 0 warnings |
| P0 | 0 |
| P1 | 0 |
| P2 | **0** (both items closed) |
| Status | **STANDBY** — all assigned work complete |

---

## Gate Operator Actions

| Gate | Action |
|------|--------|
| **westGate** | Rebuild `beardog` from latest; capability symlinks will align with family naming |
| **blueGate** | Ensure `BEARDOG_FAMILY_SEED` or `FAMILY_SEED` is set when `FAMILY_ID` is non-default |
| **sporeGate** | Rebuild depot `beardog` binary (musl + windows targets) |

---

## No Further Work Needed

bearDog has zero open items. Resume when:
- Provenance 7/7 live validation needs bearDog changes
- G6 crates.io public flip is scheduled
- New platform targets require crypto provider work
