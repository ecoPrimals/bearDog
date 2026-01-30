# TARPC Removal - Deep Debt Solution

**Date**: January 29, 2026  
**Decision**: Remove TARPC infrastructure cleanly  
**Rationale**: Deep debt means eliminating partial implementations, not leaving them incomplete

---

## Problem

TARPC infrastructure was partially implemented:
- ✅ Service trait defined (`BearDogService` with 6 methods)
- ✅ Service implementation complete (`BearDogServiceImpl`)
- ❌ Server handler incomplete (bridges to JSON-RPC instead of using tarpc server)
- ❌ No actual tarpc clients in production code (only documentation)
- ❌ Inefficient bridge pattern (bincode → JSON → bincode)

**Result**: We are NOT "JSON-RPC AND TARPC first" - only JSON-RPC first with incomplete TARPC stub.

---

## Analysis

### JSON-RPC Status
- ✅ **Comprehensive**: 8+ handler modules (crypto, graph_security, federation, btsp, capabilities, health, etc.)
- ✅ **Complete**: Full handler registry with trait-based system
- ✅ **Production-ready**: 2000+ references across codebase
- ✅ **Actually used**: Primary protocol for all inter-primal communication
- **Methods**: 30+ capabilities

### TARPC Status
- ⚠️ **Limited**: Only 6 methods (ping, capabilities, evaluate_trust, birdsong_encrypt/decrypt, security_metrics)
- ⚠️ **Incomplete**: Server handler bridges to JSON-RPC, not using tarpc service
- ⚠️ **Unused**: No production tarpc clients found (only documentation)
- ❌ **Inefficient**: Bridge pattern adds serialization overhead
- **Methods**: 6 capabilities (subset of JSON-RPC)

### Ecosystem Usage
Multiple primals have `tarpc` dependency but **no actual tarpc client usage** found in production code:
- Songbird: has dependency, uses JSON-RPC
- Squirrel: has dependency, uses JSON-RPC
- NestGate: has dependency, uses JSON-RPC
- ToadStool: has dependency, uses JSON-RPC

---

## Decision: Remove TARPC Cleanly

Following the principle of **"deep debt solutions, not symptoms"**:

1. **Remove partial implementation** - Don't leave incomplete infrastructure
2. **Be honest about architecture** - We are "JSON-RPC first" not "JSON-RPC AND TARPC first"
3. **Eliminate complexity** - Remove unused code and dependencies
4. **Follow Rust idioms** - "If you're not using it, remove it"

---

## What Was Removed

### Files Deleted
- `crates/beardog-tunnel/src/tarpc_service.rs` - Service trait and implementation
- Documentation files referencing tarpc as "PRIMARY" protocol

### Code Removed
- `handle_tarpc_persistent()` method in server.rs
- `Protocol::Tarpc` variant in types.rs
- Protocol detection for "TRPC" magic bytes
- tarpc dependency from Cargo.toml

### Documentation Updated
- Removed "tarpc AND json-rpc first" claims
- Updated to "JSON-RPC first" accurately
- Noted that tarpc can be added in future if needed

---

## Benefits

1. **Honesty**: No longer claiming to support tarpc when it's incomplete
2. **Simplicity**: Removed ~500 lines of unused/incomplete code
3. **Clarity**: Architecture is now clearly "JSON-RPC first"
4. **Maintainability**: Less code to maintain, no partial implementations
5. **Rust Idioms**: Following "you aren't gonna need it" (YAGNI)

---

## Future Consideration

If TARPC is needed in future:
1. **When**: If type-safety becomes critical for specific inter-primal protocols
2. **How**: Implement properly with actual tarpc server (not bridge pattern)
3. **Why**: JSON-RPC already provides comprehensive, flexible, production-ready RPC

**Current verdict**: JSON-RPC is sufficient and excellent for all needs.

---

## Philosophy

> **"Deep debt means addressing root causes, not symptoms."**

Partial implementations are technical debt. The right solution is:
- **Complete them properly**, OR
- **Remove them cleanly**

We chose removal because:
- JSON-RPC already provides everything TARPC would
- No production usage of TARPC found
- Bridge pattern was inefficient
- Honesty about architecture is better than claims we can't support

---

**Result**: BearDog is now cleanly "JSON-RPC first" without partial TARPC infrastructure.
