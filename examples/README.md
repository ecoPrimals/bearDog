# BearDog Examples

## Working Examples

The following examples are functional and demonstrate BearDog capabilities:

- `api_demo.rs` - API usage demonstration
- `basic_validation.rs` - Basic validation patterns

## Broken Examples (In `examples/broken/`)

The `examples/broken/` directory contains examples that are currently non-functional.
These examples are preserved for reference but should not be expected to compile or run.

### Reasons for Non-Functional State

1. **Missing Dependencies**: Several examples reference the `beardog_sovereign_science` crate
   which is part of the experimental framework in `experiments/beardog-sovereign-science/`
   and is not included in the main workspace.

2. **API Changes**: Some examples were written against older APIs that have since evolved.

3. **Configuration Mismatches**: Import paths and type names have changed during refactoring.

### Examples Requiring `beardog_sovereign_science`

- `full_validation.rs`
- `live_validation.rs`
- `data_collection_experiment.rs`
- `test_live_statistics.rs`

### Examples Requiring API Updates

- `simple_core_demo.rs` - Needs configuration type updates

## Running Working Examples

```bash
# Run API demo
cargo run --example api_demo

# Run basic validation
cargo run --example basic_validation
```

## Status

**Working**: 2/89 examples  
**Broken**: 87 examples (intentionally preserved for reference)

The broken examples will be updated as part of future documentation improvements.
For now, they serve as reference material for API patterns and use cases.

