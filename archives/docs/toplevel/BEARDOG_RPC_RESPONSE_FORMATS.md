# BearDog RPC Response Formats Reference

**Version**: 0.13.0  
**Date**: January 22, 2026  
**Purpose**: Complete reference for ALL BearDog RPC response formats  
**For**: Integration debugging and client implementation

---

## 🎯 Response Format Rules

**All BearDog Responses Follow These Rules**:

1. **Field Names**: `snake_case` (NEVER `camelCase`)
2. **Encoding**: Base64 for binary data (using `STANDARD` alphabet)
3. **No Nulls**: All documented fields are ALWAYS present (unless marked Optional)
4. **Type Safety**: Types are strictly enforced (String vs Number vs Boolean)

---

## 📋 TLS Methods (4 methods)

### `tls.derive_secrets` - Handshake Key Derivation

**Purpose**: Derive handshake traffic secrets (for encrypted handshake messages)

**Response**:
```json
{
  "derived_key": "base64_string",
  "algorithm": "HKDF-SHA256",
  "length": 32
}
```

**Field Types**:
- `derived_key`: String (base64, variable length based on requested length)
- `algorithm`: String (always "HKDF-SHA256")
- `length`: Number (usize, typically 32 or 48)

**Notes**:
- ⚠️ Has numeric field `length` - could cause u64 parsing issues
- Used for TLS handshake encryption (Certificate, CertificateVerify, Finished)

---

### `tls.derive_application_secrets` - Application Key Derivation ✨✨ v0.18.0+ RFC 8446 COMPLIANT!

**Purpose**: Derive application traffic secrets (for HTTP data encryption)

**🔥 BREAKING CHANGE (v0.18.0+)**: Now RFC 8446 compliant!

**Parameters** (RFC 8446 Compliant):
- `handshake_secret`: base64 (32 bytes) - **✨ CHANGED from `pre_master_secret`!**
  - This is the output from `tls.derive_handshake_secrets`
  - Represents the Handshake Secret stage in the RFC 8446 key schedule
- `transcript_hash`: base64 (32 bytes, SHA-256) - **✨ REQUIRED!**
  - SHA-256 hash of ALL handshake messages (ClientHello through ServerFinished)
- `cipher_suite`: number (optional, default: 0x1303 = ChaCha20-Poly1305)
  - Determines key length: 0x1301 (AES-128) = 16 bytes, 0x1302/0x1303 = 32 bytes

**Response**:
```json
{
  "client_write_key": "u1HnZw8Q7wtXXPc9axju3uehJhY6xPzFiIGcvcwEmm0=",
  "server_write_key": "OYSAPFlf/NAvJTpBtx45lnsFtRu3VEOK5tO/EK3kbx8=",
  "client_write_iv": "rkCk3xt3l2SBFeNu",
  "server_write_iv": "otHQEpR5P+EVqd9V",
  "client_application_secret": "base64_32_bytes",
  "server_application_secret": "base64_32_bytes",
  "algorithm": "HKDF-SHA256",
  "rfc": "RFC 8446 Section 7.1",
  "mode": "RFC 8446 Full Compliance",
  "stage": "application",
  "key_length": 32,
  "iv_length": 12,
  "cipher_suite": 4865
}
```

**Field Types**:
- `client_write_key`: String (base64, length depends on cipher suite: 16 or 32 bytes)
- `server_write_key`: String (base64, length depends on cipher suite: 16 or 32 bytes)
- `client_write_iv`: String (base64, always 16 chars = 12 bytes)
- `server_write_iv`: String (base64, always 16 chars = 12 bytes)
- `client_application_secret`: String (base64, 44 chars = 32 bytes) - **✨ NEW!**
- `server_application_secret`: String (base64, 44 chars = 32 bytes) - **✨ NEW!**
- `algorithm`: String (always "HKDF-SHA256")
- `rfc`: String (always "RFC 8446 Section 7.1")
- `mode`: String (always "RFC 8446 Full Compliance")
- `stage`: String (always "application")
- `key_length`: Number (16 or 32) - **✨ NEW!**
- `iv_length`: Number (always 12) - **✨ NEW!**
- `cipher_suite`: Number (echoes input) - **✨ NEW!**

**Notes**:
- ✅ RFC 8446 compliant key schedule (Handshake Secret → Master Secret → App Secrets)
- ✅ Proper two-stage derivation (use `tls.derive_handshake_secrets` first!)
- ✅ Returns application traffic secrets for key updates (RFC 8446 Section 7.2)
- ⚠️ **BREAKING**: No longer accepts `pre_master_secret`, `client_random`, `server_random`
- ⚠️ **MIGRATION**: Pass `handshake_secret` from `tls.derive_handshake_secrets` output
- Used for HTTP data encryption/decryption (GET/POST/PUT/DELETE)
- Response size: ~290-310 characters (with mode field)

**Example Response** (real output):
```json
{
  "jsonrpc": "2.0",
  "result": {
    "algorithm": "HKDF-SHA256",
    "client_write_iv": "rkCk3xt3l2SBFeNu",
    "client_write_key": "u1HnZw8Q7wtXXPc9axju3uehJhY6xPzFiIGcvcwEmm0=",
    "rfc": "RFC 8446 Section 7.1",
    "server_write_iv": "otHQEpR5P+EVqd9V",
    "server_write_key": "OYSAPFlf/NAvJTpBtx45lnsFtRu3VEOK5tO/EK3kbx8="
  },
  "id": 1
}
```

---

### `tls.sign_handshake` - Handshake Message Signing

**Purpose**: Sign TLS handshake messages with Ed25519

**Response**:
```json
{
  "signature": "base64_string",
  "algorithm": "Ed25519"
}
```

**Field Types**:
- `signature`: String (base64, 88 chars = 64 bytes encoded)
- `algorithm`: String (always "Ed25519")

**Notes**:
- ✅ NO numeric fields
- Used for CertificateVerify message

---

### `tls.verify_certificate` - X.509 Certificate Verification

**Purpose**: Verify X.509 certificate chain

**Response Success**:
```json
{
  "valid": true,
  "chain": ["cert1_base64", "cert2_base64", "root_cert_base64"]
}
```

**Response Failure**:
```json
{
  "valid": false,
  "chain": [],
  "reason": "Certificate expired"
}
```

**Field Types**:
- `valid`: Boolean
- `chain`: Array of Strings (base64-encoded certificates)
- `reason`: String (Optional, only present if valid=false)

**Notes**:
- ✅ NO u64 fields
- `reason` is optional (only on failure)

---

## 🔐 AEAD Encryption Methods

### `crypto.chacha20_poly1305_encrypt` - ChaCha20-Poly1305 Encryption

**Purpose**: Encrypt data with ChaCha20-Poly1305 AEAD

**Response**:
```json
{
  "ciphertext": "base64_string",
  "nonce": "base64_string",
  "tag": "base64_string"
}
```

**Field Types**:
- `ciphertext`: String (base64, variable length)
- `nonce`: String (base64, 16 chars = 12 bytes encoded, if auto-generated)
- `tag`: String (base64, 24 chars = 16 bytes encoded)

**Notes**:
- ✅ NO numeric fields
- Nonce is returned if auto-generated (not provided in request)

---

### `crypto.chacha20_poly1305_decrypt` - ChaCha20-Poly1305 Decryption

**Purpose**: Decrypt data with ChaCha20-Poly1305 AEAD

**Response**:
```json
{
  "plaintext": "base64_string",
  "authenticated": true
}
```

**Field Types**:
- `plaintext`: String (base64, variable length)
- `authenticated`: Boolean (always true on success, error on false)

**Notes**:
- ✅ NO numeric fields
- If authentication fails, returns error (not false)

---

### `crypto.aes256_gcm_encrypt` - AES-256-GCM Encryption

**Purpose**: Encrypt data with AES-256-GCM AEAD

**Response**:
```json
{
  "ciphertext": "base64_string",
  "nonce": "base64_string",
  "tag_bytes": 16
}
```

**Field Types**:
- `ciphertext`: String (base64, variable length)
- `nonce`: String (base64, 16 chars = 12 bytes encoded)
- `tag_bytes`: Number (usize, typically 16)

**Notes**:
- ⚠️ Has numeric field `tag_bytes` - could cause parsing issues
- Tag is embedded in ciphertext (GCM format)

---

### `crypto.aes256_gcm_decrypt` - AES-256-GCM Decryption

**Purpose**: Decrypt data with AES-256-GCM AEAD

**Response**:
```json
{
  "plaintext": "base64_string",
  "authenticated": true
}
```

**Field Types**:
- `plaintext`: String (base64, variable length)
- `authenticated`: Boolean (always true on success)

**Notes**:
- ✅ NO numeric fields (except boolean)

---

### `crypto.aes128_gcm_encrypt` / `crypto.aes128_gcm_decrypt`

**Same format as AES-256-GCM** (above)

---

## 🔑 Key Exchange Methods

### `crypto.x25519_generate_ephemeral` - Generate X25519 Keypair

**Purpose**: Generate ephemeral X25519 keypair for ECDH

**Response**:
```json
{
  "public_key": "base64_string",
  "key_id": "uuid_string"
}
```

**Field Types**:
- `public_key`: String (base64, 44 chars = 32 bytes encoded)
- `key_id`: String (UUID format)

**Notes**:
- ✅ NO numeric fields
- Private key stored internally, indexed by key_id

---

### `crypto.x25519_derive_secret` - Derive X25519 Shared Secret

**Purpose**: Perform X25519 ECDH key exchange

**Response**:
```json
{
  "shared_secret": "base64_string",
  "key_id": "uuid_string"
}
```

**Field Types**:
- `shared_secret`: String (base64, 44 chars = 32 bytes encoded)
- `key_id`: String (UUID format)

**Notes**:
- ✅ NO numeric fields

---

### `crypto.ecdh_p256_generate` - Generate P-256 Keypair

**Purpose**: Generate ephemeral ECDH P-256 keypair

**Response**:
```json
{
  "public_key": "base64_string",
  "key_id": "uuid_string"
}
```

**Field Types**:
- `public_key`: String (base64, uncompressed format, 88 chars = 65 bytes encoded)
- `key_id`: String (UUID format)

---

### `crypto.ecdh_p256_derive` - Derive P-256 Shared Secret

**Purpose**: Perform ECDH P-256 key exchange

**Response**:
```json
{
  "shared_secret": "base64_string"
}
```

**Field Types**:
- `shared_secret`: String (base64, 44 chars = 32 bytes encoded)

**Notes**:
- No key_id (ephemeral, not stored)

---

### `crypto.ecdh_p384_generate` / `crypto.ecdh_p384_derive`

**Same format as P-256** (above, different key sizes)

---

## ✍️ Signature Methods

### Ed25519 Signing

**`crypto.sign_ed25519`**:
```json
{
  "signature": "base64_string",
  "key_id": "uuid_string"
}
```

**`crypto.verify_ed25519`**:
```json
{
  "valid": true
}
```

---

### ECDSA Signing (P-256, P-384)

**`crypto.sign_ecdsa_secp256r1`** / **`crypto.sign_ecdsa_secp384r1`**:
```json
{
  "signature": "base64_string",
  "algorithm": "ECDSA-P256-SHA256"
}
```

**`crypto.verify_ecdsa_secp256r1`** / **`crypto.verify_ecdsa_secp384r1`**:
```json
{
  "valid": true
}
```

---

### RSA Signing (PKCS#1 v1.5, PSS)

**`crypto.sign_rsa_pkcs1_sha256`** / **`crypto.sign_rsa_pss_sha256`**:
```json
{
  "signature": "base64_string",
  "algorithm": "RSA-2048-PKCS1-SHA256",
  "key_size": 2048
}
```

**Field Types**:
- `signature`: String (base64, variable length based on key size)
- `algorithm`: String
- `key_size`: Number (usize, 2048/3072/4096)

**Notes**:
- ⚠️ Has numeric field `key_size`

**`crypto.verify_rsa_pkcs1_sha256`** / **`crypto.verify_rsa_pss_sha256`**:
```json
{
  "valid": true
}
```

---

## 🔐 Hashing Methods

### SHA Family

**`crypto.sha256`** / **`crypto.sha384`** / **`crypto.sha512`**:
```json
{
  "hash": "hex_string"
}
```

**Field Types**:
- `hash`: String (hex-encoded, length varies: SHA256=64, SHA384=96, SHA512=128)

**Notes**:
- ✅ Returns hex (not base64) for hashes
- ✅ NO numeric fields

---

### Legacy & Modern Hashing

**`crypto.sha1`** (legacy):
```json
{
  "hash": "hex_string"
}
```

**`crypto.sha3_256`** (quantum-resistant):
```json
{
  "hash": "hex_string"
}
```

**`crypto.blake3_hash`**:
```json
{
  "hash": "hex_string"
}
```

---

## 🔑 Password Hashing & KDF Methods

### Argon2id (Modern)

**`crypto.argon2id_hash`**:
```json
{
  "hash": "$argon2id$v=19$m=65536,t=2,p=1$...",
  "algorithm": "Argon2id",
  "version": "v19",
  "params": {
    "m_cost": 65536,
    "t_cost": 2,
    "p_cost": 1
  }
}
```

**Field Types**:
- `hash`: String (PHC format)
- `algorithm`: String
- `version`: String
- `params`: Object with numeric fields (m_cost, t_cost, p_cost)

**Notes**:
- ⚠️ Has nested numeric fields in `params`

**`crypto.argon2id_verify`**:
```json
{
  "valid": true,
  "algorithm": "Argon2id"
}
```

---

### PBKDF2 (Legacy)

**`crypto.pbkdf2_sha256`**:
```json
{
  "derived_key": "base64_string",
  "algorithm": "PBKDF2-HMAC-SHA256",
  "iterations": 100000
}
```

**Field Types**:
- `derived_key`: String (base64, variable length)
- `algorithm`: String
- `iterations`: Number (usize)

**Notes**:
- ⚠️ Has numeric field `iterations`

---

### bcrypt (Legacy Web Auth)

**`crypto.bcrypt_hash`**:
```json
{
  "hash": "$2b$12$...",
  "cost": 12
}
```

**Field Types**:
- `hash`: String (bcrypt format)
- `cost`: Number (u32, 4-31)

**Notes**:
- ⚠️ Has numeric field `cost`

**`crypto.bcrypt_verify`**:
```json
{
  "valid": true
}
```

---

### scrypt (Legacy Crypto Wallets)

**`crypto.scrypt`**:
```json
{
  "derived_key": "base64_string",
  "params": {
    "log_n": 15,
    "r": 8,
    "p": 1
  }
}
```

**Field Types**:
- `derived_key`: String (base64)
- `params`: Object with numeric fields

**Notes**:
- ⚠️ Has nested numeric fields

---

## 🔐 HMAC Methods

### HMAC Variants

**`crypto.hmac_sha256`** / **`crypto.hmac_sha384`** / **`crypto.hmac_sha512`** / **`crypto.hmac_blake3`**:
```json
{
  "hmac": "hex_string"
}
```

**Field Types**:
- `hmac`: String (hex-encoded, length varies by algorithm)

**Notes**:
- ✅ NO numeric fields
- Returns hex (not base64)

---

## 🧬 Genetic Crypto Methods

### `genetic.derive_lineage_key` - Genetic Key Derivation

**Response**:
```json
{
  "key": "base64_string",
  "method": "Blake3-KDF",
  "quality_score": 0.95
}
```

**Field Types**:
- `key`: String (base64, 44 chars = 32 bytes encoded)
- `method`: String
- `quality_score`: Number (f64, 0.0-1.0)

**Notes**:
- ⚠️ Has numeric field `quality_score` (float)

---

### `genetic.mix_entropy` - Entropy Mixing

**Response**:
```json
{
  "entropy": "base64_string",
  "quality_score": 0.98,
  "tiers_used": ["tier3_human", "tier2_supervised"]
}
```

**Field Types**:
- `entropy`: String (base64)
- `quality_score`: Number (f64)
- `tiers_used`: Array of Strings

**Notes**:
- ⚠️ Has numeric field `quality_score`

---

### `genetic.verify_lineage` - Lineage Verification

**Response**:
```json
{
  "valid": true,
  "reason": "Optional error message"
}
```

---

### `genetic.generate_lineage_proof` - Generate Proof

**Response**:
```json
{
  "proof": "base64_string",
  "timestamp": 1705948800
}
```

**Field Types**:
- `proof`: String (base64)
- `timestamp`: Number (u64, Unix timestamp)

**Notes**:
- ⚠️ Has numeric field `timestamp` (u64)

---

## ⚠️ Methods with Numeric Fields (Potential Issues)

**These methods have numeric (u64/usize/f64) fields that could cause parsing errors**:

| Method | Numeric Field | Type | Typical Value |
|--------|---------------|------|---------------|
| `tls.derive_secrets` | `length` | usize | 32 or 48 |
| `crypto.aes256_gcm_encrypt` | `tag_bytes` | usize | 16 |
| `crypto.sign_rsa_*` | `key_size` | usize | 2048/3072/4096 |
| `crypto.argon2id_hash` | `m_cost`, `t_cost`, `p_cost` | u32 | 65536, 2, 1 |
| `crypto.pbkdf2_sha256` | `iterations` | usize | 100000 |
| `crypto.bcrypt_hash` | `cost` | u32 | 12 |
| `crypto.scrypt` | `log_n`, `r`, `p` | u8 | 15, 8, 1 |
| `genetic.derive_lineage_key` | `quality_score` | f64 | 0.0-1.0 |
| `genetic.mix_entropy` | `quality_score` | f64 | 0.0-1.0 |
| `genetic.generate_lineage_proof` | `timestamp` | u64 | Unix timestamp |

---

## 🎯 Client Implementation Guide

### Rust Client (using serde)

**✅ CORRECT** - Handle optional/extra fields:
```rust
#[derive(Deserialize, Debug)]
struct TlsApplicationSecrets {
    client_write_key: String,
    server_write_key: String,
    client_write_iv: String,
    server_write_iv: String,
    #[serde(default)]
    algorithm: Option<String>,
    #[serde(default)]
    rfc: Option<String>,
}
```

**❌ WRONG** - Strict struct with missing fields:
```rust
#[derive(Deserialize, Debug)]
struct TlsApplicationSecrets {
    client_write_key: String,
    server_write_key: String,
    client_write_iv: String,
    server_write_iv: String,
    // Missing: algorithm, rfc
    // Adding any field BearDog doesn't return will cause errors!
}
```

---

### Common Parsing Errors

**Error**: "invalid type: null, expected u64 at line 1 column 261"

**Cause**: Client struct has a u64 field that BearDog doesn't return

**Solution**:
1. Make the field `Option<u64>`
2. Add `#[serde(default)]` attribute
3. Or remove the field if not needed

**Example Fix**:
```rust
// Before (ERROR):
struct Response {
    plaintext: String,
    sequence_number: u64,  // BearDog doesn't return this!
}

// After (FIXED):
struct Response {
    plaintext: String,
    #[serde(default)]
    sequence_number: Option<u64>,
}
```

---

## 📞 Support

**Questions?** Check `docs/BEARDOG_RPC_API.md` for method signatures  
**Bugs?** File issue with: method name, request, response, expected vs actual  
**Integration Issues?** See `BIOMEOS_HTTPS_DEBUG_RESPONSE_JAN_22_2026.md`

---

**Version**: 0.13.0  
**Last Updated**: January 22, 2026  
**Status**: Complete (82 methods documented)

