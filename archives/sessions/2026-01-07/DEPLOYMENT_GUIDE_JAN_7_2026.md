# 🚀 BearDog v0.15.0 - Deployment Guide

**Date**: January 7, 2026  
**Version**: 0.15.0 (Schema Fix + BTSP Contact Exchange)  
**Status**: ✅ **READY FOR PRODUCTION DEPLOYMENT**

---

## 📋 Quick Reference

**Binary**: `target/release/beardog-server`  
**MD5**: `12da9d23540ad189ea26a5c7d9b04546`  
**Size**: 6.4MB  
**Includes**:
- Schema fix (`decision` field)
- Environment variable compatibility
- BTSP contact exchange (all 6 endpoints)
- Zero unsafe code, zero hardcoding

---

## 🎯 Deployment Checklist

### Pre-Deployment ✅
- [x] Binary built successfully
- [x] All tests passing (28/28)
- [x] Documentation complete
- [x] Schema fix verified
- [x] BTSP endpoints verified

### For biomeOS Team
- [ ] Deploy binary to Tower 1
- [ ] Deploy binary to Tower 2
- [ ] Set environment variables
- [ ] Restart BearDog services
- [ ] Verify federation working
- [ ] Verify genetic lineage trust

### For Songbird Team
- [ ] Implement `SecurityAdapter.call_generic()`
- [ ] Wire `BtspClient` to `/btsp/contact/exchange`
- [ ] Test contact exchange integration
- [ ] Deploy Songbird v3.16.0

---

## 📦 Deployment Steps (biomeOS)

### Step 1: Backup Current Binary
```bash
# On each tower
sudo systemctl stop beardog
sudo cp /usr/local/bin/beardog-server /usr/local/bin/beardog-server.backup
```

### Step 2: Deploy New Binary
```bash
# Copy from build machine
scp target/release/beardog-server tower1:/tmp/beardog-server
scp target/release/beardog-server tower2:/tmp/beardog-server

# On each tower
sudo mv /tmp/beardog-server /usr/local/bin/beardog-server
sudo chmod +x /usr/local/bin/beardog-server
sudo chown root:root /usr/local/bin/beardog-server

# Verify MD5
md5sum /usr/local/bin/beardog-server
# Expected: 12da9d23540ad189ea26a5c7d9b04546
```

### Step 3: Verify Environment Variables
```bash
# On each tower, check/set these variables
cat << 'EOF' | sudo tee -a /etc/systemd/system/beardog.service.d/environment.conf
[Service]
Environment="BEARDOG_FAMILY_ID=nat0"
Environment="BEARDOG_NODE_ID=tower1"  # or tower2
Environment="FAMILY_ID=nat0"
Environment="NODE_ID=tower1"  # or tower2
EOF

sudo systemctl daemon-reload
```

### Step 4: Start Services
```bash
# On each tower
sudo systemctl start beardog
sudo systemctl status beardog

# Check logs
journalctl -u beardog -f
```

### Step 5: Verify Schema Fix
```bash
# Test trust evaluation (from another tower or local)
curl -X POST http://localhost:9000/api/trust/evaluate \
  -H "Content-Type: application/json" \
  -d '{
    "peer_id": "tower2",
    "peer_family": "nat0"
  }'

# Expected response should include:
# - "decision": "auto_accept" or "reject"
# - "trust_level": 1 (integer)
# - "trust_level_name": "limited" (string)
# - "our_family": "nat0" (not "unknown")
# - "our_node": "tower1" (not "unknown")
```

### Step 6: Verify BTSP Endpoints
```bash
# Test contact exchange
curl -X POST http://localhost:9000/btsp/contact/exchange \
  -H "Content-Type: application/json" \
  -d '{
    "target_peer_id": "tower2",
    "requester_lineage": "tower1-lineage",
    "max_hops": 3
  }'

# Expected response:
# {
#   "success": true,
#   "data": {
#     "contact": {
#       "peer_id": "tower2",
#       "addresses": ["192.168.1.5:10000", ...],
#       "lineage_proof": "lineage_proof_...",
#       "lineage_path": ["nat0", "tower2"],
#       "search_depth": 2,
#       "last_seen": "2026-01-07T..."
#     }
#   }
# }
```

---

## 🔧 Environment Variables Reference

### Required Variables
```bash
# Primary (new format)
BEARDOG_FAMILY_ID=nat0       # Family identifier
BEARDOG_NODE_ID=tower1       # Node identifier

# Fallback (legacy format - also supported)
FAMILY_ID=nat0
NODE_ID=tower1

# Note: BearDog reads FAMILY_ID first, then falls back to BEARDOG_FAMILY_ID
```

### Optional Variables
```bash
# API bind address (if using HTTP)
BEARDOG_API_BIND_ADDR=0.0.0.0:0  # Port 0 = random available port

# Unix socket path (primary IPC)
BEARDOG_SOCKET_PATH=/tmp/beardog-${BEARDOG_FAMILY_ID}.sock
```

---

## 🧪 Testing Federation

### Test 1: Basic Health Check
```bash
# On Tower 1
curl http://localhost:9000/health

# Expected: {"status":"healthy"}
```

### Test 2: Trust Evaluation (Same Family)
```bash
# On Tower 1, evaluate Tower 2
curl -X POST http://localhost:9000/api/trust/evaluate \
  -H "Content-Type: application/json" \
  -d '{
    "peer_id": "tower2",
    "peer_family": "nat0"
  }'

# Expected:
# - decision: "auto_accept"
# - trust_level: 1
# - trust_level_name: "limited"
# - reason: "same_genetic_family"
# - our_family: "nat0" (NOT "unknown")
# - our_node: "tower1" (NOT "unknown")
```

### Test 3: Trust Evaluation (Different Family)
```bash
# Evaluate unknown peer
curl -X POST http://localhost:9000/api/trust/evaluate \
  -H "Content-Type: application/json" \
  -d '{
    "peer_id": "unknown-peer",
    "peer_family": "other-family"
  }'

# Expected:
# - decision: "reject"
# - trust_level: 0
# - trust_level_name: "none"
# - reason: "different_family"
```

### Test 4: BTSP Contact Exchange
```bash
# Request contact info for Tower 2
curl -X POST http://localhost:9000/btsp/contact/exchange \
  -H "Content-Type: application/json" \
  -d '{
    "target_peer_id": "tower2",
    "requester_lineage": "tower1-lineage",
    "max_hops": 3
  }'

# Expected: Contact info with addresses and lineage proof
```

### Test 5: BTSP Tunnel Establishment
```bash
# Establish secure tunnel
curl -X POST http://localhost:9000/btsp/tunnel/establish \
  -H "Content-Type: application/json" \
  -d '{
    "peer": {
      "id": "tower2",
      "endpoint": "192.168.1.5:10000",
      "public_key": null
    }
  }'

# Expected: Tunnel handle with id and established_at timestamp
```

---

## 🐛 Troubleshooting

### Issue 1: "unknown" Family/Node in Responses
**Symptoms**: Trust evaluation returns `"our_family": "unknown"`

**Solution**:
```bash
# Check environment variables
systemctl show beardog | grep Environment

# Ensure both formats are set
Environment="BEARDOG_FAMILY_ID=nat0"
Environment="FAMILY_ID=nat0"

# Restart service
sudo systemctl restart beardog
```

### Issue 2: "Missing field 'decision'" Error
**Symptoms**: Songbird reports parse error

**Solution**:
- Verify binary MD5: `12da9d23540ad189ea26a5c7d9b04546`
- This fix is included in the new binary
- If still failing, check Songbird version (needs v3.13.2+)

### Issue 3: Port Already in Use
**Symptoms**: `Address already in use (os error 98)`

**Solution**:
```bash
# Check what's using the port
sudo lsof -i :9000

# Option 1: Kill old process
sudo pkill beardog-server

# Option 2: Use random port
echo "BEARDOG_API_BIND_ADDR=0.0.0.0:0" >> /etc/systemd/system/beardog.service.d/environment.conf
sudo systemctl daemon-reload
sudo systemctl restart beardog
```

### Issue 4: Contact Exchange Returns Empty Path
**Symptoms**: `"lineage_path": []`

**Solution**:
- This is expected for unknown peers
- Peer must be in trust database or same family
- Future enhancement: multi-hop lineage traversal

---

## 📊 Monitoring

### Key Metrics to Watch
```bash
# Service status
systemctl status beardog

# Logs (federation events)
journalctl -u beardog -f | grep -E "(Trust|Federation|BTSP)"

# Trust evaluations
journalctl -u beardog | grep "Trust evaluation"

# BTSP operations
journalctl -u beardog | grep "BTSP"

# Errors
journalctl -u beardog -p err
```

### Expected Log Patterns

**Successful Federation**:
```
INFO beardog: ✅ Trust: SAME FAMILY - level 1 (limited) - peer: tower2, family: nat0
INFO beardog: ✅ Contact exchange: found 2 addresses for tower2 (depth: 2)
INFO beardog: ✅ BTSP tunnel established: tower2 -> btsp_abc123
```

**Failed Federation** (expected for different families):
```
WARN beardog: ⚠️  Trust: DIFFERENT FAMILY - level 0 (none) - peer: unknown, their: other, ours: nat0
WARN beardog: ⚠️  Peer unknown not found in genetic lineage
```

---

## 🔄 Rollback Procedure

If deployment fails or issues occur:

```bash
# Stop service
sudo systemctl stop beardog

# Restore backup
sudo cp /usr/local/bin/beardog-server.backup /usr/local/bin/beardog-server

# Start service
sudo systemctl start beardog

# Verify
systemctl status beardog
```

---

## 🎯 Success Criteria

### Phase 1: Schema Fix Verification ✅
- [ ] BearDog responds with `decision` field
- [ ] BearDog reports correct `our_family` and `our_node`
- [ ] Songbird can parse trust evaluation responses
- [ ] Same-family peers get `decision: "auto_accept"`
- [ ] Different-family peers get `decision: "reject"`

### Phase 2: BTSP Verification ✅
- [ ] Contact exchange endpoint responds
- [ ] Lineage paths are returned for known peers
- [ ] Addresses are returned for discovered peers
- [ ] All 6 BTSP endpoints functional
- [ ] Tunnel establishment works

### Phase 3: Federation Verification
- [ ] Tower 1 and Tower 2 discover each other
- [ ] Genetic lineage trust evaluation succeeds
- [ ] Encrypted communication established
- [ ] No "unknown" values in responses

---

## 📞 Support and Escalation

### If Issues Occur

**BearDog Issues**:
- Check logs: `journalctl -u beardog -p err`
- Verify binary MD5: `12da9d23540ad189ea26a5c7d9b04546`
- Verify environment variables set correctly
- See: `SCHEMA_FIX_JAN_7_2026.md`

**Songbird Integration Issues**:
- Verify Songbird version (v3.13.2+ for Phase 1)
- Check Songbird can reach BearDog endpoint
- See: `BTSP_IMPLEMENTATION_COMPLETE.md`

**Federation Issues**:
- Verify both towers in same family (`nat0`)
- Check UDP multicast working (Songbird discovery)
- Verify Unix sockets exist: `/tmp/beardog-nat0.sock`

---

## 🚀 Next Phase: Songbird Integration

### For Songbird Team (30 minutes)

**Step 1**: Add generic security adapter call (10 min)
```rust
// In SecurityAdapter
pub async fn call_generic(&self, endpoint: &str, payload: Value) -> Result<Value> {
    let url = format!("{}{}", self.endpoint, endpoint);
    let response = self.client.post(&url)
        .json(&payload)
        .send().await?;
    Ok(response.json().await?)
}
```

**Step 2**: Wire BTSP client (10 min)
```rust
// In BtspClient
pub async fn exchange_contact(
    &self,
    target_peer_id: &str,
    max_hops: usize
) -> Result<ContactInfo> {
    self.security_adapter.call_generic(
        "/btsp/contact/exchange",
        json!({
            "target_peer_id": target_peer_id,
            "requester_lineage": self.our_lineage_id(),
            "max_hops": max_hops
        })
    ).await
}
```

**Step 3**: Test and deploy (10 min)
```bash
# Test
cargo test btsp_contact_exchange

# Build
cargo build --release

# Deploy
# ... (Songbird deployment process)
```

---

## 📚 Documentation Reference

- **Schema Fix**: `SCHEMA_FIX_JAN_7_2026.md`
- **BTSP Implementation**: `BTSP_IMPLEMENTATION_COMPLETE.md`
- **BTSP Assessment**: `BTSP_SONGBIRD_HANDOFF_RESPONSE.md`
- **Session Summary**: `JAN_7_2026_SESSION_COMPLETE.md`
- **Current Status**: `STATUS.txt`
- **Quick Start**: `START_HERE.md`

---

## ✅ Deployment Sign-Off

**Binary Verified**: ✅ MD5: `12da9d23540ad189ea26a5c7d9b04546`  
**Tests Passing**: ✅ 28/28  
**Documentation**: ✅ Complete  
**Ready for Production**: ✅ YES

**Deployed by**: _________________  
**Date**: _________________  
**Towers**: Tower 1 ☐   Tower 2 ☐  
**Verified Working**: ☐

---

**Version**: BearDog v0.15.0 (Schema Fix + BTSP Contact Exchange)  
**Date**: January 7, 2026  
**Status**: ✅ **READY FOR PRODUCTION DEPLOYMENT**

