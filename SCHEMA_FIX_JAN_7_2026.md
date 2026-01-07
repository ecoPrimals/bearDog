# 🔧 Schema Fix - Songbird Compatibility

**Date**: January 7, 2026  
**Status**: ✅ **COMPLETE** - Ready for Deployment  
**Binary**: `target/release/beardog-server` (6.4MB)  
**MD5**: `3f299e9385daae9149e6e7720051f4ef`

---

## 🎯 Issues Fixed

### Issue 1: Missing `decision` Field ✅

**Problem**: Songbird requires a `decision` field in trust evaluation responses, but BearDog wasn't sending it.

**Error**: `"missing field 'decision'"`

**Solution**: Added `decision` field to all trust evaluation responses.

**Mapping**:
- `trust_level: 0` ("none") → `decision: "reject"`
- `trust_level: 1` ("limited") → `decision: "auto_accept"` (same family = auto-accept for genetic lineage)
- `trust_level: 2` ("elevated") → `decision: "auto_accept"` (future)
- `trust_level: 3` ("highest") → `decision: "auto_accept"` (future)

---

### Issue 2: Environment Variable Compatibility ✅

**Problem**: BearDog was only reading `FAMILY_ID` and `NODE_ID`, but biomeOS sets `BEARDOG_FAMILY_ID` and `BEARDOG_NODE_ID`.

**Result**: BearDog returned `"our_family": "unknown"` even though env vars were set.

**Solution**: Added fallback support for both naming conventions:
- Tries `FAMILY_ID` first
- Falls back to `BEARDOG_FAMILY_ID`
- Same for `NODE_ID` / `BEARDOG_NODE_ID`

**Applied to all methods**:
- `capabilities` / `get_capabilities`
- `identity` / `whoami` / `get_identity`
- `trust.evaluate_peer` / `security.evaluate`
- `lineage` / `get_lineage`

---

## 📊 New Response Format

### Trust Evaluation Response

```json
{
  "decision": "auto_accept",             // ← NEW: Songbird requires this
  "trust_level": 1,                      // Integer (backward compat)
  "trust_level_name": "limited",         // String (Songbird compat)
  "reason": "same_genetic_family",
  "peer_id": "tower2",
  "peer_family": "nat0",
  "our_family": "nat0",                  // ← Now reads correctly from env
  "our_node": "tower1",                  // ← Now reads correctly from env
  "evaluated_by": "beardog",
  "capabilities": {
    "allowed": ["birdsong/*", "coordination/*", "health", ...],
    "denied": ["data/*", "commands/*", "keys/*", ...]
  },
  "metadata": {
    "policy_version": 1,
    "evaluation_method": "genetic_family_match",
    "timestamp": "2026-01-07T12:00:00Z"
  }
}
```

---

## ✅ What's Fixed

### Schema Compatibility
- ✅ Added `decision` field (required by Songbird)
- ✅ Kept `trust_level` (integer) for backward compatibility
- ✅ Kept `trust_level_name` (string) for Songbird
- ✅ Kept `capabilities` for policy hints
- ✅ Kept `metadata` for extensibility

### Environment Variables
- ✅ Reads `FAMILY_ID` or `BEARDOG_FAMILY_ID`
- ✅ Reads `NODE_ID` or `BEARDOG_NODE_ID`
- ✅ No more `"unknown"` values when env vars are set
- ✅ Applied to all IPC methods

---

## 🚀 Deployment

### Binary Details
- **File**: `target/release/beardog-server`
- **Size**: 6.4MB
- **MD5**: `3f299e9385daae9149e6e7720051f4ef`
- **Version**: 0.15.0
- **Build**: January 7, 2026

### Environment Variables (Both Work)

**Option A** (Standard):
```bash
export FAMILY_ID=nat0
export NODE_ID=tower1
export BEARDOG_HSM_MODE=software
```

**Option B** (biomeOS Convention):
```bash
export BEARDOG_FAMILY_ID=nat0
export BEARDOG_NODE_ID=tower1
export BEARDOG_HSM_MODE=software
```

**Both options work!** BearDog tries `FAMILY_ID` first, then falls back to `BEARDOG_FAMILY_ID`.

### Deploy to Towers

```bash
# Copy binary
scp target/release/beardog-server tower1:/usr/local/bin/

# Start (environment already set by biomeOS)
./beardog-server
```

---

## 🎯 Expected Behavior

### Same Family (tower1 evaluates tower2)

**Request**:
```json
{
  "method": "trust.evaluate_peer",
  "params": {
    "peer_id": "tower2",
    "peer_family": "nat0"
  }
}
```

**Response**:
```json
{
  "decision": "auto_accept",        // ← Songbird can now parse this
  "trust_level": 1,
  "trust_level_name": "limited",
  "reason": "same_genetic_family",
  "our_family": "nat0",             // ← No longer "unknown"
  "our_node": "tower1",             // ← No longer "unknown"
  "peer_family": "nat0"
}
```

**Songbird Action**: ✅ **Auto-accepts tower2 for genetic lineage federation**

---

### Different Family

**Request**:
```json
{
  "method": "trust.evaluate_peer",
  "params": {
    "peer_id": "remote-peer",
    "peer_family": "other-family"
  }
}
```

**Response**:
```json
{
  "decision": "reject",             // ← Songbird rejects
  "trust_level": 0,
  "trust_level_name": "none",
  "reason": "different_family",
  "our_family": "nat0",
  "peer_family": "other-family"
}
```

**Songbird Action**: ✅ **Rejects peer (different family)**

---

## 📊 Decision Field Mapping

| Trust Level | Name | Decision | Meaning |
|-------------|------|----------|---------|
| 0 | "none" | "reject" | Different/unknown family → Reject |
| 1 | "limited" | "auto_accept" | Same family → Auto-accept for coordination |
| 2 | "elevated" | "auto_accept" | Human approved (future Phase 2) |
| 3 | "highest" | "auto_accept" | Human entropy (future Phase 2) |

**Note**: For Phase 1, only levels 0 and 1 are used. Levels 2 and 3 are reserved for future policy phases.

---

## ✅ Verification

### Test Locally

```bash
# Set environment
export FAMILY_ID=test-family
export NODE_ID=test-node
export BEARDOG_HSM_MODE=software

# Start server
./target/release/beardog-server &

# Test trust evaluation
echo '{"jsonrpc":"2.0","method":"trust.evaluate_peer","params":{"peer_id":"test","peer_family":"test-family"},"id":1}' | nc -U /tmp/beardog-test-family-test-node.sock

# Expected: decision: "auto_accept", our_family: "test-family"
```

### Test with biomeOS Env Vars

```bash
# Set biomeOS-style environment
export BEARDOG_FAMILY_ID=nat0
export BEARDOG_NODE_ID=tower1
export BEARDOG_HSM_MODE=software

# Start server
./target/release/beardog-server &

# Test
echo '{"jsonrpc":"2.0","method":"identity.get_family","id":1}' | nc -U /tmp/beardog-nat0-tower1.sock

# Expected: family: "nat0", node: "tower1" (not "unknown")
```

---

## 🎊 What This Enables

### Immediate
- ✅ Songbird can parse BearDog responses (no more "missing field 'decision'")
- ✅ BearDog correctly identifies itself (no more "unknown" family/node)
- ✅ Same-family auto-acceptance works (genetic lineage trust)
- ✅ Different-family rejection works (security)

### Federation
- ✅ Tower 1 ↔ Tower 2 federation unblocked
- ✅ Genetic lineage trust evaluation working
- ✅ Auto-accept for same family
- ✅ Auto-reject for different families

---

## 📚 Related Documents

- **[JAN_7_2026_COMPLETE_SESSION.md](JAN_7_2026_COMPLETE_SESSION.md)** - Complete session summary
- **[TRUST_POLICY_EVOLUTION_JAN_7_2026.md](TRUST_POLICY_EVOLUTION_JAN_7_2026.md)** - Trust policy details
- **[BIOMEOS_INTEGRATION_STATUS_JAN_7_2026.md](BIOMEOS_INTEGRATION_STATUS_JAN_7_2026.md)** - Integration guide
- **[ENVIRONMENT_VARIABLES.md](ENVIRONMENT_VARIABLES.md)** - All configuration variables

---

## 🎯 Changes Made

### Code Changes
1. **`unix_socket_ipc.rs`** - Added `decision` field to trust evaluation responses
2. **`unix_socket_ipc.rs`** - Added fallback env var support (`BEARDOG_FAMILY_ID`, `BEARDOG_NODE_ID`)
3. **Applied to 4 methods**: `capabilities`, `identity`, `trust.evaluate_peer`, `lineage`

### Binary
- **New MD5**: `3f299e9385daae9149e6e7720051f4ef`
- **Old MD5**: `82814abd4564d5b5198f348ccc680d2c`
- **Size**: 6.4MB (unchanged)

---

## ✅ Status

**Schema Compatibility**: ✅ Complete  
**Environment Variables**: ✅ Complete  
**Build**: ✅ Success  
**Binary**: ✅ Ready  
**Deployment**: ⏳ Ready for biomeOS towers

---

**Built**: January 7, 2026  
**Version**: 0.15.0  
**Status**: ✅ **Production Ready - Federation Unblocked**

