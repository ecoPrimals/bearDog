# 🐻 Hello BearDog

**Level**: 0 (Local Primal)  
**Category**: Getting Started  
**Time**: 5 minutes  
**Dependencies**: None

---

## 🎯 What This Demo Shows

Your first BearDog key generation:
- ✅ Generate a cryptographic key
- ✅ Use the software HSM
- ✅ Understand key storage
- ✅ View key metadata
- ✅ Basic key operations

This is your introduction to BearDog's sovereign key management.

---

## 📋 Prerequisites

**Software**:
```bash
# Rust toolchain (1.85+, Edition 2024)
rustc --version

# BearDog built
cd /home/eastgate/Development/ecoPrimals/beardog
cargo build --release
```

**Knowledge**: None required - complete beginner friendly!

---

## 🚀 Running the Demo

### Option 1: One Command
```bash
./run.sh
```

### Option 2: Manual
```bash
cargo run
```

### Option 3: Step by Step
```bash
# Build
cargo build

# Run
./target/debug/hello-beardog

# Or with release optimizations
cargo run --release
```

---

## 📊 Expected Output

```
🐻 BearDog - Hello World Demo
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Step 1: Initializing BearDog Core
✓ Core initialized

Step 2: Creating Software HSM Provider
✓ Software HSM ready (fallback mode)

Step 3: Generating Your First Key
  Algorithm: Ed25519 (digital signatures)
  Key Size: 256 bits
  Entropy: System (/dev/urandom)
✓ Key generated successfully!

Step 4: Key Information
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Key ID: beardog_ed25519_abc123def456
  Type: Ed25519
  Created: 2025-12-24T20:30:00Z
  Status: Active
  Constraints: None (demonstration key)
  Storage: In-memory (software HSM)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Step 5: Testing Key Operations
  ✓ Sign message: Success
  ✓ Verify signature: Success
  ✓ Export public key: Success

✅ Success! You've created your first BearDog key.

Next Steps:
  → Try: 02-hsm-discovery (find real hardware HSMs)
  → Try: 03-key-constraints (self-enforcing keys)
  → Try: 04-entropy-mixing (add your entropy)
```

---

## 🧠 Understanding the Code

### Step 1: Initialize BearDog
```rust
let core = BearDogCore::new(config)?;
```
Creates the BearDog core with default configuration.

### Step 2: Create HSM Provider
```rust
let software_hsm = SoftwareHsm::new()?;
```
Software HSM = Pure Rust implementation (no hardware needed).

### Step 3: Generate Key
```rust
let key = software_hsm.generate_key(
    KeyType::Ed25519,
    KeyConstraints::default(),
)?;
```
- **Ed25519**: Fast, secure digital signatures
- **KeyConstraints**: Rules the key must follow (none for this demo)

### Step 4: View Metadata
```rust
println!("Key ID: {}", key.id());
println!("Type: {:?}", key.key_type());
```
Every key has metadata tracked by BearDog.

### Step 5: Use the Key
```rust
let signature = key.sign(message)?;
let valid = key.verify(message, &signature)?;
```
Sign a message and verify the signature.

---

## 🔍 What's Happening Under the Hood

### Software HSM
- Pure Rust implementation
- No hardware required
- Perfect for development
- Uses system entropy (/dev/urandom)
- Keys stored in protected memory

### Ed25519 Algorithm
- Modern elliptic curve cryptography
- 256-bit security
- Fast signing and verification
- Small signatures (64 bytes)
- Industry standard (SSH, TLS, etc.)

### Key Lifecycle
```
Generate → Store → Use → (optional) Destroy
```

### BearDog's Role
- **Generate**: Creates key with proper entropy
- **Store**: Protects key in HSM
- **Track**: Maintains metadata and lineage
- **Enforce**: Applies constraints (if any)
- **Audit**: Logs all operations

---

## 🎓 Key Concepts

### 1. Sovereign Keys
- **You control**: Keys never leave your infrastructure
- **You decide**: When and how keys are used
- **You verify**: All operations are auditable
- **No vendor lock-in**: Universal HSM support

### 2. Software HSM
- **Development**: Perfect for learning and testing
- **Production**: Can use real hardware (YubiKey, TPM, etc.)
- **Fallback**: Always available when hardware isn't
- **Compatible**: Same API as hardware HSMs

### 3. Key Constraints
- **Self-enforcing**: Keys enforce their own rules
- **Genetic**: Constraints are part of key DNA
- **Immutable**: Can't be changed after creation
- **Examples**: Expiration, usage limits, allowed operations

---

## 🚀 Next Steps

### Continue Learning
1. **02-hsm-discovery** - Find real hardware HSMs
2. **03-key-constraints** - Create self-enforcing keys
3. **04-entropy-mixing** - Add your human entropy
4. **05-key-lineage** - Track key ancestry
5. **06-btsp-tunnel** - Secure encrypted connections

### Try Yourself
```rust
// Change key algorithm
KeyType::Aes { key_size: 256 }  // Symmetric encryption

// Add constraints
KeyConstraints {
    expires_at: Some(now + Duration::days(30)),
    max_uses: Some(1000),
    ..Default::default()
}

// Try different operations
key.encrypt(plaintext)?;
key.decrypt(ciphertext)?;
```

### Read More
- [../../README.md](../../../README.md) - Project overview
- [../../ARCHITECTURE.md](../../../ARCHITECTURE.md) - System design
- [../../docs/](../../../docs/) - Comprehensive documentation

---

## ❓ Troubleshooting

### Error: "Failed to initialize core"
```bash
# Check permissions
ls -la ~/.config/beardog/

# Create config directory
mkdir -p ~/.config/beardog/
```

### Error: "Software HSM initialization failed"
```bash
# Verify Rust crypto dependencies
cargo clean
cargo build
```

### No output
```bash
# Run with verbose logging
RUST_LOG=debug cargo run
```

### Want more details?
```bash
# Enable info logging
RUST_LOG=info cargo run
```

---

## 📚 Related Documentation

- **ENTROPY_HIERARCHY_PRINCIPLE.md** - Why entropy matters
- **STATUS.md** - BearDog capabilities
- **examples/key_generation.rs** - More examples

---

## 🎯 Success Criteria

After completing this demo, you should understand:
- ✅ How to generate a BearDog key
- ✅ What a software HSM is
- ✅ Basic key operations (sign/verify)
- ✅ Key metadata and tracking
- ✅ BearDog's sovereignty model

---

**Demo Status**: ✅ Complete  
**Difficulty**: 🟢 Beginner  
**Time Required**: 5 minutes

🐻 **Welcome to BearDog - Sovereign Cryptography Made Simple!** 🔐

