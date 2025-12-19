# 📋 Showcase Demo Audit & Upgrade Plan

## Goal
Make ALL showcase demos:
- ✅ **Real**: Actual cryptographic operations
- ✅ **Validatable**: Cryptographic receipts for every operation
- ✅ **Live**: Using BearDog CLI where possible

---

## Current Demo Inventory

### Phase 1: Local Basics
- 📄 `01-local-basics/demo.sh`
  - **Status**: Unknown - needs audit
  - **Action**: Audit and upgrade

### Phase 2: Hardware Integration
- 📄 `02-hardware-integration/demo-hybrid.sh`
  - **Status**: CONCEPTUAL (simulated outputs)
  - **Action**: Upgrade to real crypto + receipts
  
- 📄 `02-hardware-integration/demo-human-entropy.sh`
  - **Status**: PARTIALLY REAL (data collection real, scores simulated)
  - **Action**: Add real entropy measurements + receipts
  
- 📄 `02-hardware-integration/demo-genetic-realistic.sh`
  - **Status**: CONCEPTUAL (shows ideas, not real crypto)
  - **Action**: Upgrade to real key operations + receipts
  
- 📄 `02-hardware-integration/demo-with-receipts.sh` 🆕
  - **Status**: REAL (just created!)
  - **Action**: Test and validate ✅

### Phase 3 & 4: Network & Distributed
- **Status**: Not yet created
- **Action**: Build with real crypto from the start

---

## Upgrade Strategy

### Step 1: Define Receipt Format (Standard)

Every operation MUST produce:

```json
{
  "receipt_id": "receipt-{operation}-{timestamp}",
  "session_id": "{session_id}",
  "operation": "{operation_name}",
  "timestamp": "{ISO-8601}",
  "timestamp_unix": {unix_timestamp},
  "inputs": {
    "file1.json": "sha256:hash...",
    "file2.bin": "sha256:hash..."
  },
  "outputs": {
    "output1.json": {
      "hash": "sha256:hash...",
      "size": 1024
    }
  },
  "verification": {
    "method": "sha256_hashing",
    "verifiable": true,
    "verification_commands": [
      "sha256sum <file>",
      "Compare with receipt hash"
    ]
  },
  "beardog_cli": {
    "used": true/false,
    "command": "beardog entropy collect ...",
    "fallback": "openssl (if CLI not available)"
  }
}
```

### Step 2: Audit Each Demo

For each demo script, check:

1. **Does it use BearDog CLI?**
   - YES → Keep it, add receipt generation
   - NO → Use OpenSSL for real crypto, document gap

2. **Does it produce verifiable outputs?**
   - YES → Add hashes to receipts
   - NO → Add hash generation

3. **Can user independently verify?**
   - YES → Document verification steps
   - NO → Add verification commands to receipt

4. **Are outputs unique per run?**
   - YES → Add uniqueness proof
   - NO → Fix randomness source

### Step 3: Upgrade Priority

| Demo | Priority | Reason |
|------|----------|--------|
| `demo-with-receipts.sh` | ✅ DONE | Already real! |
| `01-local-basics/demo.sh` | 🔴 HIGH | Entry point for users |
| `demo-hybrid.sh` | 🔴 HIGH | Shows HSM comparison |
| `demo-human-entropy.sh` | 🟡 MEDIUM | Experimental/research |
| `demo-genetic-realistic.sh` | 🟡 MEDIUM | Advanced concepts |

---

## Implementation Plan

### Today: Audit & Upgrade Core Demos

#### 1. Audit `01-local-basics/demo.sh`
```bash
# Check what it does
cat showcase/01-local-basics/demo.sh

# Does it use BearDog CLI? ✅/❌
# Does it generate receipts? ✅/❌
# Is crypto real? ✅/❌
```

**Action**: Upgrade to use:
- Real `beardog entropy collect`
- Real `beardog key generate`
- Real `beardog encrypt/decrypt`
- Generate receipt for each operation

#### 2. Upgrade `demo-hybrid.sh`
```bash
# Current: Simulated HSM comparison
# Target: Real crypto operations

# Part 1: SoftHSM2
- Real key generation in SoftHSM2
- Real encryption with SoftHSM2 key
- Receipt with PKCS#11 token info

# Part 2: Solo V2 Keys
- Real key generation on hardware
- Real encryption with hardware key
- Receipt with device serial number

# Part 3: Performance Comparison
- Real timing measurements
- Real throughput tests
- Receipt with benchmark data

# Part 4: Genetic Evolution
- Real key parameter optimization
- Real fitness measurements
- Receipt with evolution data
```

#### 3. Upgrade `demo-human-entropy.sh`
```bash
# Current: Data collection real, scores simulated
# Target: Real entropy analysis

# Add:
- Real Shannon entropy calculation
- Real chi-square test
- Real NIST randomness tests (if available)
- Receipt with test results
```

#### 4. Upgrade `demo-genetic-realistic.sh`
```bash
# Current: Conceptual JSON files
# Target: Real key operations

# Use BearDog CLI or OpenSSL:
- Real master key generation
- Real sub-key derivation (HKDF)
- Real key mixing (XOR + KDF)
- Real delegation key creation
- Receipt for each operation
```

---

## BearDog CLI Coverage

### What BearDog CLI CAN Do (use it!)
```bash
✅ beardog entropy collect
✅ beardog key generate
✅ beardog encrypt
✅ beardog decrypt
✅ beardog hsm discover
```

### What BearDog CLI CAN'T Do Yet (use OpenSSL)
```bash
❌ Key hierarchy (master → sub)
   → Use: openssl HKDF
   
❌ Key mixing
   → Use: openssl + XOR + KDF
   
❌ Delegation constraints
   → Use: JSON metadata + openssl
   
❌ Multi-party signatures
   → Use: openssl ed25519
   
❌ Receipt generation
   → Use: bash + sha256sum + jq
```

**Document gaps** in each receipt:
```json
{
  "beardog_cli": {
    "used": false,
    "reason": "Feature not implemented yet",
    "fallback": "OpenSSL HKDF",
    "issue_tracker": "https://github.com/.../issues/123"
  }
}
```

---

## Verification Tests

After upgrading each demo, verify:

### ✅ Uniqueness Test
```bash
# Run demo twice
./demo.sh > /dev/null
SHA1=$(sha256sum outputs/seed_*.json | awk '{print $1}')

./demo.sh > /dev/null
SHA2=$(sha256sum outputs/seed_*.json | awk '{print $1}')

# Should be different!
[ "$SHA1" != "$SHA2" ] && echo "✅ UNIQUE" || echo "❌ NOT UNIQUE"
```

### ✅ Receipt Verification Test
```bash
# Get hash from receipt
RECEIPT_HASH=$(jq -r '.outputs."seed.json".hash' receipts/receipt_*.json)

# Get actual file hash
ACTUAL_HASH=$(sha256sum outputs/seed.json | awk '{print $1}')

# Should match!
[ "$RECEIPT_HASH" = "$ACTUAL_HASH" ] && echo "✅ VALID" || echo "❌ INVALID"
```

### ✅ Encryption Test
```bash
# Encrypt
./demo.sh encrypt test.txt

# Verify ciphertext != plaintext
PLAIN_HASH=$(sha256sum test.txt | awk '{print $1}')
CIPHER_HASH=$(sha256sum test.txt.enc | awk '{print $1}')

[ "$PLAIN_HASH" != "$CIPHER_HASH" ] && echo "✅ ENCRYPTED" || echo "❌ NOT ENCRYPTED"

# Decrypt and verify
./demo.sh decrypt test.txt.enc
DECRYPT_HASH=$(sha256sum test.txt.dec | awk '{print $1}')

[ "$PLAIN_HASH" = "$DECRYPT_HASH" ] && echo "✅ VERIFIED" || echo "❌ CORRUPTED"
```

---

## Success Criteria

A demo is "real, validatable, and live" when:

✅ **Real Operations**
- Uses actual cryptographic algorithms (not simulated)
- Produces unique outputs per run
- Uses BearDog CLI where available

✅ **Validatable**
- Every operation generates a receipt
- Every output has a SHA-256 hash
- User can independently verify hashes
- Receipts include verification commands

✅ **Live**
- Can be run on any tower
- Produces visible, inspectable outputs
- Receipts saved to disk
- Verification commands work

---

## Rollout Plan

### Phase 1: Core Demos (Today)
1. ✅ Audit all existing demos
2. ⏳ Upgrade `demo-with-receipts.sh` (DONE!)
3. ⏳ Upgrade `01-local-basics/demo.sh`
4. ⏳ Upgrade `demo-hybrid.sh`

### Phase 2: Advanced Demos (This Week)
5. Upgrade `demo-human-entropy.sh`
6. Upgrade `demo-genetic-realistic.sh`
7. Test all demos end-to-end

### Phase 3: Integration (Next Week)
8. Create Phase 3 demos (network)
9. Create Phase 4 demos (distributed)
10. Full showcase walkthrough

---

## Documentation

Each demo will include:

1. **README.md**: What it demonstrates
2. **VERIFICATION.md**: How to verify receipts
3. **receipts/*.json**: All cryptographic receipts
4. **outputs/**: All generated files

---

*Audit Plan - December 10, 2025*  
*Goal: Real, validatable, live demos for ALL showcase phases*
