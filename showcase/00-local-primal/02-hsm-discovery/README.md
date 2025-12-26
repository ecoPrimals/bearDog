# 🔍 HSM Discovery - Find Your Hardware

**Level**: 0 (Local Primal)  
**Category**: Hardware Detection  
**Time**: 5 minutes  
**Dependencies**: None (hardware optional)

---

## 🎯 What This Demo Shows

Auto-discover available Hardware Security Modules (HSMs):
- ✅ Scan for YubiKey devices
- ✅ Check for TPM 2.0 availability
- ✅ Detect Android StrongBox (if on Android)
- ✅ Check iOS Secure Enclave (if on iOS)
- ✅ Software HSM fallback
- ✅ Compare capabilities

This demonstrates BearDog's **zero-knowledge** approach - it discovers what's available without hardcoded configuration.

---

## 🚀 Running the Demo

```bash
./run.sh
```

Or manually:
```bash
cargo run
```

---

## 📊 Expected Output

### With No Hardware (typical dev machine)
```
🔍 BearDog - HSM Discovery Demo
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Scanning for available HSM providers...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Provider: Software HSM
Status: ✅ Available
Type: Fallback (pure Rust)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Capabilities:
  ✓ AES-256-GCM encryption
  ✓ ChaCha20-Poly1305 encryption
  ✓ Ed25519 signatures
  ✓ ECDSA P-256 signatures
  ✓ SHA-256/512 hashing
  ✓ HKDF key derivation
Performance: Medium (software only)
Security Level: Development
Best For: Testing, development, CI/CD

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Provider: YubiKey (PKCS#11)
Status: ❌ Not Found
Reason: No YubiKey device connected
How to Fix: Insert YubiKey and ensure drivers installed

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Provider: TPM 2.0
Status: ❌ Not Available
Reason: /dev/tpm0 not found
How to Fix: Enable TPM in BIOS/UEFI

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Summary:
  Total Providers Found: 1
  Hardware HSMs: 0
  Software HSMs: 1
  
Recommendation: ✅ Software HSM ready for development
```

### With YubiKey Connected
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Provider: YubiKey 5 NFC
Status: ✅ Available
Type: Hardware FIPS 140-2 Level 2
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Device Info:
  Serial: 12345678
  Firmware: 5.4.3
  Slots Available: PIV (4 slots)
Capabilities:
  ✓ RSA-2048/4096 signatures
  ✓ ECDSA P-256/P-384 signatures
  ✓ X.509 certificates
  ✓ PIV attestation
Performance: Hardware accelerated
Security Level: Production (FIPS certified)
Best For: Production keys, certificates, code signing

Recommendation: 🏆 Use YubiKey for production keys
```

---

## 🧠 Understanding the Code

### HSM Discovery Process
```rust
// Scan for all available providers
let providers = discover_hsm_providers().await?;

for provider in providers {
    match provider.probe().await {
        Ok(info) => display_provider_info(info),
        Err(e) => display_unavailable(provider, e),
    }
}
```

### Universal HSM Pattern
BearDog uses a **universal adapter** pattern:
1. **Probe**: Check if HSM is available
2. **Capabilities**: Query what it can do
3. **Select**: Choose best provider for the task
4. **Use**: Uniform API across all HSMs

---

## 🎓 Key Concepts

### Hardware HSM Types

**YubiKey** (PKCS#11):
- USB/NFC security key
- FIPS 140-2 Level 2 certified
- 4 PIV slots
- RSA + ECDSA support
- Best for: Personal use, code signing

**TPM 2.0** (Trusted Platform Module):
- Built into modern laptops/desktops
- Hardware root of trust
- Platform attestation
- Best for: Device identity, disk encryption

**Android StrongBox**:
- Hardware-backed keystore
- Tamper-resistant chip
- Android 9+ requirement
- Best for: Mobile apps

**iOS Secure Enclave**:
- Dedicated crypto coprocessor
- Isolated from main CPU
- Face ID / Touch ID integration
- Best for: iOS apps

**Software HSM**:
- Pure Rust implementation
- Always available
- No hardware required
- Best for: Development, testing, CI/CD

### Zero-Knowledge Discovery

BearDog **never** hardcodes HSM paths or configuration:
- ❌ No `/usr/lib/libykcs11.so` hardcoded
- ❌ No `/dev/tpm0` assumptions
- ✅ Dynamic path discovery
- ✅ Capability-based selection
- ✅ Graceful fallback

This is **sovereignty** in action - adapt to your environment, not vendor requirements.

---

## 🚀 Next Steps

### Try with Real Hardware

**Get a YubiKey**:
```bash
# Install PKCS#11 library
sudo apt-get install ykcs11  # Debian/Ubuntu
brew install yubico-piv-tool  # macOS

# Run demo again
./run.sh
```

**Enable TPM**:
1. Enter BIOS/UEFI settings
2. Enable TPM 2.0
3. Install tpm2-tools
4. Run demo again

### Next Demos
1. **03-key-constraints** - Self-enforcing keys
2. **04-entropy-mixing** - Add your entropy
3. **01-hardware-integration/** - Deep dive into each HSM

---

## ❓ Troubleshooting

### "Permission denied" for YubiKey
```bash
# Add udev rules (Linux)
wget https://raw.githubusercontent.com/Yubico/yubikey-manager/main/resources/ykman.rules
sudo mv ykman.rules /etc/udev/rules.d/69-yubikey.rules
sudo udevadm control --reload-rules
```

### "TPM not found" but BIOS shows enabled
```bash
# Check TPM status
ls -la /dev/tpm*

# Install TPM tools
sudo apt-get install tpm2-tools
```

### Software HSM also fails
```bash
# This should never happen, but if it does:
RUST_LOG=debug cargo run
```

---

## 📚 Related Documentation

- **ZERO_HARDCODING_SPECIFICATION.md** - How discovery works
- **docs/hardware/** - Hardware HSM guides
- **01-hardware-integration/** - Per-HSM demos

---

**Demo Status**: ✅ Complete  
**Difficulty**: 🟢 Beginner  
**Time Required**: 5 minutes

🔍 **BearDog: Discover. Don't Hardcode.** 🔐

