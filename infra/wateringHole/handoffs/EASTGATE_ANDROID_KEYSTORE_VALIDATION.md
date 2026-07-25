# Handoff: eastGate Android Keystore Validation

**From**: bearDog (flockGate)
**To**: primalSpring / eastGate team (hardware access)
**Date**: July 25, 2026
**Wave**: 150x
**Priority**: P2 (last remaining bearDog HSM backend)
**Note**: flockGate has no Android hardware. The primalSpring team on eastGate
has access to grapheneGate (Pixel 8a, Tensor G3) and can run testing and
validation on our behalf.

---

## Context

bearDog has shipped all platform-specific HSM backends except Android Keystore
hardware validation. As of Wave 150u, both the HSM layer and the CredentialStore
layer have Android backends implemented:

| Backend | Layer | Status | Platform |
|---------|-------|--------|----------|
| RustSoftwareHsm | HSM | SHIPPED | All platforms |
| LinuxSecretServiceHsm | HSM | SHIPPED (Wave 145a) | Linux (D-Bus) |
| WindowsDpapiHsm | HSM | SHIPPED (Wave 145a) | Windows |
| **AndroidStrongBoxHsm** | HSM | **Code complete — needs hardware validation** | Android |
| InMemoryCredentialStore | CredentialStore | SHIPPED (Wave 150t) | All platforms |
| FileVaultCredentialStore | CredentialStore | SHIPPED (Wave 150t) | All platforms |
| **AndroidKeystoreCredentialStore** | CredentialStore | **Code complete — needs hardware validation** | Android |

### HSM Layer (HsmKeyProvider trait)

The Android StrongBox HSM backend code exists in bearDog
(`crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/`) with:
- `AndroidStrongBoxHsm` struct and `HsmKeyProvider` trait impl
- `UnwiredAndroidKeystoreTransport` / `MemoryKeystoreTransport` stubs
- Safe JNI wrapper (`safe_native_wrapper.rs`)
- Safe device detection (`safe_device_detection.rs`)
- Multi-credential provider (`beardog-security`)

### CredentialStore Layer (Wave 150u)

The Android Keystore credential store backend is at
`crates/beardog-tunnel/src/credential_store/android_keystore.rs`:
- `AndroidKeystoreCredentialStore` implementing `CredentialStore` trait
- `CredentialStoreBackend::AndroidKeystore` enum variant (Silicon Atheism dispatch)
- Master key derived from TEE/StrongBox via `beardog-credstore-v1` alias
- File vault encryption (ChaCha20-Poly1305 + HKDF) with hardware-bound master key
- Full test coverage on non-Android (availability probe, construction error, metadata)

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

### HSM Layer (HsmKeyProvider)

| # | Check | Method | Expected |
|---|-------|--------|----------|
| 1 | bearDog starts on aarch64 | `beardog server --listen 127.0.0.1:9100` | Clean startup, HSM registry discovers StrongBox |
| 2 | HSM registry includes AndroidStrongBox | `primal.info` → check providers | `android-strongbox` in provider list |
| 3 | Ed25519 keypair generation | `crypto.ed25519_generate_keypair` | Returns `public_key` field |
| 4 | Sign/verify roundtrip | `crypto.sign_ed25519` + `crypto.verify_ed25519` | `valid: true` |
| 5 | AES-256-GCM encrypt/decrypt | `crypto.aes256_gcm_encrypt` + `decrypt` | Plaintext matches |
| 6 | StrongBox capability probe | HSM trace logs | `StrongBox available: true` |
| 7 | Device attestation chain | `beardog.fido2.discover` (if wired) | Valid attestation |

### CredentialStore Layer (secrets.*)

| # | Check | Method | Expected |
|---|-------|--------|----------|
| 8 | Store a secret | `secrets.store({"name":"test","value":"hello"})` | `stored: true` |
| 9 | Retrieve secret | `secrets.retrieve({"name":"test"})` | `value: "hello"` |
| 10 | List secrets | `secrets.list({})` | `["test"]` in list |
| 11 | Delete secret | `secrets.delete({"name":"test"})` | `deleted: true` |
| 12 | Persistence across restart | Store → restart bearDog → retrieve | Value survives restart |
| 13 | Backend reports keystore | `primal.info` or trace logs | `backend_id: "android-keystore"` |

---

## Code Pointers

### HSM Layer

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

### CredentialStore Layer (Wave 150u)

| File | What |
|------|------|
| `crates/beardog-traits/src/unified/storage.rs` | `CredentialStore` trait + `SecretMetadata` |
| `crates/beardog-tunnel/src/credential_store/android_keystore.rs` | `AndroidKeystoreCredentialStore` impl |
| `crates/beardog-tunnel/src/credential_store/backend.rs` | `CredentialStoreBackend::AndroidKeystore` dispatch |
| `crates/beardog-tunnel/src/credential_store/mod.rs` | Module root + re-exports |
| `crates/beardog-tunnel/src/unix_socket_ipc/handlers/secrets.rs` | `secrets.*` JSON-RPC handlers |

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
