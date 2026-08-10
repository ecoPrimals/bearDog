# Wave 157e — bearDog Gossip Injection Points

**Date**: August 10, 2026
**Author**: eastGate (bearDog)
**Context**: All primals identify what events they should announce to the mesh via swarmVine.

---

## Gossip Injection Points

bearDog is a crypto/trust primal. Its gossip events fall into three categories:
trust lifecycle, entropy/HSM discovery, and provenance.

### Category 1: Trust Lifecycle (high value, low frequency)

| Event | Topic | Payload | Frequency |
|-------|-------|---------|-----------|
| **Ionic bond created** | `trust.bond.created` | `{bond_id, proposer, acceptor, scope}` | Per-bond |
| **Ionic bond revoked** | `trust.bond.revoked` | `{bond_id, reason}` | Rare |
| **Family seed rotated** | `trust.seed.rotated` | `{family_id, generation}` (no seed material) | Very rare |
| **BTSP session established** | `trust.btsp.session` | `{peer_id, cipher_suite}` | Per-connection |
| **Consent grant issued** | `trust.consent.granted` | `{subject, scope, ttl_seconds}` | Per-grant |

### Category 2: HSM / Entropy Discovery (operational awareness)

| Event | Topic | Payload | Frequency |
|-------|-------|---------|-----------|
| **FIDO2 device connected** | `hsm.fido2.discovered` | `{device_path, aaguid, supports_hmac_secret}` | Per-hotplug |
| **FIDO2 device removed** | `hsm.fido2.removed` | `{device_path}` | Per-hotplug |
| **HSM backend changed** | `hsm.backend.changed` | `{old_backend, new_backend, tier}` | Rare |
| **Entropy source quality shift** | `entropy.quality.changed` | `{source, old_quality, new_quality}` | Rare |
| **StrongBox availability** | `hsm.strongbox.status` | `{available, secure_element_type}` | At startup |

### Category 3: Provenance / Signing (data pipeline integration)

| Event | Topic | Payload | Frequency |
|-------|-------|---------|-----------|
| **Spine commit signed** | `crypto.spine.signed` | `{commit_hash, key_id, public_key}` | Per-commit |
| **Certificate issued** | `crypto.cert.issued` | `{domain, issuer, expiry}` | Per-cert |
| **Certificate approaching expiry** | `crypto.cert.expiring` | `{domain, days_remaining}` | Daily check |
| **Key derived** | `crypto.key.derived` | `{key_id, purpose, algorithm}` (no key material) | Per-derivation |

### Category 4: riboCipher (transport layer awareness)

| Event | Topic | Payload | Frequency |
|-------|-------|---------|-----------|
| **Mito tag decoded** | `ribocipher.mito.decoded` | `{protocol_type, protocol_name}` | Per-decode |
| **Unknown mito tag rejected** | `ribocipher.mito.rejected` | `{tag_hex}` (no seed) | Per-rejection |

---

## Security Constraints

- **NEVER gossip key material** — no private keys, seeds, or secrets in payloads
- **NEVER gossip family_seed** — riboCipher events include results, not inputs
- **Bond payloads are metadata only** — scope and IDs, not terms or signatures
- **Entropy events are quality signals** — quality scores, not entropy bytes
- **Rate limit gossip** — BTSP sessions and key derivations are high-frequency; debounce or sample

## Integration Notes

bearDog does not currently depend on `swarmVine`. When gossip injection is implemented:

1. Add `swarmVine` as an optional dependency (`features = ["gossip"]`)
2. Use `gossip.spread` JSON-RPC via UDS to inject events
3. Debounce high-frequency events (BTSP sessions, key derivations) — 1 gossip per 10s max
4. Inject at the handler level, not the crypto primitive level

## Priority

| Priority | Events | Rationale |
|----------|--------|-----------|
| **P1** | `crypto.spine.signed`, `trust.bond.created/revoked` | Visible to other primals for provenance and trust mesh |
| **P2** | `hsm.fido2.discovered/removed`, `crypto.cert.expiring` | Operational awareness for monitoring |
| **P3** | Everything else | Nice-to-have observability |

---

*bearDog gossip injection points identified. 16 events across 4 categories. Zero key
material in any payload. Implementation deferred until swarmVine gossip mesh is
enmeshed (current Wave 157e subwave blocker).*
