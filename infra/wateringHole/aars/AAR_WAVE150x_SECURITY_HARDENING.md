# AAR: Wave 150x — Security Hardening (Pen Test Response)

**Date**: July 24, 2026
**Wave**: 150x
**Team**: bearDog (flockGate)
**Duration**: ~1 hour
**Commits**: `4373fc7`

---

## Mission

Respond to two security findings from Tower Atomic deep analysis pen/stress
test scenarios (`s_tower_pen_enrollment_replay`, `s_tower_stress_btsp_storm`).

## What We Did

### 1. Enrollment Timestamp Window + Replay Tracking (P1 #5)

**Finding**: `enrollment.verify` had no timestamp validation and no replay
protection — an attacker could capture a valid HMAC proof and resubmit it
indefinitely.

**Implementation**:
- Added ±300s timestamp validity window (configurable via `BEARDOG_ENROLLMENT_TIMESTAMP_WINDOW`)
- Added `ReplayCache` backed by `parking_lot::Mutex<HashMap<[u8; 32], u64>>` — proof digests
  computed via BLAKE3, bounded at 10K entries with self-pruning on access
- Timestamp check runs before HMAC verification (fast-fail on expired proofs)
- Replay check runs after HMAC verification (only cache valid proofs)
- 8 new tests covering window acceptance/rejection, replay detection, cache independence

**Files changed**:
- `crates/beardog-tunnel/src/unix_socket_ipc/handlers/btsp/enrollment.rs` — core logic
- `crates/beardog-tunnel/src/unix_socket_ipc/handlers/btsp/mod.rs` — `ReplayCache` in `BtspHandler`
- `crates/beardog-config/src/env_keys/security.rs` — `ENV_ENROLLMENT_TIMESTAMP_WINDOW`

### 2. UDS Connection Cap + Backpressure (P1 #4)

**Finding**: Unix socket server accepted connections without bound — no rate
limiting, no connection cap, no backpressure. TCP server already had
`ConnectionRateLimiter` (H2-11); UDS was the gap.

**Implementation**:
- Added `tokio::sync::Semaphore` to `UnixSocketIpcServer` (default 512 permits,
  configurable via `BEARDOG_UDS_MAX_CONNECTIONS`)
- Semaphore acquired after accept, before handler dispatch — at capacity,
  new connections queue rather than being dropped (backpressure, not rejection)
- Permit auto-released when handler task completes (RAII via `OwnedSemaphorePermit`)

**Files changed**:
- `crates/beardog-tunnel/src/unix_socket_ipc/server.rs` — semaphore field + accept loop
- `crates/beardog-config/src/env_keys/security.rs` — `ENV_UDS_MAX_CONNECTIONS`

## What Went Well

- **TCP rate limiter was the reference pattern** — `ConnectionRateLimiter` in
  `tcp_ipc/rate_limiter.rs` was clean and well-tested, making the UDS
  equivalent straightforward to design
- **Clippy caught 4 issues immediately** — `abs_diff()` instead of manual
  pattern, `let...else` instead of match, `pub` instead of `pub(crate)` in
  private module. All fixed before commit
- **Pre-push hook fast** — cargo check was cached from the clippy pass, so
  push completed in 5 seconds
- **13,937 tests, 0 failures** — 8 new tests, no regressions

## What We Learned

- **Enrollment proofs need defense-in-depth**: timestamp window prevents
  stale proofs, replay cache prevents reuse, constant-time HMAC comparison
  prevents timing attacks. Three layers, each independently valuable
- **UDS != TCP for security posture**: UDS is often treated as "trusted local"
  but in a multi-primal composition, any primal on the same host can connect.
  Connection caps matter even on local sockets
- **Semaphore > rejection for IPC**: dropping connections causes hard failures
  in callers. Semaphore backpressure causes latency, which callers handle
  gracefully (timeout logic already exists)

## Risks / Open Items

- **Replay cache is per-process** — if bearDog restarts, the cache is lost.
  Acceptable because the timestamp window prevents old proofs anyway, and
  enrollment is an infrequent operation
- **No clock skew compensation** — if two gates have >5 minute clock drift,
  enrollment will fail. NTP is assumed. `BEARDOG_ENROLLMENT_TIMESTAMP_WINDOW`
  can be widened for known-bad environments
- **30 remaining pen/stress findings** — distributed across 10 scenarios,
  teams evolve independently. No single blocker

## Metrics

| Metric | Before | After |
|--------|--------|-------|
| Tests | 13,929 | 13,937 |
| Clippy warnings | 0 | 0 |
| Enrollment security layers | 1 (HMAC) | 3 (HMAC + timestamp + replay) |
| UDS connection cap | unbounded | 512 (configurable) |
| New env keys | 0 | 2 |

## P2 Remaining (bearDog)

| Task | Status | Updated |
|------|--------|---------|
| Android Keystore + grapheneGate | Code complete, awaiting hardware validation | Unchanged — eastGate handoff open |
| Enrollment seed rotation | ✅ SHIPPED (Wave 150x) | HKDF-based, grace period N/N−1 |
| CredentialStore squirrel integration | ✅ INTEGRATED (Wave 150u) | squirrel `SecurityProvider` delegates to `secrets.*` |
| UDS backpressure signaling | ✅ SHIPPED (Wave 150x) | 100ms timeout → `-32003` saturation error |
| Bond-type cipher floors | ✅ SHIPPED (Wave 150x) | Per-bond `COVALENT`/`IONIC` cipher floor env keys |
| BTSP strict UDS mode | ✅ SHIPPED (Wave 151a) | `BEARDOG_UDS_REQUIRE_BTSP=1` |
| Enrollment handler decomposition | ✅ SHIPPED (Wave 151b) | 1061L → 7 modules |
