# BearDog Handshake Secrets - Implementation Plan

**Date**: January 22, 2026  
**Priority**: 🔴 CRITICAL (Final 0.5% to 100% Pure Rust HTTPS)  
**Status**: ⏳ READY TO IMPLEMENT  
**ETA**: 1-2 hours

---

## 🎯 Executive Summary

**Mission**: Implement `tls.derive_handshake_secrets` RPC method in BearDog

**Why**: Songbird v5.8.6 needs this for RFC 8446 compliant handshake key derivation

**Impact**: THE FINAL PIECE for 100% Pure Rust HTTPS!

---

## 📊 Current State Analysis

### What Exists ✅

1. **`tls.derive_secrets`** (crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers.rs:582)
   - Status: ⚠️ **SIMPLIFIED** (not RFC 8446 compliant)
   - Uses: Simple HKDF with client_random || server_random salt
   - Problem: Doesn't follow RFC 8446 Section 7.1 key schedule
   - Does NOT use transcript hash

2. **`tls.derive_application_secrets`** (crypto_handlers.rs:760)
   - Status: ✅ **RFC 8446 COMPLIANT**
   - Uses: Proper key schedule for APPLICATION traffic keys
   - Accepts: Optional `transcript_hash` parameter
   - Derives: Master Secret → Application Traffic Secrets → Keys/IVs

### What's Missing ❌

**`tls.derive_handshake_secrets`**
- RFC 8446 Section 7.1 compliant
- Derives: ECDH → Handshake Secret → Handshake Traffic Secrets → Keys/IVs
- Uses: ClientHello + ServerHello transcript hash
- For: Decrypting handshake messages (EncryptedExtensions, Certificate, etc.)

---

## 🔑 Key Differences

| Aspect | `tls.derive_secrets` (OLD) | `tls.derive_handshake_secrets` (NEW) | `tls.derive_application_secrets` (EXISTS) |
|--------|----------------------------|--------------------------------------|-------------------------------------------|
| **RFC Compliance** | ❌ Simplified | ✅ RFC 8446 Section 7.1 | ✅ RFC 8446 Section 7.1 |
| **Key Schedule Stage** | N/A (simplified) | Early → Handshake Secret | Handshake → Master Secret |
| **Base Secret** | Pre-master (ECDH) | ECDH shared secret | Master secret |
| **Transcript** | ❌ None | ClientHello + ServerHello | ALL handshake messages |
| **HKDF Labels** | Custom | "c hs traffic", "s hs traffic" | "c ap traffic", "s ap traffic" |
| **Used For** | ⚠️ Unclear | Decrypt handshake messages | Decrypt HTTP data |
| **Status** | ⚠️ Legacy | ⏳ **NEED TO IMPLEMENT** | ✅ **IMPLEMENTED** |

---

## 📋 Implementation Checklist

### Phase 1: Core Implementation (1 hour)

- [ ] **Create `handle_tls_derive_handshake_secrets` function**
  - Location: `crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers.rs`
  - After: `handle_tls_derive_application_secrets` function
  - Lines: ~150 lines

- [ ] **Implement RFC 8446 Section 7.1 key schedule**:
  ```
  Step 1: Early Secret = HKDF-Extract(salt: 0, IKM: 0)
  Step 2: early_derived = Derive-Secret(early_secret, "derived", "")
  Step 3: Handshake Secret = HKDF-Extract(salt: early_derived, IKM: ECDH)
  Step 4: client_hs_secret = Derive-Secret(hs_secret, "c hs traffic", transcript)
  Step 5: server_hs_secret = Derive-Secret(hs_secret, "s hs traffic", transcript)
  Step 6: Derive keys/IVs from traffic secrets
  ```

- [ ] **Parameter validation**:
  - `pre_master_secret`: 32 bytes (X25519 ECDH output)
  - `client_random`: 32 bytes
  - `server_random`: 32 bytes
  - `transcript_hash`: 32 bytes (SHA-256 of ClientHello + ServerHello)

- [ ] **Response format**:
  ```json
  {
    "client_write_key": "base64_32_bytes",
    "client_write_iv": "base64_12_bytes",
    "server_write_key": "base64_32_bytes",
    "server_write_iv": "base64_12_bytes"
  }
  ```

### Phase 2: Integration (30 minutes)

- [ ] **Add to CryptoHandler registry**
  - File: `crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto.rs`
  - Add to `methods()` list: `"tls.derive_handshake_secrets"`
  - Add to `handle()` match statement

- [ ] **Add comprehensive logging**:
  ```rust
  info!("🔑 TLS: derive_handshake_secrets (RFC 8446 handshake key derivation)");
  debug!("Deriving handshake secrets: ECDH {} bytes, transcript {} bytes",
         pre_master_secret.len(), transcript_hash.len());
  ```

### Phase 3: Testing (30 minutes)

- [ ] **Unit tests**:
  - Test with RFC 8448 test vectors (if available)
  - Test parameter validation
  - Test output format
  - Test key/IV sizes (32 + 12 bytes)

- [ ] **Integration test with Songbird**:
  - Deploy updated BearDog
  - Run HTTPS endpoints test
  - Verify 8/8 passing

### Phase 4: Documentation (15 minutes)

- [ ] **Update docs/BEARDOG_RPC_API.md**:
  - Add `tls.derive_handshake_secrets` method
  - Document parameters, response, RFC 8446 compliance

- [ ] **Update CHANGELOG.md**:
  - New method for handshake key derivation
  - 82 → 83 RPC methods

- [ ] **Update README.md**:
  - Note 100% Pure Rust HTTPS achievement

---

## 🔬 Implementation Details

### HKDF-Expand-Label Helper (Already Exists?)

Need to check if `hkdf_expand_label` helper exists. If not, implement:

```rust
fn hkdf_expand_label(
    secret: &[u8],
    label: &[u8],
    context: &[u8],
    length: usize,
) -> Result<Vec<u8>, String> {
    // RFC 8446 HkdfLabel structure
    let mut hkdf_label = Vec::new();
    
    // Length (2 bytes, big-endian)
    hkdf_label.extend_from_slice(&(length as u16).to_be_bytes());
    
    // Label with "tls13 " prefix
    let full_label = format!("tls13 {}", std::str::from_utf8(label)
        .map_err(|e| format!("Invalid label: {}", e))?);
    hkdf_label.push(full_label.len() as u8);
    hkdf_label.extend_from_slice(full_label.as_bytes());
    
    // Context
    hkdf_label.push(context.len() as u8);
    hkdf_label.extend_from_slice(context);
    
    // HKDF-Expand
    use hkdf::Hkdf;
    use sha2::Sha256;
    let hk = Hkdf::<Sha256>::from_prk(secret)
        .map_err(|e| format!("Invalid PRK: {}", e))?;
    let mut output = vec![0u8; length];
    hk.expand(&hkdf_label, &mut output)
        .map_err(|e| format!("HKDF expand failed: {}", e))?;
    
    Ok(output)
}
```

### Key Schedule Flow

```
Input: ECDH shared secret (32 bytes)

1. Early Secret:
   zeros_32 = [0u8; 32]
   early_secret = HKDF-Extract(salt: zeros_32, IKM: zeros_32)

2. Early Derived:
   empty_hash = SHA-256("")
   early_derived = HKDF-Expand-Label(early_secret, "derived", empty_hash, 32)

3. Handshake Secret:
   handshake_secret = HKDF-Extract(salt: early_derived, IKM: ECDH_shared_secret)

4. Client Handshake Traffic Secret:
   client_hs_secret = HKDF-Expand-Label(
       handshake_secret,
       "c hs traffic",
       transcript_hash,  ← ClientHello + ServerHello
       32
   )

5. Server Handshake Traffic Secret:
   server_hs_secret = HKDF-Expand-Label(
       handshake_secret,
       "s hs traffic",
       transcript_hash,  ← ClientHello + ServerHello
       32
   )

6. Derive Keys and IVs:
   client_write_key = HKDF-Expand-Label(client_hs_secret, "key", "", 32)
   client_write_iv = HKDF-Expand-Label(client_hs_secret, "iv", "", 12)
   server_write_key = HKDF-Expand-Label(server_hs_secret, "key", "", 32)
   server_write_iv = HKDF-Expand-Label(server_hs_secret, "iv", "", 12)
```

---

## 🧪 Testing Strategy

### Unit Tests

File: `crates/beardog-tunnel/tests/phase8_https_comprehensive_tests.rs` (or new file)

```rust
#[tokio::test]
async fn test_tls_derive_handshake_secrets_rfc8446() {
    // Use test vectors if available, or verify structure
    let params = json!({
        "pre_master_secret": "base64_encoded_ecdh_output",
        "client_random": "base64_encoded_32_bytes",
        "server_random": "base64_encoded_32_bytes",
        "transcript_hash": "base64_encoded_sha256_hash"
    });
    
    let result = handle_tls_derive_handshake_secrets(Some(&params))
        .await
        .expect("Should derive handshake secrets");
    
    // Verify structure
    assert!(result.get("client_write_key").is_some());
    assert!(result.get("client_write_iv").is_some());
    assert!(result.get("server_write_key").is_some());
    assert!(result.get("server_write_iv").is_some());
    
    // Verify sizes
    let client_key = BASE64.decode(
        result["client_write_key"].as_str().unwrap()
    ).unwrap();
    assert_eq!(client_key.len(), 32);
    
    let client_iv = BASE64.decode(
        result["client_write_iv"].as_str().unwrap()
    ).unwrap();
    assert_eq!(client_iv.len(), 12);
}

#[tokio::test]
async fn test_handshake_vs_application_secrets_different() {
    // Verify that handshake and application secrets are different
    // (they should be - different stages of key schedule)
    
    let params = json!({
        "pre_master_secret": "...",
        "client_random": "...",
        "server_random": "...",
        "transcript_hash": "..."
    });
    
    let hs_result = handle_tls_derive_handshake_secrets(Some(&params))
        .await
        .unwrap();
    
    let app_result = handle_tls_derive_application_secrets(Some(&params))
        .await
        .unwrap();
    
    // Keys should be different!
    assert_ne!(
        hs_result["client_write_key"],
        app_result["client_write_key"]
    );
}
```

### Integration Test

```bash
# After implementation:
cd /home/eastgate/Development/ecoPrimals/phase1/beardog
cargo build --release
cp target/release/beardog ../../phase2/biomeOS/plasmidBin/primals/beardog/

cd ../../phase2/biomeOS
pkill -9 beardog; pkill -9 songbird; pkill -9 neural-api-server
./deploy_graph.sh

# Run HTTPS tests
./test_https_endpoints.sh

# Expected: 8/8 PASSING! 🎉
```

---

## 📈 Impact Assessment

### Before This Implementation

**HTTPS Coverage**: 99.5%
- ✅ Songbird: RFC 8446 compliant (v5.8.6)
- ✅ BearDog Application Keys: Implemented
- ❌ BearDog Handshake Keys: **MISSING**

**RPC Methods**: 82
- TLS methods: 4 (derive_secrets, derive_application_secrets, sign_handshake, verify_certificate)

### After This Implementation

**HTTPS Coverage**: **100%!** 🎉
- ✅ Songbird: RFC 8446 compliant (v5.8.6)
- ✅ BearDog Application Keys: Implemented
- ✅ BearDog Handshake Keys: **IMPLEMENTED**

**RPC Methods**: 83 (+1)
- TLS methods: 5 (+ derive_handshake_secrets)

**Ecosystem Impact**:
- 🎯 100% Pure Rust HTTPS complete
- 🎯 Songbird unblocked for all HTTPS endpoints
- 🎯 GitHub API, CloudFlare, Google, AWS all accessible
- 🎯 Squirrel AI can access Anthropic, OpenAI, Ollama
- 🎯 Full ecosystem HTTPS capability

---

## 🎯 Success Criteria

### Implementation Success ✅

- [ ] `tls.derive_handshake_secrets` method exists
- [ ] RFC 8446 Section 7.1 key schedule implemented correctly
- [ ] Returns client/server write keys and IVs (32 + 12 bytes each)
- [ ] Accepts transcript_hash parameter (required, 32 bytes)
- [ ] Comprehensive logging added
- [ ] Added to CryptoHandler registry

### Testing Success ✅

- [ ] Unit tests pass (handshake secret derivation)
- [ ] Integration test with Songbird passes
- [ ] HTTPS endpoints test: 8/8 passing

### Documentation Success ✅

- [ ] RPC API docs updated
- [ ] CHANGELOG updated
- [ ] README achievement updated

---

## 🏆 Why This Is THE Final Piece

### All Other Components Complete ✅

1. ✅ Songbird v5.8.6 - RFC 8446 fully compliant
2. ✅ Neural API - Capability translation configured
3. ✅ BearDog Application Keys - Working perfectly
4. ✅ All TLS 1.3 protocol fixes - Applied
5. ✅ Transcript hash tracking - Implemented
6. ✅ ChaCha20-Poly1305 - Pure Rust
7. ✅ X25519 ECDH - Pure Rust
8. ✅ Ed25519 signatures - Pure Rust

### Only This Remains ⏳

- ❌ BearDog `tls.derive_handshake_secrets` RPC method

**After this**: **TOTAL VICTORY - 100% PURE RUST HTTPS!** 🦀✨

---

## 📞 Coordination

**biomeOS**: Ready to test immediately after implementation  
**Songbird**: v5.8.6 waiting for this method  
**Neural API**: Graph configured and ready  
**BearDog**: Final implementation needed!

**Timeline**: 1-2 hours implementation + testing

---

## 🚀 Next Steps

1. ✅ Review and approve this plan
2. ⏳ Implement `handle_tls_derive_handshake_secrets`
3. ⏳ Add to handler registry
4. ⏳ Write comprehensive tests
5. ⏳ Update documentation
6. ⏳ Deploy and test with Songbird
7. 🎉 **CELEBRATE 100% PURE RUST HTTPS!**

---

**🔑 ONE METHOD AWAY FROM HISTORY! LET'S DO THIS! 🚀**

*Plan Created: January 22, 2026*  
*Priority: CRITICAL*  
*ETA: 1-2 hours*  
*Confidence: ABSOLUTE*  
*Impact: 100% Pure Rust HTTPS Complete!*

