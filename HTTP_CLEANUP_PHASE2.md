# HTTP Dependency Cleanup - Phase 2: Complete Purge
**Date**: January 17, 2026  
**Goal**: Remove ALL HTTP client dependencies from BearDog  
**Philosophy**: ecoPrimals = Unix Sockets + tarpc/json-rpc ONLY!

---

## 🎯 Critical Understanding

**ecoPrimals Architecture**:
```
✅ Inter-Primal Communication: Unix Sockets + tarpc/json-rpc
✅ Service Discovery: Unix sockets (no HTTP registration!)
✅ BTSP: Pure Unix sockets
✅ Capability Discovery: Unix sockets

❌ HTTP client (reqwest): NOT NEEDED for ANY primal!
```

**Only Songbird** needs HTTP **server** for external AI services.

---

## 📊 Complete reqwest Audit

Crates still using reqwest:
1. beardog-adapters
2. beardog-capabilities  
3. beardog-core
4. beardog-monitoring
5. beardog-tunnel (now optional ✅)

**Action**: Remove from ALL!

---

## 🚀 Systematic Cleanup

### Phase 2.1: beardog-capabilities
### Phase 2.2: beardog-core
### Phase 2.3: beardog-monitoring
### Phase 2.4: beardog-adapters

Let's proceed!

