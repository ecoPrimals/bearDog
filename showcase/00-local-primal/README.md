# 🐻 Level 0: Local Primal - BearDog Basics

**Status**: ✅ **COMPLETE** - All 6 demos ready  
**Time**: 1 hour total (10 min per demo)  
**Dependencies**: None - standalone learning

---

## 🎯 What is Level 0?

**Level 0** introduces you to BearDog's core capabilities running locally on a single machine. No network, no ecosystem integration, just pure BearDog fundamentals.

**You'll learn**:
- How to generate and manage keys
- How to discover hardware security modules
- How to create self-enforcing genetic keys
- How to mix human and machine entropy
- How to track key lineage and ancestry
- How to establish secure BTSP tunnels

---

## 📚 Available Demos (6 total)

### 01. Hello BearDog ✅
**Time**: 5 minutes  
**Difficulty**: 🟢 Beginner

**What you'll learn**:
- Generate your first cryptographic key
- Use the software HSM
- Basic operations (sign/verify)
- Key metadata and tracking

```bash
cd 01-hello-beardog
./run.sh
```

**Key Takeaway**: BearDog makes key generation sovereign and simple.

---

### 02. HSM Discovery ✅
**Time**: 5 minutes  
**Difficulty**: 🟢 Beginner

**What you'll learn**:
- Auto-discover hardware HSMs (YubiKey, TPM, etc.)
- Compare capabilities
- Zero-knowledge approach (no hardcoded paths!)
- Graceful fallback to software HSM

```bash
cd 02-hsm-discovery
./run.sh
```

**Key Takeaway**: BearDog discovers what's available, never hardcodes.

---

### 03. Key Constraints ✅
**Time**: 10 minutes  
**Difficulty**: 🟡 Intermediate

**What you'll learn**:
- Create self-enforcing genetic keys
- Time constraints (expiration)
- Usage limits (max operations)
- Witness requirements (multi-party)
- Operation restrictions (sign-only, etc.)

```bash
cd 03-key-constraints
./run.sh
```

**Key Takeaway**: Keys that know and enforce their own rules!

---

### 04. Entropy Mixing ✅
**Time**: 10 minutes  
**Difficulty**: 🟡 Intermediate

**What you'll learn**:
- Understand the entropy hierarchy
- Mix machine (60%) + human (40%) entropy
- Quality scoring and validation
- **Why simulation violates trust**

```bash
cd 04-entropy-mixing
./run.sh
```

**Key Takeaway**: Real entropy = Real sovereignty. Never simulate!

⚠️ **Important**: Read `../../docs/references/ENTROPY_HIERARCHY_PRINCIPLE.md` after this demo!

---

### 05. Key Lineage ✅
**Time**: 10 minutes  
**Difficulty**: 🟡 Intermediate

**What you'll learn**:
- Track parent-child key relationships
- Constraint inheritance
- Lineage queries and visualization
- Audit trails

```bash
cd 05-key-lineage
./run.sh
```

**Key Takeaway**: Every key has a traceable history!

---

### 06. BTSP Tunnel ✅
**Time**: 15 minutes  
**Difficulty**: 🟡 Intermediate

**What you'll learn**:
- BTSP protocol concepts
- End-to-end encryption
- Perfect Forward Secrecy
- Mutual authentication
- Sovereign identity (no CAs!)

```bash
cd 06-btsp-tunnel
./run.sh
```

**Key Takeaway**: Secure tunnels without Certificate Authorities!

---

## 🚀 Quick Start

### Run All Demos in Sequence

```bash
# From showcase/00-local-primal/
for demo in 01-hello-beardog 02-hsm-discovery 03-key-constraints 04-entropy-mixing 05-key-lineage 06-btsp-tunnel; do
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "Running: $demo"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  cd $demo
  ./run.sh
  cd ..
  echo ""
  read -p "Press Enter to continue..."
done
```

### Run Individual Demo

```bash
cd 01-hello-beardog  # or any demo
./run.sh
```

---

## 📊 Learning Path

### 🟢 Complete Beginner
**Start here!**

1. **01-hello-beardog** (5 min)
   - Understand what BearDog does
   - Generate your first key
   - Basic operations

2. **02-hsm-discovery** (5 min)
   - See zero-knowledge discovery
   - Understand software vs hardware HSM
   - Capability comparison

**Time**: 10 minutes  
**Result**: Basic BearDog understanding

---

### 🟡 Intermediate User
**After completing beginner demos**

3. **03-key-constraints** (10 min)
   - Self-enforcing genetic keys
   - Constraint types
   - Automatic validation

4. **04-entropy-mixing** (10 min)
   - Entropy hierarchy
   - Why mixing matters
   - Trust model

**Time**: 20 minutes  
**Result**: Understand BearDog's unique features

---

### 🔵 Advanced Concepts
**Complete the level**

5. **05-key-lineage** (10 min)
   - Track key families
   - Constraint inheritance
   - Audit trails

6. **06-btsp-tunnel** (15 min)
   - Secure networking
   - Perfect forward secrecy
   - Sovereign identity

**Time**: 25 minutes  
**Result**: Full Level 0 mastery

---

## 🎓 Key Concepts Learned

After completing Level 0, you understand:

### 1. Sovereign Key Management
- ✅ You control your keys
- ✅ Zero vendor lock-in
- ✅ Hardware HSM support
- ✅ Software HSM fallback

### 2. Genetic Keys
- ✅ Self-enforcing constraints
- ✅ Immutable rules
- ✅ Automatic validation
- ✅ Can't be bypassed

### 3. Entropy Hierarchy
- ✅ Hardware HSMs (quality)
- ✅ System entropy (quality)
- ✅ Real human input (uniqueness)
- ❌ **Never simulate** (trust violation)

### 4. Key Lineage
- ✅ Parent-child relationships
- ✅ Constraint inheritance
- ✅ Progressive restriction
- ✅ Complete audit trail

### 5. BTSP Protocol
- ✅ End-to-end encryption
- ✅ Perfect Forward Secrecy
- ✅ Mutual authentication
- ✅ No Certificate Authorities

---

## 📈 Progress Tracking

```
Level 0 Progress: 🟩🟩🟩🟩🟩🟩 6/6 (100%)

✅ 01-hello-beardog       (5 min)
✅ 02-hsm-discovery       (5 min)
✅ 03-key-constraints     (10 min)
✅ 04-entropy-mixing      (10 min)
✅ 05-key-lineage         (10 min)
✅ 06-btsp-tunnel         (15 min)

Total Time: 55 minutes
Status: COMPLETE
```

---

## 🎯 What's Next?

### Level 1: Hardware Integration
**Location**: `../01-hardware-integration/`

**You'll learn**:
- Real YubiKey integration (PKCS#11)
- TPM 2.0 usage (Linux)
- Android StrongBox (mobile)
- iOS Secure Enclave (mobile)
- HSM performance comparison
- Failover strategies

**Recommended**: If you have hardware HSMs

---

### Level 2: Ecosystem Integration
**Location**: `../02-ecosystem-integration/`

**You'll learn**:
- BTSP tunnels with Songbird
- Encrypted storage with NestGate
- Encrypted compute with ToadStool
- Key routing with Squirrel
- Cross-primal key lineage

**Recommended**: To see BearDog in the ecosystem

---

## 📚 Essential Reading

### Must Read (After Completing Level 0)
1. **../../docs/references/ENTROPY_HIERARCHY_PRINCIPLE.md** ⭐
   - Core principle of BearDog
   - Why simulation violates trust
   - Real vs fake entropy

2. **../../ARCHITECTURE.md**
   - System design
   - Component overview
   - Integration points

3. **../../STATUS.md**
   - Current capabilities
   - Quality metrics
   - Production readiness

### Deep Dives (Optional)
- **../../specs/current/** - Technical specifications
- **../../docs/guides/** - Comprehensive guides
- **../../STATUS.md** - Current status and quality metrics

---

## ❓ Troubleshooting

### "cargo: command not found"
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### "failed to compile"
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

### "Permission denied: ./run.sh"
```bash
# Make executable
chmod +x run.sh
```

### Want more logging?
```bash
RUST_LOG=debug cargo run
```

---

## 🤝 Contributing

Want to improve these demos?

1. **Fix bugs**: Submit PR with fix
2. **Add examples**: Extend existing demos
3. **Improve docs**: Make README clearer
4. **Create tests**: Add validation tests

See `../../README.md` for contribution guidelines.

---

## 🎉 Completion Certificate

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   🐻 BearDog Level 0 - COMPLETE! 🎓
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

You have mastered:
  ✅ Key Generation
  ✅ HSM Discovery
  ✅ Genetic Constraints
  ✅ Entropy Mixing
  ✅ Key Lineage
  ✅ BTSP Tunnels

You understand:
  ✅ Sovereign key management
  ✅ Zero-knowledge discovery
  ✅ Self-enforcing security
  ✅ Entropy hierarchy
  ✅ Perfect forward secrecy

Next: Level 1 (Hardware Integration)

🐻 BearDog: Sovereign. Secure. Self-Enforcing. 🔐
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

**Level Status**: ✅ COMPLETE  
**Total Demos**: 6  
**Total Time**: ~1 hour  
**Difficulty**: 🟢 Beginner to 🟡 Intermediate

🐻 **Welcome to BearDog - You've Got This!** 🎓

