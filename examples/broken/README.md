# Broken Examples Directory

This directory contains examples that require additional dependencies or frameworks not included in the main BearDog workspace.

## Experimental Framework Examples

The following examples require the `beardog_sovereign_science` experimental validation framework:

- `live_validation.rs` - Live validation with real cryptographic operations
- `data_collection_experiment.rs` - Data collection for validation analysis
- `test_live_statistics.rs` - Statistical calculation verification

### To Run These Examples

1. Build the experimental framework first:
   ```bash
   cd ../experiments/beardog-sovereign-science/framework
   cargo build --release
   ```

2. Add the experimental framework as a dependency to these examples (or run them from the experiments directory)

## Other Broken Examples

### `.broken` Files

Files with the `.broken` extension are incomplete or deprecated examples that have been disabled:

- `context_aware_licensing_demo.rs.broken` - Deprecated licensing demo
- `distributed_beardog_demo.rs.broken` - Incomplete distributed system demo

These files are kept for reference but are not meant to be built or run.

## Status

Most examples in this directory are:
- ✅ Documented
- ⚠️ Disabled in Cargo.toml
- 📁 Archived for reference

If you need to use any of these examples, check the comments in `Cargo.toml` and the individual file headers for requirements.

