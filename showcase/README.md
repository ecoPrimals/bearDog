# 🎬 BearDog Showcase Examples

**Status**: Expanding showcase (Mar 24, 2026)  
**Location**: `showcase/`  
**Progress**: 29 runnable demos with `src/main.rs` (~76% of the 38-demo roadmap target; count with `find showcase -name main.rs | wc -l`)

---

## 🚀 Quick Start

**Want to jump right in?**

```bash
# Read this first (5 minutes)
cat showcase/00_START_HERE.md

# Try the available demo (2 minutes)
cd showcase/05-mixed-entropy
cargo run
```

**Want the full picture?**

```bash
# See the complete showcase index
cat showcase/00_SHOWCASE_INDEX.md
```

---

## 📚 Available Content

### ✅ Ready to run

1. **00_START_HERE.md** — Entry point (start here)
2. **00_SHOWCASE_INDEX.md** — Full map
3. **29 Cargo demos** — `00-local-primal/`, `02-ecosystem-integration/`, `03-production-features/`, `04-advanced-features/`, and **`05-mixed-entropy/`** (minimal mixed-entropy placeholder)

Try any directory that contains `Cargo.toml` and `src/main.rs` with `cargo run`.

### Roadmap (remaining work)

The original map targeted **38** demos across six bands (local, hardware, ecosystem, network, advanced, production). The tree now holds **29** runnable binaries; some bands are fuller than others. See [00_SHOWCASE_INDEX.md](00_SHOWCASE_INDEX.md) for category-level status.

---

## 🎯 What Each Level Shows

### Level 0: Local Primal
**Question**: "What can BearDog do by itself?"  
**Answer**: Generate keys, use HSMs, enforce constraints, track lineage

**Demos**:
1. Hello BearDog - First key generation
2. HSM Discovery - Auto-detect hardware
3. Key Constraints - Self-enforcing rules
4. Entropy Mixing - Human + machine
5. Key Lineage - Track ancestry
6. BTSP Tunnel - Secure connections

### Level 1: Hardware Integration
**Question**: "How does BearDog use real hardware?"  
**Answer**: YubiKey, TPM, mobile HSMs, performance benchmarks

**Demos**:
1. YubiKey Basics - PKCS#11 integration
2. TPM Integration - Linux TPM 2.0
3. Android StrongBox - Mobile HSM
4. iOS Secure Enclave - iOS HSM
5. Mixed Entropy - ✅ Current demo
6. HSM Comparison - Benchmarks
7. Failover - Software HSM fallback

### Level 2: Ecosystem Integration
**Question**: "How does BearDog work with other primals?"  
**Answer**: Secure tunnels, encrypted storage/compute, key routing

**Demos**:
1. Songbird BTSP - Secure orchestration
2. BearDog Genesis - Physical ceremonies
3. BirdSong Encryption - Cross-primal messages
4. Lineage Tracking - Distributed ancestry
5. NestGate Encrypted - Secure storage
6. ToadStool Encrypted - Secure compute
7. Squirrel Routing - Key operations

### Level 3: Network & Federation
**Question**: "How does BearDog work distributed?"  
**Answer**: Key registries, multi-party ceremonies, federation

### Level 4: Advanced Features
**Question**: "What advanced crypto can BearDog do?"  
**Answer**: Zero-knowledge proofs, post-quantum, attestation

### Level 5: Production Patterns
**Question**: "How do I deploy BearDog in production?"  
**Answer**: API management, CA, disaster recovery, compliance

---

## 📊 Construction progress

Approximate inventory (each row is a `main.rs` under `showcase/`):

| Area | Runnable demos |
|------|----------------|
| `00-local-primal/` | 6 |
| `02-ecosystem-integration/` | 5 |
| `03-production-features/` | 7 |
| `04-advanced-features/` | 10 |
| `05-mixed-entropy/` | 1 |
| **Total** | **29** |

Roadmap target remains **38** comprehensive demos; gaps include hardware-focused examples under `01-hardware-integration/` and additional ecosystem/network pieces as in [00_SHOWCASE_INDEX.md](00_SHOWCASE_INDEX.md).

---

## 🧬 What Makes BearDog Unique?

### Sovereignty First
- ✅ Your keys, your control
- ✅ Zero vendor lock-in
- ✅ Universal HSM support
- ✅ Self-discovering (zero hardcoded config)

### Entropy Hierarchy
- ✅ Real human entropy (never simulated!)
- ✅ Mixed entropy (60% device + 40% human)
- ✅ Quality scoring and validation
- ✅ Trust model enforcement

### Genetic Keys
- ✅ Self-enforcing constraints
- ✅ Full lineage tracking
- ✅ Non-fungible keys
- ✅ Distributed ancestry

### World-Class Security
- 🏆 **0 unsafe** (`#![forbid(unsafe_code)]` workspace-wide)
- 🏆 High automated test volume — see [STATUS.md](../STATUS.md) for current counts
- 🏆 Perfect file discipline (0 files > 1000 lines)
- 🏆 Zero production mocks

---

## 📚 Documentation Index

### Start Here
- **00_START_HERE.md** - Your entry point ⭐
- **00_SHOWCASE_INDEX.md** - Complete map
- **../README.md** - Project overview
- **../START_HERE.md** - Getting started guide

### Learn More
- **../ARCHITECTURE.md** - System design
- **../ENTROPY_HIERARCHY_PRINCIPLE.md** - Core principle ⭐
- **../COMPREHENSIVE_AUDIT_REPORT_DEC_24_2025.md** - Quality audit
- **../STATUS.md** - Current status

### Deep Dive
- **../specs/** - Technical specs (85 files)
- **../docs/** - Documentation (166 files)
- **../examples/** - Code examples

---

## 🎯 Learning Paths

### Path 1: The Tourist (30 minutes)
"I just want to see what BearDog can do"

1. Read `00_START_HERE.md` (5 min)
2. Run `05-mixed-entropy/` (2 min)
3. Browse `00_SHOWCASE_INDEX.md` (5 min)
4. Read `../ENTROPY_HIERARCHY_PRINCIPLE.md` (10 min)
5. Skim `../README.md` (8 min)

**Result**: Understanding of BearDog's unique approach

### Path 2: The Builder (3 hours)
"I want to integrate BearDog into my app"

1. Complete Path 1 (30 min)
2. Work through Level 0 demos (45 min)
3. Work through Level 1 demos (1 hour)
4. Start Level 2 ecosystem integration (45 min)

**Result**: Able to integrate BearDog

### Path 3: The Expert (8+ hours)
"I want to master BearDog and contribute"

1. Complete Path 2 (3 hours)
2. Complete Levels 3-5 (3 hours)
3. Read all specifications (2+ hours)
4. Build custom demo

**Result**: BearDog expertise, can contribute

---

## 🏗️ Demo Structure

Each demo follows this template:

```
NN-demo-name/
├── README.md           # What, why, how
├── Cargo.toml          # Dependencies
├── run.sh              # One-command execution
├── verify.sh           # Verify it worked (optional)
├── cleanup.sh          # Clean up artifacts (optional)
└── src/
    └── main.rs         # Implementation with comments
```

### README Template
```markdown
# Demo Name

**Level**: 0-5
**Category**: Category name
**Time**: X minutes
**Dependencies**: None / Songbird / etc.

## What This Demo Shows
Brief description

## Prerequisites
What you need

## Running
How to run

## Expected Output
What you should see

## Understanding the Code
Key concepts

## Next Steps
Where to go next
```

---

## 🔗 Related Showcases

Learn from other primals:

- **Songbird**: Federation patterns, multi-tower coordination
  - Location: `../../songbird/showcase/`
  - Strong: Federation (02-federation/), protocol escalation

- **NestGate**: Storage patterns, data services
  - Location: `../../nestgate/showcase/`
  - Strong: Structure (00-local-primal/), ecosystem integration

- **ToadStool**: Compute patterns, GPU orchestration
  - Location: `../../toadstool/showcase/`
  - Strong: Multi-primal integration, GPU demos

- **Squirrel**: AI routing, privacy-preserving operations
  - Location: `../../squirrel/showcase/`
  - Strong: MCP server, local-first privacy

---

## 🤝 Contributing a Demo

Want to add a showcase demo? Here's how:

1. **Choose level and category** (00-05)
2. **Create directory**: `showcase/NN-category/MM-demo-name/`
3. **Copy template**: Use `05-mixed-entropy/` as reference
4. **Implement**: Write clear, commented code
5. **Document**: Create comprehensive README
6. **Test**: Ensure `cargo run` works cleanly
7. **Script**: Add `run.sh` for convenience
8. **Update indexes**: Add to showcase README and index

### Quality Standards
- ✅ Clear, commented code
- ✅ Comprehensive README
- ✅ One-command execution
- ✅ Expected output documented
- ✅ No hardcoded paths/secrets
- ✅ Works on fresh checkout

---

## 📞 Questions?

- **Main README**: [../README.md](../README.md)
- **Getting Started**: [../START_HERE.md](../START_HERE.md)
- **Documentation**: [../docs/](../docs/)
- **Examples**: [../examples/](../examples/)
- **Audit Report**: [../COMPREHENSIVE_AUDIT_REPORT_DEC_24_2025.md](../COMPREHENSIVE_AUDIT_REPORT_DEC_24_2025.md)

---

## ✅ Next Steps

### Immediate Actions
1. ✅ Read `00_START_HERE.md`
2. ✅ Try `05-mixed-entropy/` demo
3. 📋 Bookmark `00_SHOWCASE_INDEX.md`

### This Week
1. 🚧 Complete Level 0 demos (6 demos)
2. 🚧 Expand Level 1 (6 more demos)
3. 📚 Read ecosystem showcase patterns

### This Month
1. 🚧 Complete Level 2 (7 demos)
2. 🚧 Start Levels 3-5 (18 demos)
3. 🎥 Create video walkthroughs

---

## 📝 Archived Content

**Note**: Extensive showcase content was archived on December 23, 2025.

**Archive Location**: `../archive/beardog-dec-23-2025/showcase/`

**Archived Examples**: ~30 showcase directories

To restore:
```bash
# View archive
ls ../archive/beardog-dec-23-2025/showcase/

# Restore specific example
cp -r ../archive/beardog-dec-23-2025/showcase/XX-example showcase/
```

---

**Last Updated**: March 24, 2026  
**Status**: Active expansion  
**Target**: 38 comprehensive demos (roadmap); **29** runnable today

🐻 **BearDog Showcase - Learn by Example** 🎬
