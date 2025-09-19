

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
    /// The region value
    pub region: String,
    /// Mapping of credentials
    pub credentials: HashMap<String, String>,
    pub endpoint_overrides: HashMap<CloudServiceType, String>,
}

#[derive(Debug, Clone)]
    pub response_time_ms: f64,
    /// The last check value
    pub last_check: chrono::DateTime<chrono::Utc>,
    /// Number of error
    pub error_count: u64,
    /// Current status of the services
    pub services_status: HashMap<CloudServiceType, ServiceStatus>,
}

#[derive(Debug, Clone)]
    pub response_time_ms: Option<f64>,
    /// Optional error message
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
    /// The key usage value
    pub key_usage: KeyUsage,
    /// The key spec value
    pub key_spec: String,
    /// Optional description
    pub description: Option<String>,
    /// Mapping of tags
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    /// Optional arn
    pub arn: Option<String>,
    /// The state value
    pub state: KeyState,
    /// The creation date value
    pub creation_date: chrono::DateTime<chrono::Utc>,
    /// Optional description
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
    pub image_id: String,
    pub subnet_id: Option<String>,
    /// Collection of security groups
    pub security_groups: Vec<String>,
    /// Optional user data
    pub user_data: Option<String>,
    /// Mapping of tags
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    /// The instance type value
    pub instance_type: String,
    /// The state value
    pub state: InstanceState,
    /// Optional public ip
    pub public_ip: Option<String>,
    /// Optional private ip
    pub private_ip: Option<String>,
    pub launch_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
    /// Current status of the component_checks
    pub status_checks: HashMap<String, String>,
    /// Current status of the system
    pub system_status: String,
    /// Current status of the instance
    pub instance_status: String,
}

#[derive(Debug, Clone)]
    /// Whether versioning is enabled
    pub versioning: bool,
    /// Optional lifecycle policy
    pub lifecycle_policy: Option<String>,
    /// Optional access policy
    pub access_policy: Option<String>,
    /// Mapping of tags
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    /// Name of the enable_dns_hostitems
    pub enable_dns_hostnames: bool,
    /// Whether enable_dns_support is enabled
    pub enable_dns_support: bool,
    /// Mapping of tags
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    /// Optional availability zone
    pub availability_zone: Option<String>,
    /// Whether map_public_ip is enabled
    pub map_public_ip: bool,
    /// Mapping of tags
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    /// The port range value
    pub port_range: String,
    /// The source value
    pub source: String,
    /// Optional description
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
    /// The scheme value
    pub scheme: String,
    /// Collection of subnets
    pub subnets: Vec<String>,
    /// Collection of security groups
    pub security_groups: Vec<String>,
    /// Mapping of tags
    pub tags: HashMap<String, String>,
}
