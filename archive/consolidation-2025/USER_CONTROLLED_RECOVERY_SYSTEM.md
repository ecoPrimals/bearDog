# BearDog User-Controlled Recovery System

## Overview
The BearDog recovery system implements a **user-controlled, distributed recovery mechanism** that embodies the principle: **"Finding a key in the parking lot doesn't jeopardize anyone's security"**.

## Core Philosophy
- **Keys are worthless without context** - Individual shards provide no security value
- **User controls all trust boundaries** - No hardcoded limitations
- **Distributed trust model** - No single point of failure
- **Mixed recovery support** - Combine social, federation, and emergency methods
- **Time-bounded security** - Recovery windows prevent stale attacks

## Key Features

### 1. User-Controlled Recovery Policies
```rust
UserRecoveryPolicy {
    total_shards: 7,          // Create 7 shards
    threshold_shards: 4,      // Need 4 to recover
    allow_mixed_recovery: true,
    recovery_contexts: [
        "family",             // Parents, siblings
        "university",         // IT dept, professors  
        "work"                // Company, mentors
    ]
}
```

### 2. Custom Trust Boundaries
- **Social trust level**: 70-100% (user configurable)
- **Federation trust level**: 80-100% (user configurable)
- **Geographic restrictions**: Optional region limiting
- **Time windows**: Business hours, weekdays only, etc.
- **Recovery attempts**: Max per day (user defined)

### 3. Recovery Context System
Each context has:
- **Shard allocation**: How many shards to distribute
- **Backup strategies**: TrustWeighted, EvenDistribution, GeographicSpread
- **Verification requirements**: Email, SMS, crypto challenges, video calls
- **Standalone capability**: Can this context work alone?

### 4. Shamir's Secret Sharing
- **Threshold cryptography**: Need K of N shards to recover
- **Distributed shards**: No single party has enough to compromise
- **Encrypted shard data**: Even holders can't read raw shard content
- **Context isolation**: Each context gets specific shard allocations

### 5. Mixed Recovery Sessions
- **Combine methods**: Family + work, university + emergency
- **Progress tracking**: Real-time recovery completion status
- **Session management**: Time-bounded recovery windows
- **Multi-party verification**: Multiple sources required

## Security Properties

### Key Worthlessness Demonstration
```rust
// Scenario A: Random person finds shard (no context)
KeyWorthinessDemo {
    shard_data_found: true,
    can_decrypt_shard: false,        // ❌ No context
    can_identify_user: false,        // ❌ No metadata
    can_locate_other_shards: false,  // ❌ No directory
    can_compromise_account: false,   // ❌ Need threshold
    security_impact: "NONE"
}

// Scenario B: Malicious actor with some context
KeyWorthinessDemo {
    shard_data_found: true,
    can_decrypt_shard: true,         // ✅ Has context
    can_identify_user: false,        // ❌ Still protected
    can_locate_other_shards: false,  // ❌ Still hidden
    can_compromise_account: false,   // ❌ Still need threshold
    security_impact: "MINIMAL"
}
```

### Verification Methods
- **Email verification**: Standard email challenges
- **SMS verification**: Phone-based codes
- **Video call verification**: Human-in-the-loop verification
- **Hardware tokens**: Physical device verification
- **Biometric verification**: Fingerprint, face recognition
- **Cryptographic challenges**: Proof of key possession
- **Knowledge-based questions**: Personal information

## User Scenarios

### Example 1: Student (Sarah)
- **Context**: Family (3 shards) + University (2 shards) + Work (2 shards)
- **Threshold**: 4 of 7 shards needed
- **Recovery**: Can combine family + university OR work + university
- **Trust levels**: Family (75%), University (85%), Work (85%)

### Example 2: High-Security User
- **Total shards**: 9
- **Threshold**: 6 shards needed
- **Trust levels**: Social (90%), Federation (95%)
- **Recovery window**: 12 hours only
- **Geographic**: US-only restrictions

### Example 3: Casual User
- **Total shards**: 3
- **Threshold**: 2 shards needed
- **Trust levels**: Social (60%), Federation (70%)
- **Recovery window**: 72 hours
- **Geographic**: No restrictions

## Implementation Details

### Recovery Manager
- **Async constructor**: `RecoveryManager::new().await?`
- **Policy setup**: `setup_user_recovery_policy(user_id, policy)`
- **Mixed recovery**: `start_mixed_recovery(user_id, contexts, policy)`
- **Shard submission**: `submit_recovery_shard(session_id, shard, proof)`
- **Secret reconstruction**: `attempt_secret_reconstruction(session_id)`

### Recovery Progress
```rust
RecoveryProgress {
    shards_needed: 4,
    shards_collected: 2,
    completion_percentage: 50.0,
    contexts_responded: ["family"],
    contexts_pending: ["university", "work"],
    estimated_completion_time: Some(in_2_hours)
}
```

## HPC Basement Scenario
Perfect for your use case:
1. **Spawn federated HPC** in basement with genetic derivatives
2. **Setup recovery towers** (office, mobile, cloud backup)
3. **Configure federation recovery** (need 1 of 3 towers)
4. **Genetic derivatives** get unique crypto keys + ephemeral recovery
5. **Device failure** → use towers for recovery
6. **No system bricking** → genetic derivatives continue independently

## Testing
- **14 comprehensive tests** covering all recovery scenarios
- **Policy validation** tests for invalid configurations
- **Key worthlessness** demonstrations
- **Mixed recovery** session management
- **Shard collection** and verification
- **Secret reconstruction** processes

## Key Principles Achieved
1. **USER CONTROL**: Users set their own trust boundaries
2. **MIXED RECOVERY**: Combine social + federation shards
3. **THRESHOLD SECURITY**: Need multiple shards to recover
4. **WORTHLESS KEYS**: Individual shards are useless alone
5. **CONTEXT ISOLATION**: Each context has separate requirements
6. **FLEXIBLE POLICIES**: From casual to high-security users
7. **NO SINGLE POINT OF FAILURE**: Distributed trust model
8. **TIME-BOUNDED**: Recovery windows prevent stale attacks

## Real-World Example
**"Sarah loses her laptop and phone at a coffee shop"**

1. **Contacts family**: Mom and Dad provide shards (2/4)
2. **Contacts university**: IT dept and professor provide shards (2/4) 
3. **Threshold met**: 4 shards collected from 2 contexts
4. **Account recovered**: Sarah accesses from university computer
5. **Security maintained**: Coffee shop thief gains nothing

The system is **robust, resilient, and user-controlled** - exactly what's needed for distributed HPC infrastructure that can't afford to be "bricked" by device loss. 