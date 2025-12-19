# 🔧 Demo Refinements Needed

## Issues Found During Live Testing

### 1. Key ID Mismatch ✅ FIXED
**Issue**: 
- Key generated as `demo-key-1765416707`
- Encrypt/decrypt looking for `key_1765416707` (missing prefix)

**Fix Applied**:
```bash
# Before (line 346):
KEY_ID=$(basename "$KEY_FILE" .json)  # Returns: key_1765416707

# After:
TIMESTAMP_FROM_FILE=$(basename "$KEY_FILE" .json | sed 's/key_//')
KEY_ID="demo-key-${TIMESTAMP_FROM_FILE}"  # Returns: demo-key-1765416707
```

**Status**: ✅ Fixed in demo.sh

---

### 2. Additional Refinements Needed

#### A. Better Error Handling
Currently the demo falls back to OpenSSL when BearDog CLI fails, but we should:
- [ ] Log WHY it failed (for debugging)
- [ ] Show clearer fallback messages
- [ ] Track which method was used in receipts

#### B. Receipt Enhancement
- [ ] Add `beardog_cli_used: true/false` to receipts
- [ ] Add `fallback_reason` if OpenSSL was used
- [ ] Include CLI version in receipt

#### C. Key Storage
BearDog CLI stores keys internally, but demo also creates JSON files:
- [ ] Decide: Use BearDog's internal storage OR separate JSON files?
- [ ] Update receipts to reference correct key location
- [ ] Add key lookup helper function

#### D. Output Validation
- [ ] After each BearDog CLI command, validate the output
- [ ] Check if files were actually created
- [ ] Verify sizes are reasonable

---

## Priority Fixes

### HIGH Priority
1. ✅ **Key ID mismatch** - DONE
2. ⏳ **Key storage consistency** - Decide on storage approach
3. ⏳ **Receipt accuracy** - Add CLI usage tracking

### MEDIUM Priority
4. Better error messages
5. Output validation
6. Logging improvements

### LOW Priority
7. Performance metrics
8. Extended benchmarks

---

## Testing Checklist

After fixes, verify:
- [ ] Entropy collection works (BearDog CLI)
- [ ] Key generation works (BearDog CLI)
- [ ] Encryption works (BearDog CLI, not fallback)
- [ ] Decryption works (BearDog CLI, not fallback)
- [ ] Receipts generated correctly
- [ ] All hashes are verifiable
- [ ] Uniqueness proof works

---

## Next Steps

1. ✅ Fix key ID mismatch
2. ⏳ Test full demo end-to-end
3. ⏳ Apply same fixes to other demos
4. ⏳ Update all demos to match this pattern

---

*Refinements Tracking - December 10, 2025*
