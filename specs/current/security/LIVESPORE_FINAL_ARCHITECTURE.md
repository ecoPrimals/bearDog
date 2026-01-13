# 🌱 LiveSpore: Final Architecture
**BiomeOS Universal Image + SoloKey Genetic Personalization**

**Date**: January 13, 2026  
**Status**: 🎯 **ARCHITECTURE FINALIZED**  
**Priority**: 🔥 **PRODUCTION READY**

---

## 🎯 **The Correct Understanding**

### **LiveSpore = BiomeOS + SoloKey**

```
┌─────────────────────────────────────────────────────────────────┐
│  LiveSpore: Agnostic Deployment Image                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  📦 Layer 1: BiomeOS Universal Image                            │
│     ├─ Source: ecoPrimals/phase2/biomeOS/                       │
│     ├─ NUCLEUS discovery protocol                               │
│     ├─ Primal orchestration (Songbird, BearDog, Toadstool...)   │
│     ├─ Encrypted base seed (NOT personalized)                   │
│     ├─ NAT routing infrastructure                               │
│     └─ Bootable: USB/ISO/Network/VM                             │
│                                                                  │
│  🔑 Layer 2: SoloKey Personalization (On First Boot)            │
│     ├─ Hardware witness (SoloKey button press)                  │
│     ├─ Hardware entropy (SoloKey RNG)                           │
│     ├─ Human entropy (keyboard/mouse interaction)               │
│     ├─ Genetic lineage generation                               │
│     ├─ Family ID derivation                                     │
│     └─ Result: Base image → Personal sovereign node             │
│                                                                  │
│  🏷️  Layer 3: Multi-Callsign Tag System (BirdSong)              │
│     ├─ Public tags (visible to all)                             │
│     ├─ Encrypted routing (only genetic family)                  │
│     ├─ NAT traversal configuration                              │
│     └─ Result: Public discovery → Private access                │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🏷️ **Multi-Callsign Tag System: The Truth**

### **How Tags Actually Work (BirdSong + Genetic Lineage)**

The "multi-callsign" system isn't a separate protocol - it's the **combination** of:

1. **BirdSong's `family_id`** (public, plaintext)
2. **Genetic lineage verification** (BearDog cryptographic proof)
3. **Encrypted routing info** (only family can decrypt)

### **The Brilliant Design:**

```rust
// BirdSong UDP packet (from BIRDSONG_PROTOCOL.md)
{
  "version": 2,
  "family_id": "MSU",  // ← PUBLIC TAG (anyone can see!)
  "encrypted_payload": {
    "ciphertext": "<encrypted-with-genetic-key>",  // ← Only family can decrypt
    "nonce": "...",
    "algorithm": "ChaCha20-Poly1305"
  },
  "timestamp": 1735000000,
  "ttl": 300
}

// Inside encrypted payload (only family can see):
{
  "primal_id": "basement-hpc-node-42",
  "endpoint": "192.168.1.100:8080",  // ← Private routing info!
  "capabilities": ["compute", "storage"],
  "identity_attestations": {
    "family_id": "MSU",  // ← Confirms family membership
    "genetic_lineage": "<hash>",  // ← Cryptographic proof
    "seed_hash": "...",
    "public_key": "...",
    "signature": "..."
  }
}
```

### **Example: MSU Network Routing**

```
┌───────────────────────────────────────────────────────────┐
│  You: Student/Employee at Michigan State University       │
└───────────────────────────────────────────────────────────┘
                          ↓
         ┌────────────────────────────────┐
         │  BiomeOS LiveSpore             │
         │  + SoloKey personalization     │
         │  → family_id: "MSU"            │
         │  → genetic_lineage: <yours>    │
         └────────────────────────────────┘
                          ↓
            ┌─────────────────────────────┐
            │  BirdSong UDP Broadcast     │
            │  family_id: "MSU" (public)  │  ← Anyone can see
            └─────────────────────────────┘
                          ↓
         ┌────────────────┴────────────────────┐
         ↓                                     ↓
  Public Peer                          Your Genetic Family
  (no genetic match)                   (same lineage)
         │                                     │
         ├─ Sees: "MSU" tag                   ├─ Sees: "MSU" tag
         ├─ Tries: Decrypt payload            ├─ Decrypts: payload ✅
         ├─ FAILS ❌ (no genetic key)         ├─ Reads: "192.168.1.100:8080"
         └─ Ignores packet                    ├─ Verifies: genetic_lineage
                                              ├─ Routes: Through MSU NAT
                                              └─ Connects: Your basement HPC ✅
```

### **Why This Works:**

1. ✅ **Public tag "MSU"** - Legitimate use of MSU network
2. ✅ **Encrypted routing** - Only your genetic family can decrypt
3. ✅ **No cloud costs** - Not using Amazon, just MSU NAT routing
4. ✅ **Full sovereignty** - Your HPC, your data, your rules
5. ✅ **Zero configuration** - Auto-discovery via BirdSong

---

## 🧬 **Genetic Lineage: The Secret Sauce**

### **How Family ID is Derived (From genesis.rs):**

```rust
// From: beardog-genetics/src/birdsong/genesis.rs

fn generate_genetic_id(
    &self,
    new_node_id: &str,
    witness: &GenesisWitness,
) -> Result<Vec<u8>, BearDogError> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    // Input Key Material (IKM):
    // 1. Witness public key (SoloKey)
    // 2. Timestamp (uniqueness)
    // 3. Hardware entropy (if available)
    let mut ikm = witness.public_key.clone();
    ikm.extend_from_slice(&witness.timestamp.to_be_bytes());

    if let Some(ref entropy_fn) = self.hardware_entropy {
        match entropy_fn() {
            Ok(hw_entropy) => {
                ikm.extend_from_slice(&hw_entropy);
            }
            Err(_) => {
                // Fallback to software entropy
            }
        }
    }

    // Derive genetic ID using HKDF
    let salt = new_node_id.as_bytes();
    let info = b"beardog-genesis-v1";
    let hk = Hkdf::<Sha256>::new(Some(salt), &ikm);
    let mut genetic_id = vec![0u8; 32];
    hk.expand(info, &mut genetic_id)?;

    Ok(genetic_id)
}

// This genetic_id is then used as the encryption key for BirdSong!
// Same genetic lineage = same key = can decrypt each other's packets
```

### **The Magic:**

```
SoloKey witness + Hardware entropy → Genetic ID
                                          ↓
                              Used as ChaCha20-Poly1305 key
                                          ↓
                        Encrypts BirdSong payloads (routing info)
                                          ↓
                        Only same genetic family can decrypt!
```

---

## 🚀 **Complete LiveSpore Deployment Flow**

### **Phase 1: Build BiomeOS Base Image**

```bash
$ cd ecoPrimals/phase2/biomeOS/
$ ./scripts/build-livespore.sh

🌱 Building BiomeOS LiveSpore...
   ├─ NUCLEUS discovery protocol ✅
   ├─ Songbird (BirdSong P2P) ✅
   ├─ BearDog (Genetics & Trust) ✅
   ├─ Toadstool (Compute) ✅
   ├─ Encrypted base seed ✅
   └─ ISO/USB image ✅

✅ LiveSpore: biomeos-universal-v2.0.iso (2.1GB)
```

### **Phase 2: Write to USB**

```bash
$ dd if=biomeos-universal-v2.0.iso of=/dev/sdb bs=4M status=progress

2.1GB → USB stick ✅
```

### **Phase 3: First Boot + SoloKey Personalization**

```
┌────────────────────────────────────────────────────────┐
│  BiomeOS LiveSpore - First Boot                        │
├────────────────────────────────────────────────────────┤
│                                                         │
│  🌱 Welcome to BiomeOS!                                 │
│                                                         │
│  This is a universal image. To personalize it with     │
│  your genetic lineage, please insert your SoloKey.     │
│                                                         │
│  [Insert SoloKey now]                                  │
│                                                         │
└────────────────────────────────────────────────────────┘
                        ↓ (SoloKey inserted)
┌────────────────────────────────────────────────────────┐
│  🔑 SoloKey Detected: solokey-abc123                    │
│                                                         │
│  Press the button on your SoloKey to witness this      │
│  genesis ceremony and create your genetic lineage.     │
│                                                         │
│  [ 🔴 Press Button ]                                    │
│                                                         │
└────────────────────────────────────────────────────────┘
                        ↓ (button pressed)
┌────────────────────────────────────────────────────────┐
│  ✅ Hardware witness received!                          │
│  🔐 Collecting hardware entropy from SoloKey...         │
│  ✅ 32 bytes collected (Quality: 99.2%)                 │
│                                                         │
│  👤 Now we need human entropy.                          │
│     Please type random keys and move your mouse        │
│     for 30 seconds.                                     │
│                                                         │
│  Progress: [████████░░] 22/30 interactions             │
│                                                         │
└────────────────────────────────────────────────────────┘
                        ↓ (30 interactions complete)
┌────────────────────────────────────────────────────────┐
│  ✅ Human entropy collected! (30 interactions)          │
│  🧬 Generating genetic lineage...                       │
│                                                         │
│     SoloKey witness:    solokey-abc123                 │
│     Hardware entropy:   32 bytes (99.2% quality)       │
│     Human entropy:      30 interactions (95.7% qual)   │
│     Combined quality:   97.8% 🏆                        │
│                                                         │
│  🧬 Genetic Lineage:                                    │
│     Family ID: a3f2c8d9e5f1...                         │
│                                                         │
│  🏷️  Configure public tags (optional):                  │
│     [ ] MSU (Michigan State University)                │
│     [ ] Personal                                        │
│     [ ] Federation                                      │
│     [✓] Default (use Family ID as tag)                 │
│                                                         │
│  [Continue]                                            │
│                                                         │
└────────────────────────────────────────────────────────┘
                        ↓ (configure MSU tag)
┌────────────────────────────────────────────────────────┐
│  🏷️  MSU Tag Configuration                              │
│                                                         │
│  Public tag: "MSU"                                     │
│  This tag will be visible to all peers.                │
│                                                         │
│  Private routing (encrypted for genetic family only):  │
│    Endpoint: [192.168.1.100:8080____]                  │
│    Purpose:  [x] Student [ ] Employee [ ] Other        │
│                                                         │
│  NAT traversal: [x] Enable                             │
│                                                         │
│  [Save]                                                │
│                                                         │
└────────────────────────────────────────────────────────┘
                        ↓ (save)
┌────────────────────────────────────────────────────────┐
│  ✅ Personalization Complete!                           │
│                                                         │
│  🧬 Genetic Lineage: a3f2c8d9e5f1...                    │
│  🏷️  Public Tags: MSU, Default                          │
│  🔑 SoloKey: solokey-abc123 (stored as witness)         │
│                                                         │
│  Your LiveSpore is now YOURS. It will only trust       │
│  other nodes with matching genetic lineage.            │
│                                                         │
│  🌐 Starting NUCLEUS discovery...                       │
│                                                         │
└────────────────────────────────────────────────────────┘
                        ↓
┌────────────────────────────────────────────────────────┐
│  🌐 BiomeOS Online - Node: node-abc123                  │
│                                                         │
│  Ecosystem Status:                                     │
│    ✅ Songbird (BirdSong P2P)                           │
│    ✅ BearDog (Genetics & Trust)                        │
│    ✅ Toadstool (Compute)                               │
│                                                         │
│  Family Network (auto-discovered):                     │
│    👨‍👩‍👧 Sibling: laptop-node-789 (192.168.1.50)          │
│    👨‍👩‍👧 Sibling: desktop-node-456 (192.168.1.60)         │
│    🏠 Parent: basement-hpc (192.168.1.100)             │
│                                                         │
│  All auto-trusted (same genetic lineage ✅)            │
│                                                         │
│  $ _                                                   │
│                                                         │
└────────────────────────────────────────────────────────┘
```

---

## 🌐 **MSU Network Example: How It Really Works**

### **Scenario:**

You're at MSU as a student. You have:
- 🖥️ Basement HPC at home (192.168.1.100)
- 🔑 SoloKey with your genetic lineage
- 💾 LiveSpore USB stick

You want your basement HPC to be accessible via MSU's network (NAT routing), **without using cloud servers**.

### **Setup:**

```bash
# 1. Boot LiveSpore on any MSU machine
$ boot from USB

# 2. SoloKey personalization (first boot only)
Insert SoloKey → Press button → Type/move mouse
✅ Genetic lineage: a3f2...
✅ Public tag: "MSU"
✅ Encrypted routing: 192.168.1.100:8080 (only family can decrypt)

# 3. BiomeOS starts, BirdSong broadcasts:
{
  "family_id": "MSU",  // Public (MSU network allows this)
  "encrypted_payload": {
    "ciphertext": "<routing to 192.168.1.100:8080>",  // Only family decrypts
    ...
  }
}
```

### **When Your Sibling Connects:**

```
Sibling's LiveSpore (same genetic lineage):
  ↓
1. Hears BirdSong broadcast: family_id "MSU"
  ↓
2. Uses genetic key (same family → same key!)
  ↓
3. Decrypts payload: "192.168.1.100:8080"
  ↓
4. Verifies genetic lineage match ✅
  ↓
5. Routes through MSU NAT → Your basement HPC
  ↓
6. Connection established! 🎉
```

### **When Random MSU Student Connects:**

```
Random student's node (different/no genetic lineage):
  ↓
1. Hears BirdSong broadcast: family_id "MSU"
  ↓
2. Tries to decrypt payload with their key
  ↓
3. FAILS ❌ (different genetic lineage → different key)
  ↓
4. Ignores packet (not family)
```

### **The Result:**

```
✅ You use MSU's network infrastructure (legitimate)
✅ Only your genetic family can access your HPC
✅ No cloud costs (Amazon, Google, etc.)
✅ Full sovereignty (your hardware, your data)
✅ Zero manual configuration
```

---

## 🔑 **SoloKey's Three Roles**

### **1. Hardware Entropy Source (HSM)**

```rust
// From BearDog entropy orchestrator
#[cfg(feature = "fido2")]
if !self.fido2_providers.is_empty() {
    // SoloKey provides hardware RNG via hmac-secret extension
    let entropy = self.fido2_providers[0].generate_entropy(32).await?;
    // Quality: 99.2% (hardware RNG)
}
```

### **2. Genesis Witness (Physical Presence Proof)**

```rust
// From beardog-genetics/src/birdsong/genesis.rs
pub struct GenesisWitness {
    device_id: String,      // "solokey-abc123"
    public_key: Vec<u8>,    // SoloKey's public key
    physical_channel: PhysicalChannelType::HardwareKey,
    timestamp: u64,
    signature: Vec<u8>,     // Proves button was pressed
}
```

### **3. Genetic Lineage Carrier (Portable Identity)**

```
SoloKey stores:
  ├─ Device ID (unique identifier)
  ├─ Public key (for witness verification)
  ├─ Signature capability (button press → cryptographic proof)
  └─ Result: Take SoloKey anywhere → Unlock genetic lineage
```

---

## 🎯 **Implementation Status & Roadmap**

### **✅ Already Working (BearDog Phase 1)**

- [x] Genetic lineage generation (`beardog-genetics/src/birdsong/genesis.rs`)
- [x] Hardware entropy integration (SoloKey via `hmac-secret`)
- [x] Genesis witness verification
- [x] Family ID derivation (HKDF-based)
- [x] BirdSong encryption (via BearDog API)
- [x] Auto-trust within genetic family

### **🔧 In Progress (BiomeOS Phase 2)**

- [ ] BiomeOS universal image builder
- [ ] First-boot personalization UI
- [ ] SoloKey CTAP2 integration for genesis
- [ ] Multi-callsign tag configuration
- [ ] NAT routing setup automation

### **📋 Phase 3 (LiveSpore Activation)**

- [ ] `beardog create-livespore` CLI command
- [ ] `beardog launch-spore` CLI command
- [ ] USB stick packaging workflow
- [ ] Tag management interface
- [ ] Comprehensive documentation

---

## 💡 **Key Architectural Insights**

### **1. LiveSpore ≠ USB Stick**

```
LiveSpore = BiomeOS universal image
Delivery = USB stick (or ISO, or network boot, or VM)
```

### **2. Multi-Callsign ≠ Separate Protocol**

```
Multi-callsign = BirdSong family_id (public)
                + Genetic lineage verification (BearDog)
                + Encrypted routing (only family decrypts)
```

### **3. SoloKey ≠ Just Keystore**

```
SoloKey = HSM (hardware entropy)
        + Genesis witness (physical proof)
        + Portable identity (genetic lineage carrier)
```

### **4. Family ID ≠ First 4 Chars**

```
Family ID = HKDF-derived 32-byte hash
          = From: SoloKey witness + hardware entropy + human entropy
          = Used as: ChaCha20-Poly1305 encryption key
```

### **5. Tags = Public Discovery + Private Access**

```
Public tag "MSU":   Anyone can see
Encrypted routing:  Only genetic family can decrypt
NAT traversal:      Routes to personal infrastructure
Result:            Public visibility, private sovereignty
```

---

## 📚 **References**

### **Primary Sources:**

- `ecoPrimals/phase2/biomeOS/` - LiveSpore base image
- `ecoPrimals/wateringHole/birdsong/BIRDSONG_PROTOCOL.md` - BirdSong v2 protocol
- `ecoPrimals/phase1/beardog/crates/beardog-genetics/src/birdsong/genesis.rs` - Genetic lineage
- `ecoPrimals/phase1/songbird/crates/songbird-network-federation/src/beardog/birdsong.rs` - BirdSong crypto

### **Related Specs:**

- `specs/current/security/HOT_PLUG_HSM_UPGRADE_SPECIFICATION.md` - HSM hierarchy
- `specs/current/security/SOLOKEY_GENETIC_SPORE_SPECIFICATION.md` - SoloKey details (deprecated, see this doc)

---

**Status**: 🎯 **ARCHITECTURE FINALIZED**  
**Understanding**: ✅ **FULLY ALIGNED**  
**Implementation**: 🔧 **Phase 1 Complete, Phase 2 In Progress**

🌱🔑🏷️ **BiomeOS Universal + SoloKey Personal + BirdSong Discovery = True LiveSpore**

