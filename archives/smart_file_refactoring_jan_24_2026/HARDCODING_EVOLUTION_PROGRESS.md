# Hardcoding Evolution - Progress Report

## Session: January 24, 2026

### Completed Evolution

#### 1. `btsp_provider.rs` - Peer Discovery ✅

**Before** (Hardcoded):
```rust
addresses.push(format!("192.168.1.5:10000"));  // ❌ Hardcoded IP
addresses.push(format!("10.0.0.3:10001"));     // ❌ Hardcoded IP
```

**After** (Capability-Based):
```rust
// Query for peer discovery capability from ecosystem
match self.discover_peer_addresses_via_capability(peer_id).await {
    Ok(discovered_addresses) if !discovered_addresses.is_empty() => {
        addresses.extend(discovered_addresses);
    }
    // Graceful degradation if discovery unavailable
}
```

**Implementation**:
- Implements Primal IPC Protocol (JSON-RPC over Unix sockets)
- Queries Songbird at `/primal/songbird` for peer addresses
- Graceful fallback if discovery service unavailable
- Zero hardcoded IPs in production path

**Pattern Applied**: Runtime capability discovery per `wateringHole/PRIMAL_IPC_PROTOCOL.md`

---

## Evolution Strategy

### Phase 1: Network Discovery (In Progress)
- ✅ Peer address discovery evolved to capability-based
- 🔄 Port numbers (next)
- 🔄 Service endpoints (next)

### Phase 2: Configuration Over Hardcoding
- Environment variables for defaults
- Runtime config file support
- Zero compile-time constants

### Phase 3: Complete Self-Discovery
- Primal discovers own capabilities
- Queries ecosystem for other primals
- All addresses from discovery or config

---

## Remaining Hardcoded Values

### Network (Priority: HIGH)
- [ ] Port 8500 (Consul) - in multiple config files
- [ ] Port 9000, 8080, 9090 - default service ports
- [ ] "localhost" references (522 instances) - many in tests (acceptable)

### Paths (Priority: MEDIUM)  
- [ ] Library search paths - can use runtime discovery
- [ ] Config directories - already uses XDG standards

### Test Values (Priority: LOW)
- Tests can use hardcoded values (acceptable per policy)
- Mock values for deterministic testing

---

## Philosophy Applied

**"Primal code only has self knowledge and discovers other primals at runtime"**

✅ **Achievement**: BearDog no longer assumes peer addresses exist at specific IPs.
Instead, it queries the ecosystem via capability-based discovery.

This is TRUE primal autonomy! 🎉

---

**Next**: Continue evolving remaining network hardcoding to discovery/config.

