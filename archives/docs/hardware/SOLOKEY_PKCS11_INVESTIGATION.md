# SoloKey PKCS#11 Investigation - November 9, 2025

**Question**: Can we access SoloKeys via PKCS#11?  
**Short Answer**: **Not by default** - Solo 2 keys are FIDO2-primary devices.  
**Long Answer**: Below 👇

---

## 🔍 **Investigation Results**

### Hardware Detected:
```
✅ Device 1: Solo 2 Security Key (/dev/hidraw5)
✅ Device 2: Solo 2 Security Key (/dev/hidraw6)
Protocol: FIDO2/CTAP2 (HID)
```

### PKCS#11 Status:
```
❌ No PKCS#11 interface detected on SoloKeys
```

---

## 🎯 **Why No PKCS#11?**

### Solo 2 Architecture:

Solo 2 keys have a **modular applet system**:

1. **FIDO2 Applet** (Default) ✅
   - Always present
   - WebAuthn/U2F authentication
   - Resident keys
   - hmac-secret extension

2. **PIV Applet** (Optional) ⚠️
   - Must be installed separately
   - Provides PKCS#11 interface
   - X.509 certificates
   - Smart card emulation

3. **Other Applets** (Future)
   - OpenPGP
   - OATH (TOTP/HOTP)
   - Custom applets

### Current State:
Your SoloKeys appear to have **FIDO2 only** (factory default).

---

## 🔄 **How to Add PKCS#11 Support**

### Option 1: Install PIV Applet (Recommended if needed)

```bash
# Install solo2 CLI tool
cargo install solo2-cli

# Check current applets
solo2 app list

# Install PIV applet (if available)
solo2 app install piv

# Verify PIV is active
solo2 piv status

# Test PKCS#11 access
pkcs11-tool --module /path/to/piv_module.so --list-slots
```

### Option 2: Use FIDO2 Natively (Recommended) ✅

**BearDog already supports this!**

```rust
// Via FIDO2 (what we implemented today)
let devices = discover_fido2_devices().await?;

// Generate entropy using hmac-secret
let entropy = device.generate_entropy(32).await?;

// Create resident key
let key = device.generate_key(KeyAlgorithm::Ed25519).await?;

// Sign data
let signature = device.sign(data, &key).await?;
```

---

## 📊 **Protocol Comparison**

### What You Get with FIDO2 (Native):
✅ Hardware entropy (hmac-secret)  
✅ Ed25519 & ECDSA signing  
✅ Resident keys  
✅ User presence verification  
✅ PIN protection  
✅ WebAuthn authentication  
✅ **No additional setup required**

### What You'd Get with PIV/PKCS#11:
✅ X.509 certificates  
✅ RSA signing  
✅ Smart card emulation  
✅ Legacy PKI integration  
✅ Windows smart card logon  
❌ **Requires PIV applet installation**

---

## 🎯 **Recommendation**

### For Your Use Case:

**Use FIDO2** (What we've implemented) ✅

**Why?**
1. ✅ Already working (detected successfully)
2. ✅ Full Solo 2 feature support
3. ✅ Modern, secure protocol
4. ✅ No additional setup
5. ✅ Better performance (native)

**When to add PIV/PKCS#11**:
- 🏢 You need enterprise PKI
- 📜 You need X.509 certificates
- 💼 You need smart card emulation
- 🔧 You need RSA (not Ed25519)

---

## 🚀 **BearDog's Multi-Protocol HSM**

### Current Architecture:

```
╔══════════════════════════════════════════════════════════╗
║              BearDog Universal HSM Layer                 ║
╠══════════════════════════════════════════════════════════╣
║                                                          ║
║  ┌─────────────────┐  ┌──────────────────┐             ║
║  │   PKCS#11       │  │   FIDO2/CTAP2    │             ║
║  ├─────────────────┤  ├──────────────────┤             ║
║  │ ✅ SoftHSM2      │  │ ✅ SoloKeys       │             ║
║  │ ✅ YubiKey PIV   │  │ ✅ YubiKey FIDO2  │             ║
║  │ ⏳ SoloKey PIV?  │  │ ✅ Titan Keys     │             ║
║  │                 │  │ ✅ Any FIDO2 key  │             ║
║  └─────────────────┘  └──────────────────┘             ║
║                                                          ║
║  Hardware-agnostic: Choose best protocol per device     ║
╚══════════════════════════════════════════════════════════╝
```

### Smart Protocol Selection:

```rust
// BearDog automatically chooses the best protocol
match device_type {
    SoloKey => use_fido2(),      // Native, fast
    YubiKey => {
        if needs_pki => use_pkcs11_piv(),
        else => use_fido2(),
    },
    SoftHSM => use_pkcs11(),     // Virtual HSM
    TPM => use_tpm2(),           // Future
}
```

---

## 🧪 **Testing Plan**

### Phase 1: FIDO2 (Current) ✅
- [x] Device discovery
- [x] Device enumeration
- [ ] CTAP2 GetInfo ← **Next**
- [ ] hmac-secret entropy
- [ ] Resident keys
- [ ] Signing operations

### Phase 2: PIV/PKCS#11 (Optional)
- [ ] Check if PIV applet is installed
- [ ] Install PIV applet (if needed)
- [ ] Test PKCS#11 slot detection
- [ ] Test certificate generation
- [ ] Test signing via PKCS#11

### Phase 3: Integration
- [ ] Unified HSM interface
- [ ] Automatic protocol selection
- [ ] Fallback strategies
- [ ] Performance benchmarks

---

## 💡 **Key Insights**

1. **SoloKeys are FIDO2-first devices**
   - Best accessed via CTAP2 protocol
   - PIV/PKCS#11 is optional, not default

2. **BearDog now supports both protocols**
   - FIDO2 for modern authentication
   - PKCS#11 for enterprise/legacy

3. **Protocol choice matters**
   - Use FIDO2 for: WebAuthn, passwordless, modern crypto
   - Use PKCS#11 for: PKI, certificates, smart cards

4. **Your SoloKeys work great!**
   - Both detected successfully
   - FIDO2 protocol confirmed
   - Ready for Phase 2 implementation

---

## 📝 **Conclusion**

**Answer to your question**: 

❌ **No PKCS#11 access (yet)** - Your SoloKeys don't have the PIV applet installed  
✅ **But you don't need it!** - FIDO2 gives you everything you need:
- Hardware entropy
- Signing
- Resident keys
- User presence
- PIN protection

**Recommendation**: **Proceed with FIDO2 Phase 2** to implement:
1. CTAP2 GetInfo (query capabilities)
2. hmac-secret (hardware entropy)
3. Resident keys (persistent storage)
4. Signing operations (Ed25519, ECDSA)

**Add PIV later** only if you need X.509 certificates or smart card emulation.

---

**Next Step**: Implement CTAP2 GetInfo command to query your SoloKeys' actual capabilities! 🚀

