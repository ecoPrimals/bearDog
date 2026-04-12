<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Integration Tests

Integration tests for BearDog's cross-crate coordination. These tests validate
the JSON-RPC handler surface, IPC transport, and BTSP handshake via the
workspace's `beardog-integration-tests` crate and root `tests/` harnesses.

## Running

```bash
cargo test --workspace                     # All tests (fully concurrent)
cargo test -p beardog-integration-tests    # Integration-specific crate
```

See [STATUS.md](../../STATUS.md) for current test counts and coverage.
