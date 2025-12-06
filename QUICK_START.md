# 🚀 BearDog Quick Start Guide
## Get Started in 5 Minutes

**Version**: 1.0.0  
**Status**: Production Ready ✅  
**Last Updated**: December 2, 2025

---

## 📋 Prerequisites

- **OS**: Linux (tested on Ubuntu/Debian)
- **Rust**: 1.75+ (install from https://rustup.rs)
- **Optional**: Android device with ADB for StrongBox support
- **Optional**: FIDO2 token (Solo 2, YubiKey)

---

## ⚡ Quick Install

```bash
# Clone the repository
cd /home/eastgate/Development/ecoPrimals/beardog

# Build BearDog CLI
cargo build --release --bin beardog

# Add to PATH (optional)
export PATH=$PATH:$(pwd)/target/release

# Verify installation
beardog --help
```

**Build time**: ~2-3 minutes on first build

---

## 🎯 5-Minute Workflow

### Step 1: Discover Available Hardware (30 seconds)

```bash
beardog hsm discover
```

**Expected output**:
```
🔍 BearDog HSM Discovery
=======================

✅ Found 4 HSM(s):

HSM #1: SoftHSM2
   Tier: Software
   Type: PKCS#11

HSM #2: Android StrongBox (Pixel 8a)
   Tier: Mobile
   Type: Android-Keystore

HSM #3: Solo 2 (Primary)
   Tier: Hardware
   Type: FIDO2
```

### Step 2: Generate a Cryptographic Key (10 seconds)

```bash
beardog key generate \
  --key-id my-first-key \
  --algorithm aes256-gcm \
  --hsm auto
```

**Expected output**:
```
🔑 BearDog Key Generation
========================

✅ Selected HSM: Android StrongBox (Pixel 8a)
🔐 Generating aes256-gcm key...
✅ Key generated successfully!

📋 Key Details:
   ID: my-first-key
   Algorithm: aes256-gcm
   HSM: Android StrongBox (Pixel 8a)
   Status: Active
```

### Step 3: Encrypt a File (instant)

```bash
# Create a test file
echo "Hello, BearDog! This is secret data." > secret.txt

# Encrypt it
beardog encrypt \
  --key my-first-key \
  --input secret.txt \
  --output secret.enc
```

**Expected output**:
```
🔒 BearDog Encryption
====================

✅ Encryption complete
   Input: 37 bytes
   Output: 65 bytes
```

### Step 4: Decrypt the File (instant)

```bash
beardog decrypt \
  --key my-first-key \
  --input secret.enc \
  --output secret-decrypted.txt

# Verify it worked
cat secret-decrypted.txt
```

**Expected output**:
```
🔓 BearDog Decryption
====================

✅ Decryption complete
   Output: 37 bytes

Hello, BearDog! This is secret data.
```

### Step 5: Verify Round-Trip (5 seconds)

```bash
diff secret.txt secret-decrypted.txt && echo "✅ SUCCESS!"
```

**Expected output**: `✅ SUCCESS!`

---

## 🌱 Advanced: Human Entropy Collection

### Generate a High-Quality Entropy Seed

```bash
beardog entropy collect \
  --human-input \
  --device auto \
  --quality-tier 1 \
  --output ~/my-entropy-seed.json
```

**What it does**:
- Collects multi-modal entropy from system sources
- Uses best available HSM (auto-selected)
- Generates cryptographic seed
- Saves to JSON with full metadata

### View Seed Information

```bash
beardog entropy info --seed ~/my-entropy-seed.json
```

---

## 💡 Common Use Cases

### Use Case 1: Secure File Storage

```bash
# Generate a key
beardog key generate --key-id documents-key --algorithm aes256-gcm --hsm auto

# Encrypt sensitive documents
beardog encrypt --key documents-key --input taxes.pdf --output taxes.enc
beardog encrypt --key documents-key --input passwords.txt --output passwords.enc

# Store encrypted files safely (even in cloud storage!)
# Decrypt when needed
beardog decrypt --key documents-key --input taxes.enc --output taxes.pdf
```

### Use Case 2: Password Manager Backend

```bash
# Generate encryption key
beardog key generate --key-id pwmanager --algorithm chacha20-poly1305 --hsm auto

# Encrypt password database
beardog encrypt --key pwmanager --input passwords.db --output passwords.db.enc

# Application reads encrypted file and decrypts in memory
```

### Use Case 3: Multi-Device Key Management

```bash
# On device 1 (laptop with SoftHSM2)
beardog key generate --key-id shared-key --algorithm aes256-gcm --hsm software

# On device 2 (phone with StrongBox)
beardog key generate --key-id mobile-key --algorithm aes256-gcm --hsm mobile

# List all keys
beardog key list
```

---

## 🔧 Configuration

### Key Storage Location

Keys are stored in: `~/.beardog/keys/`

**Structure**:
```
~/.beardog/
├── keys/
│   ├── my-first-key.json
│   ├── documents-key.json
│   └── shared-key.json
└── config.toml (optional)
```

### Environment Variables

```bash
# Lower entropy threshold for testing
export BEARDOG_ENTROPY_QUALITY_THRESHOLD=0.5

# Custom key storage location
export BEARDOG_KEY_STORE=$HOME/.config/beardog/keys

# Enable verbose logging
beardog --verbose <command>
```

---

## 🛠️ Troubleshooting

### "No HSMs found"

**Solution**:
```bash
# Install SoftHSM2 (software fallback)
sudo apt install softhsm2  # Ubuntu/Debian
brew install softhsm       # macOS

# Verify installation
ls /usr/lib/softhsm/libsofthsm2.so
```

### "Entropy quality below threshold"

**Solution**:
```bash
# Lower threshold for testing
export BEARDOG_ENTROPY_QUALITY_THRESHOLD=0.5
beardog entropy collect --human-input --device software --output seed.json
```

### "Android device not detected"

**Solution**:
```bash
# Ensure ADB is installed
sudo apt install adb

# Enable USB debugging on Android device
# Connect device via USB

# Verify connection
adb devices

# Should show your device
```

### "Permission denied"

**Solution**:
```bash
# Ensure binary is executable
chmod +x target/release/beardog

# Or run with sudo if accessing hardware
sudo beardog hsm discover
```

---

## 📊 Performance Expectations

| Operation | Time | Hardware |
|-----------|------|----------|
| HSM Discovery | ~500ms | All devices |
| Key Generation | ~100ms | StrongBox |
| Encrypt (1KB) | <1ms | AES-256-GCM |
| Decrypt (1KB) | <1ms | AES-256-GCM |
| Entropy Collection | ~50ms | Software |
| Large File (1MB) | ~10ms | AES-256-GCM |

---

## 🔒 Security Best Practices

### ✅ DO

- **Use hardware HSMs** when available (StrongBox, YubiKey)
- **Backup encrypted files** (not the keys!)
- **Use unique keys** for different purposes
- **Test decryption** before deleting originals
- **Keep software updated** for security patches

### ❌ DON'T

- **Don't share private keys** between devices
- **Don't store keys in plaintext** (BearDog handles this)
- **Don't delete keys** without backing up encrypted data
- **Don't use the same key** for everything
- **Don't skip verification** after encryption

---

## 📚 Command Reference

### Key Management

```bash
# Generate key
beardog key generate --key-id <name> --algorithm <algo> --hsm <type>

# List keys
beardog key list [--hsm <filter>]

# Delete key
beardog key delete --key-id <name> --yes
```

### Encryption/Decryption

```bash
# Encrypt
beardog encrypt --key <key-id> --input <file> --output <file>

# Decrypt
beardog decrypt --key <key-id> --input <file> --output <file>
```

### Entropy

```bash
# Collect entropy
beardog entropy collect --human-input --device <type> --output <file>

# View seed info
beardog entropy info --seed <file>
```

### HSM Operations

```bash
# Discover HSMs
beardog hsm discover [--verbose]

# Test HSM
beardog hsm test --hsm-id <id> [--iterations <n>]
```

---

## 🎓 Learning Path

### Beginner (5 minutes)
1. ✅ Install and verify (`beardog --help`)
2. ✅ Discover HSMs (`beardog hsm discover`)
3. ✅ Encrypt a file (follow Step 3-5 above)

### Intermediate (30 minutes)
1. ✅ Generate entropy seed
2. ✅ Use different algorithms (aes256-gcm, chacha20-poly1305)
3. ✅ Manage multiple keys
4. ✅ Test with large files (1MB+)

### Advanced (2 hours)
1. ✅ Integrate with applications
2. ✅ Set up multiple HSM types
3. ✅ Configure custom key storage
4. ✅ Implement backup strategies

---

## 🌟 Example: Complete Workflow

```bash
#!/bin/bash
# complete-demo.sh - Complete BearDog workflow

echo "🚀 BearDog Complete Demo"
echo "======================="
echo

# 1. Discover HSMs
echo "1️⃣ Discovering HSMs..."
beardog hsm discover
echo

# 2. Collect entropy
echo "2️⃣ Collecting entropy..."
beardog entropy collect \
  --human-input \
  --device auto \
  --quality-tier 1 \
  --output demo-seed.json
echo

# 3. Generate key
echo "3️⃣ Generating encryption key..."
beardog key generate \
  --key-id demo-key \
  --algorithm aes256-gcm \
  --hsm auto \
  --seed demo-seed.json
echo

# 4. Create test data
echo "4️⃣ Creating test data..."
echo "This is confidential information that must be encrypted." > demo.txt
cat demo.txt
echo

# 5. Encrypt
echo "5️⃣ Encrypting..."
beardog encrypt \
  --key demo-key \
  --input demo.txt \
  --output demo.enc
ls -lh demo.*
echo

# 6. Decrypt
echo "6️⃣ Decrypting..."
beardog decrypt \
  --key demo-key \
  --input demo.enc \
  --output demo-decrypted.txt
echo

# 7. Verify
echo "7️⃣ Verifying..."
if diff demo.txt demo-decrypted.txt > /dev/null; then
    echo "✅ SUCCESS! Encryption/decryption works perfectly!"
else
    echo "❌ FAILED! Files don't match"
    exit 1
fi

# 8. Cleanup
echo
echo "🧹 Cleaning up..."
rm demo.txt demo.enc demo-decrypted.txt demo-seed.json
beardog key delete --key-id demo-key --yes

echo
echo "🎉 Demo complete!"
```

**Run it**:
```bash
chmod +x complete-demo.sh
./complete-demo.sh
```

---

## 🆘 Getting Help

### Documentation
- **Full Docs**: See `docs/` directory
- **Architecture**: `ARCHITECTURE.md`
- **Security**: `SECURITY.md`
- **API Reference**: Run `cargo doc --open`

### Support
- **Issues**: Report at GitHub issues
- **Questions**: See FAQ in docs
- **Contributing**: See `CONTRIBUTING.md`

### Quick Commands
```bash
# General help
beardog --help

# Command-specific help
beardog entropy --help
beardog key --help
beardog encrypt --help

# Verbose output for debugging
beardog --verbose <command>
```

---

## ✅ Success Criteria

You know BearDog is working correctly when:

1. ✅ `beardog --help` shows command list
2. ✅ `beardog hsm discover` finds at least 1 HSM
3. ✅ `beardog key generate` creates a key successfully
4. ✅ `beardog encrypt` + `beardog decrypt` produces identical files
5. ✅ `diff original.txt decrypted.txt` shows no differences

---

## 🎯 Next Steps

### For Users
1. ✅ Complete the 5-minute workflow above
2. ✅ Try the complete demo script
3. ✅ Integrate with your applications
4. ✅ Read the full documentation

### For Developers
1. ✅ Read `ARCHITECTURE.md`
2. ✅ Check `BEARDOG_CODING_STANDARDS.md`
3. ✅ Run the test suite: `cargo test --workspace`
4. ✅ Contribute improvements!

---

**Version**: 0.9.0  
**Status**: ✅ Production Ready  
**Tests**: 8,138+ passing  
**Coverage**: 78.18%  
**Last Updated**: December 4, 2025

**🎉 Happy Encrypting with BearDog! 🎉**
