# AAR: Wave 153c — iosGate Deployment

**Date:** July 26, 2026
**Status:** Partially Complete (blocked on Apple Developer certificate)

## Accomplished

### Toolchain & Infrastructure
- Installed `cargo-cross` for iOS cross-compilation from Linux
- Built and installed `zsign` from source for code signing
- Installed `libimobiledevice` tools (`idevicepair`, `ideviceinfo`, `ideviceinstaller`)
- Installed `pymobiledevice3` for advanced device management

### Device Pairing
- iPhone XS (iPhone11,2, iOS 18.5) paired to eastGate over USB
- UDID, serial, and hardware model captured
- Device registered in `infra/gates/iosGate.toml` with real hardware details

### Cross-Compilation
- Fixed five compilation errors for `aarch64-apple-ios`:
  - Duplicate `default_socket_endpoint` resolution via `cfg` exclusions
  - `SocketEndpoint::XPC` exhaustiveness in `android.rs` and `unix.rs`
  - Mutability and async issues in `biometric.rs`
  - Unused import cleanup in `ios.rs` and `mod.rs`
- Both host (`x86_64-unknown-linux-gnu`) and iOS (`aarch64-apple-ios`) builds verified clean

### iOS App Shell
- Created `Info.plist` with bundle ID, NFC, and Face ID usage descriptions
- Created `Entitlements.plist` with Secure Enclave, NFC FIDO2, and app group entitlements
- Created `build-ipa.sh` automation script with env-var-driven signing flow
- Generated unsigned IPA (18MB debug build)

### Secure Enclave (Production Wire-Up)
- Evolved `SafeSecureEnclave` from software-derived Ed25519 fallback to real
  Security.framework P-256 operations via `security-framework` crate
- On iOS: `SecKeyCreateRandomKey` with Secure Enclave token, ECDSA-SHA256
  signing, keychain-based key lookup and verification
- On non-iOS: Fail-closed `Unavailable` errors (Silicon Atheism principle)
- Private keys never leave the Secure Enclave hardware

### IPC Transport (Unix Domain Sockets)
- Evolved iOS transport from unimplemented XPC stubs to working Unix domain
  sockets within the iOS app sandbox (`NSTemporaryDirectory()/biomeos/`)
- `create_endpoint` now returns `SocketEndpoint::Filesystem` on iOS
- `bind` now creates real `UnixListener` on iOS
- `default_socket_endpoint()` updated correspondingly
- All tests updated to match new Filesystem-based transport

## Blocked

### Code Signing
- iOS 18 strictly requires Apple-issued certificates
- Ad-hoc signing (`zsign -a`) produces valid Mach-O signatures but iOS rejects
  them at `VerifyingApplication` stage (error `0xe8008014`)
- Apple Developer account sign-up in progress
- Once active: `zsign -k key.pem -c cert.pem -m profile.mobileprovision`

## Follow-Up

1. Complete Apple Developer enrollment → generate cert + profile → sign → deploy
2. Validate Secure Enclave keygen on physical device
3. Validate Unix socket IPC within real iOS sandbox
4. Run bearDog daemon on iosGate with FIDO2 NFC passthrough
5. Wire entropy orchestrator to Secure Enclave as hardware entropy source
