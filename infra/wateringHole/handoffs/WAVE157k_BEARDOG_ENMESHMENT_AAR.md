<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# AAR — bearDog Wave 157k Enmeshment (Aug 14, 2026)

**Gate**: eastGate | **Primal**: bearDog | **Wave**: 157k | **Author**: bearDog code team (eastGate)

---

## Objective

Close bearDog's two routing gaps identified by westGate provenance trio experiments (P2 #10, P2 #14), deploy to eastGate, and validate live.

## What Happened

### Session 1 (AM) — Code + Push

1. **Cascaded from Forgejo** — bearDog at HEAD, no upstream changes.
2. **Analyzed blurb** — two bearDog items: AEAD not surfaced in Neural API (#14), rootPulse step handler activation (#10).
3. **Diagnosed AEAD gap** — the `crypto.aead.*` dotted semantic namespace had AES-128-GCM and AES-256-GCM but was missing ChaCha20-Poly1305. The flat-name equivalents existed and routed correctly, but the semantic names that biomeOS Neural API routes through were absent from method list, router, and cost hints.
4. **Diagnosed rootPulse gap** — the `rootpulse_commit` and `rootpulse_harvest` graphs define `primal = "bearDog"`, `operation = "auth.sign"`. biomeOS graph executor calls this via `capability.call`. bearDog had no `auth.sign` handler.
5. **Implemented both fixes** — 3 methods added (2 crypto AEAD + 1 auth.sign), routing, cost hints, announce payload, 11 new tests.
6. **Full test suite green** — 2,483 tests pass, 0 regressions.
7. **Committed and pushed** — `ffa5a7fab9`.

### Session 2 (PM) — Deploy + Validate + Debt Scan

1. **Re-cascaded from Forgejo** — confirmed at HEAD (blurb written before AM push, so items still listed as remaining).
2. **Deployed new binary to eastGate** — stopped running bearDog (PID 2068677), rename-swapped depot binary, biomeOS auto-restarted (PID 3816169).
3. **Live validation**:
   - `health` → alive, version 0.9.0
   - `rpc.methods` → 238 methods (was 235), 6 AEAD methods (was 4), `auth.sign` present
   - `auth.sign` → signed test payload, Ed25519 signature returned
   - `crypto.aead.chacha20_poly1305.encrypt` → encrypt + decrypt roundtrip successful
4. **Gossip test** — swarmVine on eastGate is pre-topic-fix binary (Aug 9). Gossip injection hangs. Not a bearDog issue — needs sporeGate depot rebuild.
5. **Deep-debt scan**:
   - 0 clippy warnings (fixed 2 doc_markdown warnings in auth.sign docs)
   - 0 TODO/FIXME/HACK
   - 0 production files > 800 LOC (2 files at 802/808 — tests push them over, production code under threshold)
   - All `#[allow]` carry `reason`
6. **Clippy fix committed and pushed** — `4b19be3483`.

## What Went Well

- **Fast diagnosis** — both gaps were clear from reading the graph TOMLs (`rootpulse_harvest.toml`, `rootpulse_commit.toml`) and the method list vs router comparison.
- **Existing infrastructure leveraged** — `auth.sign` reuses `sign_with_primal_identity()` from `primal_signing.rs`. AEAD routing reuses existing `handle_chacha20_poly1305_encrypt/decrypt` handlers.
- **Clean deploy** — biomeOS supervision auto-restarted bearDog after binary swap. Zero downtime aside from the swap moment.
- **Live validation confirms wire-level correctness** — both features work end-to-end over UDS.

## What Could Be Better

- **Depot binary staleness** — the running bearDog on eastGate was 5 days old (Aug 9). The gap between code push and depot deployment is a recurring theme. sporeGate autonomous dispatch will help, but local gates should have a lighter-weight "pull and restart" mechanism.
- **swarmVine not rebuilt** — gossip connectivity can't be validated because the depot swarmVine binary predates the topic fix. Cross-primal validation is blocked on depot rebuilds.
- **Method count discrepancy** — `rpc.methods` reports 238 but STATUS.md says 239 dispatchable. The difference is because `auth.sign` is counted in both the SecurityHandler methods list AND the gate announce methods — but `rpc.methods` deduplicates. The 239 figure counts registry (226) + gate (13) before dedup. Both are correct from different perspectives.

## Metrics

| Metric | Before | After |
|--------|--------|-------|
| CryptoHandler methods | 114 | 116 |
| SecurityHandler methods | 18 | 19 |
| Gate announce methods | 7 | 8 |
| `rpc.methods` (live, deduped) | 235 | 238 |
| beardog-tunnel tests | 2,481 | 2,483 |
| Clippy warnings | 0 | 0 |
| Depot binary date | Aug 9 | Aug 14 |

## Commits

| Hash | Description |
|------|-------------|
| `ffa5a7fab9` | Wave 157k: AEAD Neural API routing + rootPulse auth.sign step handler |
| `4b19be3483` | fix: doc_markdown clippy warnings in auth.sign doc comments |

## Status

**All bearDog code-team items CLOSED.** Primal is in enmeshment-ready posture. No remaining work items from blurb. Next bearDog work will come from upstream cascade or new blurb assignments.

### Upstream remaining (not bearDog-owned)

- rootPulse step handlers: nestGate + loamSpine (now 3/5 with bearDog done)
- `content.put` translation: biomeOS action (not nestGate, not bearDog)
- D12/D13 merge: biomeOS team
- blueGate depot rebuild: sporeGate
- swarmVine depot rebuild: sporeGate (blocks gossip validation on eastGate)
