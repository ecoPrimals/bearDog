# 🔄 Showcase Demo Upgrade Plan

## Goal
Replace all conceptual/simulated operations in demos with REAL BearDog CLI commands.

---

## Available Real Commands

### ✅ Now Available in BearDog CLI

```bash
# 1. Enhanced key generation with KDF
beardog key generate --key-id X --kdf pbkdf2 --kdf-iterations 100000

# 2. Key derivation (master → derived)
beardog key derive --master-key X --purpose Y --output Z --expires-in 24h

# 3. Key mixing (household/multi-party)
beardog key mix --key1 X --key2 Y --output Z --threshold "2-of-2"

# 4. Delegation with constraints (tower sharing)
beardog key delegate \
    --master-key X \
    --delegate-to Y \
    --output Z \
    --time-range "9:00-17:00" \
    --weekdays "mon-fri" \
    --cpu-quota 50 \
    --memory-quota "8GB" \
    --expires-in 30d

# 5. Lineage visualization
beardog key lineage --key-id X

# 6. Revocation
beardog key revoke --key-id X --reason "reason"
beardog key check-revocation --key-id X
beardog key list-revocations

# 7. Standard operations (already working)
beardog entropy collect --device software --output seed.json
beardog encrypt --key X --input file.txt --output file.enc
beardog decrypt --key X --input file.enc --output file.txt
```

---

## Demo Upgrade Status

### 1. `demo-genetic-realistic.sh` ⏳ IN PROGRESS
**Current**: Conceptual JSON files  
**Target**: Real BearDog CLI operations

**Upgrades Needed**:
- Part 1: Use `beardog key generate` with real KDF
- Part 2: Use `beardog key derive` for sub-keys
- Part 3: Use `beardog key mix` for household keys
- Part 4: Use `beardog key delegate` for tower sharing
- Part 5: Use `beardog key lineage` to show relationships
- Part 6: Use `beardog key revoke` for revocation demo
- Add receipts (SHA-256 hashes of all outputs)
- Add verification steps

### 2. `demo-hybrid.sh`
**Current**: Simulated HSM operations  
**Target**: Real SoftHSM2 + hardware operations

**Upgrades Needed**:
- Use real `beardog key generate --hsm softhsm2`
- Use real `beardog key generate --hsm fido2` (if Solo V2 available)
- Real performance comparison
- Real timing measurements
- Add receipts

### 3. `demo-human-entropy.sh`
**Current**: Real data collection, simulated scores  
**Target**: Real entropy quality measurements

**Upgrades Needed**:
- Add real Shannon entropy calculation
- Add real chi-square tests
- Add real randomness tests
- Add receipts

### 4. `01-local-basics/demo.sh` ✅ ALREADY UPGRADED
**Status**: Uses real BearDog CLI + generates receipts

---

## Upgrade Strategy

### Step 1: Update demo-genetic-realistic.sh

Replace conceptual parts with:

```bash
# Part 1: Master Key Generation
SESSION_ID=$(date +%s)
RECEIPTS_DIR="outputs/receipts-${SESSION_ID}"
mkdir -p "$RECEIPTS_DIR"

echo "🔑 Part 1: Master Key Generation"
beardog key generate \
    --key-id master-key-gen0 \
    --algorithm aes-256-gcm \
    --hsm software \
    --kdf argon2 \
    --purpose "Master key for family" \
    --expires-in 1y

# Generate receipt
cat > "$RECEIPTS_DIR/receipt-master-key.json" << EOF
{
  "operation": "master_key_generation",
  "key_id": "master-key-gen0",
  "timestamp": "$(date -Iseconds)",
  "kdf": "argon2",
  "verifiable": true,
  "verification": "beardog key info --key-id master-key-gen0"
}
