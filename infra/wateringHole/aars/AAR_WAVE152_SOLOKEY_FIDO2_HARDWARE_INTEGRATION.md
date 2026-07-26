<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# AAR: Wave 152 — SoloKey FIDO2 Hardware Integration + iosGate Prep

**Date**: July 26, 2026
**Scope**: End-to-end FIDO2/CTAP2 hardware path on eastGate, stack consolidation, hmac-secret, iosGate prep

---

## What We Did

### Phase 1: SoloKey Hardware Enablement
- Shipped `infra/udev/70-fido2.rules` covering Solo 2, YubiKey, Titan, Feitian, and generic FIDO2 CTAPHID usage page
- Fixed hidraw interface selection in `beardog-hid` to parse HID report descriptors and filter by usage page (0xF1D0), preventing CTAP2 commands on wrong U2F interface
- Validated SoloKey GetInfo on live hardware — real AAGUID, versions, extensions, algorithms returned

### Phase 2: Production Path Fixes
- Fixed PIN authentication in `SoloV2Provider` — implemented full ClientPIN protocol 1 (ECDH + AES-256-CBC + HMAC-SHA-256) instead of passing raw PIN bytes
- Wired `discovery.rs` to call real `ctap2_get_info()` for live capability probing instead of hardcoded defaults
- Validated all 5 FIDO2 IPC methods (discover/register/authenticate/entropy/ceremony) — 18 handler tests passing

### Phase 3: Stack Consolidation
- Wired `Fido2HsmProvider` to delegate to `SoloV2Provider`/`HidCtap2Transport` — HSM trait hierarchy now has real FIDO2 backend
- Wired entropy orchestrator to construct real `Fido2MultiCredentialProvider` instances from discovered devices, with async hardware entropy path
- Assessed dual CTAPHID transports: kept both (security: lightweight for GetInfo/entropy; tunnel: production-grade with CANCEL, timing, ceremony support)

### Phase 4: hmac-secret Extension
- Implemented `hmac_secret.rs` module — full `hmac-secret` CTAP2 extension encoding (ECDH, salt encryption, GetAssertion with extensions, response parsing from both FIDO 2.0 authData and FIDO 2.1 response map)
- AES-256-CBC roundtrip tests, HMAC truncation tests, authData parsing tests all passing

### Phase 5: iosGate Infrastructure
- Installed `aarch64-apple-ios` Rust target
- Verified `beardog-security` cross-compiles for iOS
- Created gate registrations: `eastGate.toml`, `grapheneGate.toml`, `iosGate.toml`
- Documented codesigning constraint (macOS host needed)

---

## What Went Well

1. **Existing CTAP2 infrastructure was solid** — client_pin module, CBOR encoding, HID transport all worked with minor fixes
2. **Usage page filtering** prevented the Solo 2 dual-interface issue cleanly
3. **iOS cross-compilation works from Linux** for pure Rust crates (no codesigning needed for compilation)
4. **Full test suite passes** — 1,161+ tests across all crates, zero failures with `--features fido2`

## What Surprised Us

1. **EAGAIN handling** was needed in the security crate's transport (already present in tunnel's)
2. **tokio `rt-multi-thread` not available** in `beardog-security` — required restructuring entropy orchestrator from sync `block_in_place` to separate sync/async paths
3. **Doc-test failures** from missing `#[tokio::main(flavor = "current_thread")]` — pre-existing but exposed by `--features fido2`

## Risks Going Forward

- **hmac-secret requires pre-existing credential** — no automatic credential creation; users must register a credential with `hmac-secret` extension first
- **iOS codesigning** requires macOS host — Linux-only cross-compile is compilation-only, no deployment without Apple toolchain
- **Two CTAPHID transports** remain — eventual extraction to `beardog-hid` will reduce maintenance

## Test Results

| Suite | Count | Status |
|-------|-------|--------|
| beardog-security (lib) | 1,102 | PASS |
| beardog-security (doc) | 13 | PASS |
| beardog-tunnel FIDO2 handlers | 18 | PASS |
| Full workspace (--features fido2) | ~1,161 | PASS |
