# Wave 153c — iosGate Deployment Handoff

**Date:** July 26, 2026

## What Was Done

The iPhone XS has been provisioned as `iosGate` and the full bearDog stack
cross-compiles cleanly for `aarch64-apple-ios`. The Secure Enclave integration
uses real Security.framework calls (P-256 keygen, ECDSA sign/verify) and the
IPC layer uses Unix domain sockets within the iOS app sandbox.

## Current State

| Component | Status |
|-----------|--------|
| Cross-compilation | Working (cargo cross build) |
| Device pairing | Paired (iOS 18.5, UDID captured) |
| IPA packaging | Built (18MB debug) |
| Code signing | Blocked on Apple Developer cert |
| Secure Enclave | Wired to Security.framework |
| IPC transport | Unix sockets in sandbox |
| Gate registration | Complete (iosGate.toml) |

## How to Complete Deployment

Once Apple Developer account is active:

```bash
# 1. Generate certificate + profile via developer.apple.com:
#    - Certificates → iOS Development → upload CSR
#    - Devices → register UDID 00008020-001E68D00A3A002E
#    - Profiles → iOS App Development → select cert + device + app ID

# 2. Export signing materials:
export BEARDOG_IOS_KEY=/path/to/private-key.pem
export BEARDOG_IOS_CERT=/path/to/certificate.pem
export BEARDOG_IOS_PROVISION=/path/to/profile.mobileprovision

# 3. Build, sign, and install in one command:
./ios/build-ipa.sh
```

Or manually:

```bash
zsign -k $BEARDOG_IOS_KEY -c $BEARDOG_IOS_CERT -m $BEARDOG_IOS_PROVISION \
      -b org.ecoprimals.beardog -e ios/beardog-ios/Entitlements.plist \
      -o target/ios-build/beardog-signed.ipa target/ios-build/beardog.ipa
ideviceinstaller --install target/ios-build/beardog-signed.ipa
```

## Files Changed

- `crates/beardog-tunnel/src/tunnel/hsm/ios_secure_enclave/safe_secure_enclave.rs` — Real SE ops
- `crates/beardog-tunnel/src/platform/ios.rs` — Unix socket IPC
- `crates/beardog-tunnel/src/platform/mod.rs` — iOS endpoint returns Filesystem
- `infra/gates/iosGate.toml` — Real device info
- `ios/build-ipa.sh` — Automated build + sign script
- `ios/beardog-ios/Info.plist` — iOS app metadata
- `ios/beardog-ios/Entitlements.plist` — iOS entitlements

## Remaining Work

1. Sign and deploy IPA (requires Apple Developer cert)
2. Validate Secure Enclave keygen on physical device
3. Validate Unix socket IPC within real iOS sandbox
4. Wire NFC FIDO2 passthrough via CoreNFC
5. Connect entropy orchestrator to Secure Enclave
