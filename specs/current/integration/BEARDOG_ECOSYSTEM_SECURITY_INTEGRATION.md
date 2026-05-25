# BearDog Ecosystem Security Integration Specification
**Version**: 1.0.0  
**Date**: December 18, 2025  
**Status**: Draft → Implementation  
**Primal Sovereignty**: ✅ Fully Compliant

---

## 🎯 PURPOSE

Define BearDog's role as the **security primal** for the ecoPrimals ecosystem, providing:
1. Genetic encryption for cross-primal communication
2. Secure tunnel establishment (BSTP - Between-Tower Secure Protocol)
3. Encrypted compute environments for resource sharing
4. Delegation and constraint enforcement

**Core Principle**: BearDog has **self-knowledge only**. All primal interactions are capability-based with zero hardcoding.

---

## 🏛️ PRIMAL SOVEREIGNTY COMPLIANCE

### Self-Knowledge Only

**BearDog Knows**:
- ✅ Own capabilities (security, encryption, key management)
- ✅ Own genetic material (for key exchange)
- ✅ Own service endpoints
- ✅ Own configuration

**BearDog Does NOT Know**:
- ❌ Other primal names (no "Songbird", "ToadStool" hardcoded)
- ❌ Other primal locations (no IPs, no ports)
- ❌ Other primal implementations (no assumptions)
- ❌ Ecosystem topology (discovered at runtime)

### Capability-Based Discovery

```rust
// ✅ CORRECT: Capability-based (primal-agnostic)
let network_primals = discovery_service
    .discover_by_capability(UniversalCapabilityType::Network)
    .await?;

// ❌ WRONG: Hardcoded primal name
let songbird = find_primal("songbird").await?;
```

### Graceful Degradation

BearDog operates in layers:
1. **Standalone**: Full functionality without other primals
2. **Enhanced**: Network effects when primals discovered
3. **Degraded**: Graceful fallback if primals unavailable

```rust
// Example: Send encrypted message
match messenger.send_to_network_primal(data).await {
    Ok(response) => {
        // Network primal available - enhanced routing
        info!("✅ Sent via network primal");
    },
    Err(e) => {
        // Fallback: Direct peer-to-peer
        info!("⚠️ Network primal unavailable, using P2P");
        fallback_to_p2p(data).await?;
    }
}
```

---

## 📐 ARCHITECTURE OVERVIEW

### Three-Layer Security Model

```
┌─────────────────────────────────────────────────┐
│  Layer 3: Application Security                  │
│  - Encrypted compute environments               │
│  - Delegation and constraints                   │
│  - Quota enforcement                            │
└─────────────────────────────────────────────────┘
                    ↕️
┌─────────────────────────────────────────────────┐
│  Layer 2: Cross-Primal Security                 │
│  - Genetic key exchange                         │
│  - Secure sessions                              │
│  - Message encryption                           │
└─────────────────────────────────────────────────┘
                    ↕️
┌─────────────────────────────────────────────────┐
│  Layer 1: Local Security (Always Available)     │
│  - Genetic key generation                       │
│  - Local encryption/decryption                  │
│  - HSM integration                              │
└─────────────────────────────────────────────────┘
```

### Primal-Agnostic Integration Points

```rust
/// BearDog Security Provider (used by ANY primal)
#[async_trait]
pub trait SecurityProvider: Send + Sync {
    /// Create secure session with peer (primal-agnostic)
    async fn create_secure_session(
        &self,
        peer_descriptor: &UniversalServiceDescriptor, // Not "Songbird" specific!
        requirements: &SecurityRequirements,
    ) -> BearDogResult<SecureSession>;
    
    /// Encrypt data for peer (capability-based)
    async fn encrypt_for_peer(
        &self,
        peer_capabilities: &UniversalCapabilityType,
        plaintext: &[u8],
    ) -> BearDogResult<EncryptedData>;
    
    /// Validate peer request (no hardcoded policies)
    async fn validate_request(
        &self,
        request: &SecurityRequest,
        peer_descriptor: &UniversalServiceDescriptor,
    ) -> BearDogResult<SecurityDecision>;
}
```

---

## 🔐 CAPABILITY 1: GENETIC ENCRYPTION FOR SECURE TUNNELS

### Goal

Enable encrypted communication between any two primals using genetic key exchange.

### Requirements

**FR-1.1**: Genetic Key Exchange
- Use genetic algorithms to evolve shared keys
- No pre-shared secrets required
- Fitness-based key quality assurance
- Performance target: <100ms for key exchange

**FR-1.2**: Tunnel Establishment (BSTP)
- Capability-based peer discovery
- Automatic session creation
- Forward secrecy (new keys per session)
- Graceful session renewal

**FR-1.3**: Packet Encryption
- Ultra-fast encryption (target: <1ms overhead)
- Adaptive algorithm selection (AES-GCM, ChaCha20, hybrid)
- Genetic evolution based on performance
- Zero-copy where possible

### Specification

```rust
/// BSTP Security Provider (primal-agnostic)
pub struct BStpSecurityProvider {
    genetics_engine: Arc<GeneticsEngine>,
    crypto_manager: Arc<UniversalCryptoProvider>,
    session_manager: Arc<SessionManager>,
}

impl BStpSecurityProvider {
    /// Create secure tunnel with any peer (no primal names)
    pub async fn create_tunnel(
        &self,
        peer: &UniversalServiceDescriptor,
    ) -> BearDogResult<SecureTunnel> {
        // 1. Exchange genetic material
        let peer_material = peer.genetic_material
            .as_ref()
            .ok_or(BearDogError::missing_genetic_material())?;
        
        let key_exchange = GeneticKeyExchange::new(self.genetics_engine.clone())?;
        let shared_key = key_exchange
            .exchange_with_peer(peer_material, &FitnessRequirements::secure())
            .await?;
        
        // 2. Create session
        let session = SecureSession {
            session_id: Uuid::new_v4().to_string(),
            peer_id: peer.service_id.clone(),
            encryption_key: shared_key.key_material,
            established_at: SystemTime::now(),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
        };
        
        self.session_manager.register_session(session.clone()).await?;
        
        Ok(SecureTunnel {
            session_id: session.session_id,
            peer_descriptor: peer.clone(),
            encrypt: Box::new(move |data| self.encrypt_packet(&session, data)),
            decrypt: Box::new(move |data| self.decrypt_packet(&session, data)),
        })
    }
}
```

### Integration Pattern (Primal-Agnostic)

```rust
// Any primal can use BearDog security (discovered via capability)
let security_primals = discovery
    .discover_by_capability(UniversalCapabilityType::Security)
    .await?;

if let Some(beardog) = security_primals.first() {
    let security_provider = BearDogSecurityProvider::connect(beardog).await?;
    
    // Create tunnel to peer (no hardcoded primal type)
    let tunnel = security_provider
        .create_tunnel(&peer_descriptor)
        .await?;
    
    // Use tunnel for all communication
    let encrypted = tunnel.encrypt(plaintext).await?;
}
```

---

## 🔐 CAPABILITY 2: ENCRYPTED COMPUTE ENVIRONMENTS

### Goal

Enable secure resource sharing where:
- Resource owner can't see workload plaintext
- Workload submitter can't access owner's data
- Quotas enforced automatically
- Both parties maintain privacy

### Requirements

**FR-2.1**: Delegation Keys
- Hierarchical key derivation
- Constraint-based authorization
- Time, resource, and scope limits
- Instant revocation

**FR-2.2**: Workload Encryption
- Submitter encrypts with own key
- Owner executes without decryption
- Results encrypted back to submitter
- Isolated execution environment

**FR-2.3**: Quota Enforcement
- Real-time resource monitoring
- Automatic throttling/termination
- Audit trail generation
- Owner maintains control

### Specification

```rust
/// Delegation Key (for resource sharing)
pub struct DelegationKey {
    pub key_id: String,
    pub parent_key: KeyId,
    pub derived_for: String, // Peer ID (not primal name)
    pub constraints: DelegationConstraints,
    pub genetic_lineage: GeneticLineage,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
}

pub struct DelegationConstraints {
    /// Time windows (e.g., "9-17 weekdays")
    pub time_ranges: Vec<TimeRange>,
    
    /// Resource quotas
    pub cpu_max_percent: u8,
    pub memory_max_bytes: u64,
    pub storage_max_bytes: u64,
    
    /// Scope limitations
    pub allowed_operations: Vec<Operation>,
    pub allowed_namespaces: Vec<String>,
    
    /// Security requirements
    pub encryption_required: bool,
    pub isolation_level: IsolationLevel,
}

/// Compute Security Hooks (used by any compute primal)
#[async_trait]
pub trait ComputeSecurityProvider: Send + Sync {
    /// Validate workload before execution
    async fn validate_workload(
        &self,
        workload: &EncryptedWorkload,
        delegation: &DelegationKey,
    ) -> BearDogResult<ValidationResult>;
    
    /// Monitor and enforce quotas during execution
    async fn enforce_quotas(
        &self,
        delegation: &DelegationKey,
        current_usage: &ResourceUsage,
    ) -> BearDogResult<QuotaDecision>;
    
    /// Encrypt results for submitter
    async fn encrypt_results(
        &self,
        results: &[u8],
        submitter_key: &PublicKey,
    ) -> BearDogResult<EncryptedResults>;
}
```

### Usage Pattern (Primal-Agnostic)

```rust
// Submitter (any primal, any tower)
let workload = prepare_workload().await?;
let encrypted_workload = beardog
    .encrypt_with_key(&my_key, &workload)
    .await?;

// Find compute primal (capability-based, not hardcoded)
let compute_primals = discovery
    .discover_by_capability(UniversalCapabilityType::Compute)
    .await?;

// Submit to available compute (primal-agnostic)
for compute in compute_primals {
    match compute.submit_workload(encrypted_workload, delegation_key).await {
        Ok(submission) => {
            info!("✅ Submitted to: {}", compute.service_id);
            break;
        },
        Err(e) => {
            warn!("❌ Failed {}: {}", compute.service_id, e);
            continue; // Try next available
        }
    }
}
```

---

## 🔐 CAPABILITY 3: CROSS-PRIMAL SECURE MESSAGING

### Goal

Enable encrypted communication between any primals with genetic key exchange.

### Requirements

**FR-3.1**: Message Encryption
- Genetic key exchange per message or session
- Peer discovery via capabilities
- Metadata protection
- Forward secrecy

**FR-3.2**: Session Management
- Automatic session establishment
- Periodic key rotation
- Session lifecycle tracking
- Metrics and monitoring

**FR-3.3**: Graceful Degradation
- Works without other primals (P2P)
- Enhanced when network primals available
- Fallback chains
- Clear error messages

### Specification

```rust
/// Secure Cross-Primal Messenger (already implemented)
pub struct SecureCrossPrimalMessenger {
    discovery_service: Arc<dyn PrimalDiscoveryService>,
    genetics_engine: Arc<GeneticsEngine>,
    crypto_service: Arc<dyn CryptoService>,
    active_sessions: Arc<RwLock<HashMap<String, SecureSession>>>,
}

impl SecureCrossPrimalMessenger {
    /// Send to any primal with capability (no hardcoding)
    pub async fn send_to_capability(
        &self,
        capability: UniversalCapabilityType,
        message: &[u8],
    ) -> BearDogResult<SecurePrimalResponse> {
        // 1. Discover primals with capability
        let primals = self.discovery_service
            .discover_by_capability(capability)
            .await?;
        
        if primals.is_empty() {
            return Err(BearDogError::no_primal_with_capability(capability));
        }
        
        // 2. Try each primal (primal-agnostic)
        for primal in primals {
            match self.send_to_primal(&primal, message).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    warn!("Failed to send to {}: {}", primal.service_id, e);
                    continue;
                }
            }
        }
        
        Err(BearDogError::all_primals_failed(capability))
    }
    
    /// Send to specific primal (discovered, not hardcoded)
    async fn send_to_primal(
        &self,
        primal: &UniversalServiceDescriptor,
        message: &[u8],
    ) -> BearDogResult<SecurePrimalResponse> {
        // Encrypt using genetic key exchange
        let encrypted = self.encrypt_for_primal(message, primal).await?;
        
        // Send via discovery service
        self.discovery_service
            .send_request(primal, encrypted)
            .await
    }
}
```

---

## 📊 IMPLEMENTATION PHASES

### Phase 1: Wire Genetic Encryption (Weeks 1-2)

**Status**: 🟡 In Progress

**Deliverables**:
- [ ] Implement `GeneticKeyExchange` (new module)
- [ ] Wire encryption to `SecureCrossPrimalMessenger`
- [ ] Update service discovery with genetic material
- [ ] CLI commands for testing
- [ ] Integration tests

**Success Criteria**:
- Genetic key exchange completes in <100ms
- Messages actually encrypted (not plaintext)
- Works with any peer (capability-based)
- Tests passing

**Files to Modify**:
- `crates/beardog-genetics/src/genetics/key_exchange.rs` (NEW)
- `crates/beardog-core/src/ecosystem_integration/secure_cross_primal_messaging.rs` (Line 413-444)
- `crates/beardog-adapters/src/universal/primal_runtime_discovery.rs`

---

### Phase 2: BSTP Tunnel Implementation (Weeks 3-4)

**Status**: 📅 Planned

**Deliverables**:
- [ ] Implement `BStpSecurityProvider`
- [ ] Session management
- [ ] Packet encryption/decryption
- [ ] Performance optimization
- [ ] Integration with network primals (capability-based)

**Success Criteria**:
- Tunnel establishment <200ms
- Packet encryption overhead <1ms
- Graceful fallback working
- Works with any network primal

**Files to Create**:
- `crates/beardog-tunnel/src/tunnel/bstp/mod.rs` (NEW)
- `crates/beardog-tunnel/src/tunnel/bstp/security_provider.rs` (NEW)
- `crates/beardog-tunnel/src/tunnel/bstp/session.rs` (NEW)

---

### Phase 3: Compute Security Hooks (Weeks 5-6)

**Status**: 📅 Planned

**Deliverables**:
- [ ] Implement `ComputeSecurityProvider`
- [ ] Delegation key validation
- [ ] Quota enforcement
- [ ] Result encryption
- [ ] Integration with compute primals (capability-based)

**Success Criteria**:
- Validation <10ms
- Quota enforcement real-time
- Complete privacy maintained
- Works with any compute primal

**Files to Create**:
- `crates/beardog-compute-security/` (NEW CRATE)
- `crates/beardog-compute-security/src/delegation.rs`
- `crates/beardog-compute-security/src/validation.rs`
- `crates/beardog-compute-security/src/quota_enforcement.rs`

---

## 🧪 TESTING REQUIREMENTS

### Unit Tests

**Genetic Key Exchange**:
- Key exchange produces consistent results
- Fitness requirements enforced
- Performance benchmarks met
- Error handling

**Encryption**:
- Encrypt/decrypt roundtrip
- Multiple algorithms
- Large data handling
- Error cases

**Delegation**:
- Constraint validation
- Time window checks
- Quota enforcement
- Revocation

### Integration Tests

**Cross-Primal Messaging**:
- Discovery → encryption → delivery
- Multiple primal types
- Fallback chains
- Error recovery

**Tunnel Establishment**:
- Two-tower setup
- Session lifecycle
- Key rotation
- Performance

**Encrypted Compute**:
- Workload submission
- Validation
- Execution
- Result retrieval

### E2E Tests

**Scenario 1**: Friend Lends Compute
```bash
# Setup two towers
tower-a: Your resources
tower-b: Friend's tower

# Create delegation
tower-a: beardog tower delegate --to tower-b --cpu 50%

# Submit workload
tower-b: toadstool submit --encrypted --target tower-a

# Verify privacy
tower-a: Cannot see plaintext
tower-b: Can decrypt results
```

**Scenario 2**: Multi-Primal Communication
```bash
# BearDog discovers network primal
beardog: Discover by capability(Network)

# Create secure tunnel
beardog: Create tunnel to network-primal-X

# Send encrypted message
beardog: Send via tunnel

# Verify encryption
network: Traffic encrypted
```

---

## 📐 SUCCESS METRICS

### Performance

| Metric | Target | Critical |
|--------|--------|----------|
| Genetic Key Exchange | <100ms | <500ms |
| Tunnel Establishment | <200ms | <1s |
| Packet Encryption | <1ms | <5ms |
| Workload Validation | <10ms | <100ms |
| Quota Check | <5ms | <50ms |

### Security

| Metric | Target |
|--------|--------|
| Key Strength | 256-bit entropy minimum |
| Session Lifetime | 1 hour default, configurable |
| Key Rotation | Per session or hourly |
| Audit Trail | 100% coverage |
| Privacy | Zero plaintext exposure |

### Reliability

| Metric | Target |
|--------|--------|
| Graceful Degradation | 100% |
| Fallback Success | >95% |
| Error Recovery | Automatic |
| Session Restore | Transparent |

---

## 🔄 INTEGRATION POINTS

### For Network Primals (Capability: Network)

**What They Get**:
- Secure tunnel establishment
- Packet encryption/decryption
- Session management
- Performance monitoring

**What They Provide**:
- Routing capability
- Endpoint discovery
- Traffic forwarding
- Network topology

**Integration**:
```rust
// Network primal discovers BearDog
let security = discover_by_capability(Security).await?;

// Request secure tunnel
let tunnel = security
    .create_tunnel(&peer_descriptor)
    .await?;

// Use for all traffic
for packet in traffic {
    let encrypted = tunnel.encrypt(packet).await?;
    forward_packet(encrypted).await?;
}
```

---

### For Compute Primals (Capability: Compute)

**What They Get**:
- Workload validation
- Quota enforcement
- Result encryption
- Audit trail

**What They Provide**:
- Compute resources
- Execution environment
- Resource monitoring
- Workload scheduling

**Integration**:
```rust
// Compute primal discovers BearDog
let security = discover_by_capability(Security).await?;

// Validate before execution
let validation = security
    .validate_workload(workload, delegation)
    .await?;

if !validation.approved {
    return Err(WorkloadRejected(validation.reason));
}

// Enforce during execution
loop {
    let usage = monitor_resources().await?;
    let decision = security.enforce_quotas(delegation, usage).await?;
    
    match decision {
        Allow => continue,
        Throttle => adjust_resources(),
        Terminate => kill_workload(),
    }
}
```

---

## 📚 DOCUMENTATION REQUIREMENTS

### Specifications (This Document)

- [x] Architecture overview
- [x] Capability definitions
- [x] Integration patterns
- [x] Success metrics

### Implementation Guides

- [ ] Genetic Key Exchange implementation guide
- [ ] BSTP protocol implementation guide
- [ ] Compute security hooks guide
- [ ] Performance tuning guide

### User Documentation

- [ ] Tower sharing tutorial
- [ ] Delegation patterns guide
- [ ] Security best practices
- [ ] Troubleshooting guide

### API Documentation

- [ ] SecurityProvider trait docs
- [ ] ComputeSecurityProvider trait docs
- [ ] BStpSecurityProvider trait docs
- [ ] CLI command reference

---

## 🎯 TRACKING

**See**: `../../STATUS.md` for current project status.

**Phase Status**:
- Phase 1: ✅ Complete
- Phase 2: 🟡 In Progress
- Phase 3: 📅 Planned

---

## 🔗 RELATED DOCUMENTS

### Specifications
- `SONGBIRD_BEARDOG_VPN_FREE_ARCHITECTURE.md` - Network integration
- `UNIVERSAL_CRYPTO_PROVIDER_ARCHITECTURE.md` - Crypto layer
- `PRIMAL_SOVEREIGNTY_ARCHITECTURE.md` - Sovereignty principles

### Implementation
- `../../crates/beardog-core/src/ecosystem_integration/` - Core implementation
- `../../crates/beardog-genetics/` - Genetic algorithms
- `../../showcase/` - Fossilized (Wave 49); active patterns in `primalSpring/wateringHole/`

---

**Specification Version**: 1.0.0  
**Last Updated**: April 27, 2026  
**Status**: Implementation  
**Primal Sovereignty**: ✅ Fully Compliant (Zero Hardcoding)

🐻 **BearDog: Sovereign Security for Sovereign Primals** 🔐

