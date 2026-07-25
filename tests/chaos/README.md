<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Chaos Testing

Resilience, fault tolerance, and recovery validation under adverse conditions.

## Modules

| Module | Tests | Purpose |
|--------|-------|---------|
| `mod.rs` | 7 | Framework types, `ChaosEngine`, `ChaosConfig`, `ChaosType` |
| `hsm_chaos_tests.rs` | — | HSM failure scenarios |
| `network_chaos_tests.rs` | — | Network partition and latency scenarios |
| `resource_chaos_tests.rs` | — | Memory/CPU/disk exhaustion scenarios |

## Running

```bash
cargo test -- chaos
```

## Last Updated

July 25, 2026
