# 🧬 Key Constraints - Self-Enforcing Genetic Keys

**Level**: 0 (Local Primal)  
**Category**: Advanced Key Management  
**Time**: 10 minutes  
**Dependencies**: None

---

## 🎯 What This Demo Shows

Create keys with **genetic constraints** that self-enforce their own rules:
- ✅ Expiration dates
- ✅ Usage limits
- ✅ Required witnesses
- ✅ Allowed operations
- ✅ Automatic validation
- ✅ Constraint violation handling

This is BearDog's **genetic key** innovation - keys that know and enforce their own rules.

---

## 🚀 Running the Demo

```bash
./run.sh
```

---

## 📊 Expected Output

```
🧬 BearDog - Key Constraints Demo
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Creating keys with different constraints...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Key 1: Time-Limited Key
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Constraints:
  ✓ Expires: 2025-12-31T23:59:59Z (7 days)
  ✓ Usage Limit: Unlimited
  ✓ Operations: All allowed

Testing operations:
  ✓ Sign: Success (within expiration)
  ✓ Verify: Success

Fast-forwarding time to 2026-01-01...
  ❌ Sign: REJECTED (key expired)
  → Constraint enforced automatically!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Key 2: Usage-Limited Key
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Constraints:
  ✓ Expires: Never
  ✓ Usage Limit: 3 operations
  ✓ Operations: Sign only

Testing operations:
  Operation 1: ✓ Sign (2 remaining)
  Operation 2: ✓ Sign (1 remaining)
  Operation 3: ✓ Sign (0 remaining)
  Operation 4: ❌ REJECTED (usage limit exceeded)
  → Constraint enforced automatically!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Key 3: Witness-Required Key
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Constraints:
  ✓ Requires: 2 of 3 witnesses
  ✓ Witnesses: Alice, Bob, Charlie

Testing operations:
  Without witnesses: ❌ REJECTED (no witnesses)
  With Alice only: ❌ REJECTED (need 2 witnesses)
  With Alice + Bob: ✓ Success (threshold met)
  → Multi-party control enforced!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Key 4: Operation-Restricted Key
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Constraints:
  ✓ Allowed: Sign only (no encryption)

Testing operations:
  ✓ Sign: Success (allowed)
  ❌ Encrypt: REJECTED (operation not allowed)
  → Operation restrictions enforced!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Summary
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All constraint types demonstrated
✅ Self-enforcement working correctly
✅ Violations properly rejected

Key Insight:
  These constraints are part of the key's "genetic code"
  They cannot be changed or bypassed after creation
  The key enforces its own rules automatically

🧬 Genetic keys = Self-sovereign security!
```

---

## 🧠 Understanding Genetic Keys

### What Are Genetic Constraints?

Traditional keys:
```
Key = Just cryptographic material
Rules = External policies (can be changed)
Enforcement = Depends on application code
```

BearDog genetic keys:
```
Key = Cryptographic material + Constraints
Rules = Embedded in key (immutable)
Enforcement = Automatic (by the key itself)
```

### Why This Matters

**Immutability**: Constraints can't be changed after creation
**Self-Enforcement**: Key validates its own usage
**No Bypass**: Can't use key without respecting constraints
**Sovereignty**: Key's rules travel with the key

---

## 🎓 Constraint Types

### 1. Time Constraints
```rust
KeyConstraints {
    expires_at: Some(Utc::now() + Duration::days(30)),
    not_before: Some(Utc::now()),
    ..Default::default()
}
```

**Use Cases**:
- Temporary access tokens
- Time-limited certificates
- Seasonal keys

### 2. Usage Limits
```rust
KeyConstraints {
    max_uses: Some(1000),
    ..Default::default()
}
```

**Use Cases**:
- One-time passwords
- Limited-use API keys
- Metered access

### 3. Witness Requirements
```rust
KeyConstraints {
    required_witnesses: vec!["alice", "bob"],
    witness_threshold: 2,
    ..Default::default()
}
```

**Use Cases**:
- Multi-party signing
- Threshold signatures
- Distributed control

### 4. Operation Restrictions
```rust
KeyConstraints {
    allowed_operations: vec![Operation::Sign],
    ..Default::default()
}
```

**Use Cases**:
- Signing-only keys
- Encryption-only keys
- Limited-purpose keys

### 5. Combined Constraints
```rust
KeyConstraints {
    expires_at: Some(expiry),
    max_uses: Some(100),
    required_witnesses: vec!["alice"],
    allowed_operations: vec![Operation::Sign],
}
```

All constraints enforced simultaneously!

---

## 🔬 The Science: Genetic Algorithms

BearDog uses concepts from **genetic algorithms**:

1. **Genotype** (DNA): Constraint configuration
2. **Phenotype** (Behavior): How key operates
3. **Fitness** (Validation): Constraint checking
4. **Evolution** (Lineage): Parent-child relationships

When you create a key from another key:
```rust
child_key = parent_key.derive_child(new_constraints)?;
```

The child **inherits** parent constraints plus adds its own!

---

## 🚀 Next Steps

### Experiment
```bash
# Edit src/main.rs
# Try different constraint combinations
# See what happens when you:
# - Combine all 4 constraint types
# - Use extreme values (1 use, 1 second expiry)
# - Test edge cases
```

### Next Demos
1. **04-entropy-mixing** - Add your entropy
2. **05-key-lineage** - See inheritance in action
3. **02-ecosystem-integration/02-beardog-genesis** - Physical ceremonies

---

## ❓ Troubleshooting

### "Constraint validation failed" on creation
```rust
// Check your constraints make sense:
- expires_at should be in future
- max_uses should be > 0
- witness_threshold <= witness count
```

### "Operation rejected" unexpectedly
```rust
// Enable debug logging
RUST_LOG=debug cargo run

// Check constraint details
key.constraints().display();
```

---

## 📚 Related Documentation

- **ENTROPY_HIERARCHY_PRINCIPLE.md** - Constraint philosophy
- **specs/current/architecture/CAPABILITY_BASED_PRIMAL_INTERACTION.md**
- **05-key-lineage** - See constraints inherited

---

**Demo Status**: ✅ Complete  
**Difficulty**: 🟡 Intermediate  
**Time Required**: 10 minutes

🧬 **BearDog: Keys That Know Their Rules!** 🔐

