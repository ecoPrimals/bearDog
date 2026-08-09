# Wave 157d — riboCipher Tier 2 Neural API Capability

**Date**: August 9, 2026
**Author**: eastGate (bearDog)
**Status**: SHIPPED — 4 new JSON-RPC methods, auto-announced

---

## Summary

Exposed bearDog's existing `decode_mito_tag` (and related riboCipher operations) as
Neural API capabilities. biomeOS and cross-gate consumers can now decode Tier 2
mito-obfuscated transport signals via JSON-RPC without needing direct access to the
riboCipher library internals.

## New Methods

| Method | Params | Returns |
|--------|--------|---------|
| `ribocipher.decode_mito_tag` | `family_seed` (base64), `tag` (hex or base64) | `protocol_type`, `protocol_name`, `decoded` (bool) |
| `ribocipher.encode_mito_signal` | `family_seed` (base64), `protocol_type` (u8) | `signal` (base64), `signal_hex`, `protocol_name` |
| `ribocipher.protocol_name` | `protocol_type` (u8) | `protocol_name` |
| `ribocipher.list_protocols` | none | `protocols` array (8 known types with names) |

## Architecture

```text
biomeOS → capability.call("ribocipher", "decode_mito_tag", {...})
        → bearDog HandlerRegistry → RiboCipherHandler
        → ribocipher::decode_mito_tag(family_seed, tag)
        → {"protocol_type": 1, "protocol_name": "ndjson-jsonrpc", "decoded": true}
```

The handler delegates to the existing pure-Rust `ribocipher` module (`beardog-tunnel/src/ribocipher.rs`)
which uses `HMAC-SHA256(family_seed, protocol_type)` truncated to 4 bytes.

## Security

- `family_seed` must be supplied per-call — bearDog does NOT store or cache it
- Seed never appears in responses or logs
- `decode_mito_tag` iterates all 8 known protocols against the tag — brute-force is
  limited to known ecosystem protocols, not arbitrary values

## Registration

- **Handler**: `RiboCipherHandler` — 16th kind in `MethodHandlerKind` enum
- **Capability registration**: Added to `capabilities.list` response under `"ribocipher"` domain
- **Cost estimates**: `decode_mito_tag` and `encode_mito_signal` at `CRYPTO_MS` latency; `protocol_name` and `list_protocols` at 0ms
- **primal.announce**: Auto-announced via `HandlerRegistry` — 4 dotted methods picked up by `registered_announce_method_names()`

## Tests

6 new tests:
- `decode_round_trip` — encode then decode, verify protocol recovery
- `decode_wrong_seed_returns_not_decoded` — cross-seed rejection
- `protocol_name_returns_known` — name lookup
- `list_protocols_returns_all` — 8 known protocols
- `unknown_method_returns_error` — `-32601` for unknown
- `decode_missing_params_returns_error` — param validation

## Signal for Upstream

### biomeOS
- `ribocipher.decode_mito_tag` is now available as a Neural API capability
- Use `capability.call("ribocipher", "decode_mito_tag", {"family_seed": ..., "tag": ...})`
  for Tier 2 mito-obfuscated protocol detection on cross-gate connections

### sourDough
- `sourdough validate rpc-surface` segfaults on bearDog workspace (SIGSEGV). Filed as
  data point — manual self-audit completed. All 16 handlers error on unknown methods.

### overwatch
- bearDog RPC surface: 236+ JSON-RPC methods + 4 new ribocipher methods = **240+ methods**
- bearDog is now 16/16 handler kinds registered

---

*Wave 157d — riboCipher Tier 2 Neural API complete. 4 methods, 6 tests, auto-announced.*
