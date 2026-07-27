# libtower

C ABI shared library for BearDog Tower Atomic crypto operations.

**Chimera Phase 0**: `liblibtower.so` / `liblibtower.dylib` / `tower.dll` — lets other primals `dlopen` pure Rust crypto without JSON-RPC IPC overhead.

## Exported Symbols

| Symbol | Purpose |
|--------|---------|
| `tower_version` | Library version string |
| `tower_last_error` | Thread-local error message |
| `tower_hash_blake3` | BLAKE3 hash (32 bytes) |
| `tower_hmac_sha256` | HMAC-SHA256 (32 bytes) |
| `tower_encrypt_chacha20` | ChaCha20-Poly1305 AEAD |
| `tower_sign_ed25519` | Ed25519 signature (64 bytes) |
| `tower_verify_ed25519` | Ed25519 verification |
| `tower_capabilities` | JSON capability discovery |

## Build

```bash
cargo build --release -p libtower
ls -la target/release/liblibtower.so  # ~565K
```

## Error Model

Functions return `0` on success, negative values on error. Call `tower_last_error()` for a NUL-terminated error string (thread-local, valid until next call).

## License

AGPL-3.0-or-later
