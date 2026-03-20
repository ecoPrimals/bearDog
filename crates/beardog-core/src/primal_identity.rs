// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Primal Self-Identity - Zero Configuration Self-Discovery
//!
//! This module implements the core philosophy: "Primals only have self-knowledge
//! and discover other primals at runtime."
//!
//! ## Philosophy
//!
//! A primal should be able to:
//! 1. Discover its own identity without any hardcoded configuration
//! 2. Announce itself to the network
//! 3. Discover peer primals dynamically
//! 4. Establish trust relationships based on cryptographic verification
//!
//! ## Zero Configuration
//!
//! - No hardcoded primal addresses
//! - No hardcoded peer lists
//! - No centralized registry
//! - Pure runtime discovery

use beardog_errors::BearDogError;
use beardog_types::canonical::identity::PrimalId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::SystemTime;
use tracing::{debug, info};

/// Primal's self-discovered identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfIdentity {
    /// Unique primal ID (hardware-derived, not configured)
    pub id: PrimalId,
    
    /// Capabilities discovered from local hardware
    pub capabilities: DiscoveredCapabilities,
    
    /// Network interfaces discovered at runtime
    pub network_info: RuntimeNetworkInfo,
    
    /// When this identity was discovered
    pub discovered_at: SystemTime,
    
    /// How the identity was derived
    pub derivation_method: IdentityDerivation,
}

/// How the primal identity was derived
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityDerivation {
    /// From hardware security module (best)
    HardwareSecurity { hsm_type: String },
    
    /// From TPM or secure enclave
    TrustedPlatformModule,
    
    /// From system entropy + machine ID
    SystemDerived,
    
    /// Ephemeral (for testing only)
    Ephemeral,
}

/// Capabilities discovered from local system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredCapabilities {
    /// Cryptographic capabilities
    pub crypto: CryptoCapabilities,
    
    /// Storage capabilities
    pub storage: StorageCapabilities,
    
    /// Compute capabilities
    pub compute: ComputeCapabilities,
    
    /// Network capabilities
    pub network: NetworkCapabilities,
}

/// Cryptographic facilities exposed by the local platform (HSM, OS RNG, post-quantum readiness).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoCapabilities {
    /// True when a hardware or OS-backed high-entropy source is available.
    pub hardware_entropy: bool,
    /// True when secure hardware or OS APIs can perform signing without exposing raw keys.
    pub hardware_signing: bool,
    /// True when keys can be stored in an encrypted or hardware-backed vault.
    pub secure_key_storage: bool,
    /// True when PQ-resistant algorithms are advertised as available for new material.
    pub quantum_resistant: bool,
}

/// Local storage characteristics used for capability negotiation and placement decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapabilities {
    /// Best-effort free space estimate for primal-owned storage, in bytes.
    pub available_bytes: u64,
    /// Encrypted-at-rest storage is available.
    pub encrypted_storage: bool,
    /// Fast media suitable for hot metadata (e.g. NVMe) is available.
    pub fast_storage: bool,
}

/// CPU and trusted-execution features discovered at runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeCapabilities {
    /// Logical CPU cores visible to the process.
    pub cpu_cores: usize,
    /// SIMD extensions usable for bulk crypto or vector workloads.
    pub simd_support: bool,
    /// TEE, SGX, SEV, or similar confidential execution is present.
    pub trusted_execution: bool,
}

/// L3/L4 reachability and discovery features for this primal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCapabilities {
    /// IPv4 stack usable for binding or outbound traffic.
    pub ipv4: bool,
    /// IPv6 stack usable for binding or outbound traffic.
    pub ipv6: bool,
    /// mDNS / multicast DNS discovery can be used on this host.
    pub mdns: bool,
    /// UPnP / NAT-PMP style port mapping may be available.
    pub upnp: bool,
}

/// Runtime-discovered network information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeNetworkInfo {
    /// All discovered network interfaces
    pub interfaces: Vec<NetworkInterface>,
    
    /// Preferred interface for primal communication
    pub preferred_interface: Option<String>,
    
    /// Dynamically discovered or assigned ports
    pub ports: HashMap<String, u16>,
}

/// A single NIC or tunnel interface discovered during runtime network enumeration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// OS-reported interface name (e.g. `eth0`, `wlan0`).
    pub name: String,
    /// All addresses currently assigned to this interface.
    pub addresses: Vec<IpAddr>,
    /// Interface is administratively up and able to pass traffic.
    pub is_up: bool,
    /// Loopback or host-only interface (typically excluded from public announcement).
    pub is_loopback: bool,
}

/// Discover primal's own identity without any configuration
///
/// This is the foundation of zero-configuration operation.
/// Everything is discovered at runtime.
///
/// # Example
/// ```rust,no_run
/// use beardog_core::primal_identity::discover_self_identity;
///
/// # async fn example() -> Result<(), beardog_errors::BearDogError> {
/// // No configuration needed - pure discovery
/// let identity = discover_self_identity().await?;
/// 
/// println!("I am primal: {}", identity.id);
/// println!("My capabilities: {:?}", identity.capabilities);
/// println!("My network: {:?}", identity.network_info);
/// # Ok(())
/// # }
/// ```
pub async fn discover_self_identity() -> Result<SelfIdentity, BearDogError> {
    info!("Discovering primal self-identity...");
    
    // 1. Derive primal ID from hardware
    let (id, derivation_method) = derive_primal_id().await?;
    debug!("Derived primal ID: {} via {:?}", id, derivation_method);
    
    // 2. Discover local capabilities
    let capabilities = discover_capabilities().await?;
    debug!("Discovered capabilities: {:?}", capabilities);
    
    // 3. Discover network information
    let network_info = discover_network_info().await?;
    debug!("Discovered network info: {} interfaces", network_info.interfaces.len());
    
    let identity = SelfIdentity {
        id,
        capabilities,
        network_info,
        discovered_at: SystemTime::now(),
        derivation_method,
    };
    
    info!("Self-identity discovery complete: {}", identity.id);
    Ok(identity)
}

/// Derive primal ID from hardware entropy
async fn derive_primal_id() -> Result<(PrimalId, IdentityDerivation), BearDogError> {
    // Try hardware security first (best)
    if let Ok((id, method)) = try_hardware_derived_id().await {
        return Ok((id, method));
    }
    
    // Try TPM/secure enclave
    if let Ok((id, method)) = try_tpm_derived_id().await {
        return Ok((id, method));
    }
    
    // Fall back to system-derived (still secure, just not hardware-backed)
    try_system_derived_id().await
}

/// Try to derive ID from hardware security module
async fn try_hardware_derived_id() -> Result<(PrimalId, IdentityDerivation), BearDogError> {
    // This would integrate with beardog-tunnel's HSM system
    // For now, return error to try next method
    Err(BearDogError::not_found("Hardware HSM not available".to_string()))
}

/// Try to derive ID from TPM
async fn try_tpm_derived_id() -> Result<(PrimalId, IdentityDerivation), BearDogError> {
    // TPM integration would go here
    // For now, return error to try next method
    Err(BearDogError::not_found("TPM not available".to_string()))
}

/// Derive ID from system entropy and machine characteristics
async fn try_system_derived_id() -> Result<(PrimalId, IdentityDerivation), BearDogError> {
    use sha3::{Digest, Sha3_256};
    
    let mut hasher = Sha3_256::new();
    
    // Gather system-specific entropy
    
    // 1. Machine ID (if available)
    if let Ok(machine_id) = std::fs::read_to_string("/etc/machine-id") {
        hasher.update(machine_id.as_bytes());
    } else if let Ok(machine_id) = std::fs::read_to_string("/var/lib/dbus/machine-id") {
        hasher.update(machine_id.as_bytes());
    }
    
    // 2. Hostname
    if let Ok(hostname) = hostname::get() {
        hasher.update(hostname.as_encoded_bytes());
    }
    
    // 3. OS-provided entropy
    let mut entropy = vec![0u8; 32];
    getrandom::getrandom(&mut entropy)
        .map_err(|e| BearDogError::system(format!("Failed to get entropy: {}", e)))?;
    hasher.update(&entropy);
    
    // 4. Process context
    hasher.update(&std::process::id().to_le_bytes());
    
    // 5. Timestamp for uniqueness
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| BearDogError::system(format!("System time error: {}", e)))?
        .as_nanos();
    hasher.update(&timestamp.to_le_bytes());
    
    let hash = hasher.finalize();
    let id_bytes: [u8; 32] = hash.into();
    
    let id = PrimalId::from_bytes(&id_bytes);
    
    Ok((id, IdentityDerivation::SystemDerived))
}

/// Discover local system capabilities
async fn discover_capabilities() -> Result<DiscoveredCapabilities, BearDogError> {
    Ok(DiscoveredCapabilities {
        crypto: discover_crypto_capabilities().await?,
        storage: discover_storage_capabilities().await?,
        compute: discover_compute_capabilities().await?,
        network: discover_network_capabilities().await?,
    })
}

async fn discover_crypto_capabilities() -> Result<CryptoCapabilities, BearDogError> {
    // Check for hardware entropy sources
    let hardware_entropy = std::path::Path::new("/dev/hwrng").exists() ||
                          std::path::Path::new("/dev/random").exists();
    
    // Check for hardware security modules
    let hardware_signing = false; // Would check for actual HSM
    let secure_key_storage = false; // Would check for keyring/keychain
    
    // Quantum-resistant algorithms are available in software
    let quantum_resistant = true;
    
    Ok(CryptoCapabilities {
        hardware_entropy,
        hardware_signing,
        secure_key_storage,
        quantum_resistant,
    })
}

async fn discover_storage_capabilities() -> Result<StorageCapabilities, BearDogError> {
    // Get available disk space
    let available_bytes = 1_000_000_000; // Would use actual disk space check
    
    // Check if we can use encrypted storage
    let encrypted_storage = true; // Software encryption always available
    
    // Check for SSD vs HDD
    let fast_storage = true; // Would check actual storage type
    
    Ok(StorageCapabilities {
        available_bytes,
        encrypted_storage,
        fast_storage,
    })
}

async fn discover_compute_capabilities() -> Result<ComputeCapabilities, BearDogError> {
    let cpu_cores = num_cpus::get();
    
    // Check for SIMD support
    let simd_support = cfg!(target_feature = "avx2") || 
                      cfg!(target_feature = "neon");
    
    // Check for trusted execution (SGX, TrustZone, etc.)
    let trusted_execution = false; // Would check for actual TEE
    
    Ok(ComputeCapabilities {
        cpu_cores,
        simd_support,
        trusted_execution,
    })
}

async fn discover_network_capabilities() -> Result<NetworkCapabilities, BearDogError> {
    Ok(NetworkCapabilities {
        ipv4: true,  // Almost always available
        ipv6: true,  // Commonly available
        mdns: true,  // Available via software
        upnp: false, // Would check for actual UPnP support
    })
}

/// Discover network interfaces and addresses
async fn discover_network_info() -> Result<RuntimeNetworkInfo, BearDogError> {
    use beardog_utils::network::discover_port;
    
    let mut interfaces = Vec::new();
    let mut preferred_interface = None;
    
    // Discover all network interfaces
    for iface in get_if_addrs::get_if_addrs()
        .map_err(|e| BearDogError::network_error(&format!("Failed to get interfaces: {}", e)))?
    {
        let interface = NetworkInterface {
            name: iface.name.clone(),
            addresses: vec![iface.addr.ip()],
            is_up: true, // get_if_addrs only returns up interfaces
            is_loopback: iface.addr.ip().is_loopback(),
        };
        
        // Prefer non-loopback interfaces
        if !interface.is_loopback && preferred_interface.is_none() {
            preferred_interface = Some(iface.name);
        }
        
        interfaces.push(interface);
    }
    
    // Dynamically discover ports for services
    let mut ports = HashMap::new();
    
    // Discover API port (no hardcoding!)
    if let Ok(port_info) = discover_port("api", &Default::default()).await {
        ports.insert("api".to_string(), port_info.port);
    }
    
    // Discover discovery service port
    if let Ok(port_info) = discover_port("discovery", &Default::default()).await {
        ports.insert("discovery".to_string(), port_info.port);
    }
    
    Ok(RuntimeNetworkInfo {
        interfaces,
        preferred_interface,
        ports,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_self_identity_discovery() {
        // Should be able to discover identity without any configuration
        let result = discover_self_identity().await;
        assert!(result.is_ok(), "Self-identity discovery should succeed");
        
        let identity = result.unwrap();
        assert!(!identity.id.to_string().is_empty(), "Should have valid ID");
        assert!(identity.capabilities.compute.cpu_cores > 0, "Should detect CPU cores");
    }

    #[tokio::test]
    async fn test_capability_discovery() {
        let result = discover_capabilities().await;
        assert!(result.is_ok());
        
        let caps = result.unwrap();
        assert!(caps.compute.cpu_cores > 0);
        assert!(caps.crypto.quantum_resistant);
    }

    #[tokio::test]
    async fn test_network_discovery() {
        let result = discover_network_info().await;
        assert!(result.is_ok());
        
        let network = result.unwrap();
        assert!(!network.interfaces.is_empty(), "Should find at least one interface");
    }

    #[tokio::test]
    async fn test_primal_id_derivation() {
        let result = derive_primal_id().await;
        assert!(result.is_ok());
        
        let (id, method) = result.unwrap();
        assert!(!id.to_string().is_empty());
        // Should at least fall back to system-derived
        assert!(matches!(method, IdentityDerivation::SystemDerived | 
                                 IdentityDerivation::TrustedPlatformModule |
                                 IdentityDerivation::HardwareSecurity { .. }));
    }
}

