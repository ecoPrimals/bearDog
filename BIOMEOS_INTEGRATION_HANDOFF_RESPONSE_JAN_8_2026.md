# 🐻 BearDog Response to biomeOS Integration Handoff

**Date**: January 8, 2026  
**From**: BearDog Team  
**To**: biomeOS Team  
**Re**: Unix Socket API Integration for Spore Federation

---

## 🎊 Excellent Work, biomeOS Team!

Congratulations on the successful LAN federation deployment! We're excited to see the integration coming together. Your detailed requirements are exactly what we need.

---

## ✅ Current Status: We're 90% Ready!

### **Good News**: We Already Have Most of This!

1. ✅ **Unix Socket IPC Infrastructure** - Battle-tested (25 tests, 100% pass rate)
2. ✅ **HSM Integration** - Auto-initialize, software/hardware support
3. ✅ **Genetic Lineage System** - Complete implementation
4. ✅ **Crypto Primitives** - Ed25519, RSA, AES-256-GCM, HKDF
5. ✅ **JSON-RPC 2.0 Support** - Standard error codes implemented

### **What We Need to Add**: JSON-RPC Method Handlers

We just need to expose our existing functionality via Unix socket JSON-RPC methods. This is straightforward!

---

## 📋 Implementation Plan

### **Phase 1: Genetic Lineage API** (Priority: HIGH - 2-3 hours)

#### **Method 1: `verify_family_member`**

**Status**: ✅ **Foundation Ready** - Just needs JSON-RPC wrapper

**Existing Implementation**:
- `crates/beardog-genetics/src/birdsong/genesis.rs` - Genesis verification
- `crates/beardog-genetics/src/key_derivation.rs` - HKDF derivation
- `crates/beardog-genetics/src/lineage_node.rs` - Lineage tracking

**Required Work**:
```rust
// Add to crates/beardog-tunnel/src/unix_socket_ipc.rs

async fn handle_verify_family_member(&self, params: Value) -> Result<Value, JsonRpcError> {
    let family_id: String = params["family_id"].as_str()
        .ok_or_else(|| JsonRpcError::invalid_params("Missing family_id"))?
        .to_string();
    
    let seed_hash: String = params["seed_hash"].as_str()
        .ok_or_else(|| JsonRpcError::invalid_params("Missing seed_hash"))?
        .to_string();
    
    // Use existing genetics engine
    let result = self.genetics_engine
        .verify_family_membership(&family_id, &seed_hash)
        .await?;
    
    Ok(json!({
        "is_family_member": result.is_member,
        "relationship": result.relationship,
        "parent_seed_hash": result.parent_hash,
        "derivation_path": result.path,
        "verified_at": chrono::Utc::now().to_rfc3339()
    }))
}
```

**Estimate**: 2-3 hours (mostly testing)

---

#### **Method 2: `derive_subfed_key`**

**Status**: ✅ **Foundation Ready** - Just needs JSON-RPC wrapper

**Existing Implementation**:
- `crates/beardog-genetics/src/key_derivation.rs` - HKDF-SHA256
- `crates/beardog-tunnel/src/tunnel/hsm/manager.rs` - Key management
- `crates/beardog-core/src/crypto_service/` - Key storage

**Required Work**:
```rust
// Add to crates/beardog-tunnel/src/unix_socket_ipc.rs

async fn handle_derive_subfed_key(&self, params: Value) -> Result<Value, JsonRpcError> {
    let parent_family: String = params["parent_family"].as_str()
        .ok_or_else(|| JsonRpcError::invalid_params("Missing parent_family"))?
        .to_string();
    
    let subfed_name: String = params["subfed_name"].as_str()
        .ok_or_else(|| JsonRpcError::invalid_params("Missing subfed_name"))?
        .to_string();
    
    // Derive key using HKDF
    let key_ref = self.hsm_manager
        .derive_subfederation_key(&parent_family, &subfed_name)
        .await?;
    
    Ok(json!({
        "key_ref": key_ref,
        "algorithm": "AES-256-GCM",
        "key_id": format!("subfed:{}:{}:v1", parent_family, subfed_name),
        "created_at": chrono::Utc::now().to_rfc3339(),
        "expires_at": null
    }))
}
```

**Estimate**: 2-3 hours

---

### **Phase 2: Encryption API** (Priority: HIGH - 3-4 hours)

#### **Method 3: `encrypt`**

**Status**: ✅ **Foundation Ready** - AES-256-GCM already implemented

**Existing Implementation**:
- `crates/beardog-core/src/crypto_service/` - Encryption primitives
- `crates/beardog-tunnel/src/tunnel/hsm/` - HSM-backed encryption

**Required Work**:
```rust
async fn handle_encrypt(&self, params: Value) -> Result<Value, JsonRpcError> {
    let data_b64: String = params["data"].as_str()
        .ok_or_else(|| JsonRpcError::invalid_params("Missing data"))?
        .to_string();
    
    let key_ref: String = params["key_ref"].as_str()
        .ok_or_else(|| JsonRpcError::invalid_params("Missing key_ref"))?
        .to_string();
    
    let data = base64::decode(&data_b64)
        .map_err(|_| JsonRpcError::invalid_params("Invalid base64"))?;
    
    // Use HSM for encryption
    let result = self.crypto_service
        .encrypt_aes_gcm(&data, &key_ref)
        .await?;
    
    Ok(json!({
        "encrypted_data": base64::encode(&result.ciphertext),
        "nonce": base64::encode(&result.nonce),
        "tag": base64::encode(&result.tag)
    }))
}
```

**Estimate**: 2-3 hours

---

#### **Method 4: `decrypt`**

**Status**: ✅ **Foundation Ready**

**Required Work**:
```rust
async fn handle_decrypt(&self, params: Value) -> Result<Value, JsonRpcError> {
    let encrypted_data_b64: String = params["encrypted_data"].as_str()
        .ok_or_else(|| JsonRpcError::invalid_params("Missing encrypted_data"))?
        .to_string();
    
    let nonce_b64: String = params["nonce"].as_str()
        .ok_or_else(|| JsonRpcError::invalid_params("Missing nonce"))?
        .to_string();
    
    let tag_b64: String = params["tag"].as_str()
        .ok_or_else(|| JsonRpcError::invalid_params("Missing tag"))?
        .to_string();
    
    let key_ref: String = params["key_ref"].as_str()
        .ok_or_else(|| JsonRpcError::invalid_params("Missing key_ref"))?
        .to_string();
    
    // Decode base64
    let encrypted_data = base64::decode(&encrypted_data_b64)?;
    let nonce = base64::decode(&nonce_b64)?;
    let tag = base64::decode(&tag_b64)?;
    
    // Use HSM for decryption
    let plaintext = self.crypto_service
        .decrypt_aes_gcm(&encrypted_data, &nonce, &tag, &key_ref)
        .await?;
    
    Ok(json!({
        "data": base64::encode(&plaintext),
        "verified": true
    }))
}
```

**Estimate**: 1-2 hours

---

## 🎯 Total Estimate: 8-12 Hours

### **Breakdown**:
- Phase 1A: `verify_family_member` - 2-3 hours
- Phase 1B: `derive_subfed_key` - 2-3 hours
- Phase 2A: `encrypt` - 2-3 hours
- Phase 2B: `decrypt` - 1-2 hours
- Testing & Integration - 1-2 hours

### **Timeline**: 1-2 days (with testing)

---

## 🧪 Testing Strategy

### **Unit Tests** (Per Method)
```rust
#[tokio::test]
async fn test_verify_family_member_jsonrpc() {
    let server = create_test_server().await;
    
    let request = json!({
        "jsonrpc": "2.0",
        "method": "verify_family_member",
        "params": {
            "family_id": "nat0",
            "seed_hash": "aaeaa3cfd69dd379...",
            "node_id": "node-alpha"
        },
        "id": 1
    });
    
    let response = server.handle_request(request).await.unwrap();
    
    assert_eq!(response["result"]["is_family_member"], true);
    assert_eq!(response["result"]["relationship"], "sibling");
}
```

### **Integration Tests** (E2E with biomeOS)
Your existing E2E tests should pass automatically once we implement these methods!

---

## 📚 Implementation Files

### **Files to Modify**:

1. **`crates/beardog-tunnel/src/unix_socket_ipc.rs`**
   - Add method routing for new JSON-RPC methods
   - Implement handlers

2. **`crates/beardog-genetics/src/lib.rs`**
   - Expose `verify_family_membership` method
   - Add sub-federation key derivation

3. **`crates/beardog-core/src/crypto_service/mod.rs`**
   - Expose `encrypt_aes_gcm` and `decrypt_aes_gcm`

4. **`tests/biomeos_integration_tests.rs`** (NEW)
   - Test all 4 methods
   - Test error handling
   - Test with real spore seeds

---

## 🚀 Proposed Implementation Order

### **Day 1: Foundation**
1. Add method routing to `unix_socket_ipc.rs`
2. Implement `verify_family_member`
3. Unit tests for lineage verification

### **Day 2: Crypto Operations**
4. Implement `derive_subfed_key`
5. Implement `encrypt` and `decrypt`
6. Unit tests for crypto operations

### **Day 3: Integration**
7. Integration tests with biomeOS test data
8. Error handling refinement
9. Documentation update

---

## 🎯 API Specification (Final)

### **Base Configuration**

**Unix Socket Path**: `/tmp/beardog-{family_id}-{node_id}.sock`  
**Protocol**: JSON-RPC 2.0 over Unix socket  
**Transport**: Line-delimited JSON (one request per line)

### **Error Codes**

```rust
const FAMILY_NOT_FOUND: i64 = -32001;
const INVALID_SEED_HASH: i64 = -32002;
const KEY_DERIVATION_FAILED: i64 = -32003;
const ENCRYPTION_FAILED: i64 = -32004;
const DECRYPTION_FAILED: i64 = -32005;
const HSM_UNAVAILABLE: i64 = -32006;
```

### **Method Summary**

| Method | Purpose | Status |
|--------|---------|--------|
| `verify_family_member` | Genetic lineage verification | ✅ Ready to implement |
| `derive_subfed_key` | Sub-federation key derivation | ✅ Ready to implement |
| `encrypt` | AES-256-GCM encryption | ✅ Ready to implement |
| `decrypt` | AES-256-GCM decryption | ✅ Ready to implement |

---

## 🤝 Coordination with biomeOS

### **Your Test Data**

We'll use your real spore seeds for testing:
- **Family**: "nat0"
- **Seed Hash**: `aaeaa3cfd69dd379...`
- **Nodes**: node-alpha, node-beta, node-gamma, node-delta, node-epsilon

### **Testing Workflow**

1. **BearDog**: Implement methods, run unit tests
2. **BearDog**: Deploy updated binary to biomeOS test environment
3. **biomeOS**: Run E2E tests
4. **Both**: Iterate on any issues
5. **Both**: Production deployment

### **Communication**

- **Code Reviews**: We'll share PRs for your review
- **Testing**: We'll coordinate test runs with real spores
- **Issues**: Direct communication for any blockers

---

## 📊 What's Already Working

### **Unix Socket Infrastructure** ✅
- Lock-free atomic readiness
- Graceful shutdown
- Standard JSON-RPC 2.0 error codes
- 25 comprehensive tests (100% pass rate)
- Battle-tested resilience

### **Genetic Lineage System** ✅
- HKDF-SHA256 derivation
- Genesis verification
- Lineage node tracking
- Family relationship computation

### **Crypto Primitives** ✅
- AES-256-GCM (via RustCrypto)
- Ed25519 signatures
- RSA key management
- HSM integration (software + hardware)

### **HSM Management** ✅
- Auto-initialization
- Software HSM (pure Rust)
- Hardware HSM hooks
- Key lifecycle management

---

## 🎊 We're Excited!

This integration is exactly what BearDog was designed for:
- ✅ Zero-knowledge crypto for distributed systems
- ✅ Genetic lineage-based trust
- ✅ HSM-backed security
- ✅ Port-free Unix socket communication

Your detailed specification makes implementation straightforward. We'll have this ready in 1-2 days!

---

## 📞 Next Steps

### **Immediate (Today)**
1. Create GitHub issue for tracking
2. Set up integration test environment
3. Coordinate test data access

### **Day 1-2 (Implementation)**
4. Implement 4 JSON-RPC methods
5. Unit tests for each method
6. Integration tests with biomeOS test data

### **Day 3 (Integration)**
7. Deploy to biomeOS test environment
8. Run your E2E tests
9. Iterate on feedback

### **Week 2 (Production)**
10. Production deployment
11. Monitor and optimize
12. Support federation rollout

---

## 🙏 Thank You!

Excellent work on the client-side implementation! Your comprehensive documentation and E2E tests make this integration smooth. We're ready to deliver the Unix socket APIs you need.

Let's enable distributed trust! 🚀

---

**BearDog Team**  
*January 8, 2026*

---

🐻 **BearDog**: Ready to expose crypto & HSM via Unix sockets!  
🌱 **biomeOS**: Client code complete, waiting for our APIs  
🐦 **Songbird**: Discovery & P2P (separate coordination)

**Status**: ✅ **Implementation Starting Now!**

---

## 📚 References

- **Unix Socket Evolution**: `UNIX_SOCKET_EVOLUTION_PLAN.md`
- **Port-Free Architecture**: `PORT_FREE_ARCHITECTURE_JAN_8_2026.md`
- **Testing Excellence**: `TESTING_EXCELLENCE_JAN_8_2026.md`
- **Current Status**: `CURRENT_STATUS.md`

