# Handoff — Wave 155k: Windows Platform Gating

**Date**: July 30, 2026
**From**: bearDog team (eastGate)
**To**: overwatch, sporeGate depot

---

## Summary

All `UnixStream`/`UnixListener` usage in bearDog production code is now gated behind
`#[cfg(unix)]`. The Windows cross-compile target (`x86_64-pc-windows-gnu`) passes with
zero errors. The Windows depot blocker for `beardog.exe` is resolved.

## What Changed

- 10 files across 5 crates received platform gates
- Windows DPAPI HSM backend: `windows-sys` dependency replaced with direct `extern "system"` FFI
- Workspace `unsafe_code` lint: `forbid` → `deny` (allows targeted `#[allow]` on DPAPI + libtower)
- `modes/client.rs`: `ClientStream` enum for cross-platform sync transport

## For sporeGate Depot

`beardog.exe` can now be built for Windows:
```bash
cargo build --release --target x86_64-pc-windows-gnu -p beardog-cli
```

Windows binary uses TCP-only mode. Health probes, ecosystem discovery, and IPC are all
TCP-based on Windows. No Unix socket functionality is available or expected.

## bearDog Status

| Metric | Value |
|--------|-------|
| Version | 0.9.0 |
| Tests | 14,019 |
| Clippy | 0 warnings |
| P0/P1 | **ZERO** |
| Posture | **STANDBY** |

Both P1 items from Wave 155j blurb are complete:
1. `crypto.sign_ed25519` direct key signing (Provenance 7/7 unblocked)
2. Windows platform gating (`beardog.exe` in depot)

## Resume When

- biomeOS composition lifecycle needs bearDog participation
- Provenance Trio 7/7 E2E validation (sweetGrass + loamSpine)
- Second NUCLEUS on westGate
