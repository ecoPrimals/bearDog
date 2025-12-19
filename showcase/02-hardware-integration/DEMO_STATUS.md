# Demo Status: Real vs Simulated

## Current Demos: CONCEPTUAL

The demos we just ran are **conceptual/educational**:

### What They Do ✅
- Show the CONCEPTS (hierarchy, mixing, delegation)
- Show what the OUTPUT would look like
- Demonstrate the workflow
- Create JSON files with example structure

### What They DON'T Do ❌
- Not calling actual BearDog CLI commands
- Not performing real cryptographic operations
- Not generating verifiable receipts
- Not producing unique cryptographic material

## Why Simulated?

**BearDog CLI doesn't have these features yet!**

The demos show:
1. What SHOULD exist (requirements)
2. What the UX should look like
3. What outputs we expect

But the actual implementation needs to be built.

## What We Need: Real Receipts

For cryptography, we need PROOF:

```json
{
  "operation": "key_generation",
  "timestamp": "2025-12-10T20:00:00Z",
  "input_entropy_hash": "sha256:abc123...",
  "output_key_hash": "sha256:def456...",
  "proof": {
    "signature": "ed25519:...",
    "public_key": "...",
    "verifiable": true
  },
  "receipt_id": "receipt-unique-789",
  "can_verify": "yes - hash the input, compare output"
}
```

## Next: Create Real Demo

Need to:
1. Use actual `beardog` CLI commands where they exist
2. Generate real cryptographic receipts
3. Prove uniqueness with hashes
4. Show verification steps

