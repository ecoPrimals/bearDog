# Hardware Integration Showcase

This directory contains demos showcasing BearDog's hardware HSM integration capabilities and **LIVE human entropy collection**.

## Demos

### 🎤 **Interactive Human Entropy** (NEW!)
**File**: `demo-human-entropy-interactive.sh`

**LIVE demonstration of**:
- Real-time human interaction entropy collection (keyboard + mouse)
- Interactive terminal UI with progress feedback
- Sovereign key generation from YOUR entropy
- Genetic key mixing (human + device entropy)
- Hierarchical key derivation
- End-to-end encryption/decryption with human keys

**Run**:
```bash
./showcase/02-hardware-integration/demo-human-entropy-interactive.sh
```

**What happens**:
1. Interactive UI prompts you to type and move mouse
2. Collects LIVE entropy from your interactions (NO SIMULATION)
3. Generates a sovereign key from YOUR entropy
4. Creates a device key for comparison
5. Mixes both keys (genetic cryptography)
6. Derives a child key (hierarchical)
7. Encrypts/decrypts data with your human key
8. Shows full key lineage and audit trail

**Duration**: ~5-10 minutes (interactive)  
**Requirements**: Terminal with mouse support  
**Status**: ✅ **PRODUCTION READY**

---

### 🧬 **Genetic Cryptography** (Realistic)
**File**: `demo-genetic-realistic.sh`

Demonstrates genetic key operations:
- Hierarchical key derivation
- Key mixing (2-of-3 threshold)
- Delegated keys with constraints
- Sovereign revocation
- Key lineage visualization

**Run**:
```bash
./showcase/02-hardware-integration/demo-genetic-realistic.sh
```

---

### 🔐 **Real Crypto Operations**
**File**: `demo-real-crypto.sh`

Demonstrates real cryptographic operations:
- Key generation across HSMs
- Encryption/decryption
- Digital signatures
- Performance benchmarks

**Run**:
```bash
./showcase/02-hardware-integration/demo-real-crypto.sh
```

---

### 🏗️ **Hardware Comparison**
**File**: `demo-hybrid.sh`

Compares different HSM types:
- Software HSM (fast, development)
- Hardware HSM (secure, production)
- Mobile HSM (StrongBox, if available)

**Run**:
```bash
./showcase/02-hardware-integration/demo-hybrid.sh
```

---

## Quick Start: Full Showcase Suite

Run all demos automatically:

```bash
./showcase/02-hardware-integration/run-all-demos-auto.sh
```

Or run them individually for interactive experience:

```bash
# Start with human entropy (most impressive!)
./showcase/02-hardware-integration/demo-human-entropy-interactive.sh

# Then explore genetic operations
./showcase/02-hardware-integration/demo-genetic-realistic.sh

# Test crypto performance
./showcase/02-hardware-integration/demo-real-crypto.sh

# Compare hardware
./showcase/02-hardware-integration/demo-hybrid.sh
```

---

## Hardware Requirements

### Minimum (Software HSM)
- Any modern CPU
- 8GB RAM
- Linux/macOS/Windows

### Recommended (Hardware HSM)
- SoloKey V2 or YubiKey 5 (USB)
- Pixel 8a with GrapheneOS (StrongBox)
- TPM 2.0 chip

### For Interactive Entropy
- Terminal with mouse support (most modern terminals)
- Keyboard and mouse input devices
- ~5 minutes of your time for interaction

---

## Demo Outputs

All demos create:
- `showcase/output/` - Session outputs
  - `entropy/` - Entropy seeds (if applicable)
  - `keys/` - Key files
  - `receipts/` - Cryptographic receipts
  - `logs/` - Demo logs

Example structure:
```
showcase/output/human-entropy-1734645678/
├── entropy/
│   └── human-entropy-1734645678.json
├── keys/
│   ├── human-sovereign-key-1734645678
│   ├── device-key-1734645678
│   ├── genetic-mixed-1734645678
│   └── human-child-1734645678
├── receipts/
│   ├── receipt-entropy-collect-*.json
│   ├── receipt-key-generate-*.json
│   ├── receipt-key-mix-*.json
│   └── receipt-key-derive-*.json
├── secret-message.txt
├── secret-message.enc
└── secret-message-decrypted.txt
```

---

## What Makes These Demos Special?

### 🎤 **REAL Human Entropy**
Unlike most demos that simulate or fake human input:
- ✅ Collects REAL keyboard timing (nanosecond precision)
- ✅ Collects REAL mouse movements (jitter, velocity)
- ✅ LiveFeedValidator prevents simulation
- ✅ Privacy-preserved (no keystrokes stored)
- ✅ Each collection is unique (non-fungible)

### 🧬 **Genetic Cryptography**
Demonstrates cutting-edge crypto concepts:
- Parent-child key relationships
- Multi-key mixing (threshold schemes)
- Constraint enforcement (time, CPU, memory)
- Sovereign revocation
- Auditable lineage

### 🔐 **Production Ready**
Not toy examples:
- Real AES-256-GCM encryption
- Real Argon2id key derivation
- Real Ed25519 signatures
- Real hardware HSM support
- Real audit trails

---

## Interactive vs Automated

### Interactive Mode (Recommended)
- Run individual demo scripts
- Follow on-screen prompts
- See explanations at each step
- Better for learning/presentation

### Automated Mode
- Run `run-all-demos-auto.sh`
- No interaction needed
- Good for testing/CI
- Generates validation report

---

## Troubleshooting

### "Entropy collection failed"
- Ensure terminal supports mouse input
- Try typing more naturally (vary speed)
- Check for at least 50 interactions

### "HSM not found"
- Software HSMs always available (no hardware needed)
- USB HSMs require udev rules (see main README)
- Mobile HSMs require Android device

### "Quality too low"
- Type more naturally (don't rush)
- Move mouse while typing
- Aim for 50+ interactions

---

## Next Steps

After running demos:

1. **Review receipts**:
   ```bash
   cat showcase/output/*/receipts/*.json | jq '.'
   ```

2. **Check key lineage**:
   ```bash
   beardog key list
   ```

3. **Validate entropy**:
   ```bash
   beardog entropy info --seed showcase/output/*/entropy/*.json
   ```

4. **Try your own**:
   ```bash
   # Collect your own entropy
   beardog entropy collect --human-input --output my-entropy.json
   
   # Generate your sovereign key
   beardog key generate --key-id my-key --seed my-entropy.json
   ```

---

## Documentation

- **Architecture**: `../../ARCHITECTURE.md`
- **Entropy Hierarchy**: `../../ENTROPY_HIERARCHY_PRINCIPLE.md`
- **Extensibility**: `../../crates/beardog-genetics/src/genetics/human_entropy/EXTENSIBILITY.md`
- **Implementation**: `../../INTERACTIVE_ENTROPY_COMPLETE_DEC_19_2025.md`
- **Success Report**: `../../END_TO_END_SUCCESS_DEC_19_2025.md`

---

**🐻 BearDog: Integrity Over Features**  
*Your Interactions, Your Entropy, Your Sovereignty.*

**Status**: ✅ Production Ready  
**Updated**: December 19, 2025  
**New Feature**: Live Human Entropy Collection 🎤
