# BearDog Profiling

Configuration and documentation for performance profiling.

## Tools

- **Flamegraph**: `cargo install flamegraph && cargo flamegraph --bin beardog`
- **Benchmarks**: `cd benchmarks && cargo bench`
- **Coverage**: `cargo llvm-cov --workspace`

## Configuration

`config.toml` documents hot path modules for targeted profiling.

## Historical

Profiling helper scripts (flamegraph, memory, benchmarks, clone analysis)
moved to `ecoPrimals/infra/wateringHole/fossilRecord/beardog/` (March 2026).
