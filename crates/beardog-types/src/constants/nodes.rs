

use std::time::Duration;

pub mod node_types {

    pub use crate::constants::unified::nodes::SECURITY;

    pub use crate::constants::unified::nodes::PHONEBOOK;

    pub use crate::constants::unified::nodes::FEDERATION;

    pub const COMPUTE: &str = "compute";

    pub const STORAGE: &str = "storage";

    pub const RELAY: &str = "relay";

    pub const BACKUP: &str = "backup";

    pub const MONITORING: &str = "monitoring";

    pub const ANALYTICS: &str = "analytics";

    pub const GATEWAY: &str = "gateway";

    pub const ALL_NODE_TYPES: &[&str] = &[
        SECURITY, PHONEBOOK, FEDERATION, COMPUTE, STORAGE, RELAY, BACKUP, MONITORING, ANALYTICS,
        GATEWAY,
    ];
}

pub mod registry {};

    use super::Duration;

    pub const REGISTRATION_TIMEOUT: Duration = Duration::from_secs(30);

    pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);

    pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);

    pub const DISCOVERY_INTERVAL: Duration = Duration::from_secs(120); // 2 minutes

    pub const MAX_NODES_PER_REGISTRY: usize = 10000;

    pub const REGISTRATION_RETRY_ATTEMPTS: u32 = 3;

    pub const NODE_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes

    pub const REGISTRY_SYNC_INTERVAL: Duration = Duration::from_secs(600); // 10 minutes

    pub const MAX_NODE_METADATA_SIZE: usize = 4096; // 4KB

    pub const CAPABILITY_CACHE_TTL: Duration = Duration::from_secs(3600); // 1 hour

    pub const REGISTRY_BACKUP_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour

    pub const MAX_CONCURRENT_NODE_OPERATIONS: usize = 100;

pub mod status {

    pub const STATUS_ACTIVE: &str = "active";
    pub const STATUS_INACTIVE: &str = "inactive";
    pub const STATUS_STARTING: &str = "starting";
    pub const STATUS_STOPPING: &str = "stopping";
    pub const STATUS_MAINTENANCE: &str = "maintenance";
    pub const STATUS_ERROR: &str = "error";
    pub const STATUS_UNKNOWN: &str = "unknown";

    pub const AVAILABILITY_AVAILABLE: &str = "available";
    pub const AVAILABILITY_BUSY: &str = "busy";
    pub const AVAILABILITY_OVERLOADED: &str = "overloaded";
    pub const AVAILABILITY_UNREACHABLE: &str = "unreachable";

    pub const HEALTH_HEALTHY: &str = "healthy";
    pub const HEALTH_DEGRADED: &str = "degraded";
    pub const HEALTH_UNHEALTHY: &str = "unhealthy";
    pub const HEALTH_CRITICAL: &str = "critical";

    pub const ROLE_PRIMARY: &str = "primary";
    pub const ROLE_SECONDARY: &str = "secondary";
    pub const ROLE_REPLICA: &str = "replica";
    pub const ROLE_STANDBY: &str = "standby";

pub mod federation {

    pub const MAX_FEDERATION_SIZE: usize = 1000;

    pub const CONSENSUS_TIMEOUT: Duration = Duration::from_secs(30);

    pub const ELECTION_TIMEOUT: Duration = Duration::from_secs(60);

    pub const MIN_QUORUM_SIZE: usize = 3;

    pub const MAX_CLUSTER_DEPTH: u32 = 10;

    pub const PROTOCOL_VERSION: u32 = 1;

    pub const CROSS_FEDERATION_TIMEOUT: Duration = Duration::from_secs(120); // 2 minutes

    pub const METADATA_SYNC_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes

    pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1MB

    pub const PEER_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(60);

    pub const MAX_PENDING_REQUESTS: usize = 1000;
