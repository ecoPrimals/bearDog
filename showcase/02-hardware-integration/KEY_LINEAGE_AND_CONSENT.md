# 🔗 Key Lineage and Multi-Party Consent

## The Problem

**Scenario**: Trust boundary changes over time

```
Day 1: You and friend are close
├─ Create shared key for data storage
├─ Data stored on friend's computer
├─ Key valid for 1 year
└─ Trust: HIGH ✅

Day 180: You have a falling out
├─ Revoke friend's access to YOUR resources ✅
├─ But: Data is on FRIEND's computer
├─ Key is still valid (6 months left)
└─ Problem: Friend can still access the shared data ⚠️
```

**Your Question**: How do we handle this safely?

---

## The Solution: Key Lineage with Multi-Party Consent

### Core Principle

**Keys don't just expire - they need CONTINUED CONSENT to propagate**

```
Initial Key (Both parties sign)
├─ Valid: 30 days
├─ Lineage: Generation 0
├─ Requires: Both parties for renewal
    ↓
Day 30: Refresh attempt
├─ Requires: NEW signatures from BOTH parties
├─ If both sign: New key (Generation 1)
├─ If either refuses: Key expires ✅
└─ No unilateral propagation!
```

---

## Architecture: Key Generations with Consent

### Generation 0: Initial Agreement

```json
{
  "key_id": "shared-storage-gen0",
  "key_type": "shared",
  "generation": 0,
  "participants": [
    {
      "id": "you",
      "public_key": "...",
      "signature": "I agree to this key for 30 days"
    },
    {
      "id": "friend", 
      "public_key": "...",
      "signature": "I agree to this key for 30 days"
    }
  ],
  "consent_threshold": "2-of-2",
  "validity": {
    "created": "2025-12-10T00:00:00Z",
    "expires": "2026-01-09T00:00:00Z",
    "max_propagation": "12_months",
    "requires_renewal": true
  },
  "propagation_policy": {
    "auto_renew": false,
    "requires_all_parties": true,
    "can_unilaterally_extend": false
  },
  "lineage": {
    "parent": null,
    "generation": 0,
    "max_generations": 12
  }
}
```

**Key point**: Both parties signed THIS key for THIS period.

### Generation 1: First Renewal (Day 30)

**Both parties still friends**:

```
Day 30 approaches
    ↓
Both parties must sign renewal
    ↓
You sign: "I consent to Generation 1"
Friend signs: "I consent to Generation 1"
    ↓
New key derived (Generation 1)
├─ Parent: shared-storage-gen0
├─ Generation: 1
├─ Valid: Next 30 days
└─ Data re-encrypted with new key
```

**Result**: Trust continues, key renewed ✅

### Generation 6: Falling Out (Day 180)

**You have a falling out**:

```
Day 180 approaches (Generation 6 renewal)
    ↓
Renewal requires BOTH signatures
    ↓
Friend signs: "I consent to Generation 7"
You refuse: "I do NOT consent"
    ↓
Renewal FAILS (needs 2-of-2)
    ↓
Generation 6 key expires naturally
    ↓
No Generation 7 exists
    ↓
Data becomes inaccessible ✅
```

**Result**: Either party can end access by refusing renewal!

---

## Key Properties

### Mutual Consent Required

✅ **Both parties must agree to continue**
- Can't unilaterally extend
- Either party can end by refusing renewal
- Fair and symmetric

✅ **Regular re-approval**
- Short renewal periods (30 days default)
- Frequent consent checkpoints
- Trust isn't "set and forget"

✅ **Lineage tracking**
- Each generation knows its parent
- Audit trail of all renewals
- Can see who signed what when

### What This Prevents

❌ **Unilateral extension**
- Friend can't renew alone
- You can't renew alone
- Requires BOTH

❌ **Infinite access**
- Maximum generations (e.g., 12)
- Even if both keep agreeing, hard stop at 1 year
- Forces re-evaluation

❌ **Silent propagation**
- All renewals must be explicit
- Logged and auditable
- Can't "forget" about old keys

---

## Implementation

### Key Derivation with Consent

```rust
fn attempt_key_renewal(
    current_key: &SharedKey,
    signatures: &[Signature]
) -> Result<SharedKey> {
    // Check if renewal period
    if !current_key.is_near_expiry() {
        return Err("Not yet time for renewal");
    }
    
    // Check lineage limits
    if current_key.generation >= current_key.max_generations {
        return Err("Maximum generations reached - key must expire");
    }
    
    // Verify ALL parties signed
    let required = current_key.participants.len();
    if signatures.len() != required {
        return Err("Missing signatures - renewal denied");
    }
    
    // Verify each signature
    for (participant, signature) in current_key.participants.iter().zip(signatures) {
        if !verify_signature(participant, signature, current_key.next_generation_id()) {
            return Err("Invalid signature - renewal denied");
        }
    }
    
    // All signatures valid - derive next generation
    derive_next_generation(current_key, signatures)
}
```

### Renewal Timeline

```
Generation N key
├─ Created: Day 0
├─ Expires: Day 30
├─ Renewal window: Day 25-30
    ↓
Day 25: Renewal window opens
├─ Notify all participants
├─ Request signatures
├─ Wait for responses
    ↓
Scenario A: All sign (by Day 30)
├─ Generation N+1 created
├─ Data re-key'd
└─ Continues ✅
    ↓
Scenario B: Someone refuses
├─ No new generation
├─ Generation N expires Day 30
├─ Data becomes inaccessible
└─ Trust boundary enforced ✅
    ↓
Scenario C: Timeout (Day 30)
├─ Not all signatures received
├─ Assumed refusal
├─ Key expires
└─ Safe default ✅
```

---

## Your Scenario: Step by Step

### Initial Setup (Day 1)

```
You and friend create shared key
├─ Both sign Generation 0
├─ Valid for 30 days
├─ Data stored on friend's computer (encrypted)
└─ Trust: Both parties agree ✅
```

### Months 1-5: All Good (Days 1-150)

```
Every 30 days:
├─ Renewal window opens
├─ Both parties sign renewal
├─ New generation created
├─ Data re-encrypted with new generation key
└─ Access continues ✅

Generations: 0 → 1 → 2 → 3 → 4 → 5
All signed by both parties
```

### Month 6: Falling Out (Day 180)

```
Day 180: Generation 6 needs renewal
├─ Friend requests renewal (wants to keep data)
├─ You REFUSE renewal (don't trust anymore)
    ↓
Renewal fails (needs 2-of-2 signatures)
    ↓
Generation 6 expires at Day 210
    ↓
No Generation 7 exists
    ↓
Data becomes inaccessible to BOTH parties
    ↓
Trust boundary respected ✅
```

**Result**: You can't unilaterally delete (friend's computer), but you CAN refuse renewal, which causes natural expiration.

---

## Multi-Party Threshold Schemes

### 2-of-2 (Your Scenario)

```
Two parties: You and Friend
Threshold: 2-of-2 (both must sign)

Renewal outcomes:
├─ Both sign: Renew ✅
├─ You refuse: Expire ✅
├─ Friend refuses: Expire ✅
└─ Neither sign: Expire ✅
```

**Properties**: Either party has veto power

### 2-of-3 (Household with Kids)

```
Three parties: Parent A, Parent B, Child
Threshold: 2-of-3 (any two must sign)

Renewal outcomes:
├─ A + B sign: Renew ✅ (parents agree)
├─ A + Child sign: Renew ✅
├─ B + Child sign: Renew ✅
├─ Only one signs: Expire ✅
└─ None sign: Expire ✅
```

**Properties**: More flexible, majority rules

### 3-of-5 (Business)

```
Five parties: 3 owners, 2 employees
Threshold: 3-of-5 (any three must sign)

Renewal outcomes:
├─ 3+ sign: Renew ✅
├─ <3 sign: Expire ✅
└─ Majority required
```

**Properties**: Survives one person leaving

---

## Data Lifecycle

### With Consent-Based Renewal

```
Day 1: Initial encryption
├─ Data encrypted with Gen 0 key
├─ Both parties have key
└─ Data accessible ✅

Day 30: First renewal
├─ Both sign
├─ Data RE-ENCRYPTED with Gen 1 key
├─ Gen 0 key destroyed
└─ Forward secrecy maintained

Day 60, 90, 120, 150: Renewals continue
├─ Data re-encrypted each time
├─ Old generations destroyed
└─ Forward secrecy preserved

Day 180: One party refuses
├─ No renewal
├─ Gen 6 key expires
├─ Data becomes inaccessible
└─ Neither party can decrypt ✅
```

**Critical**: Data is re-encrypted at each renewal, not just key rotated!

### Why Re-Encryption?

**Forward secrecy**: If old key is compromised, can't decrypt current data

```
Scenario: Friend compromised at Day 100
├─ Attacker gets Gen 3 key
├─ Can decrypt: Data from Day 60-90 ⚠️
├─ Cannot decrypt: Data from Day 90+ ✅
└─ Limited blast radius
```

**With re-encryption**: Each generation protects future data.

---

## Edge Cases

### Case 1: Friend Tries to Unilaterally Extend

```
Friend's attempt:
├─ Forges your signature (can't - cryptographic)
├─ OR claims you signed (can't - blockchain/log proof)
├─ OR tries to bypass consent (fails - enforced in code)
└─ Result: Renewal rejected ✅
```

**Protection**: Cryptographic signatures, audit logs

### Case 2: You Change Your Mind

```
Day 180: You refuse renewal
    ↓
Day 181: You regret it, want access back
    ↓
Friend says: "Too late, key expired"
    ↓
Result: Data lost to BOTH parties
```

**Protection**: Requires careful decision during renewal window

**Mitigation**: 
- Grace period (5 days for reconsideration)
- Explicit confirmation ("Are you sure? This will make data inaccessible")
- Backup before refusing

### Case 3: Friend Doesn't Respond

```
Day 30: Renewal window opens
    ↓
Day 35: Friend hasn't signed yet
    ↓
Day 39: Still no response
    ↓
Day 40: Timeout - assumed refusal
    ↓
Key expires (safe default)
```

**Safe default**: No signature = No renewal = Expiration

### Case 4: Maximum Generations Reached

```
Generation 12: Maximum reached (1 year total)
    ↓
Renewal window opens
    ↓
Both parties sign
    ↓
System: "Maximum generations reached"
    ↓
Key expires anyway
    ↓
Forces re-evaluation and fresh agreement
```

**Protection**: Can't "set and forget" forever

---

## Comparison to Traditional Systems

### Traditional (OAuth/API Keys)

```
Day 1: Create key
├─ Set expiry: 1 year
├─ No renewals required
├─ "Set and forget"
    ↓
Day 180: Falling out
├─ Revoke? Maybe (if you have access)
├─ Friend's copy? Still works ⚠️
├─ No mutual consent
└─ Unilateral control only
```

**Problems**: 
- One party controls revocation
- No periodic re-consent
- Long-lived keys outlive trust

### BearDog (Consent-Based)

```
Day 1: Create key
├─ Set expiry: 30 days
├─ Renewal requires BOTH
├─ Active consent required
    ↓
Day 180: Falling out
├─ Refuse renewal
├─ Key expires naturally
├─ Data inaccessible to both
└─ Mutual respect ✅
```

**Benefits**:
- Symmetric power
- Regular re-consent
- Trust checkpoints every 30 days
- Fair to all parties

---

## Configuration

### Shared Key Policy

```toml
[shared_keys]
# Require multi-party consent for renewals
require_all_parties = true

# Renewal period
renewal_period = "30d"

# Grace period for reconsideration
grace_period = "5d"

# Maximum generations before forced re-agreement
max_generations = 12

# Re-encrypt data at each renewal
re_encrypt_on_renewal = true

[consent]
# Default threshold for shared keys
default_threshold = "all"

# Renewal window (days before expiry)
renewal_window = 5

# Timeout for signatures
signature_timeout = "7d"

# What to do if timeout
timeout_behavior = "expire"  # or "extend_once", "ask_mediator"
```

---

## BearDog Implementation Status

### Already Implemented ✅

- Key derivation (parent → child)
- Hierarchical keys (master → sub-keys)
- Time-limited keys
- Revocation (mix-time enforcement)

### Needs Implementation 📅

- **Multi-party signature collection**
  - Distributed signing protocol
  - Signature aggregation
  - Verification

- **Key lineage tracking**
  - Generation numbers
  - Parent references
  - Audit trail

- **Consent-based renewal**
  - Renewal windows
  - Signature collection
  - Automatic expiration if unsigned

- **Data re-encryption**
  - Automatic re-key during renewal
  - Old key destruction
  - Forward secrecy guarantee

---

## CLI Commands (Proposed)

```bash
# Create shared key with friend
beardog key create-shared \
  --parties you,friend \
  --threshold 2-of-2 \
  --renewal 30d \
  --max-generations 12

# View key lineage
beardog key lineage shared-storage-gen6
# Output:
#   Gen 0: Signed by you,friend (2025-12-10)
#   Gen 1: Signed by you,friend (2026-01-09)
#   ...
#   Gen 6: Signed by you,friend (2026-06-08)
#   Gen 7: PENDING (needs 2-of-2 signatures)

# Sign renewal
beardog key sign-renewal shared-storage-gen6

# Refuse renewal (explicit)
beardog key refuse-renewal shared-storage-gen6 \
  --reason "trust_boundary_changed"

# Check renewal status
beardog key renewal-status shared-storage-gen6
# Output:
#   Generation: 6
#   Expires: 2026-07-08
#   Renewal window: 2026-07-03 to 2026-07-08
#   Signatures: 1 of 2 (friend signed, you pending)
#   Status: Waiting for your signature
```

---

## Security Properties

### Guaranteed

✅ **Mutual consent enforced**
- No unilateral extension possible
- Either party can end by refusing

✅ **Regular re-evaluation**
- Consent required every renewal period
- Trust doesn't outlive relationship

✅ **Forward secrecy**
- Data re-encrypted at each renewal
- Old keys destroyed
- Past data protected even if current key leaked

✅ **Audit trail**
- All signatures logged
- Lineage trackable
- Accountability maintained

### Not Guaranteed

⚠️ **Can't prevent offline copies**
- If friend made plaintext copy before falling out
- That's why periodic re-encryption matters

⚠️ **Requires both parties online for renewal**
- If one party disappears, key expires
- Mitigation: Grace periods, backup contacts

⚠️ **Data lost if both refuse**
- By design - no unilateral access after trust ends
- Backup strategy needed

---

## Your Insight Applied

> "When the key self-propagates it would need signoff from both (or some k/n subgrouping) to continue to propagate so it expires safely"

**Exactly right!** ✅

This ensures:
1. Trust boundaries are regularly re-evaluated
2. Either party can end access by refusing renewal
3. No "zombie keys" that outlive relationships
4. Fair and symmetric power

**BearDog should implement this as a core feature.**

---

## Summary

| Aspect | Traditional | BearDog (Proposed) |
|--------|-------------|-------------------|
| **Initial Creation** | One party decides | All parties sign ✅ |
| **Renewal** | Automatic | Requires all parties ✅ |
| **Revocation** | Unilateral | Any party can refuse ✅ |
| **Expiration** | Fixed (1 year) | Regular (30 days) ✅ |
| **Re-encryption** | Never | Every renewal ✅ |
| **Forward Secrecy** | No | Yes ✅ |
| **Trust Checkpoints** | None | Every 30 days ✅ |
| **Lineage** | No tracking | Full audit trail ✅ |

**Result**: Keys that respect evolving trust relationships!

---

*Key Lineage and Consent - December 10, 2025*  
*Trust boundaries change. Keys should adapt.*


