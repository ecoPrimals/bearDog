

use super::providers::CloudProvider;

#[derive(Debug, Clone)]
    pub primary_provider: CloudProvider,


    pub failover_providers: Vec<CloudProvider>,


    pub connection_timeout: u64,


    pub retry_config: RetryConfig,


    pub security_config: CloudSecurityConfig,
}

#[derive(Debug, Clone)]
    /// Number of base_delay_ms
    pub base_delay_ms: u64,

    /// Number of max_delay_ms
    pub max_delay_ms: u64,

    /// The backoff multiplier value
    pub backoff_multiplier: f64,
}

#[derive(Debug, Clone)]
    /// Whether encrypt_at_rest is enabled
    pub encrypt_at_rest: bool,

    /// Whether require_mfa is enabled
    pub require_mfa: bool,

    /// Whether audit_logging is enabled
    pub audit_logging: bool,

    /// Whether network_isolation is enabled
    pub network_isolation: bool,
}
