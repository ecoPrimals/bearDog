//! Runtime Configuration Management
//!
//! This module contains the runtime logic for loading and managing production configurations.

use super::*;
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use tracing::{debug, info, warn};

impl ProductionConfigManager {
    /// Creates a new production configuration manager
    pub fn new(environment: Environment) -> BearDogResult<Self> {
        let config_sources = Self::determine_config_sources(&environment)?;
        let secrets_manager = SecretsManager::new(&environment)?;
        
        Ok(Self {
            environment,
            config_sources,
            secrets_manager,
            config_cache: HashMap::with_capacity(16),
        })
    }

    /// Loads complete production configuration
    pub fn load_configuration(&mut self) -> BearDogResult<ProductionConfig> {
        info!("Loading production configuration for environment: {:?}", self.environment);

        let mut config = self.load_base_config()?;
        self.apply_environment_overrides(&mut config)?;
        self.inject_secrets(&mut config)?;
        self.validate_configuration(&config)?;
        
        info!("Production configuration loaded successfully");
        Ok(config)
    }

    /// Loads base configuration from sources
    fn load_base_config(&mut self) -> BearDogResult<ProductionConfig> {
        let mut config = ProductionConfig::default();

        for source in &self.config_sources {
            match source {
                ConfigSource::Environment => {
                    self.load_from_environment(&mut config)?;
                }
                ConfigSource::File { path } => {
                    self.load_from_file(&mut config, path)?;
                }
                ConfigSource::UniversalSecretsManagement { endpoint, provider_type, auth_config } => {
                    self.load_from_universal_secrets(&mut config, endpoint, provider_type, auth_config)?;
                }
                ConfigSource::UniversalContainerSecrets { namespace, provider_type } => {
                    self.load_from_universal_container_secrets(&mut config, namespace, provider_type)?;
                }
            }
        }
        
        Ok(config)
    }

    /// Loads configuration from file
    fn load_from_file(&mut self, config: &mut ProductionConfig, path: &str) -> BearDogResult<()> {
        debug!("Loading configuration from file: {}", path);
        
        if !Path::new(path).exists() {
            warn!("Configuration file not found: {}", path);
            return Ok(());
        }
        
        let content = fs::read_to_string(path)
            .map_err(|e| BearDogError::system(format!("Failed to read config file {}: {}", path, e)))?;
        
        let file_config: ProductionConfig = if path.ends_with(".toml") {
            toml::from_str(&content)
                .map_err(|e| BearDogError::validation(format!("Invalid TOML config: {}", e)))?
        } else if path.ends_with(".yaml") || path.ends_with(".yml") {
            serde_yaml::from_str(&content)
                .map_err(|e| BearDogError::validation(format!("Invalid YAML config: {}", e)))?
        } else {
            serde_json::from_str(&content)
                .map_err(|e| BearDogError::validation(format!("Invalid JSON config: {}", e)))?
        };

        self.merge_configs(config, file_config);
        
        Ok(())
    }

    /// Loads configuration from environment variables
    fn load_from_environment(&self, config: &mut ProductionConfig) -> BearDogResult<()> {
        debug!("Loading configuration from environment variables");

        if let Ok(port) = env::var("BEARDOG_PORT") {
            config.application.port = port.parse()
                .map_err(|_| BearDogError::validation("Invalid BEARDOG_PORT value"))?;
        }
        
        if let Ok(bind_address) = env::var("BEARDOG_BIND_ADDRESS") {
            config.application.bind_address = bind_address;
        }
        
        if let Ok(worker_threads) = env::var("BEARDOG_WORKER_THREADS") {
            config.application.worker_threads = worker_threads.parse()
                .map_err(|_| BearDogError::validation("Invalid BEARDOG_WORKER_THREADS value"))?;
        }

        if let Ok(db_host) = env::var("BEARDOG_DB_HOST") {
            config.database.primary.host = db_host;
        }
        
        if let Ok(db_port) = env::var("BEARDOG_DB_PORT") {
            config.database.primary.port = db_port.parse()
                .map_err(|_| BearDogError::validation("Invalid BEARDOG_DB_PORT value"))?;
        }

        if let Ok(log_level) = env::var("BEARDOG_LOG_LEVEL") {
            config.logging.level = log_level;
        }
        
        Ok(())
    }

    /// Loads configuration from universal secrets management
    fn load_from_universal_secrets(
        &self,
        _config: &mut ProductionConfig,
        endpoint: &str,
        provider_type: &str,
        _auth_config: &HashMap<String, String>,
    ) -> BearDogResult<()> {
        info!("🔐 Loading configuration from universal secrets management: {} ({})", endpoint, provider_type);
        
        if self.has_secrets_capability(provider_type) {
            info!("🔐 Using capability-based secrets provider: {}", provider_type);
            Ok(())
        } else {
            warn!("Provider {} does not support required secrets capabilities, using environment fallback", provider_type);
            Ok(())
        }
    }

    /// Loads configuration from universal container secrets
    fn load_from_universal_container_secrets(
        &self,
        _config: &mut ProductionConfig,
        namespace: &str,
        provider_type: &str,
    ) -> BearDogResult<()> {
        info!("📦 Loading configuration from universal container secrets: {} ({})", namespace, provider_type);
        
        match provider_type {
            "kubernetes" | "docker_swarm" | "nomad" => {
                info!("📦 Using environment fallback for container secrets provider: {}", provider_type);
                Ok(())
            }
            _ => {
                warn!("Unknown container provider type: {}, using environment fallback", provider_type);
                Ok(())
            }
        }
    }

    /// Injects secrets into configuration
    fn inject_secrets(&mut self, config: &mut ProductionConfig) -> BearDogResult<()> {
        info!("Injecting secrets into configuration");

        if let Ok(password) = self.secrets_manager.get_secret("database/password") {
            config.database.primary.password = password.value;
        }

        if let Ok(cert) = self.secrets_manager.get_secret("tls/certificate") {
            config.networking.tls.cert_path = cert.value;
        }
        
        if let Ok(key) = self.secrets_manager.get_secret("tls/private_key") {
            config.networking.tls.key_path = key.value;
        }
        
        Ok(())
    }

    /// Validates production configuration
    fn validate_configuration(&self, config: &ProductionConfig) -> BearDogResult<()> {
        debug!("Validating production configuration");

        if config.application.port == 0 {
            return Err(BearDogError::validation("Application port cannot be 0"));
        }
        
        if config.application.worker_threads == 0 {
            return Err(BearDogError::validation("Worker threads must be greater than 0"));
        }

        if config.database.primary.host.is_empty() {
            return Err(BearDogError::validation("Database host cannot be empty"));
        }
        
        if config.database.primary.database.is_empty() {
            return Err(BearDogError::validation("Database name cannot be empty"));
        }

        if self.environment == Environment::Production {
            if !config.networking.tls.enabled {
                return Err(BearDogError::validation("TLS must be enabled in production"));
            }
            
            if !config.compliance.encryption_at_rest {
                return Err(BearDogError::validation("Encryption at rest must be enabled in production"));
            }
            
            if !config.compliance.access_logging {
                return Err(BearDogError::validation("Access logging must be enabled in production"));
            }
        }
        
        info!("Configuration validation passed");
        Ok(())
    }

    /// Determines configuration sources based on environment
    fn determine_config_sources(environment: &Environment) -> BearDogResult<Vec<ConfigSource>> {
        let mut sources = vec![ConfigSource::Environment];

        let config_file = match environment {
            Environment::Development => "configs/environments/development.toml",
            Environment::Staging => "configs/environments/staging.toml", 
            Environment::Production => "configs/environments/production.toml",
        };
        
        sources.push(ConfigSource::File {
            path: config_file.to_string(),
        });

        if matches!(environment, Environment::Production) {
            if let Ok(vault_endpoint) = env::var("VAULT_ENDPOINT") {
                if let Ok(vault_token) = env::var("VAULT_TOKEN") {
                    sources.push(ConfigSource::UniversalSecretsManagement {
                        endpoint: vault_endpoint,
                        provider_type: "HashiCorpVault".to_string(),
                        auth_config: {
                            let mut config = HashMap::new();
                            config.insert("token".to_string(), vault_token);
                            config
                        },
                    });
                }
            }
            
            if env::var("KUBERNETES_SERVICE_HOST").is_ok() {
                sources.push(ConfigSource::UniversalContainerSecrets {
                    namespace: env::var("KUBERNETES_NAMESPACE")
                        .unwrap_or_else(|_| "default".to_string()),
                    provider_type: "kubernetes".to_string(),
                });
            }
        }
        
        Ok(sources)
    }

    /// Applies environment-specific overrides
    fn apply_environment_overrides(&self, _config: &mut ProductionConfig) -> BearDogResult<()> {
        Ok(())
    }

    /// Checks if provider supports secrets management capabilities
    fn has_secrets_capability(&self, provider_type: &str) -> bool {
        let provider_lower = provider_type.to_lowercase();
        provider_lower.contains("secrets") 
            || provider_lower.contains("vault") 
            || provider_lower.contains("kms") 
            || provider_lower.contains("key")
    }

    /// Merges file configuration into base configuration
    fn merge_configs(&self, _base: &mut ProductionConfig, _file: ProductionConfig) {
        // TODO: Implement selective merging logic
    }
}

impl SecretsManager {
    /// Creates a new secrets manager
    pub fn new(environment: &Environment) -> BearDogResult<Self> {
        let providers = Self::determine_providers(environment)?;
        
        Ok(Self {
            providers,
            cache: HashMap::with_capacity(16),
            cache_ttl: 300, // 5 minutes
        })
    }

    /// Gets a secret by key
    pub fn get_secret(&mut self, key: &str) -> BearDogResult<SecretValue> {
        // Check cache first
        if let Some(cached_value) = self.cache.get(key) {
            if let Some(expires_at) = cached_value.expires_at {
                if std::time::SystemTime::now() < expires_at {
                    return Ok(cached_value.clone());
                }
            }
        }

        // Try each provider
        for provider in &self.providers {
            if let Ok(value) = self.get_secret_from_provider(provider, key) {
                self.cache.insert(key.to_string(), value.clone());
                return Ok(value);
            }
        }
        
        Err(BearDogError::system(format!("Secret not found: {}", key)))
    }

    /// Gets secret from specific provider
    fn get_secret_from_provider(&self, provider: &SecretsProvider, key: &str) -> BearDogResult<SecretValue> {
        match provider {
            SecretsProvider::EnvironmentVariables => {
                let env_key = format!("BEARDOG_SECRET_{}", key.replace('/', "_").to_uppercase());
                if let Ok(value) = env::var(&env_key) {
                    Ok(SecretValue {
                        value,
                        expires_at: None,
                        metadata: HashMap::new(),
                    })
                } else {
                    Err(BearDogError::system("Secret not found in environment"))
                }
            }
            _ => {
                // TODO: Implement other providers
                Err(BearDogError::system("Provider not implemented"))
            }
        }
    }

    /// Determines secrets providers based on environment
    fn determine_providers(environment: &Environment) -> BearDogResult<Vec<SecretsProvider>> {
        let mut providers = vec![];

        // Always add environment variables as fallback
        providers.push(SecretsProvider::EnvironmentVariables);

        if matches!(environment, Environment::Production) {
            // Add production secrets providers
            if let Ok(vault_endpoint) = env::var("VAULT_ENDPOINT") {
                if let Ok(vault_token) = env::var("VAULT_TOKEN") {
                    providers.insert(0, SecretsProvider::Vault {
                        endpoint: vault_endpoint,
                        token: vault_token,
                        mount_path: env::var("VAULT_MOUNT_PATH").unwrap_or_else(|_| "secret".to_string()),
                    });
                }
            }
        }
        
        Ok(providers)
    }
}

// Default implementations
impl Default for ProductionConfig {
    fn default() -> Self {
        Self {
            application: ApplicationConfig::default(),
            database: DatabaseConfig::default(),
            security: SecurityConfig::default(),
            monitoring: MonitoringConfig::default(),
            logging: LoggingConfig::default(),
            networking: NetworkingConfig::default(),
            scaling: ScalingConfig::default(),
            compliance: ComplianceConfig::default(),
        }
    }
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            name: "beardog".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            instance_id: uuid::Uuid::new_v4().to_string(),
            bind_address: env::var("BEARDOG_BIND_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: 8080,
            worker_threads: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
            max_connections: 1000,
            request_timeout: 30,
            graceful_shutdown_timeout: 30,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            primary: DatabaseConnection::default(),
            read_replicas: vec![],
            connection_pool: ConnectionPoolConfig::default(),
            migration: MigrationConfig::default(),
            backup: BackupConfig::default(),
        }
    }
}

impl Default for DatabaseConnection {
    fn default() -> Self {
        Self {
            host: env::var("BEARDOG_DB_HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("BEARDOG_DB_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(5432),
            database: env::var("BEARDOG_DB_NAME")
                .unwrap_or_else(|_| "beardog".to_string()),
            username: env::var("BEARDOG_DB_USER")
                .unwrap_or_else(|_| "beardog".to_string()),
            password: String::new(),
            ssl_mode: "require".to_string(),
            connection_timeout: 5,
        }
    }
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 20,
            acquire_timeout: 30,
            idle_timeout: 600,
            max_lifetime: 3600,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: LogFormat::Json,
            outputs: vec![LogOutput::Stdout],
            structured: true,
            correlation_id: true,
            performance_logging: true,
            audit_logging: true,
        }
    }
}

impl Default for NetworkingConfig {
    fn default() -> Self {
        Self {
            tls: TlsConfig::default(),
            service_mesh: ServiceMeshConfig::default(),
            load_balancer: LoadBalancerConfig::default(),
            rate_limiting: RateLimitingConfig::default(),
        }
    }
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_path: String::new(),
            key_path: String::new(),
            ca_path: None,
            min_version: "1.2".to_string(),
            cipher_suites: vec![],
        }
    }
}

impl Default for ScalingConfig {
    fn default() -> Self {
        Self {
            horizontal: HorizontalScalingConfig::default(),
            vertical: VerticalScalingConfig::default(),
            auto_scaling: AutoScalingConfig::default(),
        }
    }
}

impl Default for AutoScalingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            min_replicas: 2,
            max_replicas: 10,
            target_cpu_utilization: 70.0,
            target_memory_utilization: 80.0,
            scale_up_cooldown: 300,
            scale_down_cooldown: 600,
        }
    }
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            data_retention: DataRetentionConfig::default(),
            encryption_at_rest: false,
            encryption_in_transit: false,
            access_logging: true,
            compliance_standards: vec![],
        }
    }
}

impl Default for DataRetentionConfig {
    fn default() -> Self {
        Self {
            logs: 30,
            metrics: 90,
            audit_trails: 2555, // 7 years
            user_data: 365,
        }
    }
} 
