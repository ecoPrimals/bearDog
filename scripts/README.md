# BearDog Scripts Directory

Active development, deployment, and maintenance scripts.

## Directory Structure

### Active Scripts

Scripts currently maintained and used in production workflows:

- **`active/deployment/`** - Production deployment scripts
- **`active/validation/`** - System validation and security scripts
  - `validate-system.sh` - Comprehensive system validation
  - `security_audit.sh` - Security audit and compliance checks
  - `security_hardening_validation.sh` - Security hardening checks

### Profiling

- **`profiling/`** - Performance profiling helpers
  - `profile_memory.sh` - Memory profiling
  - `generate_flamegraph.sh` - CPU flame graphs

### Utility Programs

- **`beardog_unwrap_migrator.rs`** - Unwrap migration utility
- **`pixel8a_simple_hsm_test.rs`** - HSM testing for Pixel devices

## Usage

```bash
# Run system validation
./active/validation/validate-system.sh

# Security audit
./active/validation/security_audit.sh
```

## Archived Scripts

Historical scripts (migration, pedantic, performance, modernization) have been moved
to `ecoPrimals/archive/beardog-archives-mar23-2026/scripts-archived/` as fossil record.

---

**BearDog**: 100% Pure Rust Cryptographic Service Provider
