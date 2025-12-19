# 🐻 BearDog - Quick Reference Guide
**Version**: 0.9.0 (Production Ready)  
**Last Updated**: December 2, 2025

---

## 🚀 **QUICK START**

```bash
# Navigate to BearDog
cd /home/eastgate/Development/ecoPrimals/beardog

# Build (if needed)
cargo build --release

# Use the CLI
./target/debug/beardog --help
```

---

## 📋 **COMMON COMMANDS**

### **1. Discover Available HSMs**
```bash
./target/debug/beardog hsm discover
```
**Expected Output**: 4 HSMs (SoftHSM2, Pixel StrongBox, 2x Solo 2)

---

### **2. Collect Human Entropy**
```bash
# Auto-select best HSM (Pixel StrongBox preferred)
./target/debug/beardog entropy collect \
  --human-input \
  --device auto \
  --output ~/entropy-seed.json

# Or specify device type
./target/debug/beardog entropy collect \
  --human-input \
  --device mobile \
  --output ~/pixel-seed.json
```

**Device Options**: `auto`, `software`, `mobile`, `hardware`

---

### **3. Generate Encryption Key**
```bash
# Auto-select best HSM
./target/debug/beardog key generate \
  --key-id my-secure-key \
  --algorithm aes256-gcm \
  --hsm auto

# Or use specific HSM
./target/debug/beardog key generate \
  --key-id soft-key \
  --algorithm aes256-gcm \
  --hsm software
```

**HSM Options**: `auto`, `software`, `mobile`, `hardware`  
**Algorithm Options**: `aes256-gcm` (more coming: `chacha20-poly1305`, `ed25519`)

---

### **4. Encrypt Files**
```bash
./target/debug/beardog encrypt \
  --key my-secure-key \
  --input ~/secret-data.txt \
  --output ~/secret-data.enc
```

---

### **5. Decrypt Files**
```bash
./target/debug/beardog decrypt \
  --key my-secure-key \
  --input ~/secret-data.enc \
  --output ~/secret-data-decrypted.txt
```

---

### **6. List Keys**
```bash
# List all keys
./target/debug/beardog key list

# Filter by HSM
./target/debug/beardog key list --hsm-filter mobile
```

---

### **7. Delete Key**
```bash
./target/debug/beardog key delete --key-id old-key --yes
```

---

### **8. Show System Status**
```bash
./target/debug/beardog status
```

---

## 🖥️ **AVAILABLE HARDWARE**

| Device | Type | Tier | Use For |
|--------|------|------|---------|
| **SoftHSM2** | Software | Software | Testing, general use |
| **Pixel 8a StrongBox** | Mobile | Mobile | High security, mobile entropy |
| **Solo 2 Primary** | USB Token | Hardware | Signing operations |
| **Solo 2 Secondary** | USB Token | Hardware | Signing operations |

---

## 📁 **KEY STORAGE LOCATION**

Keys are stored in: `~/.beardog/keys/`

Each key is a JSON file: `~/.beardog/keys/{key-id}.json`

**Backup your keys**: 
```bash
cp -r ~/.beardog/keys ~/beardog-keys-backup-$(date +%Y%m%d)
```

---

## 🔧 **ENVIRONMENT VARIABLES**

```bash
# Set entropy quality threshold (default: 0.6, production: 0.8)
export BEARDOG_ENTROPY_QUALITY_THRESHOLD=0.6

# Enable hardware tests
export BEARDOG_ENABLE_HARDWARE_TESTS=1

# Prefer specific HSM type
export BEARDOG_PREFER_SOFTWARE_HSM=1  # For SoftHSM2
export BEARDOG_PREFER_MOBILE_HSM=1     # For Pixel StrongBox
export BEARDOG_PREFER_FIDO2=1          # For Solo 2
```

---

## 🎯 **TYPICAL WORKFLOWS**

### **Workflow 1: Secure Your Research Data**
```bash
# 1. Generate key (uses best available HSM)
./target/debug/beardog key generate \
  --key-id research-2025 \
  --algorithm aes256-gcm \
  --hsm auto

# 2. Encrypt your data
./target/debug/beardog encrypt \
  --key research-2025 \
  --input ~/research-data.txt \
  --output ~/research-data.enc

# 3. Delete original (optional)
shred -u ~/research-data.txt

# 4. Later: decrypt when needed
./target/debug/beardog decrypt \
  --key research-2025 \
  --input ~/research-data.enc \
  --output ~/research-data-recovered.txt
```

---

### **Workflow 2: Generate Human Entropy Seed**
```bash
# 1. Collect entropy from Pixel StrongBox
./target/debug/beardog entropy collect \
  --human-input \
  --device mobile \
  --output ~/my-entropy-seed.json

# 2. View seed info
./target/debug/beardog entropy info \
  --seed ~/my-entropy-seed.json

# 3. Use seed for key generation (future feature)
./target/debug/beardog key generate \
  --key-id seed-based-key \
  --algorithm aes256-gcm \
  --seed ~/my-entropy-seed.json
```

---

### **Workflow 3: Batch Encrypt Multiple Files**
```bash
# Create encryption script
cat > encrypt-all.sh << 'EOF'
#!/bin/bash
KEY_ID="batch-encrypt-$(date +%Y%m%d)"

# Generate key once
./target/debug/beardog key generate \
  --key-id "$KEY_ID" \
  --algorithm aes256-gcm \
  --hsm auto

# Encrypt all files
for file in ~/sensitive/*.txt; do
  ./target/debug/beardog encrypt \
    --key "$KEY_ID" \
    --input "$file" \
    --output "${file}.enc"
  echo "Encrypted: $file"
done

echo "All files encrypted with key: $KEY_ID"
EOF

chmod +x encrypt-all.sh
./encrypt-all.sh
```

---

## 🔍 **TROUBLESHOOTING**

### **Problem: No HSMs found**
```bash
# Check SoftHSM2
ls -l /usr/lib/softhsm/libsofthsm2.so

# Check Pixel connection
adb devices

# Check Solo 2
lsusb | grep -i solo

# Install SoftHSM2 if missing
sudo apt install softhsm2
```

---

### **Problem: Entropy quality too low**
```bash
# Lower threshold for testing
export BEARDOG_ENTROPY_QUALITY_THRESHOLD=0.5

# Or collect from better source
./target/debug/beardog entropy collect \
  --human-input \
  --device mobile \
  --output seed.json
```

---

### **Problem: Key not found**
```bash
# List all keys
./target/debug/beardog key list

# Check key storage
ls -la ~/.beardog/keys/

# Regenerate if needed
./target/debug/beardog key generate \
  --key-id my-key \
  --algorithm aes256-gcm
```

---

### **Problem: Decryption fails**
```bash
# Ensure using same key that encrypted the file
./target/debug/beardog key list

# Check file isn't corrupted
file encrypted-file.enc

# Verify key exists
ls ~/.beardog/keys/your-key-id.json
```

---

## 📚 **DOCUMENTATION**

### **Quick Reference**:
- This file: `BEARDOG_QUICK_REFERENCE.md`
- Overview: `README.md`
- Navigation: `START_HERE.md`

### **Detailed Guides**:
- Session summary: `SESSION_SUMMARY_FINAL_DEC_2_2025.md`
- Phase 2 results: `PHASE_2_EXECUTION_COMPLETE.md`
- Demo results: `DEMO_RESULTS_DEC_2_2025.md`

### **Specifications**:
- Integration: `specs/current/integration/PHASE_1_INTEGRATION_REQUIREMENTS.md`
- Songbird: `specs/current/integration/SONGBIRD_BEARDOG_VPN_FREE_ARCHITECTURE.md`

### **Testing Guides**:
- Android: `docs/testing-guides/ANDROID_STRONGBOX_TESTING_GUIDE.md`
- Solo 2: `docs/testing-guides/SOLOKEY_TESTING_GUIDE.md`

### **Reference**:
- Constants: `docs/reference/CONSTANTS_DOCUMENTATION.md`
- Environment: `docs/reference/ENVIRONMENT_VARIABLES_REFERENCE.md`

---

## ⚡ **ALIASES (Optional)**

Add to your `~/.bashrc` or `~/.zshrc`:

```bash
# BearDog aliases
alias bd='cd /home/eastgate/Development/ecoPrimals/beardog && ./target/debug/beardog'
alias bd-discover='cd /home/eastgate/Development/ecoPrimals/beardog && ./target/debug/beardog hsm discover'
alias bd-entropy='cd /home/eastgate/Development/ecoPrimals/beardog && ./target/debug/beardog entropy collect --human-input --device auto'
alias bd-keygen='cd /home/eastgate/Development/ecoPrimals/beardog && ./target/debug/beardog key generate --algorithm aes256-gcm --hsm auto --key-id'
alias bd-keylist='cd /home/eastgate/Development/ecoPrimals/beardog && ./target/debug/beardog key list'
alias bd-encrypt='cd /home/eastgate/Development/ecoPrimals/beardog && ./target/debug/beardog encrypt'
alias bd-decrypt='cd /home/eastgate/Development/ecoPrimals/beardog && ./target/debug/beardog decrypt'
alias bd-status='cd /home/eastgate/Development/ecoPrimals/beardog && ./target/debug/beardog status'

# Then use like:
# bd-discover
# bd-keygen my-key
# bd-encrypt --key my-key --input data.txt --output data.enc
```

---

## 🔐 **SECURITY BEST PRACTICES**

1. **Backup Keys Regularly**:
   ```bash
   tar -czf beardog-keys-backup-$(date +%Y%m%d).tar.gz ~/.beardog/keys/
   ```

2. **Use Mobile HSM for Sensitive Operations**:
   ```bash
   --hsm mobile  # Forces use of Pixel StrongBox
   ```

3. **Verify Encryption**:
   ```bash
   # Always test decrypt after encrypt
   ./target/debug/beardog decrypt --key mykey --input test.enc --output test-out.txt
   diff test.txt test-out.txt
   ```

4. **Secure Delete Originals**:
   ```bash
   shred -u original-file.txt  # After encrypting
   ```

5. **Regular HSM Discovery**:
   ```bash
   ./target/debug/beardog hsm discover  # Verify all hardware present
   ```

---

## 🎯 **NEXT STEPS**

1. **Start Using BearDog**: Encrypt your research data now!
2. **Songbird Integration**: When Songbird is ready (1-2 days)
3. **Feedback**: Share results from your scientific proving ground
4. **Optimization**: Performance tuning based on real usage

---

## 📞 **SUPPORT**

- **Documentation**: `START_HERE.md`
- **Issues**: `TODO_GITHUB_ISSUES.md`
- **Architecture**: `ARCHITECTURE.md`
- **Security**: `SECURITY.md`

---

🐻 **BearDog: Your sovereign cryptography companion!** ✨

**All code is vendor/primal/algorithm/transport agnostic!**

