# Wave 157e — swarmVine Gossip Injection Scaffolding

**Date**: August 10, 2026
**Primal**: bearDog v0.9.0
**Gate**: eastGate

---

## Summary

bearDog now has a complete gossip injection scaffolding that other teams can converge on. Events are fired at handler boundaries — fire-and-forget to the local swarmVine socket.

## Architecture

```
Handler (crypto/bonds/ribocipher)
    ↓ success path
beardog_ipc::gossip() → GossipClient::spread()
    ↓ tokio::spawn (non-blocking)
gossip.spread JSON-RPC → /run/biomeos/ipc/swarmvine.sock
```

### Global Singleton Pattern

- `init_global_gossip(client)` — called once at server startup
- `gossip()` → `Option<&'static GossipClient>` — handlers check for `Some` and skip if gossip isn't wired
- No handler signature changes needed
- `watch::Receiver<bool>` shutdown channel suppresses gossip during graceful stop

### Topic Constants (`beardog_ipc::gossip::topics`)

| Category | Topic | Trigger |
|----------|-------|---------|
| Trust | `trust.bond.created` | Ionic bond accepted |
| Trust | `trust.bond.revoked` | Ionic bond revoked |
| Trust | `trust.seed.rotated` | Family seed rotation |
| Trust | `trust.btsp.session` | BTSP tunnel established |
| Trust | `trust.consent.granted` | Consent grant issued |
| HSM | `hsm.fido2.discovered` | FIDO2 key connected |
| HSM | `hsm.fido2.removed` | FIDO2 key removed |
| HSM | `hsm.backend.changed` | HSM backend switched |
| Entropy | `entropy.quality.changed` | Entropy quality shift |
| HSM | `hsm.strongbox.status` | StrongBox availability |
| Crypto | `crypto.spine.signed` | Spine commit signed |
| Crypto | `crypto.cert.issued` | TLS cert issued |
| Crypto | `crypto.cert.expiring` | TLS cert nearing expiry |
| Crypto | `crypto.key.derived` | Key derivation event |
| riboCipher | `ribocipher.mito.decoded` | Mito tag decoded |
| riboCipher | `ribocipher.mito.rejected` | Unknown mito tag |

### Injection Points (Live)

1. **`crypto.sign_ed25519`** → `crypto.spine.signed` (key_id + public_key, no private key)
2. **`ionic_bond.accept`** → `trust.bond.created` (bond_id, proposer, acceptor, scope)
3. **`ionic_bond.revoke`** → `trust.bond.revoked` (bond_id)
4. **`ribocipher.decode_mito_tag`** → `ribocipher.mito.decoded` (on success) or `ribocipher.mito.rejected` (on failure)

### Security Constraints

- **No key material** in any gossip payload — only IDs, algorithm names, quality scores
- **No family seeds** — riboCipher events carry protocol type and name only
- **Fire-and-forget** — gossip failures never block handler execution
- Shutdown channel prevents lingering gossip during graceful stop

## Wire Format

```json
{
  "jsonrpc": "2.0",
  "method": "gossip.spread",
  "params": {
    "topic": "crypto.spine.signed",
    "origin": "beardog",
    "payload": {"commit_hash": "", "key_id": "spine-key", "public_key": "..."},
    "seq": 42
  },
  "id": 42
}
```

## Test Coverage

- 7 new tests in `beardog_ipc::gossip::tests`
- Event serialization round-trip
- Socket path resolution
- Client creation and availability check
- Fire-and-forget to missing socket (non-fatal)
- Shutdown suppression
- Topic constant validation (all 16 dotted)

## Upstream Signals

### swarmVine
- bearDog is ready to publish to `gossip.spread` — awaiting mesh enmeshment
- Wire format and topic namespace are stable

### All Primals
- **Pattern to converge on**: global singleton `GossipClient`, `gossip()` accessor, `topics::*` constants
- Topic namespace: `<domain>.<subdomain>.<event>` (e.g. `trust.bond.created`)
- The `GossipClient` in `beardog-ipc` can be extracted to a shared crate (e.g. `ecosystem-gossip`) if needed

### overwatch
- 4 injection points wired, 16 topics defined
- No handler signature changes — fully backward compatible
- Gossip is best-effort: if swarmVine is down, events are silently dropped
