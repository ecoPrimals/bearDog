# Handoff — Wave 155j: crypto.sign_ed25519 Direct Key Signing

**Date**: Jul 29, 2026  
**From**: bearDog (eastGate)  
**To**: overwatch, loamSpine, sporeGate

---

## What Changed

`crypto.sign_ed25519` now accepts `secret_key` (base64, 32-byte Ed25519 seed) directly.
Previously only supported `key_id`-based derivation. This unblocks loamSpine
`entry.append` for Provenance Trio 7/7.

## Method Contract

```
Method: crypto.sign_ed25519
        (also: crypto.sign, crypto.ed25519.sign, beardog.crypto.sign_ed25519,
         capability.call → {crypto, sign_ed25519})

Input (Mode 1 — Direct Key):
  message:    string (base64)  — required
  secret_key: string (base64)  — 32-byte Ed25519 seed

Input (Mode 2 — Derived Key):
  message:    string (base64)  — required
  key_id:     string           — optional (default: "default_signing_key")
  purpose:    string           — optional (default: "general")

Output:
  signature:  string (base64)  — 64-byte Ed25519 signature
  algorithm:  "Ed25519"
  public_key: string (base64)  — 32-byte Ed25519 public key
  key_id:     string           — only present in Mode 2
```

## Deployment Note

westGate bearDog binary is stale (shows 11,993 tests; current: 14,019).
**Rebuild from `main` and redeploy** to pick up this fix and all Wave 155 improvements.

## bearDog Status

| Metric | Value |
|--------|-------|
| Tests | 14,019 |
| Clippy | 0 |
| Crates | 27 |
| JSON-RPC methods | 236 |
| P0/P1 remaining | 0 |

bearDog has no remaining P0/P1 items. Awaiting:
- crates.io token for G6 public flip (overwatch)
- songBird ACME absorption (songBird)
- sporeGate depot refresh (sporeGate)
- loamSpine verification of Provenance 7/7 (loamSpine)

---

*Wave 155j handoff — bearDog signing is live.*
