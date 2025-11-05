//! Ecosystem Identification - Capability-Based Approach
//! 
//! This module provides ecosystem identification through capability-based discovery
//! rather than hardcoded primal names, supporting true primal sovereignty.

use beardog_types::canonical::capabilities::ServiceCapabilityType;

/// BearDog ecosystem identifier - the only hardcoded identity
/// as BearDog only knows itself per sovereignty principles
pub const BEARDOG: &str = "beardog";

/// Capability-based ecosystem identification
/// Replaces hardcoded primal names with dynamic capability discovery
pub struct CapabilityBasedEcosystemId;

impl CapabilityBasedEcosystemId {
    /// Get ecosystem ID for compute capability providers
    /// Replaces hardcoded TOADSTOOL references
    pub fn compute_capability() -> String {
        ServiceCapabilityType::ComputeIntelligence.as_str().to_string()
    }
    
    /// Get ecosystem ID for service mesh capability providers  
    /// Replaces hardcoded SONGBIRD references
    pub fn mesh_capability() -> String {
        ServiceCapabilityType::ServiceMesh.as_str().to_string()
    }
    
    /// Get ecosystem ID for storage capability providers
    /// Replaces hardcoded NESTGATE references  
    pub fn storage_capability() -> String {
        ServiceCapabilityType::DataStorage.as_str().to_string()
    }
    
    /// Get ecosystem ID for AI capability providers
    /// Replaces hardcoded SQUIRREL references
    pub fn ai_capability() -> String {
        ServiceCapabilityType::DistributedIntelligence.as_str().to_string()
    }
    
    /// Generate dynamic ecosystem ID from capability type
    /// This is the modern approach for ecosystem identification
    pub fn from_capability(capability: ServiceCapabilityType) -> String {
        format!("{}-provider", capability.as_str())
    }
}

// Legacy ecosystem IDs removed - Use CapabilityBasedEcosystemId methods instead

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn test_capability_based_ids() {
        // Test modern capability-based approach
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        assert_eq!(
            CapabilityBasedEcosystemId::compute_capability(),"compute-intelligence"
        );
        
        assert_eq!(
            CapabilityBasedEcosystemId::mesh_capability(),"service-mesh"
        );
        
        assert_eq!(
            CapabilityBasedEcosystemId::storage_capability(),"data-storage"
        );
        
        assert_eq!(
            CapabilityBasedEcosystemId::ai_capability(),"distributed-intelligence"
        );
    }
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[test]
    async fn test_dynamic_id_generation() {
        let compute_id = CapabilityBasedEcosystemId::from_capability(
            ServiceCapabilityType::ComputeIntelligence
        );
        assert!(compute_id.contains("compute"));
        assert!(compute_id.contains("provider"));
    }
} 