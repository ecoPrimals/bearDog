# bearDog Crypto JSON-RPC Handoff — Wave 149b

**Date**: Jul 18, 2026 | **Consumer**: esotericWebb (primary), any primal via IPC
**Status**: **CONFIRMED** — signatures stable since Wave 142a

## Connection

Connect to bearDog via `TransportEndpoint` (UDS-first, TCP fallback):

```
unix:///run/user/$UID/beardog/beardog.sock   (Linux)
\\.\pipe\beardog                              (Windows)
127.0.0.1:9735                                (TCP fallback)
```

All requests are JSON-RPC 2.0 over the IPC stream, prefixed with the
`riboCipher` protocol signal (handled by `beardog_ipc::connect_transport()`).

## Core Signing Methods

### `crypto.sign_ed25519` / `crypto.ed25519.sign` / `crypto.sign.ed25519`

All three names route to the same handler.

**Params:**

```json
{
  "message": "<base64-encoded bytes to sign>",
  "key_id": "my-signing-key",
  "purpose": "game-state"
}
```

- `message` (required): Standard Base64 (RFC 4648 §4)
- `key_id` (optional): Defaults to `"default_signing_key"`
- `purpose` (optional): Domain separation string. Defaults to `"general"`

**Returns:**

```json
{
  "signature": "<base64, 64 bytes>",
  "algorithm": "Ed25519",
  "key_id": "my-signing-key",
  "public_key": "<base64, 32 bytes>"
}
```

### `crypto.verify_ed25519` / `crypto.ed25519.verify` / `crypto.verify.ed25519`

**Params:**

```json
{
  "message": "<base64>",
  "signature": "<base64, 64 bytes>",
  "public_key": "<base64, 32 bytes>"
}
```

Optional encoding overrides (all default to `"base64"`):
- `encoding`: Global default for all fields
- `message_encoding`, `signature_encoding`, `public_key_encoding`: Per-field overrides
- Supported values: `"base64"`, `"hex"`

**Returns:**

```json
{
  "valid": true,
  "algorithm": "Ed25519"
}
```

### `crypto.public_key`

Retrieve the Ed25519 public key for a `key_id` without signing.

**Params:**

```json
{
  "key_id": "my-signing-key",
  "purpose": "game-state"
}
```

**Returns:**

```json
{
  "public_key": "<base64, 32 bytes>",
  "algorithm": "Ed25519",
  "key_id": "my-signing-key"
}
```

### `crypto.did_from_key`

Derive a W3C `did:key` identifier from a bearDog signing key.

**Params:**

```json
{
  "key_id": "my-signing-key",
  "purpose": "game-state"
}
```

**Returns:**

```json
{
  "did": "did:key:z6Mk..."
}
```

## Hashing

### `crypto.blake3_hash`

**Params:**

```json
{
  "data": "<base64-encoded bytes>"
}
```

**Returns:**

```json
{
  "hash": "<base64, 32 bytes>",
  "algorithm": "BLAKE3"
}
```

### `crypto.sha256` / `crypto.sha384` / `crypto.sha512` / `crypto.sha3_256`

Same param shape as `blake3_hash`. Returns `hash` + `algorithm`.

### `crypto.hmac_sha256`

**Params:**

```json
{
  "data": "<base64>",
  "key": "<base64>"
}
```

**Returns:**

```json
{
  "mac": "<base64>",
  "algorithm": "HMAC-SHA256"
}
```

## Symmetric Encryption

### `crypto.aes256_gcm_encrypt` / `crypto.chacha20_poly1305_encrypt`

**Params:**

```json
{
  "plaintext": "<base64>",
  "key": "<base64, 32 bytes>",
  "aad": "<base64, optional>"
}
```

**Returns:**

```json
{
  "ciphertext": "<base64>",
  "nonce": "<base64, 12 bytes>",
  "algorithm": "AES-256-GCM"
}
```

### `crypto.aes256_gcm_decrypt` / `crypto.chacha20_poly1305_decrypt`

**Params:**

```json
{
  "ciphertext": "<base64>",
  "key": "<base64, 32 bytes>",
  "nonce": "<base64, 12 bytes>",
  "aad": "<base64, optional>"
}
```

**Returns:**

```json
{
  "plaintext": "<base64>",
  "algorithm": "AES-256-GCM"
}
```

## Key Exchange

### `crypto.x25519_generate_ephemeral`

**Params:** None (or `{}`)

**Returns:**

```json
{
  "public_key": "<base64, 32 bytes>",
  "secret_key": "<base64, 32 bytes>",
  "algorithm": "X25519"
}
```

### `crypto.x25519_derive_secret`

**Params:**

```json
{
  "secret_key": "<base64, 32 bytes>",
  "peer_public_key": "<base64, 32 bytes>"
}
```

**Returns:**

```json
{
  "shared_secret": "<base64, 32 bytes>",
  "algorithm": "X25519-ECDH"
}
```

## Cross-Primal Namespace

All `crypto.*` methods are also available as `beardog.crypto.*` for
explicit cross-primal addressing through songBird's drawbridge relay:

```
crypto.sign_ed25519       → beardog.crypto.sign_ed25519
crypto.blake3_hash        → beardog.crypto.blake3_hash
crypto.hmac_sha256        → beardog.crypto.hmac_sha256
```

## Semantic Aliases

| Alias | Routes to |
|-------|-----------|
| `crypto.hash` | `crypto.blake3_hash` |
| `crypto.hmac` | `crypto.hmac_sha256` |
| `crypto.sign` | `crypto.sign_ed25519` |
| `crypto.verify` | `crypto.verify_ed25519` |
| `crypto.encrypt` | `crypto.chacha20_poly1305_encrypt` |
| `crypto.decrypt` | `crypto.chacha20_poly1305_decrypt` |
| `crypto.generate_keypair` | `crypto.ed25519_generate_keypair` |

## esotericWebb Integration Pattern

For game state signing:

1. **On startup**: Call `crypto.public_key` with a game-specific `key_id`
   to get the verifying key
2. **On state change**: Call `crypto.sign` with the serialized game state
   as the `message`
3. **On verification**: Call `crypto.verify` with the state, signature,
   and public key

All data fields are Base64-encoded bytes. Serialize game state to
canonical JSON (sorted keys) before hashing/signing for determinism.

## Wire Format

Standard JSON-RPC 2.0:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "crypto.sign_ed25519",
  "params": {
    "message": "SGVsbG8gd29ybGQ=",
    "key_id": "game-state-signer"
  }
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "signature": "...",
    "algorithm": "Ed25519",
    "key_id": "game-state-signer",
    "public_key": "..."
  }
}
```

## Tower Atomic — Enrollment Verification (Wave 150u)

songBird delegates `mesh.enroll` proof verification to bearDog via
`enrollment.verify`. Enrollment keys are derived through the genetic HKDF
hierarchy rather than using raw `FAMILY_SEED` bytes:

```text
enrollment_key(gen) = HKDF-SHA256(
    ikm  = FAMILY_SEED,
    salt = FAMILY_ID (or "default"),
    info = "enrollment-v{gen}"
)
proof = HMAC-SHA256(enrollment_key(gen), node_id|public_key|timestamp|gen)
```

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "enrollment.verify",
  "params": {
    "node_id": "southGate",
    "public_key": "<wg-or-ed25519-pubkey>",
    "timestamp": 1753128000,
    "proof": "<base64 HMAC-SHA256>",
    "seed_generation": 0
  }
}
```

Response:

```json
{"jsonrpc":"2.0","id":1,"result":{"verified":true,"verified_generation":0}}
```

Or on failure:

```json
{"jsonrpc":"2.0","id":1,"result":{"verified":false,"reason":"HMAC proof does not match enrollment data"}}
```

**Requires**: `FAMILY_SEED` or `BEARDOG_FAMILY_SEED` env var set.

### Seed Rotation (Wave 150x)

- **`BEARDOG_ENROLLMENT_SEED_GENERATION`**: sets the current generation (default 0).
  Bump this value to rotate — each generation derives a completely different
  HMAC key from the same root `FAMILY_SEED`.
- **Grace period**: during rotation, the verifier accepts generation N **and** N−1.
  After a rotation window, set generation to N+1 to retire N−1.
- **Wire contract**: `seed_generation` field defaults to 0 for backward
  compatibility. Callers that don't send it verify against generation 0.
- **`verified_generation`**: response includes which generation matched,
  allowing callers to detect nodes still on old generation.

### Security Hardening (Wave 150x)

- **Timestamp window**: proofs are rejected if the timestamp differs from
  bearDog's wall clock by more than `BEARDOG_ENROLLMENT_TIMESTAMP_WINDOW`
  seconds (default 300 = ±5 minutes). Failure reason:
  `"Timestamp outside validity window (Ns drift, max 300s)"`.
- **Replay tracking**: a successfully verified proof cannot be resubmitted.
  bearDog caches proof digests and rejects duplicates within the validity
  window. Failure reason: `"Enrollment proof already used (replay rejected)"`.
- Callers should use a fresh timestamp (current Unix seconds) for each
  enrollment attempt.

## Full Method List (109 methods)

See `crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto_handler/method_list.rs`
for the complete canonical list. All methods are tested via the crypto E2E
test suite in `crates/beardog-tunnel/tests/crypto_api/`.
