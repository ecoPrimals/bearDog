// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Node Constants
///
/// **CANONICAL NODE-RELATED CONSTANTS**
/// Node types, federation settings, and node management parameters.

use std::time::Duration;
/// **CANONICAL NODE TYPE CONSTANTS** - Eliminates node type duplicates
/// Consolidates node type constants from:
/// - `beardog-node-registry/src/node_registry/types/node.rs`
/// - `beardog-node-registry/src/node_registry/types/federation.rs`
pub mod node_types {
    /// Security node type
    pub use crate::constants::unified::nodes::SECURITY;
    /// Phonebook node type
    pub use crate::constants::unified::nodes::PHONEBOOK;
    /// Federation node type
    pub use crate::constants::unified::nodes::FEDERATION;
    /// Compute node type
    pub const COMPUTE: &str = "compute";
    /// Storage node type
    pub const STORAGE: &str = "storage";
    /// Relay node type
    pub const RELAY: &str = "relay";
    /// Backup node type
    pub const BACKUP: &str = "backup";
    /// Monitoring node type
    pub const MONITORING: &str = "monitoring";
    /// Analytics node type
    pub const ANALYTICS: &str = "analytics";
    /// Gateway node type
    pub const GATEWAY: &str = "gateway";
    /// All supported node types
    pub const ALL_NODE_TYPES: &[&str] = &[
        SECURITY, PHONEBOOK, FEDERATION, COMPUTE, STORAGE, RELAY, BACKUP, MONITORING, ANALYTICS,
        GATEWAY,
    ];
}
/// **CANONICAL NODE REGISTRY CONSTANTS** - Node discovery and registration
pub mod registry {};


    use super::Duration;
    /// Default node registration timeout
    pub const REGISTRATION_TIMEOUT: Duration = Duration::from_secs(30);
    /// Node heartbeat interval
    pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
    /// Node health check interval
    pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);
    /// Node discovery interval
    pub const DISCOVERY_INTERVAL: Duration = Duration::from_secs(120); // 2 minutes
    /// Maximum nodes per registry
    pub const MAX_NODES_PER_REGISTRY: usize = 10000;
    /// Node registration retry attempts
    pub const REGISTRATION_RETRY_ATTEMPTS: u32 = 3;
    /// Node timeout before marked as unavailable
    pub const NODE_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes
    /// Registry sync interval between nodes
    pub const REGISTRY_SYNC_INTERVAL: Duration = Duration::from_secs(600); // 10 minutes
    /// Maximum node metadata size (in bytes)
    pub const MAX_NODE_METADATA_SIZE: usize = 4096; // 4KB
    /// Node capability cache TTL
    pub const CAPABILITY_CACHE_TTL: Duration = Duration::from_secs(3600); // 1 hour
    /// Registry backup interval
    pub const REGISTRY_BACKUP_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour
    /// Maximum concurrent node operations
    pub const MAX_CONCURRENT_NODE_OPERATIONS: usize = 100;
/// **CANONICAL NODE STATUS CONSTANTS** - Node operational states
pub mod status {
    /// Node operational states
    pub const STATUS_ACTIVE: &str = "active";
    pub const STATUS_INACTIVE: &str = "inactive";
    pub const STATUS_STARTING: &str = "starting";
    pub const STATUS_STOPPING: &str = "stopping";
    pub const STATUS_MAINTENANCE: &str = "maintenance";
    pub const STATUS_ERROR: &str = "error";
    pub const STATUS_UNKNOWN: &str = "unknown";
    /// Node availability states
    pub const AVAILABILITY_AVAILABLE: &str = "available";
    pub const AVAILABILITY_BUSY: &str = "busy";
    pub const AVAILABILITY_OVERLOADED: &str = "overloaded";
    pub const AVAILABILITY_UNREACHABLE: &str = "unreachable";
    /// Node health states
    pub const HEALTH_HEALTHY: &str = "healthy";
    pub const HEALTH_DEGRADED: &str = "degraded";
    pub const HEALTH_UNHEALTHY: &str = "unhealthy";
    pub const HEALTH_CRITICAL: &str = "critical";
    /// Node role states
    pub const ROLE_PRIMARY: &str = "primary";
    pub const ROLE_SECONDARY: &str = "secondary";
    pub const ROLE_REPLICA: &str = "replica";
    pub const ROLE_STANDBY: &str = "standby";
/// **CANONICAL FEDERATION CONSTANTS** - Distributed system coordination
pub mod federation {
    /// Maximum federation size
    pub const MAX_FEDERATION_SIZE: usize = 1000;
    /// Federation consensus timeout
    pub const CONSENSUS_TIMEOUT: Duration = Duration::from_secs(30);
    /// Federation election timeout
    pub const ELECTION_TIMEOUT: Duration = Duration::from_secs(60);
    /// Minimum federation quorum size
    pub const MIN_QUORUM_SIZE: usize = 3;
    /// Maximum federation cluster depth
    pub const MAX_CLUSTER_DEPTH: u32 = 10;
    /// Federation protocol version
    pub const PROTOCOL_VERSION: u32 = 1;
    /// Cross-federation communication timeout
    pub const CROSS_FEDERATION_TIMEOUT: Duration = Duration::from_secs(120); // 2 minutes
    /// Federation metadata sync interval
    pub const METADATA_SYNC_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes
    /// Maximum federation message size (in bytes)
    pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1MB
    /// Federation peer discovery timeout
    pub const PEER_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(60);
    /// Maximum pending federation requests
    pub const MAX_PENDING_REQUESTS: usize = 1000;
