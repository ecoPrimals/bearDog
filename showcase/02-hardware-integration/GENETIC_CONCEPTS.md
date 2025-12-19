# 🧬 Genetic Cryptography - Real Concepts

## The Problem with Simple Evolution

**Old approach**: All keys evolve to "perfect" (fitness = 1.0)
- Unrealistic: No trade-offs
- Useless: Doesn't model real constraints
- Boring: Doesn't solve real problems

**New approach**: Genetic concepts solve REAL use cases
- Hierarchical keys (master → sub-keys)
- Key mixing (household sharing)
- Delegated access (tower sharing with constraints)
- Realistic evolution (trade-offs matter)

---

## Use Case 1: Hierarchical Keys

### Problem
You want sub-keys with LESS power than your master key.

### Solution
```
Master Key (YOU)
├─ Full permissions
├─ Can derive sub-keys
├─ Can revoke sub-keys
    │
    ├─ Sub-Key: Daily Operations
    │   ├─ Limited permissions
    │   ├─ 24h expiry
    │   ├─ Document scope only
    │   └─ Revocable by master
    │
    └─ Sub-Key: Emergency Backup
        ├─ Decrypt only
        ├─ Backups scope only
        ├─ Requires 2FA
        └─ Revocable by master
```

### Benefits
- Limit blast radius if sub-key compromised
- Time-limited keys (rotate frequently)
- Scope-limited keys (least privilege)
- Master retains control

---

## Use Case 2: Household Key Mixing

### Problem
Two people with different seeds want shared access.

### Solution
```
Person A (Seed A) → Master Key A
Person B (Seed B) → Master Key B
        ↓
   Mix Keys (2-of-2 threshold)
        ↓
Household Shared Key
├─ Requires both keys
├─ OR: 1 key + 2FA
├─ Shared data access
└─ Either can revoke
```

### Benefits
- Each person has unique seed (biometric)
- Shared resources need both (or 1+2FA)
- Personal data stays private
- Fair revocation (either party)

---

## Use Case 3: Tower Sharing (YOUR PROBLEM!)

### Problem
Let friend use your tower, but:
- Only certain hours
- Their work encrypted (you can't see)
- They can't access your data
- Limited compute quota

### Solution
```
Your Tower Master Key
    +
Friend's Personal Key
    ↓
Delegated Key (constrained)
├─ Time: 9 AM - 5 PM weekdays
├─ CPU: 50% max
├─ Memory: 8GB max
├─ Storage: 100GB temp
├─ Encryption: Friend's key (mandatory)
├─ Isolation: Can't see your data
└─ Revocable: Instant
```

### Workflow
1. Friend encrypts workload with THEIR key
2. Submits to your tower
3. Tower checks delegation constraints
   - Time? ✅ 2 PM Tuesday
   - Quota? ✅ Using 30% CPU
4. Executes in isolated environment
5. Results encrypted with friend's key
6. Friend retrieves and decrypts
7. You NEVER see plaintext!

### Benefits
- Friend gets compute
- You keep privacy (encrypted)
- Friend's privacy protected (you can't decrypt)
- Quota limits enforced
- Time constraints respected
- Revocable instantly

---

## Use Case 4: Realistic Evolution

### Problem
Old demo: All keys reach 0.97 fitness (unrealistic)

### Solution: Model Trade-offs
```
Generation 1: Random (fitness 0.10 - 0.69)
    ↓
Generation 2: Selection (fitness 0.35 - 0.84)
    ↓
Generation 3: Mutation (fitness 0.65 - 0.90)
    ↓
Generation 4: Converged (fitness 0.80 - 0.95)
```

### Why Not Perfect (1.0)?

**Trade-offs are REAL**:
- Faster keys → Less secure
- More secure → Slower
- More features → More complex
- Simpler → Less flexible

**Optimal ≠ Perfect**:
- Daily ops key: Speed 0.92, Security 0.83 → Balance 0.87
- Backup key: Speed 0.60, Security 0.98 → Balance 0.79
- Different use cases → Different optima

---

## Implementation Status

### Demo Created ✅
```bash
./demo-genetic-realistic.sh
```

Shows:
1. Master → Sub-key hierarchy
2. Household key mixing
3. Tower sharing delegation
4. Realistic evolution

### BearDog CLI Integration 📅
- Implement key derivation
- Implement key mixing
- Implement constraint enforcement
- Implement delegation

### Phase 3: Network 📅
- Use delegated keys with Songbird
- Cross-tower workload sharing
- Distributed with privacy

---

## Key Insights

1. **Genetics ≠ Just Evolution**
   - It's about key relationships
   - Parent/child hierarchies
   - Mixing different sources
   - Constrained delegation

2. **Real World = Trade-offs**
   - No perfect key exists
   - Optimize for YOUR use case
   - Different keys for different purposes
   - Balance is key

3. **Privacy Through Delegation**
   - You can share compute WITHOUT seeing data
   - Friend encrypts with THEIR key
   - Tower enforces isolation
   - Both parties maintain privacy

4. **Constraints Enable Trust**
   - Time limits → Can't abuse after hours
   - Quotas → Fair resource sharing
   - Scope limits → Can't escape sandbox
   - Revocable → You stay in control

---

## Run the Demo

```bash
cd showcase/02-hardware-integration
./demo-genetic-realistic.sh
```

**Shows real concepts**, not fake evolution!

---

*Genetic Concepts - December 10, 2025*

---

## Revocation Without Phone Home

### Your Question: How?

**Answer**: Mix-time enforcement!

```
Revoked Key (still exists)
    ↓
Tries to mix with your key
    ↓
Your key: "You're on my revocation list"
    ↓
Mixing REFUSED ❌
```

### Properties

✅ **Your tower**: Protected immediately  
⚠️ **Friend offline**: Works until natural expiry (30d)  
✅ **Friend online**: Learns via Songbird (~5m)  
✅ **No phone home**: Fully sovereign!

### Why This Works

1. Can't remote-kill (preserves sovereignty)
2. CAN refuse cooperation (enforces revocation)
3. Time limits provide hard stop
4. Eventually consistent

**Your insight was correct!** 🎯

See: `REVOCATION_ARCHITECTURE.md` for full details

---

*Updated: December 10, 2025*

---

## Key Lineage and Consent (Critical!)

### The Problem

**Scenario**: You store data on friend's computer for 1 year. After 6 months, you have a falling out.

**Question**: How does the data handle this?

### The Solution: Multi-Party Renewal

```
Initial Key (Both sign)
├─ Valid: 30 days
├─ Generation: 0
    ↓
Day 30: Renewal
├─ Requires: BOTH signatures
├─ Both sign → Gen 1 ✅
├─ Either refuses → Expires ✅
    ↓
Day 180: Falling out
├─ You refuse renewal
├─ Key expires
├─ Data inaccessible to BOTH
└─ Trust boundary respected ✅
```

### Key Properties

✅ **Mutual consent required** - Can't unilaterally extend  
✅ **Regular re-evaluation** - 30-day consent checkpoints  
✅ **Either party can end** - Refuse renewal → Expiration  
✅ **Forward secrecy** - Data re-encrypted at each renewal  
✅ **Fair and symmetric** - No one party controls  

### Why This Matters

**Without lineage**: "Set and forget" keys outlive trust  
**With lineage**: Trust is actively re-confirmed every 30 days

**Your question identified a critical sovereignty issue!**

See: `KEY_LINEAGE_AND_CONSENT.md` for full details

---

*Updated: December 10, 2025*
