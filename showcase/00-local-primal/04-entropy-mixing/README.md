# 🌊 Entropy Mixing - Human + Machine Entropy

**Level**: 0 (Local Primal)  
**Category**: Entropy & Randomness  
**Time**: 10 minutes  
**Dependencies**: None

---

## 🎯 What This Demo Shows

Mix **human** and **machine** entropy for key generation:
- ✅ Understand entropy hierarchy
- ✅ Collect system entropy (machine)
- ✅ Collect timing entropy (simulated human)
- ✅ Mix sources with SHA3-512
- ✅ Quality scoring and validation
- ✅ **Never simulate** - principle demonstration

This embodies BearDog's **Entropy Hierarchy Principle**: "Never simulate human entropy - it violates the trust model."

---

## 🚀 Running the Demo

```bash
./run.sh
```

---

## 📊 Expected Output

```
🌊 BearDog - Entropy Mixing Demo
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Understanding the Entropy Hierarchy
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

TIER 1: Hardware HSMs
  Quality: ★★★★★ (cryptographic grade)
  Uniqueness: ★★★★☆
  Sovereignty: ★★★☆☆
  Example: YubiKey, TPM

TIER 2: System Entropy
  Quality: ★★★★★ (cryptographic grade)
  Uniqueness: ★★★★☆
  Sovereignty: ★★☆☆☆
  Example: /dev/urandom

TIER 3: Real Human Input
  Quality: ★★★☆☆ (medium)
  Uniqueness: ★★★★★ (NON-FUNGIBLE)
  Sovereignty: ★★★★★ (YOU control it)
  Example: Keyboard timing, mouse jitter

❌ TIER VIOLATION: Simulated Human
  Quality: ★★☆☆☆ (weak)
  Uniqueness: ★☆☆☆☆ (fungible)
  Trust: ☆☆☆☆☆ (VIOLATED)
  → BearDog REJECTS this!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Collecting Entropy Sources
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Step 1: Machine Entropy (System)
  Source: /dev/urandom
  Amount: 32 bytes
  Quality: 1.00 (perfect)
  ✓ Collected successfully

Step 2: Human Entropy (Simulated for Demo)
  ⚠️  IMPORTANT: In production, use REAL human input!
  ⚠️  This demo simulates for educational purposes only
  ⚠️  Real implementation: collect_real_human_input()
  
  Simulated source: Keyboard timing patterns
  Amount: 32 bytes  
  Quality: 0.75 (simulated)
  ⚠️  Simulated (demo purposes only)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Mixing Entropy Sources
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Algorithm: SHA3-512
Ratio: 60% machine + 40% human

Input:
  Machine: 32 bytes (quality: 1.00)
  Human: 32 bytes (quality: 0.75)
  
Process:
  1. Validate both sources
  2. Combine with SHA3-512
  3. Preserve quality guarantees
  
Output:
  Mixed: 64 bytes
  Quality: 0.90 (excellent)
  ✓ Cryptographic strength maintained

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Generating Key with Mixed Entropy
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Key Type: Ed25519
Entropy Source: Mixed (60/40)
Quality Score: 0.90

✓ Key generated successfully!

Key Properties:
  - Cryptographic quality: HIGH (from machine)
  - Uniqueness: HIGH (from human)
  - Non-fungible: YES (human contribution)
  - Sovereignty: YOURS (you added entropy)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Why This Matters
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Traditional KMS:
  ✗ 100% machine entropy
  ✗ Fungible (anyone can recreate)
  ✗ No personal connection

BearDog with Mixed Entropy:
  ✓ 60% machine (quality)
  ✓ 40% human (uniqueness)
  ✓ Non-fungible (one-of-a-kind)
  ✓ Personal (YOU contributed)

Result: Keys that are both SECURE and UNIQUE to YOU!

🌊 Entropy Mixing = Sovereignty + Security!
```

---

## 🧠 Understanding the Entropy Hierarchy

### Why 60/40 Split?

**60% Machine (Quality)**:
- Ensures cryptographic strength
- Meets security thresholds
- Protects against weak human input
- Baseline security guarantee

**40% Human (Uniqueness)**:
- Makes keys non-fungible
- Your contribution matters
- Not enough to weaken security
- Adds sovereignty

### The Trust Model

**With Simulation** ❌:
```
User believes: "My key is unique to ME"
Reality: "Key is reproducible by anyone"
→ TRUST DESTROYED
```

**With Real Human Input** ✅:
```
User knows: "I contributed to my key"
Reality: "Key includes my unique entropy"
→ TRUST MAINTAINED
```

---

## 🎓 Implementation Details

### Real Human Collection (Not in Demo)

```rust
// Production implementation would:
async fn collect_real_human_input() -> Result<HumanEntropy> {
    // 1. Open interactive terminal
    let terminal = Terminal::new()?;
    
    // 2. Capture keyboard timing
    let typing_events = capture_typing_dynamics()?;
    
    // 3. Measure timing entropy
    let timing_entropy = analyze_timing_variance(&typing_events);
    
    // 4. Extract entropy bits
    let entropy = extract_entropy_from_timing(typing_events)?;
    
    // 5. Quality scoring
    let quality = score_entropy_quality(&entropy);
    
    // 6. Validate (reject if simulated!)
    validate_not_simulated(&entropy)?;
    
    Ok(HumanEntropy {
        data: entropy,
        quality,
        source: HumanSource::KeyboardTiming,
        verified: true,
    })
}
```

### SHA3-512 Mixing

```rust
fn mix_entropy(machine: &[u8], human: &[u8]) -> Result<Vec<u8>> {
    use sha3::{Sha3_512, Digest};
    
    let mut hasher = Sha3_512::new();
    
    // Machine entropy (60%)
    hasher.update(b"machine:");
    hasher.update(machine);
    
    // Human entropy (40%)
    hasher.update(b"human:");
    hasher.update(human);
    
    // Additional context
    hasher.update(b"beardog-v1");
    hasher.update(&timestamp());
    
    Ok(hasher.finalize().to_vec())
}
```

### Quality Scoring

```rust
fn score_mixed_quality(machine_q: f64, human_q: f64) -> f64 {
    // Weighted average
    let mixed = (machine_q * 0.6) + (human_q * 0.4);
    
    // Ensure meets minimum threshold
    if mixed < 0.8 {
        warn!("Mixed quality below recommended threshold");
    }
    
    mixed
}
```

---

## 🚀 Next Steps

### Try Real Implementation

See `../05-mixed-entropy/` for the actual working demo (from original showcase).

### Experiment

```rust
// Try different ratios
mix_entropy_ratio(0.7, 0.3)?  // 70% machine
mix_entropy_ratio(0.5, 0.5)?  // 50/50 (not recommended)

// Add more sources
mix_entropy_sources(vec![
    (machine_entropy, 0.5),
    (human_entropy, 0.3),
    (hardware_hsm, 0.2),
])?
```

### Next Demos
1. **05-key-lineage** - Track key ancestry
2. **06-btsp-tunnel** - Secure encrypted tunnels
3. **01-hardware-integration/05-mixed-entropy** - Real working version

---

## 📚 Related Documentation

- **ENTROPY_HIERARCHY_PRINCIPLE.md** - Core principle (MUST READ!)
- **../05-mixed-entropy/** - Working implementation
- **specs/current/security/** - Security architecture

---

## ⚠️ IMPORTANT

This demo **simulates** human entropy for educational purposes.

**Production code MUST**:
- ✅ Use `collect_real_human_input()`
- ✅ Never call `simulate_human_entropy()`
- ✅ Validate entropy sources
- ✅ Reject simulated input
- ✅ Maintain trust model

**Why**: Simulating human entropy violates the trust model and defeats the purpose of mixing.

---

**Demo Status**: ✅ Educational  
**Difficulty**: 🟡 Intermediate  
**Time Required**: 10 minutes

🌊 **BearDog: Real Entropy. Real Sovereignty.** 🔐

