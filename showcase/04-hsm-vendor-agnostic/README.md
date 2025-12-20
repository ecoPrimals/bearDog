# 🔐 HSM Vendor-Agnostic Showcase

**Claim:** "BearDog is 100% vendor-agnostic - works with ANY HSM without code changes"  
**Spec:** `specs/current/security/UNIVERSAL_HSM_SPECIFICATION.md`  
**Status:** 🚧 Building demonstrations

---

## 🎯 What We're Proving

### **The Bold Claim:**
BearDog's Universal HSM Architecture provides **true vendor independence**. You can:
- Use any HSM vendor without code changes
- Switch HSMs at runtime
- Mix multiple HSM vendors in one system
- Automatically discover and select best HSM
- Gracefully fall back when hardware unavailable

### **Why This Matters:**
- **No Vendor Lock-In:** Never trapped with one HSM vendor
- **Future-Proof:** Easy to adopt new HSMs as they emerge
- **Cost Optimization:** Use what you have, not what vendor requires
- **Flexibility:** Mix hardware, software, mobile, cloud HSMs

---

## 📋 Demonstrations

### **Demo 1: Universal HSM Discovery**
**Script:** `demos/01-discover-all-hsms.sh`  
**Status:** ✅ READY

**What It Proves:**
- BearDog automatically discovers ALL available HSMs
- Detects capabilities of each HSM
- No configuration needed (zero-config)

**Expected Output:**
```
Discovered 5 HSMs:
  1. BearDog Native Software HSM (v1.0.0)
     - Capabilities: AES-256-GCM, ChaCha20-Poly1305, Ed25519
     - Type: Software
     - Status: Available
  
  2. SoftHSM2 (v2.6.1)
     - Capabilities: AES, RSA, ECDSA, HMAC
     - Type: Software (PKCS#11)
     - Status: Available
  
  3. Solo V2 Hacker (Serial: 12345678)
     - Capabilities: Ed25519, ECDSA-P256, HMAC
     - Type: Hardware USB
     - Status: Available
  
  4. Pixel 8a StrongBox (Android 14)
     - Capabilities: AES-256-GCM, ECDSA-P256, HMAC
     - Type: Mobile Hardware
     - Status: Available
  
  5. TPM 2.0 (Manufacturer: Intel)
     - Capabilities: RSA-2048, ECDSA-P256, HMAC
     - Type: Platform Hardware
     - Status: Available
```

---

### **Demo 2: Runtime HSM Switching**
**Script:** `demos/02-runtime-hsm-switch.sh`  
**Status:** 🚧 IN PROGRESS

**What It Proves:**
- Generate key on Software HSM
- Export key (encrypted)
- Import to Hardware HSM (Solo V2)
- Use Hardware HSM for operations
- Switch back to Software
- **No code changes, just configuration**

**Expected Flow:**
```
1. Generate key "test-key" on Software HSM
   ✅ Key created: test-key (Software HSM)

2. Encrypt message with Software HSM
   ✅ Encrypted 100 bytes (Software HSM)

3. Export key (encrypted with transport key)
   ✅ Key exported: test-key.enc

4. Import key to Solo V2
   ✅ Key imported to Solo V2

5. Encrypt message with Solo V2
   ✅ Encrypted 100 bytes (Hardware HSM - Solo V2)

6. Switch back to Software
   ✅ Using Software HSM

7. Decrypt both messages
   ✅ Both decrypt successfully!
   ✅ Proof: HSM switching works!
```

---

### **Demo 3: Multi-HSM Operations**
**Script:** `demos/03-multi-hsm-operations.sh`  
**Status:** ✅ VERIFIED

**What It Proves:**
- Use MULTIPLE HSMs in ONE workflow
- Key A on Software HSM
- Key B on Hardware HSM (Solo V2)
- Key C on Mobile HSM (Pixel 8a StrongBox)
- Mix all keys genetically
- Prove all HSMs worked together

**Expected Flow:**
```
1. Generate Key A (Software HSM)
   ✅ Key A: AES-256-GCM on Software

2. Generate Key B (Solo V2)
   ✅ Key B: Ed25519 on Hardware USB

3. Generate Key C (Pixel 8a StrongBox)
   ✅ Key C: ECDSA-P256 on Mobile Hardware

4. Mix keys genetically
   ✅ Mixed Key: Combines A + B + C
   ✅ Uses capabilities from all HSMs!

5. Encrypt with mixed key
   ✅ Encrypted successfully
   ✅ Required: Software + Solo + StrongBox

6. Receipts show all HSMs used
   ✅ Receipt references: 3 HSMs
   ✅ Full audit trail
```

---

### **Demo 4: HSM Comparison**
**Script:** `demos/04-hsm-comparison.sh`  
**Status:** 📋 PLANNED

**What It Proves:**
- Compare performance across HSMs
- Compare security levels
- Compare capabilities
- Show when to use each HSM

**Expected Output:**
```
ENCRYPTION PERFORMANCE (1MB file):
┌─────────────────────┬─────────┬──────────┬────────────┐
│ HSM                 │ Time    │ Speed    │ Security   │
├─────────────────────┼─────────┼──────────┼────────────┤
│ Software (Native)   │ 18ms    │ 55 MB/s  │ High       │
│ Software (SoftHSM2) │ 24ms    │ 42 MB/s  │ High       │
│ Hardware (Solo V2)  │ 120ms   │ 8 MB/s   │ Maximum    │
│ Mobile (StrongBox)  │ 85ms    │ 12 MB/s  │ Very High  │
│ Platform (TPM)      │ 200ms   │ 5 MB/s   │ High       │
└─────────────────────┴─────────┴──────────┴────────────┘

RECOMMENDATIONS:
  • Bulk encryption: Software (fastest)
  • Signing operations: Hardware (maximum security)
  • Mobile scenarios: StrongBox (hardware-backed)
  • Boot/system keys: TPM (platform-integrated)
```

---

### **Demo 5: Automatic HSM Selection**
**Script:** `demos/05-auto-hsm-selection.sh`  
**Status:** 📋 PLANNED

**What It Proves:**
- BearDog automatically selects best HSM for task
- Selection based on requirements (speed, security, availability)
- Graceful fallback when preferred HSM unavailable

**Expected Flow:**
```
SCENARIO 1: High-security operation
  Requirement: Maximum security, signing key
  Selected: Solo V2 (Hardware USB)
  Reason: Highest security available

SCENARIO 2: High-speed operation
  Requirement: Bulk encryption, >50MB/s
  Selected: Software (Native)
  Reason: Fastest available

SCENARIO 3: Mobile operation
  Requirement: Mobile context, hardware-backed
  Selected: Pixel 8a StrongBox
  Reason: Mobile hardware available

SCENARIO 4: Hardware unavailable
  Requirement: Maximum security
  Available: Software only
  Selected: Software (Native)
  Fallback: YES
  Warning: Using software fallback (hardware unavailable)
```

---

### **Demo 6: PKCS#11 Integration**
**Script:** `demos/06-pkcs11-integration.sh`  
**Status:** 📋 PLANNED

**What It Proves:**
- BearDog works with ANY PKCS#11 HSM
- No vendor-specific code
- Standard interface

**Expected Flow:**
```
1. Detect PKCS#11 HSMs
   ✅ Found: SoftHSM2 (slot 0)
   ✅ Found: YubiKey 5 (slot 1)
   ✅ Found: Nitrokey 3 (slot 2)

2. Generate key on YubiKey via PKCS#11
   ✅ Key created (PKCS#11 slot 1)

3. Sign message
   ✅ Signed with YubiKey

4. Verify signature
   ✅ Signature valid

5. Show audit trail
   ✅ Receipt shows: PKCS#11 YubiKey 5
   ✅ Full provenance
```

---

## 🏗️ Architecture Demonstrated

### **Universal HSM Manager:**
```
┌─────────────────────────────────────────────────────────┐
│              APPLICATION LAYER                          │
│  "I need to sign this message with max security"       │
└─────────────────┬───────────────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────────────┐
│          UNIVERSAL HSM MANAGER                          │
│  • Discovers: All available HSMs                        │
│  • Evaluates: Security level, speed, capabilities       │
│  • Selects: Best HSM for requirement                    │
│  • Fallback: Graceful degradation                       │
└─────────────────┬───────────────────────────────────────┘
                  │
        ┌─────────┼─────────┬─────────┬─────────┐
        │         │         │         │         │
┌───────▼───┐ ┌──▼──────┐ ┌▼────────┐ ┌▼───────┐ ┌▼──────┐
│ Software  │ │ Solo V2 │ │StrongBox│ │  TPM   │ │PKCS#11│
│    HSM    │ │ (USB)   │ │ (Mobile)│ │(Platform│ │ (Any) │
└───────────┘ └─────────┘ └─────────┘ └────────┘ └───────┘
```

**Key Insight:** Application never knows WHICH HSM is used, only that requirements are met.

---

## 📊 Success Metrics

| Metric | Target | Demo |
|--------|--------|------|
| HSM vendors supported | 5+ | Demo 1 |
| Runtime switching | Yes | Demo 2 |
| Multi-HSM workflows | Yes | Demo 3 |
| Performance comparison | Yes | Demo 4 |
| Automatic selection | Yes | Demo 5 |
| PKCS#11 compliance | Yes | Demo 6 |
| Code changes for new HSM | 0 | All demos |

---

## 🚀 Running the Demos

### **Prerequisites:**
```bash
# Build BearDog
cd /home/eastgate/Development/ecoPrimals/beardog
cargo build --workspace --release

# Verify beardog binary
./target/debug/beardog --version
```

### **Run Demo 1 (Discovery):**
```bash
cd showcase/04-hsm-vendor-agnostic
./demos/01-discover-all-hsms.sh
```

**Duration:** ~5 minutes  
**Hardware:** None required (software HSMs work)

### **Run Demo 2 (Switching):**
```bash
./demos/02-runtime-hsm-switch.sh
```

**Duration:** ~10 minutes  
**Hardware:** Optional (Solo V2 or YubiKey)

### **Run All Demos:**
```bash
./run-all-hsm-demos.sh
```

**Duration:** ~30-40 minutes  
**Hardware:** Full demo needs at least 2 HSMs

---

## 🎯 Value Proposition

### **Without Universal HSM (Traditional):**
```rust
// Code locked to specific HSM
let hsm = YubiKeyHSM::new()?;
let key = hsm.generate_key()?;

// Want to switch to Nitrokey?
// REWRITE EVERYTHING! 😱
```

### **With BearDog Universal HSM:**
```bash
# Generate key (auto-selects best HSM)
beardog key generate --key-id my-key

# Switch HSM in config (no code changes!)
export BEARDOG_PREFERRED_HSM=solo-v2

# Same command, different HSM!
beardog key generate --key-id my-key
```

**Result:** True vendor independence!

---

## 📚 Related Specifications

- `specs/current/security/UNIVERSAL_HSM_SPECIFICATION.md` - Architecture
- `specs/current/security/MULTI_PROTOCOL_HSM_SPECIFICATION.md` - Protocol support
- `specs/current/security/HARDWARE_INTEGRATION_IMPLEMENTATION_STATUS_NOV_2025.md` - Hardware status

---

## 🏆 Why This Matters

**Vendor Lock-In is EXPENSIVE:**
- Switching costs: 6-12 months
- Re-training: Entire team
- Code changes: Thousands of lines
- Risk: High (migration failures)

**BearDog's Solution:**
- Switching cost: ~5 minutes (config change)
- Re-training: Zero (same API)
- Code changes: Zero
- Risk: Low (same codebase)

**This is the future of HSM integration.** 🚀

---

**🐻 BearDog: Universal HSM Architecture**

*No vendor lock-in. Ever.*

