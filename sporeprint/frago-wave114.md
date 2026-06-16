+++
title = "bearDog FRAGO — Wave 114 Response"
description = "Fragmentary order: mito-beacon acceptance implemented. Mode-detection race resolved."
date = 2026-06-16

[taxonomies]
primals = ["beardog"]
springs = ["primalSpring"]
+++

## FRAGO: bearDog Wave 114 — Mito-Beacon Acceptance SHIPPED

**From**: southGate (bearDog owner)
**To**: primalSpring overwatch / eastGate
**Date**: Jun 16, 2026
**Re**: Genetics-layer wiring — bearDog mito-beacon acceptance complete

---

### Overwatch Assessment (ADDRESSED)

Wave 114 lists bearDog as: "**HAS CODE** — debug mode-detection race"

**Root cause identified and fixed.** The mito-beacon signal `0xED` was correctly
detected by riboCipher signal detection but then immediately rejected with
"mito-tier not yet implemented — closing connection". From NUCLEUS launcher's
perspective, bearDog detected the mode but raced to close it.

---

### Fix: HMAC-Based Mito-Beacon Decode

| Component | What |
|-----------|------|
| `ribocipher.rs` | Added `mito_tag()`, `mito_signal()`, `decode_mito_tag()` |
| Scheme | `HMAC-SHA256(family_seed, &[protocol_type])` truncated to 4 bytes |
| Decode | Iterate known protocol types, compare HMAC tags, route on match |
| UDS server | `connection_handlers.rs` — mito routes to NDJSON/BTSP/probe/HTTP |
| TCP server | `connection.rs` — identical mito decode + routing |
| Health socket | Already tolerant (consumes signal prefix, no HMAC needed) |

### Verification

- 7 new unit tests: determinism, per-protocol differentiation, per-seed differentiation,
  round-trip all 8 known protocols, wrong-seed rejection, garbage rejection
- 2,332 tunnel lib tests passing (2 pre-existing platform-only failures)
- 414 CLI lib tests passing
- Clean workspace `cargo check`

### Corrected Compliance Table

```
bearDog riboCipher clear (0xEC):   ✅ (Wave 111)
bearDog riboCipher mito (0xED):    ✅ (Wave 114) — HMAC decode + route
bearDog riboCipher nuclear (0xEE): ⏳ (Wave 115+ — per-peer encrypted)
bearDog health socket:             ✅ (Wave 113 — beardog-default.sock)
bearDog health method:             ✅ (7 aliases → {status, primal, version})
bearDog legacy ERROR:              ✅ (Wave 112 — unsignalled → error!)
```

**bearDog should now pass NUCLEUS mito-beacon probes.** Rebuild from HEAD on VPS.
