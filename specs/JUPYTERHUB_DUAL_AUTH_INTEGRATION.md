<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# JupyterHub Dual-Auth Integration — BearDog as Auth Provider (S4)

**Status:** Design Complete — Implementation Ready
**Wave:** 24 (Shadow Run Execution)
**Deliverable:** D4
**Blocks:** S4 Auth Shadow cutover

---

## Goal

Replace the OAuth2 proxy in front of JupyterHub with BearDog's native
authentication system. During the shadow period, both auth paths run
simultaneously (`BEARDOG_TLS_MODE=shadow`) to prove parity.

---

## Architecture

```
Browser → JupyterHub :8000
       ↗ dual-auth middleware
      │
      ├─ (shadow) → OAuth2 proxy → GitHub OAuth
      │
      └─ (sovereign) → BearDog BTSP token verification
                      → auth.verify_ionic (Ed25519 + expiry)
                      → session creation
```

### Dual-Auth Mode (`BEARDOG_TLS_MODE=shadow`)

When `BEARDOG_TLS_MODE=shadow` is set:
1. Both authentication paths are active simultaneously
2. Requests are validated against both paths
3. Metrics compare auth latency, success rate, and session behavior
4. Discrepancies are logged but do not block — the commercial path remains authoritative

### Sovereign Mode (post-cutover)

After 7-day parity proof:
1. OAuth2 proxy is removed
2. BearDog is the sole authentication provider
3. All JupyterHub sessions derive from BearDog ionic tokens

---

## Token → Session Mapping

### BearDog Ionic Token Structure

```json
{
  "sub": "user@domain.com",
  "iss": "beardog",
  "iat": 1716000000,
  "exp": 1716086400,
  "scopes": ["jupyterhub:user"],
  "jti": "uuid-v4"
}
```

### Session Creation Flow

1. User authenticates via FIDO2/CTAP2 or BTSP token
2. BearDog issues an ionic token with `jupyterhub:user` scope
3. JupyterHub authenticator validates the token:
   - Fetches BearDog's public key via `auth.public_key` IPC
   - Verifies Ed25519 signature
   - Checks `exp` (expiry) and `scopes`
4. On success, JupyterHub creates a server session
5. Session ID is mapped to the BearDog token's `jti`

### Token Refresh

- Ionic tokens have configurable `ttl_seconds` (default: 24h)
- JupyterHub session cookies track the token `jti`
- When a token expires, the authenticator triggers re-auth
- FIDO2 hardware attestation can provide seamless re-auth

---

## JupyterHub Authenticator

A custom `Authenticator` class (`BearDogAuthenticator`) integrates with
JupyterHub's authentication plugin system:

```python
# jupyterhub_config.py
c.JupyterHub.authenticator_class = 'beardog_jupyterhub.BearDogAuthenticator'
c.BearDogAuthenticator.beardog_socket = '/run/beardog/beardog.sock'
c.BearDogAuthenticator.verify_method = 'auth.verify_ionic'
c.BearDogAuthenticator.public_key_method = 'auth.public_key'
c.BearDogAuthenticator.required_scopes = ['jupyterhub:user']
```

The authenticator:
1. Reads the `Authorization: Bearer <ionic_token>` header
2. Calls `auth.verify_ionic` over the BearDog Unix socket
3. Extracts `sub` as the JupyterHub username
4. Returns the authenticated user to JupyterHub

---

## IPC Surface

Existing BearDog IPC methods used:

| Method | Purpose |
|--------|---------|
| `auth.verify_ionic` | Validate token signature + expiry |
| `auth.public_key` | Fetch Ed25519 verifying key for external verification |
| `auth.issue_ionic` | Issue new tokens (admin interface) |

No new IPC methods required — the existing surface fully covers the auth flow.

---

## Metrics (S4 Shadow)

| Metric | Target | Collection |
|--------|--------|------------|
| Auth latency | < 50ms | Time from token receipt to session grant |
| Session creation time | < 100ms | Time from auth success to notebook ready |
| Token refresh reliability | > 99.9% | Successful re-auth on token expiry |
| FIDO2 enrollment success | > 95% | First-time hardware key registration |

Metrics are emitted in `skunkBat` audit format and feed into the
`membrane_telemetry` pipeline alongside S1 TLS metrics.

---

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `BEARDOG_TLS_MODE` | `static` | `shadow` enables dual-auth |
| `BEARDOG_JUPYTERHUB_SOCKET` | `/run/beardog/beardog.sock` | IPC path |
| `BEARDOG_JUPYTERHUB_SCOPES` | `jupyterhub:user` | Required token scopes |
| `BEARDOG_AUTH_TOKEN_TTL` | `86400` | Token lifetime in seconds |

### JupyterHub Configuration

```yaml
# helm values for JupyterHub deployment
hub:
  config:
    JupyterHub:
      authenticator_class: beardog_jupyterhub.BearDogAuthenticator
    BearDogAuthenticator:
      beardog_socket: /run/beardog/beardog.sock
      dual_auth_mode: true  # shadow mode
      fallback_authenticator: oauthenticator.github.GitHubOAuthenticator
```

---

## Security Considerations

- **Token scope isolation**: JupyterHub tokens carry only `jupyterhub:user` —
  no access to other BearDog capabilities
- **Hardware attestation**: FIDO2 enrollment provides phishing-resistant auth
- **Replay protection**: Token `jti` is checked against a deduplication window
- **Expiry enforcement**: Both BearDog and JupyterHub independently check `exp`

---

## Implementation Phases

| Phase | Scope | Status |
|-------|-------|--------|
| 1. Design | This document | ✓ Complete |
| 2. Authenticator | Python `BearDogAuthenticator` class | Planned |
| 3. Session mapping | Token `jti` → session tracking | Planned |
| 4. Shadow deployment | Dual-auth mode with metrics | Planned |
| 5. Parity proof | 7-day < 50ms auth latency | Pending shadow |
| 6. Cutover | Remove OAuth2 proxy | After parity |
