<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Wave 156m — G65 Protocol Negotiation Handoff

**Date**: August 6, 2026
**Author**: bearDog (eastGate)
**Status**: Shipped

---

## Summary

bearDog now supports G65 Phase 3 protocol negotiation on a single socket.
Clients can send `PROTOCOLS: tarpc,jsonrpc\n` and the server selects the
best mutual protocol, responding with `PROTOCOL: tarpc\n` (or `jsonrpc`).
The connection then proceeds in the selected protocol.

This is **backward-compatible**: clients that send `{` as the first byte
get JSON-RPC (no negotiation needed), and the `.tarpc.sock` sibling socket
continues to work for C2-era tarpc clients.

---

## Changes

### beardog-ipc (`protocol_router.rs`)

- **`Protocol` enum**: Added `Tarpc`, `Negotiation` variants
- **`Protocol::wire_name()`**: Returns canonical names for negotiation lines (`tarpc`, `jsonrpc`)
- **`ProtocolNegotiator`**: New struct for G65 negotiation logic
  - `parse_client_greeting()`: Parses `PROTOCOLS:` line into protocol list
  - `negotiate()`: Selects best mutual protocol (client priority order)
  - `format_response()` / `format_error()`: Formats `PROTOCOL:` response lines
- **`RouterConfig`**: Added `enable_tarpc` field
- **`ProtocolCapabilities`**: Includes tarpc in capabilities advertisement
- **31 tests** covering all negotiation paths

### beardog-tunnel (`connection_handlers.rs`, `types.rs`, `server.rs`)

- **`Protocol` enum** (tunnel-local): Added `Negotiation` variant with detection
- **`handle_protocol_negotiation()`**: Parses greeting, negotiates, routes to handler
- **`serve_tarpc_on_stream()`**: Serves tarpc binary RPC on an already-connected
  stream via `tarpc::serde_transport::Transport::from((stream, Bincode))`
- **`parse_g65_greeting()`**: Lightweight inline parser (no cross-crate dependency)
- **`server.rs`**: `handle_connection` routes `Protocol::Negotiation` before JSON-RPC/HTTP match
- **6 tests** for G65 greeting parsing

### Bug fix

- Removed unused `SystemErrorCategory` import in `ios_safe.rs` tests (pre-existing warning)

---

## Protocol Wire Format

```text
Client → Server:  PROTOCOLS: tarpc,jsonrpc\n
Server → Client:  PROTOCOL: tarpc\n
<connection proceeds as tarpc binary framing>
```

```text
Client → Server:  PROTOCOLS: jsonrpc\n
Server → Client:  PROTOCOL: jsonrpc\n
<connection proceeds as NDJSON JSON-RPC>
```

```text
Client → Server:  PROTOCOLS: grpc\n          (no mutual protocol)
Server → Client:  PROTOCOL: NONE supported=tarpc,jsonrpc\n
<connection closed>
```

```text
Client → Server:  {"jsonrpc":"2.0",...}\n    (no negotiation — legacy)
<connection proceeds as JSON-RPC>
```

---

## For Upstream

### squirrel / sourDough (C7)

bearDog's G65 implementation is independent of squirrel's reference impl.
When C7 extracts the negotiation logic to sourDough or cellMembrane,
bearDog can adopt the shared crate. The wire format is identical:
`PROTOCOLS:` / `PROTOCOL:` lines are the same as squirrel's 432-line
reference.

### cellMembrane

cellMembrane's tarpc-aware discovery should advertise G65 support in
the primal registry so routers know which primals accept negotiation
vs requiring dual-socket.

### overwatch

- **E1** (Neural API routing stub) was shipped in Wave 156e. The blurb
  still lists it as a work item — please update.
- bearDog is G65-ready. The `.tarpc.sock` sibling socket is retained
  for backward compatibility but can be deprecated once all clients
  migrate to negotiation.

---

## Metrics

- 0 Clippy warnings
- 37 new tests (31 + 6)
- 0 breaking changes (backward-compatible)
- Wire format matches squirrel reference
