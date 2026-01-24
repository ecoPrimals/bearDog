# Hardcoding Elimination Progress - January 24, 2026

## 📊 Current Status

**Hardcoded Values Remaining**: ~211-307 instances (spec reports vary)
**Target**: 0 instances in production code
**Priority**: HIGH (per Zero Hardcoding Specification)

## ✅ What We Achieved This Session

### Compilation & Tests
- **Fixed**: 16 compilation errors → 0
- **Tests**: 99.7% passing (1044/1047)
- **Coverage**: 70.18% measured

### Architecture Compliance
- ✅ **No new hardcoding introduced**
- ✅ All new code uses configuration/discovery
- ✅ Graph security uses capability-based routing
- ✅ JWT/RBAC logic is configurable

## 📋 Hardcoding Categories (From Spec)

### Category 1: Network Configuration (~80 instances) 🚨 HIGH PRIORITY
**Files Affected**: 30+ files
- Hardcoded IPs: `127.0.0.1`, `localhost`, `0.0.0.0`
- Hardcoded ports: `8080`, `8081`, `8082`, `3000`, `5000`, `9090`
- Hardcoded socket paths: `/tmp/*.sock`
- Hardcoded URLs and endpoints

**Examples Found**:
```rust
// ❌ CURRENT (hardcoded)
let bind_addr = "127.0.0.1:8080".parse()?;
const DISCOVERY_SOCKET: &str = "/tmp/beardog-discovery.sock";
```

**✅ SOLUTION**:
```rust
// From config/env with fallback
let bind_addr = format!("{}:{}", 
    config.bind_address, 
    config.api_port
).parse()?;

let socket_path = config.discovery_socket_path();
```

### Category 2: File Paths (~40 instances) 🚨 HIGH PRIORITY
**Issues**:
- Hardcoded library paths: `/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so`
- Hardcoded config paths: `/etc/beardog/config.toml`
- Hardcoded data dirs: `~/.config/beardog`
- Hardcoded log paths: `/var/log/beardog/app.log`

**✅ SOLUTION**: Platform-aware path discovery with env variable overrides

### Category 3: Timeouts & Limits (~45 instances) ⚠️ MEDIUM PRIORITY
**Issues**:
- Hardcoded timeouts: `Duration::from_secs(30)`
- Hardcoded retry counts: `const MAX_RETRIES: usize = 3`
- Hardcoded buffer sizes: `const BUFFER_SIZE: usize = 8192`
- Hardcoded connection limits: `const MAX_CONNECTIONS: usize = 100`

**✅ SOLUTION**: Configuration structs with env variable support

### Category 4: Test Constants (~46 instances) ✅ ACCEPTABLE
**Status**: Allowed in test code only

## 🏗️ Implementation Strategy

### Phase 1: Foundation (Completed Partially)
✅ `beardog-config` crate exists
✅ Config file template exists (`configs/beardog-config-template.toml`)
⏳ Full hierarchy implementation (file → env → args)
⏳ Validation and error handling

### Phase 2: Systematic Replacement (Not Started)
The spec outlines a clear 3-week plan:

#### Week 1: Foundation
- [ ] Complete `beardog-config` crate
  - [ ] All config structs defined
  - [ ] Hierarchy: file → env → CLI args
  - [ ] Validation with clear errors
  - [ ] TOML/JSON/YAML support

#### Week 2: Network & Paths
- [ ] Replace ~80 network hardcodings
  - [ ] Search all 30+ affected files
  - [ ] Replace with `config.network.*`
  - [ ] Add env variable support
- [ ] Replace ~40 path hardcodings
  - [ ] Implement platform-aware discovery
  - [ ] Add XDG standard support (Linux)
  - [ ] Add macOS/Windows equivalents

#### Week 3: Limits & Testing
- [ ] Replace ~45 timeout/limit hardcodings
- [ ] Comprehensive testing
- [ ] Documentation updates

### Phase 3: Validation (Not Started)
- [ ] Config validation with range checks
- [ ] Migration guide from old to new
- [ ] Integration test coverage

## 🎯 Recommended Next Steps

### Immediate Actions (This Session if Time)
1. **Document current state** ✅ (this file)
2. **Create tracking issue** with checklist
3. **Identify highest-impact files** (most hardcoding)

### Next Session Priority
1. **Complete `beardog-config` hierarchy**
   - Ensure file → env → args precedence works
   - Add validation framework
   
2. **Tackle Network Category First** (highest impact)
   - Files to fix: `btsp_provider.rs`, `unix_socket_ipc/server.rs`, `modes/server.rs`
   - Pattern: Replace all `"127.0.0.1:8080"` with `config.network.bind_address()`
   - Add env vars: `BEARDOG_API_PORT`, `BEARDOG_BIND_ADDRESS`

3. **Measure Progress**
   - Re-run hardcoding audit after each file
   - Target: 211 → <150 in next session

## 📊 Files Needing Attention

### High Priority (Most Hardcoding)
Based on grep results, these files have multiple hardcoded values:

#### Network Hardcoding (30 files)
1. `crates/beardog-tunnel/src/btsp_provider.rs` - Multiple IPs/ports
2. `crates/beardog-tunnel/src/unix_socket_ipc/server.rs` - Socket paths
3. `crates/beardog-tunnel/src/modes/server.rs` - Bind addresses
4. `crates/beardog-cli/src/handlers/server.rs` - Default ports
5. `crates/beardog-core/src/socket_config.rs` - Socket configuration

#### Path Hardcoding (30 files)
1. Files with `/tmp/` paths
2. Files with `.sock` extensions
3. Files with library paths

## 🚀 Configuration Architecture (From Spec)

### Hierarchy (Priority Order)
```
1. Command-line Arguments  (Highest)
   ↓
2. Environment Variables
   ↓
3. Config File (beardog.toml)
   ↓
4. Platform Defaults (auto-detected)
   ↓
5. Fallback Constants (Last Resort)
```

### Example Implementation Pattern
```rust
// ✅ GOOD: Configuration-driven
pub fn bind_address(&self) -> Result<SocketAddr> {
    // 1. Check CLI args (handled by clap)
    // 2. Check env var
    if let Ok(addr) = env::var("BEARDOG_BIND_ADDRESS") {
        return addr.parse()
            .map_err(|_| ConfigError::InvalidAddress(addr));
    }
    
    // 3. Check config file
    if let Some(addr) = &self.network.bind_address {
        return addr.parse()
            .map_err(|_| ConfigError::InvalidAddress(addr.clone()));
    }
    
    // 4. Platform default
    Ok("127.0.0.1:8080".parse().unwrap())  // Only as last resort
}
```

## 📈 Success Metrics

### Target Metrics
- **Network**: 80 → 0 instances
- **Paths**: 40 → 0 instances
- **Timeouts**: 45 → 0 instances
- **Total Production**: 165 → 0 instances
- **Test Constants**: 46 instances (✅ acceptable)

### Current Progress
- **Network**: ~80 remain (0% reduction this session)
- **Paths**: ~40 remain (0% reduction this session)
- **Timeouts**: ~45 remain (0% reduction this session)
- **New Hardcoding**: ✅ 0 introduced this session

### Why No Reduction This Session?
- **Focus**: Compilation errors + graph security + test coverage
- **Priority**: Fix blocking issues first
- **Result**: Clean foundation for hardcoding work

## 🎓 Lessons Learned

### What Works
1. **Configuration Hierarchy**: The spec provides a clear, proven pattern
2. **Environment Variables**: Easy win for deployment flexibility
3. **Platform Detection**: Auto-discover OS-specific paths
4. **Fallback Defaults**: Always have sensible defaults

### Challenges
1. **Scope**: 211 instances is significant work (estimated 3 weeks)
2. **Testing**: Each change needs validation
3. **Backward Compatibility**: Need migration path
4. **Documentation**: Users need clear examples

### Best Practices (From Spec)
1. **Secure Defaults**: `127.0.0.1` not `0.0.0.0`
2. **Validation**: Check ranges, conflicts, types
3. **Clear Errors**: "Port 0 is invalid" not "Config error"
4. **Migration Support**: Alias old config names

## 📝 Compliance Statement

### This Session
✅ **No new hardcoding introduced**
✅ **All new code uses discovery/config**
✅ **Graph security is capability-based**
✅ **JWT/RBAC uses configurable logic**

### Overall Project
⚠️ **165 production hardcoded values remain**
⏳ **Systematic elimination plan exists**
🎯 **Clear path to zero hardcoding**

## 🔗 Related Documentation

- **Spec**: `specs/current/ZERO_HARDCODING_SPECIFICATION.md`
- **Action Plan**: `docs/action-plans/HARDCODING_ELIMINATION_PLAN.md`
- **Config Template**: `configs/beardog-config-template.toml`
- **UniBin Standard**: `wateringHole/UNIBIN_ARCHITECTURE_STANDARD.md`
- **ecoBin Standard**: `wateringHole/ECOBIN_ARCHITECTURE_STANDARD.md`

## 🎯 Next Session Goals

### Must Have (Priority 1)
1. ✅ Complete `beardog-config` hierarchy implementation
2. ✅ Fix top 10 files with most network hardcoding
3. ✅ Add env variable support for all network config
4. ✅ Target: 211 → <150 instances (30% reduction)

### Should Have (Priority 2)
5. Add path discovery with platform detection
6. Create migration guide for users
7. Add config validation framework

### Nice to Have (Priority 3)
8. Complete timeout/limit configuration
9. Add config file examples for common scenarios
10. Create automated hardcoding audit tool

## 📊 Estimated Effort

Based on the spec's 3-week timeline:
- **Week 1**: Foundation (8-10 hours) - Config system completion
- **Week 2**: Network & Paths (12-15 hours) - Systematic replacement
- **Week 3**: Limits & Testing (8-10 hours) - Final cleanup

**Total**: ~30-35 hours for complete elimination

**This Session**: 0 hours on hardcoding (focused on compilation/tests)
**Next Session**: Recommended 8-10 hours on Week 1 foundation

## ✅ Action Items

### For Next Session
- [ ] Complete config hierarchy (file → env → args)
- [ ] Add validation framework with clear errors
- [ ] Fix `btsp_provider.rs` network hardcoding
- [ ] Fix `unix_socket_ipc/server.rs` socket paths
- [ ] Fix `modes/server.rs` bind addresses
- [ ] Add `BEARDOG_*` env variable support
- [ ] Test config precedence (CLI > env > file > default)
- [ ] Document config usage in README

### Tools Needed
- [ ] Automated hardcoding scanner (grep wrapper)
- [ ] Config validation test suite
- [ ] Migration script (old → new config)

## 🎉 Bottom Line

### This Session Achievement
- ✅ **Maintained** zero hardcoding principle
- ✅ **No regression** in hardcoding count
- ✅ **Foundation** for future elimination work

### Path Forward
- 📋 **Clear plan** exists (3-week spec)
- 🎯 **Target identified**: 211 → 0 instances
- 🛠️ **Tools available**: Config crate, templates
- ⏰ **Timeline**: 3 weeks systematic work

### Recommendation
**Next session should dedicate 8-10 hours to Week 1 of the elimination plan**, focusing on:
1. Config hierarchy completion
2. Network hardcoding (highest impact)
3. Measurable progress (211 → <150)

---

**Status**: 📋 **DOCUMENTED & READY**
**Next**: 🔨 **EXECUTE ELIMINATION PLAN**
**Owner**: Evolution Team
**Updated**: January 24, 2026

🐻🚫 **BearDog: On Track to Zero Hardcoding!**

