# Handoff: eastGate Android Keystore Validation

**From**: bearDog (flockGate)
**To**: eastGate team
**Date**: July 18, 2026
**Wave**: 149b+
**Priority**: P2 (last remaining bearDog HSM backend)

---

## Context

bearDog has shipped all platform-specific HSM backends except Android Keystore:

| Backend | Status | Platform |
|---------|--------|----------|
| RustSoftwareHsm | SHIPPED | All platforms |
| LinuxSecretServiceHsm | SHIPPED (Wave 145a) | Linux (D-Bus) |
| WindowsDpapiHsm | SHIPPED (Wave 145a) | Windows |
| **AndroidStrongBoxHsm** | **PENDING — needs hardware validation** | Android (Pixel 8) |

The Android Keystore backend code exists in bearDog
(`crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/`) with:
- `AndroidStrongBoxHsm` struct and `HsmKeyProvider` trait impl
- `UnwiredAndroidKeystoreTransport` / `MemoryKeystoreTransport` stubs
- Safe JNI wrapper (`safe_native_wrapper.rs`)
- Safe device detection (`safe_device_detection.rs`)
- Multi-credential provider (`beardog-security`)

**What's missing**: end-to-end validation on real Titan M2 / StrongBox
hardware. The software mock path works; we need to confirm:

1. JNI bridge initializes on GrapheneOS
2. StrongBox reports correct capabilities
3. Ed25519 key generation succeeds in hardware enclave
4. Sign/verify roundtrip via hardware-backed keys
5. Device attestation chain validates

---

## Validation Substrate

An agentReagents template is provided at:

```
primals/bearDog/infra/agentReagents/beardog-android-keystore-validation.yaml
```

This template provisions either:
- **Real Pixel 8**: ADB-driven validation from eastGate (preferred)
- **QEMU ARM64**: Emulated aarch64 VM for CI pre-screening

### Quick Start (Real Pixel 8)

```bash
# On eastGate with USB-attached Pixel 8:
adb devices   # confirm Pixel 8 visible

# Build bearDog for aarch64-linux-android
cargo build --target aarch64-linux-android --release

# Push binary + validation script
adb push target/aarch64-linux-android/release/beardog /data/local/tmp/
adb push infra/agentReagents/beardog-android-keystore-validate.sh /data/local/tmp/

# Run validation
adb shell "chmod +x /data/local/tmp/beardog-android-keystore-validate.sh && /data/local/tmp/beardog-android-keystore-validate.sh"
```

### Quick Start (benchScale ARM64 Lab)

```bash
# From canonical benchScale:
benchscale lab create \
    --topology ecoprimals-tower-2node \
    --reagent ../../primals/bearDog/infra/agentReagents/beardog-android-keystore-validation.yaml

benchscale exec node-1 -- /opt/beardog-android-keystore-validate.sh
```

---

## Validation Checklist

| # | Check | Method | Expected |
|---|-------|--------|----------|
| 1 | bearDog starts on aarch64 | `beardog server --listen 127.0.0.1:9100` | Clean startup, HSM registry discovers StrongBox |
| 2 | HSM registry includes AndroidStrongBox | `primal.info` → check providers | `android-strongbox` in provider list |
| 3 | Ed25519 keypair generation | `crypto.ed25519_generate_keypair` | Returns `public_key` field |
| 4 | Sign/verify roundtrip | `crypto.sign_ed25519` + `crypto.verify_ed25519` | `valid: true` |
| 5 | AES-256-GCM encrypt/decrypt | `crypto.aes256_gcm_encrypt` + `decrypt` | Plaintext matches |
| 6 | StrongBox capability probe | HSM trace logs | `StrongBox available: true` |
| 7 | Device attestation chain | `beardog.fido2.discover` (if wired) | Valid attestation |

---

## Code Pointers

| File | What |
|------|------|
| `crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/mod.rs` | Module root |
| `crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/core/hsm_key_provider.rs` | `HsmKeyProvider` trait impl |
| `crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/safe_native_wrapper.rs` | Safe JNI wrapper |
| `crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/safe_device_detection.rs` | StrongBox detection |
| `crates/beardog-tunnel/src/tunnel/hsm/types/android_transports.rs` | Transport stubs |
| `crates/beardog-tunnel/src/tunnel/hsm/providers/registry.rs` | Provider discovery |
| `crates/beardog-tunnel/src/tunnel/hsm/hsm_key_provider_backend.rs` | Enum dispatch |
| `crates/beardog-types/src/hsm/provider_types.rs` | `HsmProviderType::AndroidStrongBox` |

---

## Cross-Compile Notes

bearDog's `Cargo.toml` includes `aarch64-linux-android` as a recognized target.
The Android StrongBox module is gated behind `#[cfg(target_os = "android")]`.

```bash
# Install Android NDK target
rustup target add aarch64-linux-android

# Build (requires Android NDK in PATH or ANDROID_NDK_HOME set)
cargo build --target aarch64-linux-android --release
```

On non-Android targets, the module compiles out entirely — this is by design.
The `HsmProviderRegistry::discover()` will not attempt to register the
AndroidStrongBox provider on non-Android platforms.

---

## Expected Outcomes

**Success**: eastGate confirms StrongBox operations work on real Titan M2.
bearDog ships the Android Keystore backend (P2 complete, 4/4 platform HSMs).

**Partial**: JNI bridge works but StrongBox reports limited capabilities
(e.g., no Ed25519). We may need to fall back to ECDSA P-256 for hardware
keys and use software Ed25519 for operations StrongBox doesn't support.

**Blocked**: GrapheneOS restricts Keystore access in a way that prevents
the JNI bridge from initializing. We'd need to investigate alternative
Android keystore APIs or test on stock Android.

---

## Report Back

When validation is complete, update this handoff with results and push
via cascade. The bearDog agent on flockGate will pick up the results
in the next wave.
