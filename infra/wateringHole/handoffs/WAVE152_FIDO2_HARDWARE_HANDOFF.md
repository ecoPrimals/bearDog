<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Wave 152 — FIDO2 Hardware Validation Handoff

**Date**: July 26, 2026
**From**: bearDog code team @ eastGate
**To**: Upstream overwatch / primal teams

---

## What's Wired

The FIDO2/CTAP2 hardware path is now end-to-end functional on eastGate with a SoloKey v2:

```
JSON-RPC IPC  →  SoloV2Provider  →  HidCtap2Transport  →  /dev/hidraw  →  SoloKey
```

### Completed

| Component | File(s) | Status |
|-----------|---------|--------|
| udev rules | `infra/udev/70-fido2.rules` | Shipped |
| HID usage page filter | `beardog-hid/src/types.rs`, `linux.rs` | Live |
| CTAP2 GetInfo | `beardog-security/fido2/ctap2/` | Validated on hardware |
| ClientPIN protocol 1 | `beardog-security/fido2/ctap2/client_pin.rs` | Full ECDH+AES+HMAC |
| Live discovery | `beardog-security/fido2/discovery.rs` | Real capability probing |
| IPC handlers | `beardog-tunnel/handlers/fido2/*.rs` | All 5 methods validated |
| Fido2HsmProvider | `beardog-security/fido2/provider.rs` | Wired to HID transport |
| Entropy orchestrator | `beardog-security/entropy_orchestrator/` | FIDO2 providers wired |
| hmac-secret | `beardog-security/fido2/ctap2/hmac_secret.rs` | Extension implemented |
| Gate registrations | `infra/gates/*.toml` | eastGate, grapheneGate, iosGate |

### Architecture

Two CTAPHID transports exist (by design):
1. **`beardog-security/fido2/ctap2/transport.rs`** — Lightweight, for GetInfo and HSM entropy
2. **`beardog-tunnel/solo_v2/hid_transport.rs`** — Production-grade with CANCEL, timing metadata, ceremony support

Both will eventually migrate to a shared `beardog-hid::ctaphid` module.

## What's Ready for Wave 153

- `aarch64-apple-ios` target installed
- `beardog-security` cross-compiles for iOS
- `iosGate.toml` registered (iPhone XS, A12 Bionic, Secure Enclave)
- Needs: macOS host for codesigning, Xcode + provisioning profile

## Remaining Phase 2 Work

The following CTAP2 operations in `Fido2MultiCredentialProvider` are architecturally ready but return `requires_capability` errors until wired:
- `ctap2_make_credential` — needs IPC path (`beardog.fido2.register`)
- `ctap2_get_assertion` — needs IPC path (`beardog.fido2.authenticate`)
- `ctap2_enumerate_credentials` — uses in-memory cache (device query planned)
- `ctap2_delete_credential` — not yet implemented

## Testing

```bash
# Build with FIDO2 support
cargo build --features fido2

# Run all tests
cargo test --features fido2

# Run FIDO2-specific tests
cargo test -p beardog-security -- fido2
cargo test -p beardog-tunnel -- handlers::fido2

# Live hardware smoke test (requires SoloKey plugged in)
cargo run --example test_solo2_with_button --features fido2
```
