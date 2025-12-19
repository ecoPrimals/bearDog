# 🚀 Making All Demos Live - Action Plan

## Current Status

### ✅ Completed
1. **Receipt Library** (`lib/receipt_functions.sh`) - Shared functions for all demos
2. **`01-local-basics/demo.sh`** - Upgraded with real BearDog CLI + receipts ✅
3. **`demo-with-receipts.sh`** - Created with real crypto ✅
4. **`demo-real-crypto.sh`** - Created with real crypto ✅
5. **Key ID mismatch** - Fixed ✅

### ⏳ In Progress
- Upgrading remaining demos to be live

---

## Remaining Demos to Upgrade

### 1. `demo-hybrid.sh` (HSM Comparison)
**Current**: Simulated HSM operations  
**Target**: Real SoftHSM2 + Solo V2 key operations

**Changes Needed**:
```bash
# Real SoftHSM2 operation
beardog key generate --hsm softhsm2 --key-id test-soft

# Real Solo V2 operation (when available)
beardog key generate --hsm fido2 --key-id test-solo

# Real performance comparison
# Real genetic evolution with actual fitness measurements
```

**Receipts**: Generate for each HSM operation

---

### 2. `demo-genetic-realistic.sh` (Advanced Key Operations)
**Current**: Conceptual JSON files  
**Target**: Real cryptographic operations

**Changes Needed**:
```bash
# Real master key
beardog key generate --key-id master-key

# Real sub-key derivation (HKDF if not in CLI, use OpenSSL)
openssl kdf -kdfopt digest:SHA256 -kdfopt key:$MASTER -kdfopt salt:$SALT

# Real key mixing
# Combine two keys with XOR + KDF

# Real delegation with constraints
# JSON metadata + real key material
```

**Receipts**: For master, sub-keys, mixed keys, delegated keys

---

### 3. `demo-human-entropy.sh` (Entropy Experiments)
**Current**: Real data collection, simulated scores  
**Target**: Real entropy quality measurements

**Changes Needed**:
```bash
# Real Shannon entropy calculation
ent entropy_file.bin  # Use 'ent' tool if available

# Real chi-square test
# Calculate chi-square statistic

# Real NIST tests (if available)
# Or implement basic randomness tests
```

**Receipts**: For each modality and mixed result

---

## Standardization Approach

All demos will follow this pattern:

### 1. Try BearDog CLI First
```bash
if "$BEARDOG_CLI" <command> 2>&1; then
    print_success "Operation with BearDog CLI!"
    BEARDOG_USED=true
else
    print_info "Fallback: Using OpenSSL/other tool..."
    # Real crypto with fallback
    BEARDOG_USED=false
fi
```

### 2. Always Generate Receipts
```bash
# Generate receipt with CLI usage tracking
RECEIPT=$(generate_receipt_with_method \
    "operation_name" \
    "$BEARDOG_USED" \
    "$FALLBACK_REASON" \
    "$INPUT_FILES" \
    "$OUTPUT_FILES")
```

### 3. Include Verification Instructions
```bash
# In receipt
{
  "verification": {
    "commands": [
      "sha256sum output.bin",
      "Compare with receipt hash"
    ],
    "expected_hash": "abc123..."
  }
}
```

---

## Receipt Enhancement

Update `lib/receipt_functions.sh` to include:

```bash
generate_receipt_with_method() {
    local operation=$1
    local beardog_used=$2
    local fallback_reason=$3
    local input_files=$4
    local output_files=$5
    
    # Add to receipt JSON:
    "beardog_cli": {
        "used": $beardog_used,
        "version": "$(beardog --version 2>/dev/null || echo 'N/A')",
        "fallback_used": $([ "$beardog_used" = "false" ] && echo "true" || echo "false"),
        "fallback_reason": "$fallback_reason",
        "fallback_tool": "openssl" # or whatever was used
    }
}
```

---

## Testing Plan

For each upgraded demo:

### Phase 1: Basic Functionality
- [ ] Demo runs without errors
- [ ] All operations complete
- [ ] Files are generated

### Phase 2: Real Crypto Verification
- [ ] Entropy is unique (run twice, different hashes)
- [ ] Encryption changes data (plaintext ≠ ciphertext)
- [ ] Decryption restores data (decrypted = original)

### Phase 3: Receipt Verification
- [ ] All receipts generated
- [ ] Hashes in receipts match actual files
- [ ] Verification commands work

### Phase 4: CLI Integration
- [ ] BearDog CLI used where available
- [ ] Fallback works when CLI unavailable
- [ ] Receipt tracks which method used

---

## Rollout Schedule

### Today (December 10)
1. ✅ Fix `01-local-basics/demo.sh` key ID mismatch
2. ⏳ Upgrade `demo-hybrid.sh`
3. ⏳ Upgrade `demo-genetic-realistic.sh`
4. ⏳ Upgrade `demo-human-entropy.sh`

### Testing
5. ⏳ Test all demos end-to-end
6. ⏳ Verify all receipts
7. ⏳ Document any remaining gaps

### Documentation
8. ⏳ Update showcase README
9. ⏳ Create verification guide
10. ⏳ Mark Phase 2 as complete

---

## Success Criteria

All demos are "live" when:
- ✅ Use real cryptographic operations (no simulations)
- ✅ Generate verifiable receipts
- ✅ Use BearDog CLI where available
- ✅ Have working fallbacks
- ✅ Pass all verification tests
- ✅ Are documented and reproducible

---

## Known Gaps (To Be Implemented Later)

Features that demos conceptually show but aren't in BearDog CLI yet:
1. Key hierarchy (master → sub-keys)
2. Key mixing (combining seeds)
3. Delegation constraints (time, quota, scope)
4. Multi-party renewal
5. Threshold schemes (k-of-n)

**Approach**: Use OpenSSL/cryptographic libraries for real operations, document as "not yet in CLI"

---

*Making Demos Live - December 10, 2025*  
*Goal: Every demo uses real crypto and generates verifiable receipts*
