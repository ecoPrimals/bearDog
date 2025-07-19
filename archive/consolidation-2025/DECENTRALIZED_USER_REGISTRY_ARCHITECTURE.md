# Decentralized User Registry Architecture

## Overview

BearDog implements a **decentralized user registry** that functions like a "personal phone book that can be shared" rather than a centralized database. This architecture aligns with decentralized crypto principles and ensures no single point of control or failure.

## Core Principles

### 1. **Self-Sovereign Identity**
- Users control their own identity and cryptographic keys
- No central authority can revoke or modify user identities
- Identity verification is cryptographically provable
- Users can exist on multiple nodes simultaneously

### 2. **Node-Local Registries**
- Each BearDog node maintains its own user registry
- Local storage is encrypted and secured per node
- No requirement to synchronize with other nodes
- Nodes can operate independently (offline-capable)

### 3. **Federated Discovery**
- Nodes can share user information with trusted peers
- Federation is opt-in and configurable
- Trust relationships are established between nodes
- Information sharing respects privacy preferences

### 4. **Distributed Verification**
- User identity is verified through cryptographic proofs
- Multiple nodes can independently verify the same user
- No single source of truth required
- Consensus emerges from distributed verification

## Architecture Components

### 1. Local User Registry

```rust
/// Local user registry maintained by each node
pub struct LocalUserRegistry {
    /// Node-local user database (encrypted at rest)
    local_users: HashMap<String, LocalUserRecord>,
    /// Cache of federated users from trusted peers
    federated_cache: HashMap<String, FederatedUserRecord>,
    /// Trust relationships with other nodes
    trusted_peers: HashSet<String>,
    /// Registry configuration
    config: RegistryConfig,
}

/// User record stored locally on the node
pub struct LocalUserRecord {
    /// Username (not globally unique)
    username: String,
    /// Cryptographic public key (globally unique)
    public_key: Vec<u8>,
    /// Password hash (node-local authentication)
    password_hash: String,
    /// User metadata (preferences, settings)
    metadata: HashMap<String, String>,
    /// Creation timestamp
    created_at: DateTime<Utc>,
    /// Last authentication
    last_auth: Option<DateTime<Utc>>,
}
```

### 2. Federated User Discovery

```rust
/// Federated user discovery across trusted peer network
pub struct FederatedDiscovery {
    /// Known peer nodes and their trust levels
    peer_nodes: HashMap<String, PeerNodeInfo>,
    /// Cache of users discovered from peers
    federated_users: HashMap<String, FederatedUserRecord>,
    /// Discovery configuration
    config: DiscoveryConfig,
}

/// User record received from a federated peer
pub struct FederatedUserRecord {
    /// Username as reported by the peer
    username: String,
    /// Cryptographic public key for verification
    public_key: Vec<u8>,
    /// Source node that provided this record
    source_node: String,
    /// Trust score based on peer reputation
    trust_score: f64,
    /// Verification timestamp
    verified_at: DateTime<Utc>,
}
```

### 3. Distributed Hash Table (DHT) Integration

```rust
/// DHT-based user discovery for wider network reach
pub struct DhtUserDiscovery {
    /// DHT network connection
    dht_client: Arc<DhtClient>,
    /// Local DHT node identifier
    node_id: String,
    /// DHT configuration
    config: DhtConfig,
}

/// User identity record stored in DHT
pub struct DhtUserRecord {
    /// Cryptographic identity (public key hash)
    identity_hash: String,
    /// Self-signed identity proof
    identity_proof: Vec<u8>,
    /// Contact information (optional, encrypted)
    contact_info: Option<EncryptedContactInfo>,
    /// Timestamp of last update
    updated_at: DateTime<Utc>,
}
```

## User Lookup Flow

### 1. **Local Lookup (Fastest)**
```rust
async fn lookup_user_local(&self, username: &str) -> Option<LocalUserRecord> {
    // Check node-local user registry first
    self.local_users.get(username).cloned()
}
```

### 2. **Federated Lookup (Trusted Peers)**
```rust
async fn lookup_user_federated(&self, username: &str) -> Option<FederatedUserRecord> {
    // Check federated cache first
    if let Some(cached) = self.federated_cache.get(username) {
        return Some(cached.clone());
    }
    
    // Query trusted peer nodes
    for peer in &self.trusted_peers {
        if let Some(user_record) = self.query_peer_for_user(peer, username).await {
            // Cache the result for future lookups
            self.federated_cache.insert(username.to_string(), user_record.clone());
            return Some(user_record);
        }
    }
    
    None
}
```

### 3. **DHT Lookup (Network-Wide)**
```rust
async fn lookup_user_dht(&self, username: &str) -> Option<DhtUserRecord> {
    // Generate identity hash from username
    let identity_hash = self.generate_identity_hash(username);
    
    // Query DHT network
    if let Ok(dht_record) = self.dht_client.get(&identity_hash).await {
        // Verify cryptographic proof
        if self.verify_identity_proof(&dht_record).await {
            return Some(dht_record);
        }
    }
    
    None
}
```

### 4. **Complete Lookup Implementation**
```rust
async fn lookup_user_in_decentralized_registry(&self, username: &str) -> BearDogResult<Option<String>> {
    // Priority order: Local -> Federated -> DHT
    
    // 1. Check local registry (fastest)
    if let Some(local_user) = self.lookup_user_local(username).await {
        return Ok(Some(local_user.password_hash));
    }
    
    // 2. Check trusted peer federation (moderate speed)
    if let Some(federated_user) = self.lookup_user_federated(username).await {
        // Note: Federated users might not have local password hashes
        // They authenticate via cryptographic proof instead
        return Ok(None); // Or handle federated auth differently
    }
    
    // 3. Check DHT network (slower, wider reach)
    if let Some(dht_user) = self.lookup_user_dht(username).await {
        // DHT users use cryptographic identity proofs
        return Ok(None); // Handle identity proof verification
    }
    
    // User not found in any registry
    Ok(None)
}
```

## Privacy and Security

### 1. **Data Minimization**
- Only essential user data is stored locally
- Sensitive data is encrypted at rest
- Optional data (like contact info) is user-controlled
- Automatic expiration of unused cached data

### 2. **Consent-Based Sharing**
- Users control which information can be shared
- Peer nodes must be explicitly trusted
- Federation settings are user-configurable
- Opt-out mechanisms for all sharing

### 3. **Cryptographic Verification**
- All user records include cryptographic proofs
- Public key infrastructure ensures authenticity
- Identity verification doesn't require central authority
- Tamper detection through digital signatures

### 4. **Trust Management**
- Node-to-node trust relationships are explicit
- Trust scores decay over time without interaction
- Malicious behavior results in trust revocation
- Trust is not transitive (A trusts B, B trusts C ≠ A trusts C)

## Implementation Phases

### Phase 1: Local Registry (Current)
- Implement local user storage with encryption
- Basic user management (create, update, delete)
- Local authentication with password hashes
- Node-specific user preferences

### Phase 2: Federated Discovery
- Peer node registration and trust management
- Federated user query protocol
- Cached federated user records
- Trust-based result filtering

### Phase 3: DHT Integration
- DHT network integration for user discovery
- Cryptographic identity proof system
- Self-sovereign identity registration
- Network-wide user search capabilities

### Phase 4: Advanced Features
- Cross-node authentication protocols
- Reputation-based trust scoring
- Privacy-preserving user matching
- Decentralized user verification

## Configuration Example

```toml
[user_registry]
# Local registry settings
local_storage_path = "~/.beardog/users.db"
encryption_enabled = true
auto_backup = true

# Federation settings
enable_federation = true
max_trusted_peers = 50
federated_cache_ttl = "24h"
trust_threshold = 0.7

# DHT settings
enable_dht_discovery = true
dht_bootstrap_nodes = [
    "beardog://bootstrap1.example.com:8443",
    "beardog://bootstrap2.example.com:8443"
]
dht_replication_factor = 3

# Privacy settings
allow_peer_queries = true
share_user_metadata = false
require_mutual_trust = true
```

## Benefits of This Architecture

### 1. **Decentralization**
- No single point of failure or control
- Each node can operate independently
- User data is distributed across the network
- Resistance to censorship or shutdown

### 2. **Privacy by Design**
- Users control their own data and sharing preferences
- Minimal data collection and storage
- Encrypted storage and transmission
- Consent-based information sharing

### 3. **Scalability**
- Linear scaling with network growth
- Local caching reduces network load
- DHT provides efficient distributed lookup
- No bottlenecks from centralized services

### 4. **Security**
- Cryptographic identity verification
- Tamper-resistant user records
- Trust-based peer relationships
- Self-sovereign identity control

## Conclusion

This decentralized user registry architecture ensures that BearDog maintains its commitment to decentralized crypto principles while providing efficient user discovery and authentication. The "personal phone book that can be shared" model respects user privacy, eliminates central authorities, and provides the foundation for a truly decentralized security ecosystem.

---

**Document Version**: 1.0  
**Last Updated**: 2025-01-11  
**Next Review**: 2025-01-25  
**Owner**: BearDog Engineering Team 