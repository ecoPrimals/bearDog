# 🎵 BirdSong Integration Guide for Songbird Team

**Date**: December 21, 2025  
**Status**: Ready for Phase 3  
**Audience**: Songbird Team

---

## 🎯 Quick Start

BearDog Phase 1-2 is **complete** and ready for Songbird integration. This guide shows exactly how to use BearDog's BirdSong API.

---

## 📦 What BearDog Provides

### **Complete Lineage-Based Encryption System**:
```rust
use beardog_genetics::birdsong::{
    BirdSongManager,
    LineageHint,
    BirdSongEncryptRequest,
    BirdSongDecryptRequest,
    BirdSongBroadcast,
};
```

### **Core Capabilities**:
1. ✅ **Lineage Management** - Create cryptographic family trees
2. ✅ **Lineage Proofs** - Verify node ancestry
3. ✅ **Key Derivation** - HKDF keys from lineage
4. ✅ **Broadcast Encryption** - ChaCha20-Poly1305 "family-only" encryption
5. ✅ **Key Distribution** - Automatic to all descendants

---

## 🚀 Integration Steps for Songbird

### **Step 1: Initialize BirdSong Manager**

```rust
use beardog_genetics::birdsong::BirdSongManager;

// In Songbird's initialization
pub struct SongbirdNode {
    // ... existing fields ...
    birdsong_manager: Arc<BirdSongManager>,
}

impl SongbirdNode {
    pub async fn new(config: SongbirdConfig) -> Result<Self, SongbirdError> {
        // Get master secret from HSM or secure config
        let master_secret = hsm.generate_master_secret()?;
        
        // Initialize BirdSong
        let birdsong_manager = Arc::new(
            BirdSongManager::new(master_secret, None).await?
        );
        
        Ok(Self {
            // ... existing fields ...
            birdsong_manager,
        })
    }
}
```

### **Step 2: Create or Join Lineage**

```rust
// For root node (federation founder)
async fn create_federation_lineage(&self) -> Result<String, SongbirdError> {
    let chain = self.birdsong_manager
        .generate_root_lineage(
            self.node_id.clone(),
            Some(self.create_metadata())
        )
        .await?;
    
    // Store chain_id in node config
    self.config.write().chain_id = chain.chain_id.clone();
    
    Ok(chain.chain_id)
}

// For child node (joining federation)
async fn join_federation_lineage(
    &self,
    parent_id: &str,
    chain_id: &str,
) -> Result<(), SongbirdError> {
    let _child_node = self.birdsong_manager
        .add_child(
            chain_id,
            parent_id,
            self.node_id.clone(),
            Some(self.create_metadata())
        )
        .await?;
    
    Ok(())
}

fn create_metadata(&self) -> LineageMetadata {
    LineageMetadata {
        biome_type: Some("songbird".to_string()),
        capabilities: vec!["federation".to_string(), "relay".to_string()],
        trust_level: 1.0,
        custom: HashMap::new(),
    }
}
```

### **Step 3: Replace Plaintext Discovery with BirdSong**

**Before (plaintext LAN discovery)**:
```rust
// Old: Plaintext UDP broadcast
let discovery_msg = serde_json::to_vec(&DiscoveryMessage {
    node_id: self.node_id.clone(),
    endpoint: self.endpoint.clone(),
    capabilities: self.capabilities.clone(),
})?;

udp_socket.send_to(&discovery_msg, BROADCAST_ADDR)?;
```

**After (encrypted BirdSong)**:
```rust
// New: Encrypted broadcast
async fn broadcast_presence(&self) -> Result<(), SongbirdError> {
    // 1. Create discovery message
    let discovery_msg = serde_json::to_vec(&DiscoveryMessage {
        node_id: self.node_id.clone(),
        endpoint: self.endpoint.clone(),
        capabilities: self.capabilities.clone(),
    })?;
    
    // 2. Create lineage hint (who can decrypt)
    let hint = LineageHint {
        root_id: self.config.read().chain_id.clone(),
        min_depth: 0,  // Everyone in family
        max_depth: 100,
        biome_filter: Some("songbird".to_string()),
        version: 1,
    };
    
    // 3. Encrypt for lineage
    let request = BirdSongEncryptRequest {
        plaintext: discovery_msg,
        lineage_hint: hint,
        associated_data: Some(b"discovery-v1".to_vec()),
    };
    
    let broadcast = self.birdsong_manager.encrypt_broadcast(&request)?;
    
    // 4. Send encrypted broadcast
    let serialized = serde_json::to_vec(&broadcast)?;
    self.udp_socket.send_to(&serialized, BROADCAST_ADDR)?;
    
    Ok(())
}
```

### **Step 4: Receive and Decrypt BirdSong**

```rust
async fn handle_received_broadcast(&self, data: &[u8]) -> Result<(), SongbirdError> {
    // 1. Deserialize broadcast
    let broadcast: BirdSongBroadcast = serde_json::from_slice(data)?;
    
    // 2. Check if we can decrypt (quick check before full proof)
    let our_depth = self.get_our_lineage_depth().await?;
    if !self.birdsong_manager.can_decrypt(&broadcast, our_depth) {
        // Not for us - ignore
        return Ok(());
    }
    
    // 3. Generate lineage proof
    let chain_id = self.config.read().chain_id.clone();
    let proof = self.birdsong_manager.generate_lineage_proof(
        &chain_id,
        &self.node_id
    )?;
    
    // 4. Decrypt broadcast
    let decrypt_request = BirdSongDecryptRequest {
        broadcast,
        proof,
    };
    
    let plaintext = self.birdsong_manager.decrypt_broadcast(&decrypt_request)?;
    
    // 5. Process decrypted discovery message
    let discovery_msg: DiscoveryMessage = serde_json::from_slice(&plaintext)?;
    self.handle_discovery(discovery_msg).await?;
    
    Ok(())
}
```

### **Step 5: Key Request Flow (for new nodes)**

```rust
// New node requests keys from ancestor
async fn request_lineage_key(
    &self,
    ancestor_endpoint: &str,
    hint: &LineageHint,
) -> Result<BirdSongKey, SongbirdError> {
    // 1. Generate our lineage proof
    let chain_id = self.config.read().chain_id.clone();
    let proof = self.birdsong_manager.generate_lineage_proof(
        &chain_id,
        &self.node_id
    )?;
    
    // 2. Request key from ancestor (HTTP or custom protocol)
    let request = KeyRequestMessage {
        hint: hint.clone(),
        proof,
    };
    
    let response: KeyResponseMessage = self.http_client
        .post(format!("{}/birdsong/key-request", ancestor_endpoint))
        .json(&request)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response.key)
}

// Ancestor handles key request
async fn handle_key_request(
    &self,
    request: KeyRequestMessage,
) -> Result<KeyResponseMessage, SongbirdError> {
    let chain_id = self.config.read().chain_id.clone();
    
    // BearDog verifies proof and returns key
    let key = self.birdsong_manager.request_key(
        &request.hint,
        &request.proof,
        &chain_id
    )?;
    
    Ok(KeyResponseMessage { key })
}
```

### **Step 6: Backward Compatibility (LAN + P2P)**

```rust
async fn broadcast_with_fallback(&self) -> Result<(), SongbirdError> {
    // Try BirdSong first (P2P mode)
    if let Some(ref birdsong) = self.birdsong_manager {
        match self.broadcast_encrypted(birdsong).await {
            Ok(()) => return Ok(()),
            Err(e) => {
                warn!("BirdSong broadcast failed, falling back to plaintext: {}", e);
            }
        }
    }
    
    // Fallback to plaintext (LAN mode)
    self.broadcast_plaintext().await
}

async fn handle_received_data(&self, data: &[u8]) -> Result<(), SongbirdError> {
    // Try to parse as BirdSong broadcast first
    if let Ok(broadcast) = serde_json::from_slice::<BirdSongBroadcast>(data) {
        return self.handle_received_broadcast(&broadcast).await;
    }
    
    // Otherwise, treat as plaintext discovery
    if let Ok(discovery) = serde_json::from_slice::<DiscoveryMessage>(data) {
        return self.handle_discovery(discovery).await;
    }
    
    Err(SongbirdError::InvalidMessage)
}
```

---

## 🔑 Key Management

### **Distribute Keys to Federation**

```rust
// After lineage is established, distribute keys
async fn distribute_federation_keys(&self) -> Result<(), SongbirdError> {
    let chain_id = self.config.read().chain_id.clone();
    let root_id = self.node_id.clone();
    
    // Distribute to all descendants
    let count = self.birdsong_manager.distribute_keys_to_descendants(
        &chain_id,
        &root_id,
        0  // Generation 0
    )?;
    
    info!("Distributed keys to {} nodes", count);
    Ok(())
}
```

### **Rotate Keys Periodically**

```rust
// Every 24 hours (or based on config)
async fn scheduled_key_rotation(&self) -> Result<(), SongbirdError> {
    let chain_id = self.config.read().chain_id.clone();
    let root_id = self.node_id.clone();
    let new_generation = self.config.read().current_key_generation + 1;
    
    self.birdsong_manager.rotate_all_keys(
        &chain_id,
        &root_id,
        new_generation
    )?;
    
    self.config.write().current_key_generation = new_generation;
    info!("Rotated to key generation {}", new_generation);
    Ok(())
}
```

### **Revoke Keys (for compromised nodes)**

```rust
async fn revoke_node_keys(&self, node_id: &str) -> Result<(), SongbirdError> {
    let chain_id = self.config.read().chain_id.clone();
    
    let revoked_count = self.birdsong_manager.revoke_keys(
        &chain_id,
        node_id
    );
    
    warn!("Revoked keys from {} nodes (including descendants)", revoked_count);
    Ok(())
}
```

---

## 🧪 Testing Integration

### **Unit Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_birdsong_integration() -> Result<(), SongbirdError> {
        // Setup
        let master_secret = vec![0xAB; 32];
        let manager = BirdSongManager::new(master_secret, None).await?;
        
        // Create lineage
        let chain = manager.generate_root_lineage("node-1".to_string(), None).await?;
        manager.add_child(&chain.chain_id, "node-1", "node-2".to_string(), None).await?;
        
        // Encrypt
        let hint = LineageHint {
            root_id: "node-1".to_string(),
            min_depth: 0,
            max_depth: 10,
            biome_filter: None,
            version: 1,
        };
        
        let request = BirdSongEncryptRequest {
            plaintext: b"test message".to_vec(),
            lineage_hint: hint,
            associated_data: None,
        };
        
        let broadcast = manager.encrypt_broadcast(&request)?;
        
        // Decrypt
        let proof = manager.generate_lineage_proof(&chain.chain_id, "node-2")?;
        let decrypt_request = BirdSongDecryptRequest { broadcast, proof };
        let plaintext = manager.decrypt_broadcast(&decrypt_request)?;
        
        assert_eq!(plaintext, b"test message");
        Ok(())
    }
}
```

### **Integration Tests**

```rust
#[tokio::test]
async fn test_songbird_beardog_full_flow() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Start BearDog with BirdSong
    let beardog = BearDogNode::new().await?;
    let chain = beardog.create_lineage("root".to_string()).await?;
    
    // 2. Start two Songbird nodes
    let songbird1 = SongbirdNode::new_with_beardog(beardog.clone()).await?;
    let songbird2 = SongbirdNode::new_with_beardog(beardog.clone()).await?;
    
    // 3. Join lineage
    songbird1.join_lineage(&chain.chain_id, "root").await?;
    songbird2.join_lineage(&chain.chain_id, "root").await?;
    
    // 4. Songbird1 broadcasts
    songbird1.broadcast_presence().await?;
    
    // 5. Songbird2 should receive and decrypt
    let received = songbird2.wait_for_discovery().await?;
    assert_eq!(received.node_id, songbird1.node_id);
    
    Ok(())
}
```

---

## 🔧 Configuration

### **Recommended Config**

```toml
[birdsong]
enabled = true
key_rotation_interval_hours = 24
max_lineage_depth = 10
enable_witnesses = true
min_witnesses = 1

[birdsong.fallback]
# Backward compatibility
enable_plaintext_lan = true
auto_detect_mode = true  # Try BirdSong first, fallback to plaintext
```

---

## 📊 Performance Expectations

| Operation | Expected Latency | Notes |
|-----------|-----------------|-------|
| Key derivation | <10μs | HKDF-SHA256 |
| Encrypt (1KB) | <0.5ms | ChaCha20-Poly1305 |
| Decrypt (1KB) | <0.5ms | ChaCha20-Poly1305 |
| Proof generation | <1ms | Depends on lineage depth |
| Proof verification | <2ms | Ed25519 signature checks |

**Throughput**: ~2GB/s encryption/decryption on modern CPU

---

## 🐛 Common Issues

### **Issue 1: "Lineage mismatch" error**
```rust
Error: "Lineage mismatch: broadcast for family-a, proof for family-b"
```
**Solution**: Ensure node is generating proof for correct chain_id

### **Issue 2: "Node depth not in allowed range"**
```rust
Error: "Node depth 5 not in allowed range [0, 3]"
```
**Solution**: Check `LineageHint` min/max depth values

### **Issue 3: "Decryption failed"**
```rust
Error: "Decryption failed (wrong key or tampered data)"
```
**Possible causes**:
- Wrong key generation
- Corrupted ciphertext
- Tampering detected (AEAD failure)

---

## 📚 API Reference

### **BirdSongManager Methods**

```rust
// Lineage management
async fn generate_root_lineage(&self, root_node_id: String, metadata: Option<LineageMetadata>) -> Result<LineageChain>
async fn add_child(&self, chain_id: &str, parent_id: &str, child_id: String, metadata: Option<LineageMetadata>) -> Result<LineageNode>
fn get_lineage_chain(&self, chain_id: &str) -> Option<LineageChain>
fn get_descendants(&self, chain_id: &str, node_id: &str) -> Vec<LineageNode>

// Proofs
fn generate_lineage_proof(&self, chain_id: &str, node_id: &str) -> Result<LineageProof>
fn verify_lineage_proof(&self, proof: &LineageProof, chain_id: &str) -> Result<LineageVerificationResult>

// Encryption
fn encrypt_broadcast(&self, request: &BirdSongEncryptRequest) -> Result<BirdSongBroadcast>
fn decrypt_broadcast(&self, request: &BirdSongDecryptRequest) -> Result<Vec<u8>>
fn can_decrypt(&self, broadcast: &BirdSongBroadcast, node_depth: u32) -> bool

// Key management
fn distribute_keys_to_descendants(&self, chain_id: &str, root_id: &str, generation: u32) -> Result<usize>
fn get_distributed_keys(&self, node_id: &str) -> Vec<BirdSongKey>
fn request_key(&self, lineage_hint: &LineageHint, proof: &LineageProof, chain_id: &str) -> Result<BirdSongKey>
fn revoke_keys(&self, chain_id: &str, node_id: &str) -> usize
fn rotate_all_keys(&self, chain_id: &str, root_id: &str, new_generation: u32) -> Result<usize>
```

---

## 🎯 Next Steps for Songbird Team

### **Week 1-2**:
- [ ] Review this integration guide
- [ ] Add `beardog-genetics` dependency to Songbird
- [ ] Initialize `BirdSongManager` in Songbird nodes
- [ ] Create lineage structures for test federations

### **Week 3-4**:
- [ ] Replace plaintext discovery with BirdSong broadcasts
- [ ] Implement encrypted broadcast handling
- [ ] Add backward compatibility mode

### **Week 5-6**:
- [ ] Implement key request flow
- [ ] Add key distribution to federation
- [ ] Write integration tests

### **Week 7-9**:
- [ ] Performance testing
- [ ] Privacy verification
- [ ] Production deployment preparation

---

## 📬 Support

**Questions?** Contact BearDog team:
- **Documentation**: See `BIRDSONG_PHASE1_2_COMPLETE_DEC_21_2025.md`
- **Tests**: Run `cargo test -p beardog-genetics --lib birdsong`
- **Live Demo**: See `showcase/02-ecosystem-integration/01-songbird-btsp/phase0-foundation/`

---

**Document Version**: 1.0  
**Date**: December 21, 2025  
**Status**: Ready for Songbird Phase 3 Integration  
**Next**: Joint kickoff meeting

🎵 **Happy integrating! Let's build true P2P together!** 🐻✨

