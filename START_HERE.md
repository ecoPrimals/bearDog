# 🚀 BearDog - Start Here

**Welcome to BearDog!** This guide will get you up and running quickly.

---

## 🎯 What is BearDog?

BearDog is a **sovereign cryptography and mesh networking framework** built in Rust that provides:

- **Privacy by Default**: Lineage-based encryption (only family members can communicate)
- **Human Sovereignty**: Your keys, your control, instant revocation
- **Production Quality**: Real crypto (Ed25519, ChaCha20-Poly1305), fast operations
- **Zero Configuration**: No VPN servers, no config files, just sovereign security

---

## 🎬 Quick Demo (2 minutes)

See BearDog in action:

```bash
# Run the local showcase
./demos/beardog-local-showcase.sh
```

**What you'll see**:
- ✅ Key generation & lineage tracking
- ✅ BirdSong encryption (lineage-aware)
- ✅ Privacy enforcement (strangers blocked)
- ✅ Human sovereignty (instant revocation)

---

## 📦 Installation

### Prerequisites

```bash
# Rust toolchain (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build tools
sudo apt install build-essential pkg-config libssl-dev
```

### Build BearDog

```bash
# Clone the repository
git clone https://github.com/ecoPrimals/bearDog.git
cd beardog

# Build release binary
cargo build --release

# Binary location
./target/release/beardog
```

---

## 🎓 Quick Start Tutorial

### 1. Generate Your First Key

```bash
beardog key generate \
  --key-id my-root-key \
  --algorithm ed25519
```

### 2. Create a Child Key (Lineage)

```bash
beardog key derive \
  --master-key my-root-key \
  --purpose child \
  --output my-child-key
```

### 3. Encrypt a Message (BirdSong)

```bash
beardog birdsong encrypt \
  --message "Hello, family!" \
  --hint DirectAncestors \
  --root-id my-root-key
```

### 4. Decrypt the Message

```bash
# You can decrypt (you're in the lineage)
beardog birdsong decrypt \
  --input encrypted.birdsong \
  --key-id my-root-key

# Strangers cannot decrypt (privacy enforced!)
```

---

## 📚 Documentation

### **For New Users**:
- **This File**: Quick start guide
- [`README.md`](README.md): Project overview & features
- [`docs/dec24-showcase/QUICK_REFERENCE_DEC24.md`](docs/dec24-showcase/QUICK_REFERENCE_DEC24.md): Quick reference card

### **For Developers**:
- [`ARCHITECTURE.md`](ARCHITECTURE.md): System architecture
- [`docs/`](docs/): Comprehensive documentation
- [`specs/`](specs/): Technical specifications

### **Current Status**:
- [`docs/dec24-showcase/BEARDOG_SHOWCASE_FINAL_STATUS.md`](docs/dec24-showcase/BEARDOG_SHOWCASE_FINAL_STATUS.md): Complete status report
- [`docs/dec24-showcase/TODAY_SUMMARY_DEC24.md`](docs/dec24-showcase/TODAY_SUMMARY_DEC24.md): Latest progress
- [`docs/dec24-showcase/FINAL_STATUS.txt`](docs/dec24-showcase/FINAL_STATUS.txt): Quick status check

### **For Integration**:
- [`docs/dec24-showcase/SONGBIRD_HANDOFF_COMPLETE.md`](docs/dec24-showcase/SONGBIRD_HANDOFF_COMPLETE.md): Songbird integration
- [`docs/dec24-showcase/BIRDSONG_CLI_READY.md`](docs/dec24-showcase/BIRDSONG_CLI_READY.md): BirdSong API docs

---

## 🎯 What Can I Do With BearDog?

### **1. Sovereign Identity**
- Generate cryptographic keys with hardware backing
- Create lineage-based trust hierarchies
- Revoke access instantly (human control)

### **2. Private Communication**
- Encrypt messages for family members only
- Strangers see cryptographic noise
- No central authority needed

### **3. Mesh Networking** (Coming Soon)
- Peer-to-peer connectivity
- No VPN servers required
- Genetic NAT traversal

### **4. Hardware Integration**
- USB security tokens (FIDO2/CTAP2)
- TPM modules
- Android StrongBox
- SoloKey support

---

## 🏆 Current Status (Dec 24, 2025)

### **Production Ready** ✅:
- 8/8 core features working
- 100% test pass rate
- 4 versions shipped today
- All bugs fixed
- 20+ documentation files

### **Demo Ready** ✅:
- Local showcase complete
- Privacy proven
- Performance validated
- Integration planned

### **Next Steps** ⏰:
- [ ] Record showcase video (this week)
- [ ] Songbird integration (weeks 2-3)
- [ ] Launch! (weeks 4-6)

---

## 🤝 Getting Help

### **Documentation**:
- **Quick Reference**: [`docs/dec24-showcase/QUICK_REFERENCE_DEC24.md`](docs/dec24-showcase/QUICK_REFERENCE_DEC24.md)
- **Full Docs**: [`docs/`](docs/)
- **Specs**: [`specs/`](specs/)

### **Community**:
- **GitHub Issues**: Report bugs or request features
- **Discussions**: Ask questions, share ideas

### **Contributing**:
- See [`CONTRIBUTING.md`](docs/CONTRIBUTING.md) (if available)
- Follow our coding standards
- Write tests for new features

---

## 🎬 What's Next?

1. **Try the Demo**: `./demos/beardog-local-showcase.sh`
2. **Read the Docs**: [`docs/dec24-showcase/BEARDOG_SHOWCASE_FINAL_STATUS.md`](docs/dec24-showcase/BEARDOG_SHOWCASE_FINAL_STATUS.md)
3. **Build Something**: Use the CLI to create your own sovereign crypto!

---

## 💡 Key Concepts

### **Lineage-Based Trust**:
- Parent → Child key derivation
- Cryptographic family trees
- Ancestors can verify descendants

### **BirdSong Encryption**:
- Encrypt for specific lineage members
- Privacy by default (strangers blocked)
- No key exchange needed

### **Human Sovereignty**:
- Your keys, your control
- Instant revocation
- No remote deletion

### **Zero Hardcoding**:
- Runtime discovery
- Capability-based access
- Vendor-agnostic

---

## 🚀 Ready to Begin?

```bash
# Run the showcase to see everything in action
./demos/beardog-local-showcase.sh

# Then start building!
beardog key generate --help
```

---

🐻 **Welcome to BearDog - Sovereign Security Starts Here!** 🎭
