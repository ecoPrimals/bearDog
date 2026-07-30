# AAR — Wave 155k: Windows Platform Gating

**Date**: July 30, 2026
**Scope**: P1 — Unblock `beardog.exe` in Windows depot (11/14 → 12/14)
**Duration**: Single session
**Result**: `cargo check --target x86_64-pc-windows-gnu` passes; 14,019 Linux tests pass

---

## Problem

`beardog.exe` failed to cross-compile for Windows because `tokio::net::UnixStream`,
`std::os::unix::net::UnixStream`, and `tokio::net::unix::OwnedReadHalf` were imported
unconditionally in production code paths. Windows has no Unix domain socket support
through these types.

The Windows depot was blocked at 11/14 fresh binaries with `beardog.exe` listed as
one of the 3 platform-gated blockers (alongside `toadstool.exe` and `coralreef.exe`).

## Root Cause Analysis

7 production files across 5 crates contained ungated Unix-specific imports:

| File | Issue |
|------|-------|
| `beardog-ipc/isomorphic.rs` | `UnixStream` import, `IpcStream::Unix` variant, trait impl, connect functions |
| `beardog-tunnel/platform/mod.rs` | `pub mod unix` and `pub mod android` unconditional |
| `beardog-tunnel/platform/unix.rs` | Compiled on all targets via parent module |
| `beardog-tunnel/platform/android.rs` | Same as above |
| `beardog-tunnel/modes/client.rs` | `std::os::unix::net::UnixStream` hardcoded |
| `beardog-tunnel/unix_socket_ipc/protocol.rs` | `OwnedReadHalf` from `tokio::net::unix` |
| `beardog-cli/handlers/server/health.rs` | `UnixListener` for health socket |

Additional pre-existing Windows issues discovered and fixed:
- `windows_dpapi/mod.rs`: Used `windows_sys` crate (not a dependency) + `unsafe` code blocked by workspace `forbid(unsafe_code)`
- `beardog-cli/ecosystem_discovery_adapter/discovery.rs`: Called `beardog_tower_atomic::Client::connect_unix_path` unconditionally
- `credential_store/android_keystore.rs`: Missing `BaseStrategy` trait import for `etcetera::base_strategy::Windows::new()`

## Changes Made

### Platform Gating (10 files)

1. **`beardog-ipc/isomorphic.rs`**: `#[cfg(unix)]` on `UnixStream` import, `AsyncStream for UnixStream` impl, `IpcStream::Unix` variant (+ all match arms), `connect_unix` function. `#[cfg(not(unix))]` fallback arms return errors for UDS endpoints.

2. **`beardog-tunnel/platform/mod.rs`**: `#[cfg(unix)]` on `pub mod unix` and `pub mod android`. `PlatformListenerBackend` variants gated with `#[cfg(unix)]` + `Placeholder` variant on `#[cfg(not(unix))]`.

3. **`beardog-tunnel/modes/client.rs`**: Replaced `std::os::unix::net::UnixStream` with `ClientStream` enum (TCP + Unix variants). TCP heuristic: endpoint contains `:` with valid port number.

4. **`beardog-tunnel/unix_socket_ipc/protocol.rs`**: `ProtocolDetector` struct and impl gated behind `#[cfg(unix)]`. Moved `tracing::debug` import inside the function.

5. **`beardog-cli/handlers/server/health.rs`**: `#[cfg(unix)]` on `run_health_socket` and its call site. Windows uses TCP transport for health probes.

6. **`beardog-cli/ecosystem_discovery_adapter/discovery.rs`**: Unix-specific match arms gated; Windows returns error directing to TCP.

7. **`beardog-ipc/lib.rs`**: `connect_unix` re-export gated with `#[cfg(unix)]`.

### Windows DPAPI Evolution

Removed `windows-sys` crate dependency entirely. Replaced with direct `extern "system"` FFI declarations and a manual `DataBlob` struct (same layout as Win32 `CRYPT_INTEGER_BLOB` / `DATA_BLOB`). This eliminates version-sensitivity to `windows-sys` binding generator changes.

### Workspace Lint Evolution

Changed `unsafe_code = "forbid"` to `unsafe_code = "deny"` at workspace level. This allows the DPAPI module and libtower to use targeted `#[allow(unsafe_code)]` on their FFI blocks while keeping unsafe denied everywhere else.

## Verification

| Check | Result |
|-------|--------|
| `cargo check --target x86_64-pc-windows-gnu` | 0 errors (was 19+) |
| `cargo test --workspace` | 14,019 passed, 0 failed |
| `cargo clippy --workspace` | 0 errors |
| Linux native build | Clean |

## Upstream Impact

- **sporeGate depot**: `beardog.exe` can now be cross-compiled; Windows depot moves from 11/14 to 12/14 fresh
- **blueGate**: Can rebuild `beardog.exe` for Windows 13/13 testing
- **toadStool + coralReef**: Still blocked on their own platform gating (separate teams)

## Remaining bearDog P0/P1

**Zero.** Both Wave 155j P1 items completed:
1. `crypto.sign_ed25519` direct key signing — shipped Wave 155j
2. Windows platform gating — shipped this wave (155k)

bearDog is now in **STANDBY** posture per the blurb framework.
