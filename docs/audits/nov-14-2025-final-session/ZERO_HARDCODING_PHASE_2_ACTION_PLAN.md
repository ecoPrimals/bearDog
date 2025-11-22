# Zero Hardcoding Phase 2 - Action Plan

**Date**: November 14, 2025  
**Priority**: 🔴 **#1 CRITICAL**  
**Status**: IN PROGRESS (Partial Migration Complete)

---

## 🎯 MISSION

Eliminate all hardcoded values from production code per `specs/current/ZERO_HARDCODING_SPECIFICATION.md`

**Target**: 0 hardcoded values  
**Current**: 546 instances (346 IPs + 200 ports)  
**Progress**: ~40% complete (infrastructure exists, execution needed)

---

## ✅ WHAT'S ALREADY DONE

### Infrastructure Complete
- ✅ `beardog-config` crate exists
- ✅ Global `BEARDOG_CONFIG` available
- ✅ Deprecation notices on old constants
- ✅ Environment variable hierarchy established
- ✅ Config file support implemented

### Partial Migration Complete
Reviewing `crates/beardog-types/src/constants/domains/network.rs` shows:

- ✅ `default_api_port()` - migrated to BEARDOG_CONFIG
- ✅ `default_metrics_port()` - migrated to BEARDOG_CONFIG
- ✅ `default_health_port()` - migrated to BEARDOG_CONFIG
- ✅ `default_admin_port()` - migrated to BEARDOG_CONFIG
- ✅ `default_service_host()` - migrated to BEARDOG_CONFIG
- ✅ `default_bind_address()` - uses env vars
- ⚠️ `default_debug_port()` - still uses env vars directly (not in config)

**Deprecation Notices**: Present on old constants ✅

---

## 🔴 WHAT STILL NEEDS TO BE DONE

### Category 1: Network Hardcoding (346 instances)

**Audit found 346 hardcoded IPs across codebase**

**Top violators** (from audit):
- `crates/beardog-types/src/constants/domains/network.rs`: 14 instances
- `crates/beardog-config/src/domains/security.rs`: 13 instances
- `crates/beardog-types/src/canonical/config/runtime_config.rs`: 14 instances

**Patterns still hardcoded**:
```rust
// ❌ Still in code:
"127.0.0.1"
"0.0.0.0"
"localhost"
"192.168.x.x"
"10.0.x.x"
```

**Action Required**:
1. Find all remaining hardcoded IPs
2. Move to config or discovery
3. Update call sites to use config functions

### Category 2: Port Hardcoding (200 instances)

**Audit found 200 hardcoded ports**

**Common patterns**:
```rust
// ❌ Still in code:
:8080  (API)
:9090  (Metrics)
:5432  (PostgreSQL)
:6379  (Redis)
:3000  (Grafana)
```

**Action Required**:
1. Find all remaining hardcoded ports
2. Move to config
3. Update call sites

### Category 3: Constants as Configuration

**Issue**: Some constants should be configuration

**Currently hardcoded as constants**:
- DNS servers: `["8.8.8.8", "8.8.4.4", "1.1.1.1"]`
- Multicast addresses: `"224.0.0.251"`
- Timeout values: (many - but these MAY be okay as defaults)
- Buffer sizes: (these are probably okay as constants)

**Decision needed**: Which constants are legitimate compile-time values vs should be configurable?

---

## 📋 PHASE 2 ACTION PLAN

### Task 1: Complete Network IP Migration (8 hours)

**Goal**: Reduce 346 → <50 hardcoded IPs

**Steps**:

1. **Find all remaining hardcoded IPs** (2 hrs)
   ```bash
   cd /home/eastgate/Development/ecoPrimals/beardog
   grep -r "127\.0\.0\.1\|localhost\|192\.168\.\|10\.0\.\|172\.16\.\|0\.0\.0\.0" \
     --include="*.rs" crates/ | grep -v "test" | grep -v "\.md" > hardcoded_ips.txt
   ```

2. **Categorize by urgency** (1 hr)
   - Production code (CRITICAL)
   - Test code (ACCEPTABLE)
   - Comments/docs (IGNORE)
   - Constants (EVALUATE)

3. **Add to BEARDOG_CONFIG** (2 hrs)
   - Update `beardog-config` structs
   - Add environment variable support
   - Add config file support

4. **Update call sites** (3 hrs)
   - Replace hardcoded values with config calls
   - Test changes
   - Verify no regressions

**Acceptance Criteria**:
- [ ] <50 hardcoded IPs in production code
- [ ] All production IPs use config or discovery
- [ ] Tests still pass
- [ ] Documentation updated

### Task 2: Complete Port Migration (4 hours)

**Goal**: Reduce 200 → <20 hardcoded ports

**Steps**:

1. **Find all remaining hardcoded ports** (1 hr)
   ```bash
   grep -r ":[0-9]{4,5}" --include="*.rs" crates/ | \
     grep -v "test" | grep -v "http://" > hardcoded_ports.txt
   ```

2. **Categorize ports** (1 hr)
   - Service ports (migrate to config)
   - Well-known ports (may keep as constants: 80, 443, 22)
   - Database ports (migrate to config)

3. **Expand BEARDOG_CONFIG** (1 hr)
   - Add database port config
   - Add service discovery port config
   - Add custom service ports

4. **Update call sites** (1 hr)
   - Replace with config calls
   - Test
   - Verify

**Acceptance Criteria**:
- [ ] <20 hardcoded ports in production code
- [ ] All configurable ports use config
- [ ] Well-known ports documented as constants
- [ ] Tests pass

### Task 3: Path Configuration (4 hours)

**Goal**: Eliminate hardcoded paths

**Patterns to address**:
```rust
// ❌ Hardcoded paths
PathBuf::from("/etc/beardog/config.toml")
PathBuf::from("/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so")
PathBuf::from("~/.config/beardog")
```

**Steps**:

1. **Find hardcoded paths** (1 hr)
   ```bash
   grep -r '"/etc/\|"/usr/\|"/var/\|"~/' --include="*.rs" crates/ > hardcoded_paths.txt
   ```

2. **Implement path discovery** (2 hrs)
   - XDG Base Directory support
   - Platform-specific defaults
   - Library search paths

3. **Update call sites** (1 hr)
   - Use path discovery
   - Test on multiple platforms

**Acceptance Criteria**:
- [ ] No hardcoded absolute paths in production
- [ ] XDG compliance
- [ ] Platform-specific discovery working
- [ ] Tests pass on Linux, macOS, Windows

---

## 🎯 SUCCESS METRICS

### Before Phase 2
- **Hardcoded IPs**: 346
- **Hardcoded ports**: 200
- **Total**: 546 instances
- **Grade**: D+ (65/100)

### After Phase 2 (Target)
- **Hardcoded IPs**: <50
- **Hardcoded ports**: <20
- **Total**: <100 instances
- **Grade**: B (82/100)

### Phase 3 Target (Future)
- **Hardcoded IPs**: 0
- **Hardcoded ports**: 0
- **Total**: 0 instances
- **Grade**: A+ (100/100) - Spec compliance! ✅

---

## 📊 TRACKING PROGRESS

### Week 1 Checkpoint (End of this week)
- [ ] Task 1 complete (IPs: 346 → <50)
- [ ] Task 2 complete (Ports: 200 → <20)
- [ ] Task 3 complete (Paths: all moved to discovery)
- [ ] Documentation updated
- [ ] Tests passing
- [ ] **Target: <100 total hardcoded values**

### Measurement Commands
```bash
# Count remaining hardcoded IPs
grep -r "127\.0\.0\.1\|localhost\|192\.168\.\|10\.0\." \
  --include="*.rs" crates/ | grep -v test | wc -l

# Count remaining hardcoded ports
grep -r ":[0-9]{4,5}" --include="*.rs" crates/ | \
  grep -v test | grep -v "http://" | wc -l

# Total
echo "Total remaining: $((IP_COUNT + PORT_COUNT))"
```

---

## 🛠️ TOOLS & RESOURCES

### Available Tools
- `beardog-config` crate (already exists)
- `BEARDOG_CONFIG` global (already implemented)
- `scripts/hardcoding_eliminator.py` (if exists)

### Documentation
- `specs/current/ZERO_HARDCODING_SPECIFICATION.md` (the spec)
- `configs/beardog-config-template.toml` (config template)

### Helper Scripts to Create
```bash
# Script: find_hardcoded_values.sh
#!/bin/bash
echo "Finding hardcoded IPs..."
grep -r "127\.0\.0\.1\|localhost" --include="*.rs" crates/ | \
  grep -v test | wc -l

echo "Finding hardcoded ports..."
grep -r ":[0-9]{4,5}" --include="*.rs" crates/ | \
  grep -v test | wc -l
```

---

## 💡 BEST PRACTICES

### Do's ✅
1. **Use config hierarchy**:
   - CLI args (highest priority)
   - Environment variables
   - Config file
   - Platform defaults
   - Fallback constants (last resort)

2. **Deprecate gradually**:
   ```rust
   #[deprecated(note = "Use BEARDOG_CONFIG.network.api.port")]
   pub const DEFAULT_API_PORT: u16 = 8080;
   ```

3. **Document why**:
   ```rust
   // Universal constant - IP protocol definition
   pub const LOCALHOST_IPV4: &str = "127.0.0.1"; // OK
   ```

4. **Test with different configs**:
   - Default config
   - Custom env vars
   - Custom config file

### Don'ts ❌
1. **Don't hardcode in production code**:
   ```rust
   // ❌ BAD
   let addr = "127.0.0.1:8080".parse()?;
   
   // ✅ GOOD
   use beardog_config::global::BEARDOG_CONFIG;
   let addr = format!("{}:{}", 
       BEARDOG_CONFIG.network.api.bind_address,
       BEARDOG_CONFIG.network.api.port
   ).parse()?;
   ```

2. **Don't remove constants prematurely**:
   - Deprecate first
   - Give users time to migrate
   - Then remove

3. **Don't break tests unnecessarily**:
   - Test fixtures can use hardcoded values
   - But they should be clearly marked as test-only

---

## 🎓 EXAMPLES

### Before (Hardcoded)
```rust
// ❌ Hardcoded
pub fn connect_to_api() -> Result<Client> {
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(30))
        .build()?;
    client.get("http://127.0.0.1:8080/api").send()
}
```

### After (Configurable)
```rust
// ✅ Configurable
pub fn connect_to_api() -> Result<Client> {
    use beardog_config::global::BEARDOG_CONFIG;
    
    let client = Client::builder()
        .connect_timeout(BEARDOG_CONFIG.network.timeouts.connection)
        .build()?;
    
    let url = format!("http://{}:{}/api",
        BEARDOG_CONFIG.network.api.bind_address,
        BEARDOG_CONFIG.network.api.port
    );
    
    client.get(&url).send()
}
```

---

## 🚀 GETTING STARTED

### Immediate Next Steps (Today)

1. **Run the audit scripts** (30 min)
   ```bash
   cd /home/eastgate/Development/ecoPrimals/beardog
   
   # Find hardcoded IPs
   grep -r "127\.0\.0\.1\|localhost\|192\.168\.\|10\.0\.\|172\.16\.\|0\.0\.0\.0" \
     --include="*.rs" crates/ | grep -v test | grep -v "\.md" > hardcoded_ips_audit.txt
   
   # Find hardcoded ports
   grep -r ":[0-9]{4,5}" --include="*.rs" crates/ | \
     grep -v test | grep -v "http://" > hardcoded_ports_audit.txt
   
   # Review the files
   less hardcoded_ips_audit.txt
   less hardcoded_ports_audit.txt
   ```

2. **Prioritize files** (30 min)
   - Identify top 10 files with most violations
   - Focus on production code first
   - Defer test code

3. **Start with worst offender** (1-2 hrs)
   - Pick file with most hardcoded values
   - Migrate to config
   - Test
   - Commit

4. **Repeat systematically** (ongoing)
   - One file at a time
   - Test after each change
   - Track progress

### This Week's Goal
- [ ] Complete Task 1 (IPs: 346 → <50)
- [ ] Complete Task 2 (Ports: 200 → <20)
- [ ] Complete Task 3 (Paths: all migrated)
- [ ] **Total: 546 → <100 hardcoded values**

---

## 📞 NEED HELP?

### Questions & Answers

**Q: Which constants are okay to keep?**
A: Protocol constants (HTTP status codes, well-known ports 80/443), mathematical constants, protocol versions.

**Q: What about test fixtures?**
A: Test code can use hardcoded values, but should be clearly marked as test-only.

**Q: How do I know if I'm done?**
A: Run the measurement commands above. Target: <100 total instances.

**Q: What if something breaks?**
A: Revert the change, add tests, then try again with better testing.

---

**Status**: 🔴 **READY TO START**  
**Priority**: #1 CRITICAL  
**Time Estimate**: 16-24 hours  
**Target Completion**: End of week

**Next Action**: Run audit scripts above, then start with Task 1 (Network IP Migration)

🐻 **BearDog: Zero Hardcoding - Let's Execute on Our Own Spec!**

