# BearDog Scripts Directory

Active development, deployment, and maintenance scripts.

## Directory Structure

### Active Scripts

Scripts currently maintained and used in production workflows:

- **`active/deployment/`** - Production deployment scripts
  - `deploy.sh` - Deployment orchestration
- **`active/validation/`** - System validation and security scripts
  - `validate-system.sh` - Comprehensive system validation
  - `security_audit.sh` - Security audit and compliance checks
  - `security_hardening_validation.sh` - Security hardening checks

### Deployment

- **`deployment/`** - Unified ecosystem deployment
  - `deploy-unified-ecosystem.sh` - Full ecosystem deployment

### Profiling

- **`profiling/`** - Performance profiling helpers
  - `analyze_clones.sh` - Clone site analysis
  - `generate_flamegraph.sh` - CPU flame graphs
  - `profile_benchmarks.sh` - Benchmark profiling
  - `profile_memory.sh` - Memory profiling
  - `setup_profiling.sh` - Profiling environment setup

## Usage

```bash
# Run system validation
./scripts/active/validation/validate-system.sh

# Security audit
./scripts/active/validation/security_audit.sh
```

## Archived Scripts

Historical scripts (migration, pedantic, performance, modernization) have been moved
to `ecoPrimals/archive/` as fossil record.

---

**BearDog**: 100% Pure Rust Cryptographic Service Provider
