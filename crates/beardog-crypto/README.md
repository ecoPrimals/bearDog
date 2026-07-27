# beardog-crypto

Pure Rust cryptographic primitives for the [BearDog](https://github.com/ecoPrimals/bearDog) Tower Atomic pattern.

This crate provides the core algorithms used by BearDog's 231 JSON-RPC crypto methods as a standalone library — no IPC, no runtime, just crypto.

## Algorithms

- **Signatures**: Ed25519, ECDSA P-256, RSA-PSS
- **Key Exchange**: X25519
- **AEAD**: ChaCha20-Poly1305, AES-128/256-GCM
- **Hashing**: BLAKE3, SHA-256/384/512, SHA3-256
- **MAC**: HMAC-SHA256/384/512
- **KDF**: HKDF-SHA256
- **Passwords**: Argon2id

100% pure Rust. Zero C dependencies. Part of the [ecoPrimals](https://github.com/ecoPrimals) ecosystem.

## License

AGPL-3.0-or-later
