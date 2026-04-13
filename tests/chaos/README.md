<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Chaos Testing

Resilience, fault tolerance, and recovery validation under adverse conditions.

## Modules

| Module | Tests | Purpose |
|--------|-------|---------|
| `mod.rs` | — | Framework types, `ChaosTestConfig`, `FaultType`, `ChaosController` |
| `network_chaos.rs` | 5 | Network partitions, latency injection, packet loss |
| `resource_chaos.rs` | 5 | Memory/CPU/disk exhaustion, OOM scenarios |
| `comprehensive_fault_testing.rs` | 5 | Sequential and concurrent multi-fault injection |
| `integration_tests.rs` | 8 | Full lifecycle: init, inject, monitor, recover, report |

## Running

```bash
cargo test -- chaos
```

## Fault Types

- **Network**: partition, latency, packet loss, Byzantine
- **Resource**: memory, CPU, disk exhaustion
- **Security**: auth failure, certificate expiry
- **Database**: timeout, corruption
- **Component**: crash (graceful/panic/OOM), slowdown

## Last Updated

April 13, 2026
