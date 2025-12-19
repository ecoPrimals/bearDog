# 🔒 Revocation Architecture - No Phone Home

## The Problem

**Question**: How does revocation work without "phone home"?

**Challenge**: In a sovereign system (air-gap capable, no central authority):
- Can't remotely delete a key
- Can't force expiration from a server
- Can't "kill switch" a delegated key

**Your Insight**: ✅ Correct approach!

---

## The Solution: Mix-Time Revocation

### Key Principle

**Revocation isn't remote deletion - it's refusal to cooperate**

```
Revoked Key (still exists on device)
    ↓
Tries to mix with non-revoked key
    ↓
Non-revoked key: "You're on my revocation list"
    ↓
Mixing FAILS ❌
    ↓
Revoked key can't do anything that requires cooperation
```

---

## How It Works

### Scenario: You Revoke Friend's Tower Access

**Step 1: You create revocation**
```
Your Master Key
    ↓
Signs revocation certificate
    ↓
"Delegated key delegated-tower-123 is REVOKED as of 2025-12-10T18:00:00Z"
    ↓
Stores in your local revocation list
```

**Step 2: Friend still has the delegated key**
```
Friend's Delegated Key (still exists!)
├─ Not deleted
├─ Not expired
├─ Still on their device
└─ But... can't cooperate with your keys anymore
```

**Step 3: Friend tries to use tower**
```
Friend submits workload
    ↓
Tower checks: Does this require my master key?
    ↓
YES: Must mix delegated key + master key
    ↓
Master key checks revocation list
    ↓
"delegated-tower-123 is REVOKED"
    ↓
Mixing REFUSED ❌
    ↓
Workload rejected
```

---

## Revocation Modes

### Mode 1: Immediate (Network Available)

**When connected**:
```
Tower A (You)
    ↓
Publishes revocation certificate via Songbird
    ↓
Tower B, Tower C (Friend's machines)
    ↓
Receive revocation certificate
    ↓
Update local revocation lists
    ↓
Immediate effect across network
```

**Timeline**: Seconds to minutes (depending on network)

### Mode 2: Lazy (Air-Gap / Isolated)

**When NOT connected**:
```
You revoke key
    ↓
Your tower knows immediately
    ↓
Friend's tower: Still accepts key (doesn't know yet)
    ↓
Eventually: Friend connects to network
    ↓
Receives revocation certificate
    ↓
Updates local list
    ↓
Key becomes ineffective
```

**Timeline**: Until next connection (could be hours/days)

**Mitigation**: Time limits on delegated keys!
- Max delegation: 30 days
- Even if isolated, expires naturally
- Limits blast radius

### Mode 3: Trust-On-First-Use (TOFU)

**For new interactions**:
```
Friend tries to use Tower X (never seen before)
    ↓
Tower X: "Let me check if this key is valid"
    ↓
Queries network for revocation status
    ↓
If revoked: Refuse
If not revoked: Accept (and cache status)
```

**Timeline**: Checked before first use

---

## Key Properties

### What Revocation DOES

✅ **Prevents mixing**
- Revoked key can't mix with non-revoked keys
- Cooperation refused at mix-time

✅ **Prevents new operations**
- Can't submit new workloads
- Can't derive new keys
- Can't grant new permissions

✅ **Eventually consistent**
- Network nodes eventually learn
- Air-gapped nodes learn when connected

✅ **Respects time limits**
- Even if isolated, key expires
- Maximum damage window: time limit

### What Revocation DOESN'T Do

❌ **Doesn't remote delete**
- Key still exists on device
- Can't force deletion

❌ **Doesn't work instantly in air-gap**
- If friend is offline, they don't know yet
- But your tower knows immediately

❌ **Can't retrieve already-done work**
- If friend finished work before revocation, it's done
- Only prevents NEW operations

❌ **Can't decrypt existing data**
- If friend encrypted data with their key, you still can't read it
- Privacy preserved even after revocation

---

## Architecture Details

### Revocation Certificate Structure

```json
{
  "revocation_id": "rev-abc123",
  "issuer": "master-key-xyz",
  "revoked_key": "delegated-tower-123",
  "timestamp": "2025-12-10T18:00:00Z",
  "reason": "access_no_longer_needed",
  "signature": "...",
  "propagation": {
    "network_broadcast": true,
    "songbird_channels": ["tower-network"],
    "priority": "high"
  }
}
```

### Revocation List Storage

```
~/.beardog/revocations/
├── master-key-xyz.revlist
    ├── delegated-tower-123 (revoked 2025-12-10)
    ├── daily-ops-old-456 (revoked 2025-11-15)
    └── backup-key-789 (revoked 2025-10-01)
```

**Format**: Append-only log (cryptographically signed)

### Mix-Time Check

```rust
fn mix_keys(key_a: &Key, key_b: &Key) -> Result<MixedKey> {
    // Check if either key is revoked
    if is_revoked(key_a)? || is_revoked(key_b)? {
        return Err("Cannot mix with revoked key");
    }
    
    // Proceed with mixing
    perform_mixing(key_a, key_b)
}

fn is_revoked(key: &Key) -> Result<bool> {
    // Check local revocation list
    if local_revocation_list.contains(key.id()) {
        return Ok(true);
    }
    
    // If network available, check for updates
    if network_available() {
        update_revocation_list()?;
        return Ok(local_revocation_list.contains(key.id()));
    }
    
    // If air-gapped, trust local list
    Ok(false)
}
```

---

## Trade-offs

### Benefits of Mix-Time Revocation

✅ **No phone home required**
- Works fully air-gapped
- No central authority
- Sovereign architecture preserved

✅ **Privacy preserved**
- Can't revoke access to already-encrypted data
- Friend keeps their work
- Only prevents new operations

✅ **Eventually consistent**
- Network nodes sync when available
- Converges to correct state

✅ **Respects sovereignty**
- You can't force deletion on someone else's device
- Can only refuse to cooperate

### Limitations

⚠️ **Not instant in air-gap**
- If friend is offline, they don't know yet
- Your tower refuses immediately
- Their tower refuses when it learns

**Mitigation**: Time limits on all delegations

⚠️ **Can't prevent offline use**
- If friend cached your data before revocation, they still have it
- Can't remote wipe

**Mitigation**: Don't delegate to untrusted parties

⚠️ **Requires correct clocks**
- Time limits depend on accurate clocks
- Clock skew could extend access

**Mitigation**: NTP sync, certificate timestamps

---

## Comparison to Centralized Revocation

### Centralized (OAuth/PKI)

```
Revoke key
    ↓
Update central server
    ↓
All clients check server before EVERY operation
    ↓
Server down = nobody can work ❌
```

**Problems**:
- Single point of failure
- Requires always-online
- Privacy: Server sees all operations
- Sovereignty: Central authority required

### BearDog (Decentralized)

```
Revoke key
    ↓
Sign revocation certificate
    ↓
Propagate via Songbird (when network available)
    ↓
Nodes update local lists
    ↓
Mix-time checks against local list
    ↓
Network down = still works with local knowledge ✅
```

**Benefits**:
- No single point of failure
- Air-gap capable
- Privacy: No central observer
- Sovereignty: Your tower, your rules

---

## Real-World Scenarios

### Scenario 1: Network Available

**Timeline**:
```
10:00 AM - You revoke friend's key
10:00 AM - Your tower: Immediate effect
10:01 AM - Songbird broadcasts revocation
10:02 AM - Friend's tower: Receives and applies
10:03 AM - Friend tries to submit work: DENIED ✅
```

**Result**: ~3 minutes to full propagation

### Scenario 2: Friend Offline

**Timeline**:
```
10:00 AM - You revoke friend's key
10:00 AM - Your tower: Immediate effect
10:01 AM - Songbird: No route to friend (offline)
10:05 AM - Friend tries to submit to YOUR tower: DENIED ✅
2:00 PM  - Friend comes online
2:01 PM  - Receives revocation certificate
2:02 PM  - Friend tries to submit to OTHER tower: DENIED ✅
```

**Result**: Your tower protected immediately, friend's tower learns later

### Scenario 3: Air-Gap Both Sides

**Timeline**:
```
10:00 AM - You revoke friend's key (your tower only)
10:05 AM - Friend tries to submit to YOUR tower: DENIED ✅
10:10 AM - Friend tries to use cached data offline: WORKS ⚠️
30 days  - Delegated key expires naturally
```

**Result**: 
- Your resources protected immediately
- Friend keeps their already-done work
- Time limit provides hard stop

---

## Implementation in BearDog

### Config: Revocation Policy

```toml
[revocation]
# How often to check for revocation updates
check_interval = "1h"

# Trust local revocation list when offline
trust_local_when_offline = true

# Maximum time a key can work after revocation if offline
max_stale_time = "24h"

# Require network check for new key first use
tofu_network_check = true

[delegation]
# Maximum delegation time (hard limit)
max_delegation_time = "30d"

# Require time limits on all delegations
require_time_limits = true
```

### CLI Commands

```bash
# Revoke a key
beardog key revoke delegated-tower-123 --reason "no_longer_needed"

# Check revocation status
beardog key check delegated-tower-123

# List revoked keys
beardog key list-revoked

# Propagate revocation (explicit)
beardog revocation propagate delegated-tower-123 --via songbird

# Update revocation list from network
beardog revocation sync
```

---

## Security Properties

### Guaranteed

✅ **Your tower is protected immediately**
- Revocation takes effect on your resources instantly

✅ **Eventually consistent**
- All connected nodes learn eventually
- Converges to correct state

✅ **Time-bounded**
- Even if revocation doesn't propagate, key expires

✅ **Cryptographically signed**
- Revocations are signed by master key
- Can't be forged

### Not Guaranteed

⚠️ **Instant propagation to offline nodes**
- They learn when they reconnect

⚠️ **Prevention of already-granted access**
- Can't take back what was already given

⚠️ **Remote deletion**
- Sovereign system: can't force deletion

---

## Your Insight Was Correct!

> "The revoked key will expire as normal if isolated, but will not be able to mix with any non-revoked keys"

**Exactly right!** ✅

This is the elegant solution:
1. Can't remote-kill (preserves sovereignty)
2. CAN refuse cooperation (enforces revocation)
3. Time limits provide hard stop
4. Eventually consistent (network propagation)

**Result**: Revocation that works WITHOUT phone home!

---

## Summary

| Aspect | Centralized | BearDog |
|--------|-------------|---------|
| **Phone Home** | Required | Not required ✅ |
| **Air-Gap** | Doesn't work | Works ✅ |
| **Instant** | Yes | Eventually (seconds-hours) |
| **Sovereign** | No | Yes ✅ |
| **Privacy** | Server sees all | No central observer ✅ |
| **Your Resources** | Protected immediately | Protected immediately ✅ |
| **Remote Resources** | Protected immediately | Protected when they learn |

**Best of both worlds**: Sovereignty + Effective revocation!

---

*Revocation Architecture - December 10, 2025*  
*No phone home required. Sovereignty preserved. Revocation works.*


