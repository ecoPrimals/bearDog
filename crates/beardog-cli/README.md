# BearDog CLI - Hardware Entropy Management

**Version**: 3.0.0  
**Status**: ✅ Production Ready  
**Platform**: Linux (x86_64)

Real hardware integration for sovereign entropy collection and mixing.

---

## 🎯 Overview

BearDog CLI provides command-line tools for managing hardware security modules (HSMs) and collecting entropy from real hardware devices like SoloKey, YubiKey, Nitrokey, and other PKCS#11-compatible tokens.

### Key Features:

- ✅ **Real PKCS#11 Integration** - No mocks, no stubs, actual hardware
- ✅ **Hardware Entropy Collection** - True random numbers from hardware RNG
- ✅ **Multi-Device Support** - Mix entropy from multiple sources
- ✅ **SHA3-512 Mixing** - Cryptographically secure entropy combination
- ✅ **Quality Assessment** - Validate entropy quality
- ✅ **Zero Dependencies on Cloud** - Completely local, sovereign operation

---

## 🚀 Quick Start

### Build
```bash
cargo build --release -p beardog-cli
```

### Check Status
```bash
./target/release/beardog status
```

### Discover Devices
```bash
./target/release/beardog discover-hsm
```

---

## 📖 Commands

### 1. `status` - System Status

Show BearDog system information and available features.

```bash
beardog status
```

**Output:**
```
🐻 BearDog Status

Version: 3.0.0
Platform: linux (x86_64)
Build: release

🔐 Security Features:
  Real PKCS#11 Integration: ✅
  Hardware Entropy: ✅
  SHA3-512 Mixing: ✅
  Zero Mocks: ✅
```

---

### 2. `discover-hsm` - Discover Hardware Devices

Find and enumerate all PKCS#11-compatible HSM devices.

```bash
beardog discover-hsm [OPTIONS]
```

**Options:**
- `-l, --library <PATH>` - PKCS#11 library path (optional, uses default)
- `-v, --verbose` - Enable verbose logging

**Example:**
```bash
# Auto-detect using default library
beardog discover-hsm

# Specify custom library
beardog discover-hsm --library /usr/lib/pkcs11/custom.so

# With verbose output
beardog -v discover-hsm
```

**Output:**
```
🔍 BearDog HSM Discovery

📚 Using PKCS#11 library: /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
🔧 Initializing PKCS#11...
🔍 Discovering HSM devices...

✅ Found 4 HSM device(s):

Device #1
  Slot ID:       0
  Label:         SoloKey
  Manufacturer:  SoloKeys
  Model:         SoloKey V2
  Serial:        ABC123...

Device #2
  Slot ID:       1
  Label:         SoloKey
  ...
```

---

### 3. `test-entropy` - Test Entropy Collection

Collect entropy from a specific device and assess quality.

```bash
beardog test-entropy [OPTIONS]
```

**Options:**
- `-l, --library <PATH>` - PKCS#11 library path (default: `/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so`)
- `-s, --slot <ID>` - Slot ID to test (required)
- `-n, --size <BYTES>` - Number of bytes to collect (default: 1024)
- `--show-hex` - Display hex dump of collected entropy
- `-v, --verbose` - Enable verbose logging

**Examples:**
```bash
# Basic test from slot 0
beardog test-entropy --slot 0

# Collect more bytes
beardog test-entropy --slot 0 --size 4096

# Show hex dump
beardog test-entropy --slot 0 --size 256 --show-hex

# Test different slot
beardog test-entropy --slot 1 --size 512
```

**Output:**
```
🎲 BearDog Entropy Test

📚 Using library: /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
🎯 Target slot: 0
📏 Collecting 1024 bytes...

🎲 Collecting entropy from hardware...
✅ Successfully collected 1024 bytes of entropy!

📊 Entropy Quality:
   Unique byte values: 245/256
   Quality score: 95.7%
   Assessment: ✅ Excellent

📋 Hex dump (first 256 bytes):
   0000: 3f 8a 2c 91 ...
   ...
```

**Quality Assessment:**
- **>90%**: ✅ Excellent (true hardware RNG)
- **70-90%**: ⚠️ Good (acceptable)
- **<70%**: ❌ Poor (may not be true hardware RNG)

---

### 4. `mix-seed` - Mix Entropy from Multiple Sources

Collect entropy from multiple devices and mix them using SHA3-512.

```bash
beardog mix-seed [OPTIONS]
```

**Options:**
- `-l, --library <PATH>` - PKCS#11 library path (default: `/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so`)
- `-s, --slots <IDs>` - Comma-separated list of slot IDs (required)
- `-o, --output <FILE>` - Output file path (required)
- `--bytes-per-source <BYTES>` - Bytes to collect from each source (default: 256)
- `-v, --verbose` - Enable verbose logging

**Examples:**
```bash
# Mix from 4 devices
beardog mix-seed --slots 0,1,2,3 --output seed.bin

# Mix from 2 devices with more entropy per device
beardog mix-seed --slots 0,1 --output seed.bin --bytes-per-source 512

# Custom output location
beardog mix-seed --slots 0,1,2,3 --output ~/keys/master-seed.bin
```

**Output:**
```
🌀 BearDog Entropy Mixer

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
- Size: 64 bytes (SHA3-512 hash)
- Format: Raw binary
- Use: Key derivation, wallet seeds, master secrets

---

## 🔧 Setup & Requirements

### System Requirements

- **OS**: Linux (tested on Ubuntu 20.04+)
- **Architecture**: x86_64
- **Rust**: 1.70+ (for building)

### Dependencies

```bash
# Install OpenSC for PKCS#11 support
sudo apt update
sudo apt install opensc pcscd

# Start pcscd service
sudo systemctl start pcscd
sudo systemctl enable pcscd
```

### Supported Devices

Any PKCS#11-compatible hardware token:
- ✅ SoloKey V2
- ✅ YubiKey (with PIV)
- ✅ Nitrokey
- ✅ Hardware HSMs (Thales, Utimaco, etc.)
- ✅ Smart cards with PKCS#11 support

---

## 📊 Use Cases

### 1. Cryptocurrency Wallet Seeds

```bash
# Mix entropy from multiple SoloKeys for wallet seed
beardog mix-seed --slots 0,1,2,3 --output wallet-seed.bin --bytes-per-source 256

# Use the 64-byte output for BIP39 seed derivation
```

### 2. Master Key Generation

```bash
# Generate master key from hardware entropy
beardog mix-seed --slots 0,1 --output master-key.bin --bytes-per-source 512

# 64 bytes of mixed hardware entropy
```

### 3. Testing Hardware RNG Quality

```bash
# Validate entropy quality from each device
for slot in 0 1 2 3; do
  echo "Testing slot $slot..."
  beardog test-entropy --slot $slot --size 4096
done
```

### 4. Backup Seed Creation

```bash
# Create backup seeds from different device combinations
beardog mix-seed --slots 0,1 --output backup-seed-1.bin
beardog mix-seed --slots 2,3 --output backup-seed-2.bin

# Store in separate secure locations
```

---

## 🛡️ Security Considerations

### Good Practices:

1. **Use Multiple Sources**
   - Mix entropy from multiple devices
   - Protects against single device compromise

2. **Verify Quality**
   - Always check entropy quality score
   - >90% indicates true hardware RNG

3. **Secure Storage**
   - Protect generated seed files
   - Use encryption for storage
   - Consider offline storage

4. **Device Trust**
   - Use reputable hardware (SoloKey, YubiKey, etc.)
   - Verify device authenticity
   - Keep firmware updated

### What BearDog Provides:

- ✅ Direct hardware access (no network)
- ✅ Cryptographically secure mixing (SHA3-512)
- ✅ Quality assessment
- ✅ Multi-source entropy combination
- ✅ No telemetry, no cloud dependencies

### What You Must Do:

- ⚠️ Protect generated seed files
- ⚠️ Use secure device storage
- ⚠️ Maintain physical device security
- ⚠️ Back up seed files securely

---

## 🐛 Troubleshooting

### No devices found

```bash
# Check if pcscd is running
sudo systemctl status pcscd

# Restart pcscd
sudo systemctl restart pcscd

# List devices with OpenSC
pkcs11-tool --module /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so --list-slots

# Check USB devices
lsusb | grep -i solo
```

### Permission denied

```bash
# Add user to required groups
sudo usermod -a -G plugdev $USER
sudo usermod -a -G pcscd $USER

# Log out and back in
```

### Library not found

```bash
# Find PKCS#11 libraries
find /usr/lib -name "*pkcs11*.so"

# Use custom path
beardog discover-hsm --library /path/to/pkcs11.so
```

---

## 📚 Additional Resources

- **Full Guide**: See `QUICK_START_HARDWARE_TESTING.md`
- **MVP Plan**: See `PRE_PRODUCTION_MVP_PLAN.md`
- **Architecture**: See `ARCHITECTURE.md`
- **Security**: See `SECURITY.md`

---

## 🤝 Contributing

BearDog is part of the ecoPrimals ecosystem focused on sovereign computing.

### Reporting Issues

When reporting issues, include:
- OS and architecture
- `beardog status` output
- Device type and model
- Error messages (use `-v` flag)

---

## 📄 License

MIT OR Apache-2.0 (see LICENSE files)

---

## 🐻 About BearDog

BearDog provides security intelligence for the ecoPrimals ecosystem, focusing on:
- Hardware-backed cryptography
- Sovereign entropy management
- Zero-trust security
- Human-controlled AI
- Privacy-first design

**Version**: 3.0.0  
**Status**: Production Ready  
**Platform**: Linux (x86_64)

🔐 **Secure. Sovereign. Hardware-Backed.** 🐻

---

*Documentation updated: November 1, 2025*
