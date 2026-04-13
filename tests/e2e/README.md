<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# End-to-End (E2E) Testing

Complete production scenario validation from request through persistence and recovery.

## Scenarios

| Module | Tests | Purpose |
|--------|-------|---------|
| `production_deployment.rs` | 3 | Init, config, startup, health, shutdown |
| `full_stack_integration.rs` | 3 | API, business logic, security, storage, cross-layer |
| `security_flow.rs` | 3 | Auth, encryption, key management, audit |
| `disaster_recovery.rs` | 3 | Failure simulation, failover, data integrity, restore |
| `device_deployment.rs` | 1 | Device-level deployment and validation |
| `mod.rs` | — | `E2ETestFramework`, `E2EScenario`, `E2EMetrics` |
| `helpers.rs` | — | Test utilities |

## Running

```bash
cargo test -- e2e
```

## Last Updated

April 13, 2026
