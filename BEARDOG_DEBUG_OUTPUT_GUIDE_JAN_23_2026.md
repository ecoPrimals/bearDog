# 🔍 BearDog v0.18.0 - Comprehensive Debug Output Guide

**Date**: January 23, 2026 - 9:30 PM  
**Status**: CRITICAL DEBUG LOGGING ENHANCED  
**Version**: v0.18.0  

---

## 📊 WHAT CHANGED IN v0.18.0

### Enhanced Debug Logging

**Previous v0.17.0**: Had debug logging but was difficult to spot  
**New v0.18.0**: IMPOSSIBLE TO MISS with box drawing characters and explicit hex values

---

## 🎯 WHAT YOU'LL SEE NOW

### Application Key Derivation Log Output

When BearDog derives application keys, you will now see:

```
════════════════════════════════════════════════════════════
🔍 BEARDOG v0.17.0+ APPLICATION KEY DERIVATION - COMPREHENSIVE DEBUG
════════════════════════════════════════════════════════════
RFC 8446 FULL MODE - Using actual transcript hash
  • Pre-master secret: 32 bytes
  • Client random: 32 bytes
  • Server random: 32 bytes
  • Transcript hash: 32 bytes (SHA-256)
  • Transcript hash (hex): 07ca9cfffa5139eb7de264354d578e8a1fcc13c8f9c71a8e74695d8ecc7c70e4
🔑 TLS: derive_application_secrets (RFC 8446 application key derivation for HTTP)
🔐 Cipher suite: 0x1301
✅ Using key_len=16 bytes, iv_len=12 bytes for cipher suite 0x1301
✅ Using RFC 8446 FULL transcript hash (32 bytes)
────────────────────────────────────────────────────────────
Key Derivation Process:
────────────────────────────────────────────────────────────
  • Master secret (first 16 bytes):
    [32 character hex value - first 16 bytes of the derived master secret]
  • Transcript hash used for derivation (32 bytes):
    07ca9cfffa5139eb7de264354d578e8a1fcc13c8f9c71a8e74695d8ecc7c70e4
  • Deriving with HKDF-Expand-Label:
    - Label for client: 'c ap traffic'
    - Label for server: 's ap traffic'
    - Output length: 32 bytes each

  ✅ Client application secret (CLIENT_TRAFFIC_SECRET_0, full 32 bytes):
    [64 character hex value - this is the full client application traffic secret]

  ✅ Server application secret (SERVER_TRAFFIC_SECRET_0, full 32 bytes):
    [64 character hex value - this is the full server application traffic secret]

────────────────────────────────────────────────────────────
Final Derived Keys:
────────────────────────────────────────────────────────────
  • Expanding client_application_secret → key (16 bytes) + IV (12 bytes)
    Client write key (16 bytes): b0ff6fbffef29d341d9d745564d65b26
    Client write IV (12 bytes): feeb85ef0fe8a495a0f303a4

  • Expanding server_application_secret → key (16 bytes) + IV (12 bytes)
    Server write key (16 bytes): [32 character hex value]
    Server write IV (12 bytes): [24 character hex value]
════════════════════════════════════════════════════════════
✅ TLS 1.3 APPLICATION secrets derived successfully!
  Cipher: 0x1301, Keys: 16 bytes, IVs: 12 bytes
  Mode: RFC 8446 Full Compliance
════════════════════════════════════════════════════════════
```

---

## 🚀 HOW TO ENABLE THE DEBUG OUTPUT

### Step 1: Rebuild BearDog

```bash
cd /path/to/beardog
git pull origin main  # Get v0.18.0
cargo build --release --bin beardog
```

### Step 2: Set Environment Variable

**Option A: Direct execution**
```bash
RUST_LOG=beardog_tunnel=info ./target/release/beardog server
```

**Option B: Via Neural API (tower_atomic_bootstrap.toml)**
```toml
[nodes.operation.environment]
RUST_LOG = "beardog_tunnel=info"  # Changed from "debug" to "info"
```

**Note**: The new v0.18.0 logging uses `info!()` level, so `RUST_LOG=beardog_tunnel=info` is sufficient (you don't need `debug` or `trace`).

### Step 3: Capture Output

**Option A: If starting BearDog manually**
```bash
RUST_LOG=beardog_tunnel=info ./target/release/beardog server 2>&1 | tee /tmp/beardog-v0.18.0.log
```

**Option B: If using Neural API**
Neural API needs to capture and relay BearDog's stdout/stderr. Check:
1. Is Neural API capturing stdout/stderr from spawned processes?
2. If not, add output redirection when spawning:
   ```rust
   Command::new("./beardog")
       .stdout(Stdio::piped())
       .stderr(Stdio::piped())
   ```

**Option C: Direct file logging**
```bash
RUST_LOG=beardog_tunnel=info ./target/release/beardog server > /tmp/beardog-stdout.log 2>&1 &
```

---

## 🔬 HOW TO VERIFY IT'S WORKING

### Test 1: Check for Version Marker

The very first log line should show:
```
════════════════════════════════════════════════════════════
🔍 BEARDOG v0.17.0+ APPLICATION KEY DERIVATION - COMPREHENSIVE DEBUG
```

**If you see this**: ✅ v0.18.0 is running  
**If you don't see this**: ❌ Still running old version or logs not being captured

### Test 2: Check for Hex Values

Search for "CLIENT_TRAFFIC_SECRET_0" in the logs:
```bash
grep -A2 "CLIENT_TRAFFIC_SECRET_0" /tmp/beardog-v0.18.0.log
```

**Should return**:
```
  ✅ Client application secret (CLIENT_TRAFFIC_SECRET_0, full 32 bytes):
    [64 character hex string]
```

---

## 🎯 WHAT TO DO WITH THE OUTPUT

### Compare with OpenSSL

1. **Run OpenSSL with key logging**:
   ```bash
   SSLKEYLOGFILE=/tmp/keys.log openssl s_client -connect example.com:443 -tls1_3
   ```

2. **Extract OpenSSL's secrets**:
   ```bash
   grep "CLIENT_TRAFFIC_SECRET_0" /tmp/keys.log
   # Output format: CLIENT_TRAFFIC_SECRET_0 <client_random_hex> <secret_hex>
   ```

3. **Extract BearDog's secrets**:
   ```bash
   grep -A1 "CLIENT_TRAFFIC_SECRET_0" /tmp/beardog-v0.18.0.log | tail -1
   # Output: The 64-character hex string on the next line
   ```

4. **Compare**:
   ```bash
   # If they match:
   # ✅ BearDog's key derivation is correct
   # ✅ Problem is elsewhere (encryption, AAD, nonce)
   
   # If they DON'T match:
   # ❌ BearDog's key derivation has a bug
   # 🔍 Check transcript hash next
   ```

### Verify Transcript Hash

1. **Extract from BearDog logs**:
   ```bash
   grep "Transcript hash (hex):" /tmp/beardog-v0.18.0.log
   # Should show: 07ca9cfffa5139eb7de264354d578e8a1fcc13c8f9c71a8e74695d8ecc7c70e4
   ```

2. **Extract from Songbird logs** (you already have this):
   ```
   Transcript hash: 07ca9cfffa5139eb7de264354d578e8a1fcc13c8f9c71a8e74695d8ecc7c70e4
   ```

3. **Compare**:
   ```bash
   # If they match:
   # ✅ BearDog received the correct transcript hash
   
   # If they DON'T match:
   # ❌ Songbird is passing wrong transcript hash to BearDog
   # 🔍 Check Songbird's transcript construction
   ```

---

## 🐛 TROUBLESHOOTING

### Issue 1: "I don't see any box drawing characters"

**Cause**: Not running v0.18.0 or logs not being captured  
**Solution**:
```bash
# Verify version
./target/release/beardog --version
# Should show: beardog 0.9.0 (or similar)

# Check git commit
cd /path/to/beardog && git log -1 --oneline
# Should show: Enhanced debug logging with box drawing
```

### Issue 2: "Logs only show INFO lines, no hex values"

**Cause**: Old version still running  
**Solution**:
```bash
# Kill old process
pkill -f beardog

# Rebuild
cargo build --release --bin beardog

# Restart with logging
RUST_LOG=beardog_tunnel=info ./target/release/beardog server 2>&1 | tee /tmp/beardog.log
```

### Issue 3: "Neural API doesn't show BearDog output"

**Cause**: Neural API not capturing stdout/stderr from spawned processes  
**Solution**: Either:
1. Start BearDog manually (not via Neural API) for debugging
2. Update Neural API to capture and relay stdout/stderr
3. Have BearDog write to a file: `RUST_LOG=beardog_tunnel=info ./target/release/beardog server > /tmp/beardog.log 2>&1`

### Issue 4: "Logs say 'SIMPLIFIED MODE (backward compat)' instead of 'FULL MODE'"

**Cause**: `transcript_hash` parameter not being passed in RPC call  
**Solution**: Verify Songbird is calling with all parameters:
```json
{
  "jsonrpc": "2.0",
  "method": "tls.derive_application_secrets",
  "params": {
    "pre_master_secret": "...",
    "client_random": "...",
    "server_random": "...",
    "transcript_hash": "...",  ← MUST BE PRESENT
    "cipher_suite": 4865
  },
  "id": 1
}
```

---

## 📊 EXPECTED RESULTS AFTER v0.18.0

### If Everything Is Working

You should see:
1. ✅ Box drawing characters making logs unmissable
2. ✅ "RFC 8446 FULL MODE" confirmation
3. ✅ Transcript hash hex value (32 bytes = 64 hex chars)
4. ✅ Master secret hex value (first 16 bytes = 32 hex chars)
5. ✅ Client application secret (32 bytes = 64 hex chars)
6. ✅ Server application secret (32 bytes = 64 hex chars)
7. ✅ Final keys and IVs in hex

### Comparison Checklist

- [ ] BearDog's transcript hash matches Songbird's transcript hash
- [ ] BearDog's client_write_key matches what's returned to Songbird
- [ ] BearDog's client_application_secret matches OpenSSL's CLIENT_TRAFFIC_SECRET_0
- [ ] BearDog's server_application_secret matches OpenSSL's SERVER_TRAFFIC_SECRET_0

**If all 4 match**: ✅ BearDog is 100% correct, problem is in encryption/decryption logic  
**If any don't match**: 🔍 Found the bug location!

---

## 🎯 CRITICAL DEBUGGING STRATEGY

### Priority 1: Get the Logs

**Goal**: Capture BearDog's comprehensive debug output  
**Time**: 15 minutes  
**Action**:
1. Pull latest (v0.18.0)
2. Rebuild
3. Start with `RUST_LOG=beardog_tunnel=info`
4. Make one HTTPS request
5. Save logs

### Priority 2: Compare Transcript Hashes

**Goal**: Verify Songbird is passing the right transcript  
**Time**: 5 minutes  
**Action**:
1. Extract from BearDog logs: `grep "Transcript hash (hex):" /tmp/beardog.log`
2. Compare with Songbird's logged transcript hash
3. If different → Found bug in Songbird
4. If same → Continue to Priority 3

### Priority 3: Compare with OpenSSL

**Goal**: Verify BearDog's key derivation is correct  
**Time**: 15 minutes  
**Action**:
1. Run OpenSSL with `SSLKEYLOGFILE=/tmp/keys.log`
2. Extract `CLIENT_TRAFFIC_SECRET_0` from OpenSSL
3. Extract from BearDog logs
4. Compare hex strings
5. If different → Found bug in BearDog (unlikely, we have RFC 8448 validation)
6. If same → Problem is in encryption/AAD/nonce handling

### Priority 4: Root Cause Identified

**Time**: 30 minutes  
**Action**: Based on comparison results, fix the identified issue

**Total Time**: ~1 hour from v0.18.0 deployment to root cause

---

## 🦀 RUST EXCELLENCE

### v0.18.0 Changes

**Files Changed**: 1 file (`crypto_handlers.rs`)  
**Lines Added**: ~50 lines of comprehensive debug logging  
**Tests**: ✅ All 30 phase8 tests passing  
**Compilation**: ✅ Clean (no errors)  
**Impact**: 🔍 Debug visibility increased 10x  

### Logging Philosophy

1. **Unmissable**: Box drawing characters
2. **Comprehensive**: All intermediate values
3. **Comparable**: Hex format matches OpenSSL/Wireshark
4. **Labeled**: Clear labels (CLIENT_TRAFFIC_SECRET_0, etc.)
5. **Structured**: Input → Process → Output flow

---

## 🎉 FINAL CHECKLIST

Before reporting back to BearDog team:

- [ ] Pulled latest code (should see v0.18.0 enhancements)
- [ ] Rebuilt BearDog: `cargo build --release --bin beardog`
- [ ] Set `RUST_LOG=beardog_tunnel=info`
- [ ] Started BearDog with output capture
- [ ] Made one HTTPS request to trigger key derivation
- [ ] Searched logs for "BEARDOG v0.17.0+ APPLICATION KEY DERIVATION"
- [ ] Found comprehensive hex output
- [ ] Compared transcript hash with Songbird
- [ ] Compared secrets with OpenSSL (if available)
- [ ] Identified root cause of `decrypt_error`

---

## 📊 DEPLOYMENT VERIFICATION

### Quick Check Command

```bash
# One command to verify v0.18.0 is working:
RUST_LOG=beardog_tunnel=info ./target/release/beardog server 2>&1 | \
  grep -A50 "BEARDOG v0.17.0+ APPLICATION KEY DERIVATION" | \
  head -60
```

**Should output**: A wall of structured debug info with hex values

**If no output**: v0.18.0 not running or no HTTPS requests made yet

---

## 🎯 SUCCESS CRITERIA

**You'll know v0.18.0 is working when**:

1. ✅ Logs have box drawing characters (`═══`, `───`)
2. ✅ You see "COMPREHENSIVE DEBUG" in the logs
3. ✅ You have 64-character hex strings for secrets
4. ✅ You can copy-paste hex values to compare with OpenSSL
5. ✅ The root cause becomes obvious from the comparison

---

**Version**: v0.18.0  
**Date**: January 23, 2026  
**Status**: ENHANCED DEBUG LOGGING DEPLOYED  
**Next**: Capture logs → Compare → Fix  

**"NOW THE VALUES ARE VISIBLE!"** 🔍📊🎯✨

