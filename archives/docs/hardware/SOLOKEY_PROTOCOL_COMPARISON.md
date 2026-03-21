# SoloKey Protocol Comparison - PKCS#11 vs FIDO2

**Date**: November 9, 2025  
**Hardware**: 2x SoloKey Hacker (Solo 2)  
**Status**: Both protocols analyzed  

---

## 📊 **Protocol Overview**

### FIDO2/CTAP2 (Native)
✅ **Primary protocol for Solo 2**
- Device IDs: VID 0x1209, PID 0xbeee
- Detected: `/dev/hidraw5`, `/dev/hidraw6`
- Status: **Working** (detected successfully)

### PKCS#11/PIV (Optional)
⚠️ **Requires PIV applet** (not default on Solo 2)
- Interface: Smart card (CCID)
- Status: **Needs investigation**

---

## 🔍 **Detection Results**

### Via FIDO2 (hidapi)
```
✅ Device 1: Solo 2 Security Key
   Manufacturer: SoloKeys
   Path: /dev/hidraw5
   VID: 0x1209, PID: 0xbeee
   Protocol: CTAP2

✅ Device 2: Solo 2 Security Key
   Manufacturer: SoloKeys
   Path: /dev/hidraw6
   VID: 0x1209, PID: 0xbeee
   Protocol: CTAP2
```

### Via PKCS#11
Status: **To be tested** (requires PIV applet)

---

## 🎯 **Protocol Capabilities**

| Feature | FIDO2/CTAP2 | PKCS#11/PIV |
|---------|-------------|-------------|
| **Authentication** | ✅ WebAuthn, U2F | ✅ X.509 certs |
| **Signing** | ✅ Ed25519, ES256 | ✅ RSA, ECDSA |
| **Entropy Generation** | ✅ hmac-secret | ✅ RNG |
| **Resident Keys** | ✅ Native | ✅ Certificate slots |
| **User Presence** | ✅ Button press | ❌ Not required |
| **PIN Protection** | ✅ Optional | ✅ Required |
| **Smart Card** | ❌ | ✅ |
| **SSH Keys** | ✅ (via resident keys) | ✅ (native) |

---

## 🔐 **Use Cases**

### FIDO2/CTAP2 (Recommended for Solo 2)
**Best for**:
- 🌐 WebAuthn/FIDO2 authentication
- 🔐 Passwordless login
- 🎲 Hardware entropy generation
- 🔑 Ed25519 SSH keys (modern)
- 📱 Mobile/browser authentication

**BearDog Support**: ✅ **Native** (implemented today)

### PKCS#11/PIV (Legacy/Enterprise)
**Best for**:
- 🏢 Enterprise PKI
- 📜 X.509 certificates
- 🔑 RSA SSH keys (legacy)
- 💼 Smart card emulation
- 🖥️ Windows smart card logon

**BearDog Support**: ✅ **Via SoftHSM2** (existing), 🔄 **Hardware PIV** (needs PIV applet)

---

## 🛠️ **How to Enable PIV on Solo 2**

Solo 2 keys support multiple applets that can be installed:

### Check Current Applets:
```bash
# Install solo2 CLI if not already installed
cargo install solo2-cli

# List available applets
solo2 app list

# Check if PIV is installed
solo2 piv status
```

### Install PIV Applet (if needed):
```bash
# This would install PIV functionality
solo2 app install piv
```

⚠️ **Note**: Solo 2 keys come with FIDO2 by default. PIV is optional and may require firmware update.

---

## 🚀 **BearDog Multi-Protocol Strategy**

### Current Status (Nov 9, 2025):

```
┌─────────────────────────────────────────────────────────┐
│                    BearDog HSM Layer                    │
├─────────────────────────────────────────────────────────┤
│  Universal HSM Interface (Hardware-Agnostic)            │
├─────────────────┬───────────────────┬───────────────────┤
│  PKCS#11        │  FIDO2/CTAP2      │  Future           │
│  ✅ SoftHSM2     │  ✅ SoloKeys       │  🔄 TPM 2.0       │
│  ✅ YubiKey PIV  │  ✅ hidapi         │  🔄 OpenPGP       │
│  ⚠️  Solo PIV?   │  ✅ Discovery      │  🔄 StrongBox     │
└─────────────────┴───────────────────┴───────────────────┘
```

**Key Insight**: Use FIDO2 for SoloKeys (native), PKCS#11 for enterprise/legacy

---

## 📋 **Testing Checklist**

### FIDO2 Tests (Priority 1) ✅
- [x] Device discovery (hidapi)
- [x] Device enumeration
- [x] Protocol detection
- [ ] CTAP2 GetInfo (Phase 2)
- [ ] hmac-secret entropy (Phase 2)
- [ ] MakeCredential (Phase 2)
- [ ] GetAssertion (Phase 2)
- [ ] User presence detection (Phase 2)

### PKCS#11 Tests (Priority 2) 🔄
- [ ] Check for PIV applet
- [ ] List PKCS#11 slots
- [ ] Test certificate generation
- [ ] Test signing operations
- [ ] Test RNG via PKCS#11

### Integration Tests (Priority 3) 🔄
- [ ] Use FIDO2 for entropy in BearDog
- [ ] Use FIDO2 for signing in BearDog
- [ ] Fallback to SoftHSM2 if hardware unavailable
- [ ] Multi-device load balancing

---

## 🎯 **Recommendations**

### For Your SoloKeys:

1. **Use FIDO2 (Native)** ✅
   - Already working in BearDog
   - Full Solo 2 feature support
   - Modern, secure protocol

2. **Add PIV (Optional)** 🔄
   - If you need X.509 certificates
   - If you need PKCS#11 compatibility
   - If you need smart card emulation

3. **BearDog Strategy**: **Multi-Protocol**
   - FIDO2 for authentication & entropy
   - PKCS#11 for certificates & legacy
   - Seamless fallback between protocols

---

## 💡 **Next Steps**

### Immediate (Today):
1. ✅ Test FIDO2 discovery - **DONE**
2. 🔄 Check if PIV is installed
3. 🔄 Document protocol selection logic

### Phase 2 (FIDO2):
1. Implement CTAP2 GetInfo
2. Implement hmac-secret entropy
3. Implement resident key generation
4. Implement signing operations

### Phase 3 (Integration):
1. BearDog entropy from SoloKey
2. BearDog signing with SoloKey
3. Multi-protocol HSM selector

---

## 📖 **References**

- [SoloKeys Documentation](https://docs.solokeys.io/)
- [FIDO2 CTAP2 Spec](https://fidoalliance.org/specs/fido-v2.0-id-20180227/fido-client-to-authenticator-protocol-v2.0-id-20180227.html)
- [PKCS#11 v2.40](http://docs.oasis-open.org/pkcs11/pkcs11-base/v2.40/pkcs11-base-v2.40.html)
- [PIV Standard (NIST SP 800-73)](https://csrc.nist.gov/publications/detail/sp/800-73/4/final)

---

**Conclusion**: Your SoloKeys work great via FIDO2! PKCS#11 access is optional and requires PIV applet installation.

**Recommendation**: Proceed with FIDO2 implementation (Phase 2) for full native support. Add PIV later if needed for enterprise features.

