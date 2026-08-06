<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Wave 156h Handoff — bearDog G64 Cephalization: tarpc Dual-Protocol

**Date**: Aug 5, 2026
**From**: eastGate (bearDog team)
**To**: overwatch, all gate teams

---

## What Changed

### G64 Cephalization — bearDog evolves from "tarpc-absent" to "tarpc-wired"

bearDog had explicitly removed tarpc in Wave 55 (Jan 2026) because tarpc 0.34 transitively pulled `async-trait` (banned in `deny.toml`). tarpc 0.37 no longer has this dependency, so re-integration is safe.

### Implementation

| Component | Details |
|-----------|---------|
| **Dep** | `tarpc = "0.37"` in workspace, `tarpc-rpc` feature in `beardog-tunnel` + `beardog-cli` |
| **Service trait** | `#[tarpc::service] trait BearDogRpc` — 7 methods |
| **Server** | `spawn_tarpc_listener()` binds `.tarpc.sock` sibling socket (bincode framing) |
| **Startup** | CLI server spawns tarpc listener alongside `MultiTransportServer` when `--features tarpc-rpc` + Unix |
| **Capabilities** | `capabilities.list` advertises `["json-rpc", "tarpc"]` when feature active |
| **Build impact** | Feature-gated — default build unchanged, zero new deps in default tree |

### BearDogRpc Service Methods

| Method | Args | Returns | Delegates to |
|--------|------|---------|--------------|
| `health_check` | — | `TarpcHealthStatus` | Identity + version |
| `blake3_hash` | `Vec<u8>` | `Vec<u8>` | `beardog_crypto::hash_blake3` |
| `sign_ed25519` | `key_id`, `message` | `SignResult` | KDF(FAMILY_SEED, key_id) → ed25519 |
| `verify_ed25519` | `public_key`, `message`, `signature` | `bool` | `beardog_crypto::verify_ed25519` |
| `sha256` | `Vec<u8>` | `Vec<u8>` | `beardog_crypto::hash_sha256` |
| `hmac_sha256` | `key`, `data` | `Vec<u8>` | `beardog_crypto::hmac_sha256` |
| `version` | — | `String` | `CARGO_PKG_VERSION` |

### Socket Convention

```text
beardog.sock              ← JSON-RPC 2.0 (always present, bootstrap + diagnostic)
beardog.tarpc.sock        ← tarpc bincode (feature-gated, high-perf intra-gate)
```

Follows `biomeos-primal-sdk::tarpc_transport::tarpc_socket_name()` convention.

### Performance Thesis

JSON-RPC crypto operations encode/decode base64 strings through `serde_json::Value`. tarpc bypasses this entirely — raw `Vec<u8>` over bincode binary framing. For high-frequency patterns (provenance braiding at 217/s, CAS BLAKE3 hashing), this eliminates the serde roundtrip bottleneck.

---

## Verification

- **14,019 tests**, 0 failures (default build)
- **2 tarpc-specific tests** pass (socket path convention)
- **0 Clippy errors** (pre-existing warnings only)
- **`cargo deny check`** — all 4 pass (no `async-trait`, no banned crates)
- **Default build unaffected** — tarpc is purely opt-in via `--features tarpc-rpc`

---

## Updated Cephalization Table

| tarpc State | Primals | Count |
|-------------|---------|-------|
| **tarpc-default** | coralReef, barraCuda, toadStool, nestGate, squirrel | 5 |
| **tarpc-wired** | songBird, sweetGrass, loamSpine, rhizoCrypt, petalTongue, biomeOS, **bearDog** | **7** |
| **tarpc dep only** | skunkBat, sourDough, bingoCube | 3 |

---

## For Upstream

- **Depot rebuild**: bearDog binary with `--features tarpc-rpc` will expose `.tarpc.sock`. Without the feature, behavior is identical to current.
- **loamSpine/nestGate consumers**: Can use `negotiate_protocol_from()` to discover bearDog's `.tarpc.sock` and escalate from JSON-RPC to tarpc for crypto hot-path.
- **biomeOS `SecurityRpc`**: bearDog's `BearDogRpc` covers the `sign`, `verify`, `health_check` surface. Full `SecurityRpc` trait alignment (e.g. `get_jwt_secret`, `verify_lineage`) can be added incrementally.
- **E1 (Neural API stub)**: Already shipped in Wave 156e commit `321218009`. Blurb was stale.
