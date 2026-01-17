# HTTP Client Complete Removal - The Correct Approach

**Date**: January 17, 2026  
**Revelation**: BearDog should have ZERO HTTP client code!

## 🎯 The Truth

**ecoPrimals = Unix + tarpc, NOT HTTP!**

- BearDog: Unix sockets ONLY
- Songbird: HTTP SERVER (not client!)
- Inter-primal: tarpc/json-rpc over Unix sockets

**HTTP client (reqwest) in BearDog = LEGACY MISTAKE!**

## 🔥 Correct Action: DELETE, Not Feature-Gate

Instead of making reqwest optional, DELETE IT:

1. Remove reqwest from Cargo.toml (all crates)
2. Delete/stub HTTP client code
3. If tests break, delete those tests (they're testing legacy code!)

## 📋 Complete Removal Plan

### Step 1: Delete reqwest from dependencies
```toml
# beardog-capabilities/Cargo.toml - DONE ✅
# beardog-monitoring/Cargo.toml - DONE ✅
# beardog-core/Cargo.toml - DELETE (not optional!)
# beardog-adapters/Cargo.toml - DELETE (not optional!)
# beardog-tunnel/Cargo.toml - DELETE (not optional!)
```

### Step 2: Delete HTTP client code
- auth_services.rs - DELETE (uses HTTP)
- infant_discovery.rs - DELETE (uses HTTP)
- universal_service_mesh_client.rs - DELETE (uses HTTP)
- network_discoverer.rs - DELETE (uses HTTP)
- UpaClient - DELETE (uses HTTP to talk to Songbird, should be Unix!)

### Step 3: If it breaks, it's legacy!
- Production server (modes/server.rs) doesn't use this code
- Tests for HTTP code should be deleted
- Adapters for external HTTP services can stay feature-gated

## 🎊 Result

TRUE UniBin: Zero HTTP client dependencies!

Let's execute this correctly!

