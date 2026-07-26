# Wave 154 — HSM Agnostic Evolution Handoff

**Date:** July 26, 2026

## Summary

Full audit and evolution of the HSM layer across all platforms. Eliminated
iOS production stubs by wiring real Security.framework calls. Documented
the IPC transport reality: Unix domain sockets work on all platforms except
Windows (Named Pipes) and WASM (in-process). XPC on iOS is unnecessary.

## What Changed

| Component | Before | After |
|-----------|--------|-------|
| iOS Safe FFI | `not_yet_available` stubs | Delegates to real `SafeSecureEnclave` |
| iOS Secure Enclave | Orphaned prototype code | Clean module with real P-256/ECDSA ops |
| ios_secure_enclave module | Not in module tree | Properly declared in `hsm/mod.rs` |
| Health monitor | "probe not yet wired" log | Clean timestamp refresh |
| iOS IPC | XPC stub (unimplemented) | Unix domain sockets (working) |

## Provider Status Matrix

```
Platform    Provider              Backend          Status
─────────── ───────────────────── ──────────────── ────────────
Linux       RustSoftwareHsm       AES-GCM/Ed25519  Production
Linux       LinuxSecretServiceHsm libsecret        Production
Linux       Fido2HsmProvider      CTAP2/USB HID    Partial (entropy + IPC)
Android     AndroidStrongBoxHsm   keystore_cli_v2  Production
Windows     WindowsDpapiHsm       DPAPI            Production
iOS         SafeSecureEnclave     Security.fwk     Wired (needs registry)
All         SoloV2Provider        USB CTAP2        Feature-gated
```

## Remaining Work for Next Wave

1. Register `SafeSecureEnclave` as `HsmKeyProviderBackend::IosSecureEnclave` variant
   in `hsm_key_provider_backend.rs` so iOS participates in `discover()`
2. Consolidate `HsmProvider` (legacy) → `HsmKeyProvider` (canonical) migration
3. Wire health monitoring to call `is_available()` on registered providers
4. Remove `SocketEndpoint::XPC` variant (dead code since iOS now uses Filesystem)
5. Evolve legacy iOS Secure Enclave prototypes (`capability.rs`, `operations.rs`,
   `types.rs`) or archive them

## Key Architectural Finding for Upstream

**Unix domain sockets are the universal IPC primitive for ecoPrimals.**

The `PlatformSocket` trait + `SocketEndpoint` enum + compile-time `Socket` type
alias is architecturally sound. The only transport that fundamentally differs
is Windows Named Pipes. Everything else (Linux, macOS, Android, iOS) uses
some form of Unix socket — filesystem paths or abstract namespace.

XPC was investigated and determined unnecessary for bearDog's single-process
daemon model. It would only be needed for inter-app IPC with launchd-registered
services, which is an upstream ecosystem concern (not a primal concern).
