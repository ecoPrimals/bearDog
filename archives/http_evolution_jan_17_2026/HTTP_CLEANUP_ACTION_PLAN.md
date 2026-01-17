# BearDog HTTP Dependency Cleanup - Action Plan
**Date**: January 17, 2026  
**Goal**: Remove ALL unnecessary HTTP dependencies from BearDog  
**Impact**: Achieve TRUE UniBin (pure Rust, trivial cross-compilation)

---

## 🎯 Core Insight

**BTSP is pure Unix sockets now!** BearDog has **ZERO legitimate reason** for HTTP client dependencies.

### Architecture Reality

```
✅ PRODUCTION: Pure Unix Sockets
    BearDog ←→ Songbird = Unix sockets (NO HTTP client needed!)
    BearDog ←→ NestGate = Unix sockets
    BearDog ←→ ToadStool = Unix sockets
    BearDog ←→ Squirrel = Unix sockets

⚠️  OPTIONAL: HTTP Server (feature-gated, DEPRECATED)
    External clients → BearDog HTTP API (for debugging/legacy)
    This is HTTP SERVER, not HTTP CLIENT!
```

**Key Point**: BearDog needs HTTP **server** (axum, optional), NOT HTTP **client** (reqwest)!

---

## 📊 Current HTTP Dependencies

### Crates Using reqwest (HTTP Client)

1. **beardog-tunnel** - Main binary
2. **beardog-capabilities** - Capability discovery
3. **beardog-core** - Auth services, infant discovery
4. **beardog-monitoring** - Metrics/monitoring
5. **beardog-client** - HTTP client library (KEEP - it's a library!)
6. **beardog-discovery** - Service registry
7. **beardog-integration** - Integration tests
8. **beardog-node-registry** - Bootstrap, federation, discovery
9. **beardog-adapters** - Universal adapters, HTTP adapter

### Analysis

**HTTP Client (reqwest) Use Cases**:
- ❌ BTSP communication (NOW Unix sockets!)
- ❌ Songbird registration (NOW Unix socket discovery!)
- ❌ Service mesh (NOW Unix sockets!)
- ❌ Capability discovery (NOW Unix sockets!)
- ✅ **KEEP**: `beardog-client` crate (it's an HTTP client library)
- ✅ **KEEP**: Optional HTTP server (axum, for external access)

**Result**: ~90% of HTTP client usage is LEGACY!

---

##🎯 Three-Tier Cleanup Strategy

### Tier 1: Core Production Binary (IMMEDIATE - 2 hours)
**Goal**: Remove reqwest from production binary path

**Targets**:
- `beardog-tunnel` - Only needs HTTP server (axum), not client!
- `beardog-core` - Unix socket communication only
- `beardog-capabilities` - Unix socket discovery

**Impact**: Production binary 100% HTTP client free!

---

### Tier 2: Support Crates (1 week)
**Goal**: Migrate adapters and discovery to Unix sockets

**Targets**:
- `beardog-adapters` - Migrate to Unix socket adapters
- `beardog-discovery` - Unix socket service registry
- `beardog-node-registry` - Unix socket bootstrap
- `beardog-monitoring` - Unix socket metrics

**Impact**: Support ecosystem 100% Unix sockets

---

### Tier 3: Library Crates (KEEP AS-IS)
**Goal**: Maintain HTTP client libraries for external use

**Keep**:
- `beardog-client` - HTTP client library (public API)
- `beardog-integration` - Integration tests (may need HTTP)

**Rationale**: These are libraries for OTHER projects to use HTTP with BearDog

---

## 🚀 Phase 1: Immediate Action (TODAY!)

### Step 1: Audit Actual reqwest Usage

Check if reqwest is actually used in production paths:

```bash
# Find actual reqwest usage
cd crates/beardog-tunnel
rg "reqwest::" --type rust

# Check if it's only in deprecated/feature-gated code
rg "reqwest::" --type rust -C 5 | rg "deprecated|cfg\(feature"
```

### Step 2: Feature-Gate Existing HTTP Client Code

If reqwest is used, feature-gate it:

```toml
# Cargo.toml
[features]
default = []
http-client = ["reqwest"]  # Optional HTTP client

[dependencies]
# Remove from dependencies, add as optional
# reqwest = { workspace = true }

[dependencies.reqwest]
workspace = true
optional = true
```

Then in code:

```rust
#[cfg(feature = "http-client")]
use reqwest::Client;

#[cfg(feature = "http-client")]
pub async fn legacy_http_method() {
    // ... HTTP client code ...
}
```

### Step 3: Remove if Unused

If reqwest isn't actually used (just transitive dep):

```bash
# Simply remove from Cargo.toml
# crates/beardog-tunnel/Cargo.toml
# DELETE: reqwest = { workspace = true }

# Test build
cargo build --release --bin beardog
```

---

## 📋 Detailed Cleanup Checklist

### beardog-tunnel (Main Binary)

**Current Dependencies**:
- ✅ Keep: `axum`, `tower`, `tower-http` (HTTP server, feature-gated)
- ❌ Remove: `reqwest` (HTTP client, NOT NEEDED!)
- ✅ Keep: `tokio`, `tokio-rustls` (async runtime, TLS)
- ✅ Keep: `rustls`, `aws-lc-rs` (TLS, no HTTP)

**Action**:
```bash
# 1. Check if reqwest is actually used
cd crates/beardog-tunnel
rg "reqwest::" --type rust

# 2. If not used, remove from Cargo.toml
# DELETE line: reqwest = { workspace = true }

# 3. Test build
cargo build --release --bin beardog
cargo test -p beardog-tunnel
```

---

### beardog-core

**Current Usage**: Auth services, infant discovery, service mesh

**Analysis**: ALL of these should use Unix sockets now!

**Action**:
```bash
# 1. Check reqwest usage
cd crates/beardog-core
rg "reqwest::" --type rust -l

# Files found:
# - src/core/auth_services.rs
# - src/universal_service_mesh_client.rs
# - src/discovery/infant_discovery.rs

# 2. Check if these are legacy/unused
rg "auth_services::|ServiceMeshClient|InfantDiscovery" --type rust crates/beardog-tunnel/

# 3. If unused, comment out or feature-gate
# 4. Remove reqwest from Cargo.toml
```

**Expected Result**: These are likely LEGACY code paths not used in UniBin!

---

### beardog-capabilities

**Current Usage**: Capability discovery

**Analysis**: Songbird discovers BearDog via Unix socket!

**Action**:
```bash
# Check if HTTP client is actually used
cd crates/beardog-capabilities
rg "reqwest::" --type rust

# If used, check if it's for external capability registration
# This should be Unix socket based now!
```

---

### beardog-monitoring

**Current Usage**: Metrics/monitoring

**Analysis**: Prometheus metrics can be Unix socket or HTTP server

**Action**:
- Keep HTTP **server** for Prometheus scraping (axum)
- Remove HTTP **client** (reqwest) if unused

---

## 🎯 Success Criteria

### Tier 1 Complete (TODAY)

✅ **Production Binary**:
```bash
# Build without reqwest
cargo build --release --bin beardog

# Check dependencies
cargo tree -p beardog-tunnel -i reqwest
# Should show: reqwest NOT FOUND or OPTIONAL

# Verify no C crypto deps (except via rustls)
cargo tree -p beardog-tunnel | rg "ring|openssl"
# Should ONLY show: aws-lc-rs (via rustls, acceptable)
```

✅ **Tests Pass**:
```bash
cargo test -p beardog-tunnel
# All UniBin tests pass (48/48)
```

✅ **Binary Works**:
```bash
./target/release/beardog doctor
# Health check succeeds

./target/release/beardog server --socket /tmp/test.sock
# Server starts on Unix socket
```

---

## 🏆 Expected Results

### Before Cleanup

**C Dependencies**:
```
beardog-tunnel dependencies:
├── reqwest (HTTP client)
│   ├── rustls
│   │   └── aws-lc-rs → aws-lc-sys (C)
│   ├── ring (C) [DEPRECATED but present]
│   └── openssl-sys (C) [DEPRECATED but present]
```

**Build Time**: ~50s (with aws-lc-rs only)

---

### After Tier 1 Cleanup

**C Dependencies**:
```
beardog-tunnel dependencies:
├── axum (HTTP server, feature-gated)
│   └── NO HTTP CLIENT!
├── tokio
├── rustls
│   └── aws-lc-rs → aws-lc-sys (C) [ACCEPTABLE]
```

**Build Time**: ~40s (less to compile)

**Cross-Compilation**:
```bash
# Should "just work" now!
cargo build --target aarch64-linux-android --bin beardog
cargo build --target riscv64gc-unknown-linux-gnu --bin beardog
```

---

## 🚨 CRITICAL: What NOT to Remove

### Keep These HTTP-Related Crates

1. **axum**, **tower**, **tower-http**
   - These are HTTP **server** for external API
   - Feature-gated with `btsp-api`
   - Optional, not required for production

2. **reqwest** in beardog-client
   - This is a PUBLIC HTTP client library
   - Other projects use it to call BearDog's HTTP API
   - NOT part of BearDog binary

3. **rustls**, **tokio-rustls**
   - TLS is used for Unix socket mTLS
   - NOT just for HTTPS!

---

## 📊 Timeline

| Phase | Duration | Impact |
|-------|----------|--------|
| **Audit** | 30 min | Understand actual reqwest usage |
| **Tier 1 (Binary)** | 2 hours | Production binary HTTP-client-free |
| **Tier 2 (Support)** | 1 week | Support crates Unix-socket-based |
| **Tier 3 (Verify)** | 1 day | Cross-compilation testing |
| **Total** | **~2 weeks** | **TRUE UniBin achieved!** |

---

## 🎯 Next Steps

### Immediate (Next 30 minutes)

1. **Audit beardog-tunnel reqwest usage**
   ```bash
   cd crates/beardog-tunnel
   rg "reqwest::" --type rust -C 3
   ```

2. **Audit beardog-core reqwest usage**
   ```bash
   cd crates/beardog-core  
   rg "reqwest::" --type rust -l
   ```

3. **Check if code paths are active**
   ```bash
   # Search for usage in main binary
   rg "auth_services|InfantDiscovery|ServiceMeshClient" crates/beardog-tunnel/src/
   ```

### Follow-up (Next 2 hours)

4. **Remove or feature-gate unused reqwest**
5. **Test build and runtime**
6. **Verify cross-compilation**

---

## 📚 References

- [PURE_RUST_EVOLUTION_JAN_17_2026.md](../../PURE_RUST_EVOLUTION_JAN_17_2026.md)
- [Unix Socket IPC Server](crates/beardog-tunnel/src/unix_socket_ipc/mod.rs)
- [BTSP Provider](crates/beardog-tunnel/src/btsp_provider.rs)

---

**Status**: ✅ **READY TO EXECUTE**  
**Confidence**: **VERY HIGH** (BTSP is pure Unix, confirmed!)  
**Risk**: **VERY LOW** (just removing unused deps)

**Next**: Audit actual reqwest usage! 🚀

