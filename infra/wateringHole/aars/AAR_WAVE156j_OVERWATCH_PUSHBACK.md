<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# AAR: bearDog Status Corrections for Overwatch — Wave 156j

**Date**: August 6, 2026
**Author**: bearDog team (eastGate)
**Context**: Overwatch blurb 156j classifies bearDog as "tarpc dep only (not yet serving)" and lists E1 (Neural API routing stub) as open. Both are stale.

---

## Corrections

### 1. bearDog tarpc Status: "dep only" → **tarpc-wired + dual-socket (30 methods)**

The blurb table shows:

> | **tarpc dep only** (not yet serving) | bearDog, skunkBat, bingoCube | 3 |

**This is wrong for bearDog.** bearDog has been tarpc-wired since Wave 156h (Aug 5) and near-converged since Wave 156i (Aug 6):

| What | Evidence |
|------|----------|
| **30 tarpc RPC methods** | `TARPC_METHOD_COUNT = 30` in `types.rs` — full crypto + auth domains |
| **Dual-socket pattern** | `spawn_tarpc_listener()` creates `.tarpc.sock` alongside `.sock` |
| **Server startup wired** | `mod.rs` line 347: `spawn_tarpc_listener(&socket_path, identity)` |
| **Capabilities advertised** | `capabilities.list` returns `tarpc_methods: 30` + `protocols: ["json-rpc", "tarpc"]` |
| **E2E tests** | 3 tarpc roundtrip tests over real UDS (health, crypto, auth) |
| **Feature-gated** | `tarpc-rpc` feature — zero impact on default build |

**Correct classification**: bearDog should be in the **"tarpc-default + dual-socket"** row alongside songBird and petalTongue, not in "tarpc dep only."

### 2. E1 (Neural API Routing Stub): **Already DONE — Wave 156e**

The blurb lists:

> | **E1** | **bearDog Neural API routing stub** — register with Neural API for nestgate.io. | eastGate | bearDog | nestgate.io 11/12 |

**This was shipped in Wave 156e (Aug 5)**:
- `capability.register` domains expanded 4→8 (added `auth`, `btsp`, `bonding`, `secrets`, `relay`, `consent`)
- Crypto domain operations updated to 36 canonical dotted names
- `primal.announce` TCP path fixed for Neural API routing table
- Handoff: `WAVE156e_NEURAL_API_ROUTING_HANDOFF.md`
- Committed: `321218009 fix: Wave 156e — Neural API routing stub (E1 debt item)`

**E1 should be marked DONE.**

### 3. C2 (Dual-Socket Pattern): **Already DONE for bearDog — Wave 156h**

The blurb lists bearDog under C2 (dual-socket pattern to be done). bearDog shipped this in Wave 156h:
- `.tarpc.sock` sibling socket alongside `.sock`
- Bincode binary framing via `tarpc::serde_transport::unix`
- Spawned at server startup when `tarpc-rpc` feature + Unix platform
- Committed: `d9108a8a0 feat: Wave 156h — G64 Cephalization: tarpc dual-protocol integration`

**C2 should be marked DONE for bearDog.**

---

## Corrected bearDog Row for Overwatch Blurb

```
| **tarpc-default + dual-socket** | songBird (C1a DONE), petalTongue (C1b+C2 DONE),
|                                 | **bearDog** (156h dual-socket, 156i 30 methods),
|                                 | coralReef, barraCuda, toadStool, nestGate, squirrel | 8 |
```

## bearDog Current State Summary

| Metric | Value |
|--------|-------|
| Version | 0.9.0 |
| HEAD | `754297f` |
| Tests | 14,026 |
| Clippy warnings | 0 |
| JSON-RPC methods | 236 |
| tarpc methods | 30 (full crypto + auth) |
| Dual-socket | `.sock` + `.tarpc.sock` |
| E1 Neural API | DONE (Wave 156e) |
| C2 Dual-socket | DONE (Wave 156h) |
| grapheneGate | Validated (13/13 checks) |
| iOS Secure Enclave | Registered in HsmKeyProviderBackend |
| MobileHsmCapability | Trait shipped in beardog-traits |
| Platform targets | linux, android (validated), windows, ios, macos |

## What's Actually Remaining for bearDog

1. **C2 was already done** — no new work from this blurb
2. **E1 was already done** — no new work from this blurb
3. **Filesystem bind-mode bug on Android** — P2 (found during grapheneGate validation)
4. **iOS deployment** — blocked on Apple Developer cert (external)
5. **`Keystore2CliTransport` → JNI** — P3 for production mobile HSM
6. **tarpc convergence gap**: bearDog is at 30/236 methods on tarpc. Full parity would mean all 236 JSON-RPC methods also available via tarpc. This is a long-tail item, not a blocker.

---

*bearDog team requests overwatch update the primal mountain table to reflect shipped work. bearDog has been tarpc-wired + dual-socket since Aug 5–6 (Waves 156h–156i). E1 closed Aug 5 (Wave 156e). C2 closed Aug 5 (Wave 156h).*
