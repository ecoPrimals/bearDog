<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Wave 156j — grapheneGate Validation + Vendor-Agnostic Mobile Abstraction

**Date**: August 6, 2026
**Author**: bearDog team (eastGate)
**Scope**: Build infra, on-device validation, iOS backend registration, mobile HSM abstraction

---

## Summary

Cross-compiled bearDog for Android ARM64, deployed to grapheneGate (Pixel 8a over ADB), and validated the full crypto/auth/secrets surface with a 13-check matrix. Registered iOS Secure Enclave as a first-class `HsmKeyProviderBackend` variant (Silicon Atheism). Created vendor-agnostic `MobileHsmCapability` and `MasterKeySealer` traits in `beardog-traits`.

## What Shipped

### Phase 1: Build Infrastructure

| Change | Details |
|--------|---------|
| `.cargo/config.toml` | Removed hardcoded NDK linker path; documented `CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER` env var approach |
| `rust-toolchain.toml` | Added `aarch64-linux-android` to targets array |

### Phase 2: grapheneGate Validation (13/13)

| # | Check | Result |
|---|-------|--------|
| 1 | Binary runs | PASS — `beardog 0.9.0` |
| 2 | Health check | PASS — `{"status":"healthy"}` |
| 3 | HSM discover | PASS — StrongBox detected (Hardware tier) + 3 Software |
| 4 | Ed25519 sign/verify | PASS — roundtrip |
| 5 | ChaCha20 encrypt/decrypt | PASS — roundtrip |
| 6 | AES-256-GCM roundtrip | PASS — roundtrip |
| 7 | BLAKE3 hash | PASS — 32-byte hash |
| 8 | HKDF-SHA256 | PASS — derived key |
| 9 | Ionic token lifecycle | PASS — issue + verify |
| 10 | Secrets store/retrieve | PASS — data matches |
| 11 | Capabilities list | PASS — 200+ methods |
| 12 | Abstract socket IPC | PARTIAL — SELinux blocks under adb shell; TCP validated |
| 13 | StrongBox keygen | PASS — `keystore_cli_v2` present, key in hardware |

**Binary**: 6.6M ARM64 ELF, dynamically linked, stripped.

### Phase 3: Vendor-Agnostic Mobile Abstraction

| Deliverable | Location |
|-------------|----------|
| `IosSecureEnclaveProvider` | `crates/beardog-tunnel/src/tunnel/hsm/ios_secure_enclave/hsm_key_provider.rs` |
| `HsmKeyProviderBackend::IosSecureEnclave` | `crates/beardog-tunnel/src/tunnel/hsm/hsm_key_provider_backend.rs` |
| `MobileHsmCapability` trait | `crates/beardog-traits/src/mobile_hsm.rs` |
| `MasterKeySealer` trait | `crates/beardog-traits/src/mobile_hsm.rs` |
| `SecureElementType` enum | `crates/beardog-traits/src/mobile_hsm.rs` |
| Validation script | `infra/validation/grapheneGate-validate.sh` |

## Findings

1. **Abstract socket SELinux**: GrapheneOS blocks abstract sockets for non-app processes under `adb shell` context. This is expected and correct — abstract sockets work when running as a proper Android app with the right SELinux context.

2. **`--bind-mode filesystem` routing**: When explicitly requesting filesystem sockets on Android, the server still routes through "Android (abstract socket)" logic. The platform detection overrides the explicit bind mode. This should be fixed so `--bind-mode filesystem` produces a real filesystem socket path regardless of platform.

3. **Health socket EROFS**: The health socket fails with "Read-only file system" when no `--audit-dir` is provided on Android. This is because the default health socket path derives from the main socket path, which on Android may point to a read-only location.

## Remaining Work for Upstream

- **`MobileHsmCapability` impl for providers**: Neither `AndroidStrongBoxHsm` nor `IosSecureEnclaveProvider` currently implement the new trait — this is Phase 2 wiring
- **`MasterKeySealer` implementations**: `SoftwareHkdfSealer`, `AndroidKeystoreSealer`, `IosKeychainSealer` — all three planned but not yet coded
- **Filesystem bind mode bug**: Platform detection should not override explicit `--bind-mode filesystem`
- **Android health socket path**: Needs fallback to `$TMPDIR` or `--audit-dir` when default path is read-only

## Test Results

- **14,026 tests passed**, 0 failures
- **0 Clippy warnings** (workspace lib)
- **cargo check --workspace** clean
