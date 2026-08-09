# Wave 157a — P0-A: Health Socket Guard + Socket Rename + Self-Audit

**Date**: August 9, 2026
**Author**: eastGate (bearDog)
**Status**: P0-A RESOLVED — code shipped, depot rebuild required

---

## Root Cause Analysis

The P0-A symptom was: bearDog returns `{"status":"alive","primal":"beardog","version":"0.9.0"}`
for ALL methods including `crypto.sign_ed25519`. All spine commits unsigned.

**Root cause**: Consumers connected to `beardog-default.sock` (health probe socket)
instead of the main socket (`beardog-{family_id}.sock`). The health socket:

1. **Never inspected the `method` field** — extracted only `id` for correlation
2. **Returned health for every request** — any JSON-RPC call got `"status":"alive"`
3. **Had a confusingly similar name** — `beardog-default.sock` sounds like "the default
   socket to connect to", not "a monitoring-only endpoint"

The **main socket was never broken** — `crypto.sign_ed25519` works correctly there and
has full test coverage (725 handler tests). This was a socket naming + silent fallback
footgun, not a missing implementation.

## What Changed

### 1. Health socket method guard (`beardog-cli/src/handlers/server/health.rs`)

The health socket now **parses the `method` field** and only returns health for
recognized health methods:

```
ALLOWED: ping, health, health.liveness, health.readiness, health.check, status, check
ALL OTHER: -32601 Method not found (with diagnostic message pointing to main socket)
```

Error response includes:
```json
{
  "error": {
    "code": -32601,
    "message": "Method not found",
    "data": {
      "method": "crypto.sign_ed25519",
      "reason": "This is the health probe socket (beardog-health.sock). For RPC methods, connect to the main socket (beardog-{family_id}.sock or beardog.sock).",
      "supported_methods": ["ping", "health", "health.liveness", ...]
    }
  }
}
```

### 2. Socket rename (`beardog-default.sock` → `beardog-health.sock`)

| Before | After | Rationale |
|--------|-------|-----------|
| `beardog-default.sock` | `beardog-health.sock` | "default" implies "the one to use"; "health" clearly labels its purpose |

Updated in: `server/mod.rs`, `server.rs` (3 doc refs), `capabilities.rs` (1 doc ref),
`env_keys/security.rs` (1 doc ref), `lib.rs` (1 CLI help string).

### 3. HealthHandler catch-all removal (`beardog-tunnel/.../handlers/health.rs`)

| Before | After |
|--------|-------|
| `_ => Ok(json!({"status":"healthy",...}))` | `"status" \| "check" \| "health.check" => Ok(...)` + `other => Err(MethodNotFound)` |

The `HealthHandler` in the tunnel's handler registry had a `_ =>` catch-all that
returned `"healthy"` for any method routed to it. While the registry only routes
7 methods to this handler, the catch-all was sloppy. Now explicitly matches all
3 deep-check aliases and errors on anything else.

### 4. Self-audit results

All 15 handler kinds in the registry were verified:

| Handler | Unknown method behavior | Status |
|---------|------------------------|--------|
| HealthHandler | `Err(MethodNotFound)` | FIXED (was `_ =>` catch-all) |
| CryptoHandler | `Err("Unknown crypto method")` | Clean |
| SecurityHandler | `Err("Method not found")` | Clean |
| BtspHandler | `Err("Unknown BTSP method")` | Clean |
| IonicBondHandler | `Err("Unknown ionic bond method")` | Clean |
| FederationHandler | `Err("Unknown federation method")` | Clean |
| EncryptionHandler | `Err("Unknown encryption method")` | Clean |
| GraphSecurityHandler | `Err("Method not found")` | Clean |
| BeaconHandler | `Err("Unknown beacon method")` | Clean |
| SecretsHandler | `Err("Unknown secrets method")` | Clean |
| RelayHandler | `Err("Unknown relay method")` | Clean |
| Fido2Handler | `Err("Unknown FIDO2 method")` | Clean |
| CapabilitiesHandler | `Err(MethodNotFound)` | Clean |
| CapabilityCallHandler | Re-dispatches to registry | Clean |
| IntrospectionHandler | `Err("Unknown introspection method")` | Clean |

**Registry-level**: `HandlerRegistry::route()` returns `Err(MethodNotFound)` for any
method not claimed by a registered handler. No silent swallowing.

## Action Required

### Depot Team (sporeGate/overwatch)
- **Rebuild beardog depot binary** from HEAD to pick up P0-A fix
- The new binary will:
  - Bind health socket as `beardog-health.sock` (not `beardog-default.sock`)
  - Return `-32601` for non-health methods on the health socket
  - Main socket unchanged — full crypto surface operational

### Consumers (loamSpine, westGate pipeline)
- Connect to `beardog-{family_id}.sock` or `beardog.sock` for RPC methods
- `beardog-health.sock` is monitoring-only (cellMembrane, systemd health checks)
- If connecting to health socket by accident, you'll now get a clear error with
  instructions

### cellMembrane
- Update discovery to look for `beardog-health.sock` instead of `beardog-default.sock`
  for health probes (backward compat: old name will 404, not silently work)

## Tests

- `health_socket_responds_to_plain_json_rpc` — health method → alive (existing, passes)
- `health_socket_tolerates_ribocipher_prefix` — riboCipher prefix → alive (existing, passes)
- `health_socket_rejects_non_health_method` — `crypto.sign_ed25519` → `-32601` (new, passes)
- 725 handler tests pass, 0 Clippy warnings

---

*Wave 157a — P0-A RESOLVED. Health socket guarded. Socket renamed. Self-audit clean.
Depot rebuild required to deploy fix.*
