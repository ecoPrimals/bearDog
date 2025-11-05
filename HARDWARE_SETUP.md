# 🔐 BearDog Hardware Setup Guide

**Target Hardware:** Eastgate (i9-12900K) + 4x SoloKey V2 Hacker C  
**Status:** Phase 1 Complete - Ready for Testing  
**Updated:** October 29, 2025

---

## 🎯 **Quick Start**

Your CLI is ready! Here's how to use it with your SoloKeys:

```bash
cd /home/eastgate/Development/ecoPrimals/beardog

# 1. Discover your devices
./target/release/beardog discover-hsm

# 2. Test each device
./target/release/beardog test-entropy --slot 0

# 3. Mix entropy from all 4 devices
./target/release/beardog mix-seed \
  --slots 0,1,2,3 \
  --output my-seed.bin
```

---

## 📋 **Prerequisites**

### **System Packages:**
```bash
# Install PKCS#11 support
sudo apt update
sudo apt install opensc pcscd pcsc-tools

# Start the PC/SC daemon
sudo systemctl enable pcscd
sudo systemctl start pcscd

# Verify service is running
sudo systemctl status pcscd
```

### **Verify Hardware Detection:**
```bash
# Check if PC/SC sees your cards
pcsc_scan

# Should show your SoloKeys
# Press Ctrl+C to exit
```

---

## 🔌 **Hardware Setup**

### **Your Hardware:**
- **Host:** Eastgate (i9-12900K, 20 cores, 64GB RAM)
- **Devices:** 4x SoloKey V2 Hacker C (FIDO2 + PKCS#11)
- **OS:** Linux (Ubuntu/Debian-based)
- **Kernel:** 6.16.3

### **Connection:**
1. Connect all 4 SoloKeys to USB ports
2. Wait for device recognition (~2-3 seconds each)
3. Verify with `pcsc_scan`

### **Device Identification:**
Each SoloKey will appear as a separate slot:
- **Slot 0:** First SoloKey
- **Slot 1:** Second SoloKey
- **Slot 2:** Third SoloKey
- **Slot 3:** Fourth SoloKey

---

## 🔍 **Usage Guide**

### **1. Discover HSM Devices**

**Command:**
```bash
./target/release/beardog discover-hsm
```

**Expected Output:**
```
📚 Using PKCS#11 library: /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
🔧 Initializing PKCS#11...
🔍 Discovering HSM devices...

✅ Found 4 HSM device(s):

Device #1
  Slot ID:       0
  Label:         SoloKey-01
  Manufacturer:  SoloKeys
  Model:         Solo V2
  Serial:        XXXXX

Device #2
  Slot ID:       1
  Label:         SoloKey-02
  Manufacturer:  SoloKeys
  Model:         Solo V2
  Serial:        XXXXY

[... devices 3 & 4 ...]
```

**Troubleshooting:**
If no devices found:
```bash
# 1. Check PC/SC daemon
sudo systemctl status pcscd

# 2. Restart daemon
sudo systemctl restart pcscd

# 3. Check USB connections
lsusb | grep -i solo

# 4. Try with sudo
sudo ./target/release/beardog discover-hsm
```

---

### **2. Test Entropy Quality**

**Basic Test:**
```bash
./target/release/beardog test-entropy --slot 0 --size 1024
```

**With Hex Dump:**
```bash
./target/release/beardog test-entropy \
  --slot 0 \
  --size 1024 \
  --show-hex
```

**Expected Output:**
```
📚 Using library: /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
🎯 Target slot: 0
📏 Collecting 1024 bytes...

🎲 Collecting entropy from hardware...
✅ Successfully collected 1024 bytes of entropy!

📊 Entropy Quality:
   Unique byte values: 256/256
   Quality score: 100.0%
   Assessment: ✅ Excellent

📋 Hex dump (first 256 bytes):
   0000: a3 f2 8b 7c 45 d1 9e 3a 82 f4 1b 6d c8 93 54 2f
   0010: e7 19 ab 5e d3 72 8f 41 bc 0a 67 f5 34 d8 91 2c
   ...
```

**Quality Assessment:**
- **100.0%:** ✅ Excellent (true hardware RNG)
- **90-99%:** ✅ Very Good
- **70-89%:** ⚠️ Good (acceptable)
- **<70%:** ❌ Poor (not true RNG)

---

### **3. Mix Entropy from Multiple Devices**

**Basic Mixing (4 devices):**
```bash
./target/release/beardog mix-seed \
  --slots 0,1,2,3 \
  --output seed.bin
```

**Custom Bytes Per Source:**
```bash
./target/release/beardog mix-seed \
  --slots 0,1,2,3 \
  --output seed.bin \
  --bytes-per-source 512  # Default: 256
```

**Expected Output:**
```
📚 Using library: /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
🎯 Collecting from 4 slot(s): [0, 1, 2, 3]
📏 256 bytes per source

🎲 Collecting from slot 0... (1/4)
   ✅ Collected 256 bytes
🎲 Collecting from slot 1... (2/4)
   ✅ Collected 256 bytes
🎲 Collecting from slot 2... (3/4)
   ✅ Collected 256 bytes
🎲 Collecting from slot 3... (4/4)
   ✅ Collected 256 bytes

🌀 Mixed 1024 bytes from 4 sources
🔐 SHA3-512 digest: 64 bytes

✅ Seed mixing complete!
📁 Wrote 64 bytes to: seed.bin

💡 This seed combines entropy from 4 hardware sources
   Use it for: key derivation, wallet seeds, etc.
```

**Output File:**
- **Size:** 64 bytes (SHA3-512 digest)
- **Format:** Raw binary
- **Security:** Cryptographically mixed from 4 independent hardware sources

---

## 🔐 **Security Best Practices**

### **Seed File Handling:**

1. **Secure Storage:**
```bash
# Set restrictive permissions
chmod 400 seed.bin

# Move to secure location
mv seed.bin ~/.config/beardog/seeds/
```

2. **Verification:**
```bash
# Check file size (should be 64 bytes)
ls -lh seed.bin

# View hex dump
hexdump -C seed.bin
```

3. **Backup:**
```bash
# Encrypt before backup
gpg --symmetric --cipher-algo AES256 seed.bin

# Result: seed.bin.gpg (encrypted)
```

### **Device Trust:**

- ✅ **Use all 4 devices:** Maximum entropy diversity
- ✅ **Test each device:** Verify quality before mixing
- ✅ **Keep firmware updated:** Check SoloKeys for updates
- ✅ **Physical security:** Store devices securely when not in use

---

## 📊 **Configuration**

### **Custom Library Path:**
```bash
# Use specific PKCS#11 library
./target/release/beardog discover-hsm \
  --library /path/to/your/pkcs11.so
```

### **Common Library Locations:**
- **OpenSC:** `/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so`
- **SoftHSM:** `/usr/lib/softhsm/libsofthsm2.so`
- **Custom:** Check `configs/eastgate-production.toml`

### **Configuration File:**
```toml
# configs/eastgate-production.toml
[pkcs11]
default_library = "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so"
timeout_ms = 5000

[devices]
solokey_slots = [0, 1, 2, 3]
```

---

## 🔬 **Advanced Usage**

### **Test Individual Devices:**
```bash
# Test each device separately
for slot in 0 1 2 3; do
  echo "Testing slot $slot..."
  ./target/release/beardog test-entropy \
    --slot $slot \
    --size 1024
  echo ""
done
```

### **Large Entropy Collection:**
```bash
# Collect more bytes per source
./target/release/beardog mix-seed \
  --slots 0,1,2,3 \
  --output large-seed.bin \
  --bytes-per-source 4096
```

### **Automated Seed Generation:**
```bash
#!/bin/bash
# generate-seed.sh
DATE=$(date +%Y%m%d_%H%M%S)
OUTPUT="seeds/seed_${DATE}.bin"

./target/release/beardog mix-seed \
  --slots 0,1,2,3 \
  --output "$OUTPUT" \
  --bytes-per-source 512

echo "Generated: $OUTPUT"
chmod 400 "$OUTPUT"
```

---

## 🐛 **Troubleshooting**

### **Problem: "No HSM devices found"**

**Solution:**
```bash
# 1. Check PC/SC daemon
sudo systemctl status pcscd
sudo systemctl restart pcscd

# 2. Verify USB devices
lsusb | grep -i solo

# 3. Check permissions
sudo ./target/release/beardog discover-hsm

# 4. Test with pcsc_scan
pcsc_scan
```

### **Problem: "Failed to open session"**

**Solution:**
```bash
# Device may be in use
# 1. Close other applications using the device
# 2. Unplug and replug the device
# 3. Try a different USB port
```

### **Problem: "Poor entropy quality"**

**Solution:**
```bash
# 1. Test with larger sample
./target/release/beardog test-entropy --slot 0 --size 4096

# 2. Try different device
./target/release/beardog test-entropy --slot 1 --size 1024

# 3. Check device firmware
# Visit: https://solokeys.com
```

### **Problem: "Permission denied"**

**Solution:**
```bash
# Add your user to the scard group
sudo usermod -a -G scard $USER

# Re-login or use newgrp
newgrp scard

# Or run with sudo (temporary)
sudo ./target/release/beardog discover-hsm
```

---

## 📈 **Performance**

### **Typical Times (Eastgate i9-12900K):**
- **Discovery:** ~0.5-1 seconds
- **Entropy collection (1KB):** ~0.1-0.2 seconds per device
- **Seed mixing (4 devices, 256 bytes each):** ~1-2 seconds total

### **Benchmarking:**
```bash
# Time the operation
time ./target/release/beardog mix-seed \
  --slots 0,1,2,3 \
  --output bench-seed.bin
```

---

## ✅ **Verification Checklist**

Before production use:

- [ ] All 4 SoloKeys detected
- [ ] Each device passes entropy quality test (>90%)
- [ ] Seed mixing works with all 4 devices
- [ ] Output file is exactly 64 bytes
- [ ] Permissions set correctly (400)
- [ ] Backup strategy in place

---

## 🎯 **Next Steps**

### **Phase 1: ✅ COMPLETE**
- Real PKCS#11 integration
- Multi-device support
- Entropy mixing
- CLI fully functional

### **Phase 2: In Progress**
- Android NDK setup
- Pixel 8a cross-compilation
- StrongBox integration

### **Phase 3: Planned**
- Cross-platform demo
- Eastgate + Pixel working together
- Production deployment

---

## 📚 **Related Documentation**

- **CLI Help:** `./target/release/beardog --help`
- **Status Check:** `./target/release/beardog status`
- **MVP Plan:** `PRE_PRODUCTION_MVP_PLAN.md`
- **Configuration:** `configs/eastgate-production.toml`
- **Phase 1 Success:** `PHASE1_COMPLETE_SUCCESS.md`

---

## 🔗 **Useful Commands**

```bash
# Quick discovery
alias hsm-discover='./target/release/beardog discover-hsm'

# Quick test
alias hsm-test='./target/release/beardog test-entropy --slot'

# Quick mix (all 4 devices)
alias hsm-mix='./target/release/beardog mix-seed --slots 0,1,2,3 --output'

# Usage:
hsm-discover
hsm-test 0
hsm-mix my-seed.bin
```

---

## 🎉 **You're Ready!**

Your BearDog CLI is fully functional and ready to use with your 4x SoloKeys!

**Start with:**
```bash
./target/release/beardog discover-hsm
```

**Happy entropy mixing!** 🔐🐻

---

*For support, see: `docs/` or create an issue*  
*Last updated: October 29, 2025*

