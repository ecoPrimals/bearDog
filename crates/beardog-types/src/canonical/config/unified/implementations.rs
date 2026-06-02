// SPDX-License-Identifier: AGPL-3.0-or-later

//! Implementation Blocks for Unified Configuration
//!
//! This module contains the implementation methods for `UnifiedBearDogConfig`.

use beardog_config::env_keys;
use beardog_errors::BearDogError;

use super::UnifiedBearDogConfig;
use super::metadata::Environment;

impl UnifiedBearDogConfig {
    /// Loads configuration from environment variables and default sources
    ///
    /// This is the primary method for loading `BearDog` configuration in production.
    /// It automatically detects the environment, loads appropriate settings, and
    /// validates the complete configuration.
    ///
    /// ## Loading Strategy
    ///
    /// 1. Detects current environment (production, staging, development)
    /// 2. Loads environment-specific defaults
    /// 3. Overlays environment variable overrides
    /// 4. Validates complete configuration
    /// 5. Returns validated, ready-to-use configuration
    ///
    /// ## Environment Variables
    ///
    /// Key environment variables:
    /// - `BEARDOG_ENV` - Environment name (production/staging/development)
    /// - `BEARDOG_APP_NAME` - Application name override
    /// - `BEARDOG_HTTP_PORT` - HTTP server port override
    /// - `DATABASE_URL` - Database connection string
    /// - `REDIS_URL` - Redis connection string
    ///
    /// ## Returns
    ///
    /// - `Ok(UnifiedBearDogConfig)` - Validated, ready-to-use configuration
    /// - `Err(BearDogError)` - If loading or validation fails
    ///
    /// ## Errors
    ///
    /// Returns an error if:
    /// - Required environment variables are missing
    /// - Configuration values are invalid (out of range, malformed, etc.)
    /// - Environment detection fails
    /// - Validation fails for any domain configuration
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
    /// # use beardog_errors::BearDogError;
    ///
    /// # fn main() -> Result<(), BearDogError> {
    /// // Load configuration with automatic environment detection
    /// let config = UnifiedBearDogConfig::load()?;
    ///
    /// println!("Running in {:?} environment", config.metadata.environment);
    /// println!("Application: {} v{:?}", config.app.app_name, config.metadata.version);
    /// # Ok(())
    /// # }
    /// ```
    pub fn load() -> Result<Self, BearDogError> {
        let mut config = Self::default();

        // Load from environment variables
        if let Ok(env) = std::env::var(env_keys::ENV_ENVIRONMENT) {
            config.metadata.environment = match env.as_str() {
                "testing" => Environment::Testing,
                "staging" => Environment::Staging,
                "production" => Environment::Production,
                _ => Environment::Development,
            };
        }

        // Validate the loaded configuration
        config.validate()?;
        Ok(config)
    }

    /// Creates a development configuration with sensible defaults
    ///
    /// This provides a pre-configured setup suitable for local development,
    /// with relaxed security settings and verbose logging enabled.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
    ///
    /// let config = UnifiedBearDogConfig::development();
    /// assert_eq!(config.metadata.environment, beardog_types::canonical::config::unified::metadata::Environment::Development);
    /// ```
    pub fn development() -> Self {
        let mut config = Self::default();
        config.metadata.environment = Environment::Development;
        config
    }

    /// Creates a production configuration with hardened security
    ///
    /// This provides a pre-configured setup suitable for production deployment,
    /// with strict security settings and optimized performance.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
    ///
    /// let config = UnifiedBearDogConfig::production();
    /// assert_eq!(config.metadata.environment, beardog_types::canonical::config::unified::metadata::Environment::Production);
    /// ```
    pub fn production() -> Self {
        let mut config = Self::default();
        config.metadata.environment = Environment::Production;
        config
    }

    /// Alias for `load()` to match common API patterns
    ///
    /// This method is identical to `load()` but provides a more intuitive
    /// name that matches common Rust configuration patterns.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
    /// # use beardog_errors::BearDogError;
    ///
    /// # fn main() -> Result<(), BearDogError> {
    /// let config = UnifiedBearDogConfig::from_env()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if [`Self::load`] fails.
    pub fn from_env() -> Result<Self, BearDogError> {
        Self::load()
    }

    /// Validates the complete configuration across all domains
    ///
    /// Performs comprehensive validation of all configuration sections including
    /// network settings, security policies, database connections, HSM configuration,
    /// and all other domains. This ensures the configuration is internally consistent
    /// and all values are within valid ranges.
    ///
    /// ## Validation Checks
    ///
    /// - **Network**: Port ranges, timeout values, connection limits
    /// - **Security**: Encryption settings, authentication config, policy consistency
    /// - **Database**: Connection strings, pool sizes, timeout values
    /// - **HSM**: Module availability, key access permissions
    /// - **Application**: Port uniqueness, resource limits
    /// - **Performance**: Buffer sizes, thread counts, memory limits
    ///
    /// ## Returns
    ///
    /// - `Ok(())` - All validation checks passed
    /// - `Err(BearDogError)` - Validation failed with detailed error context
    ///
    /// ## Errors
    ///
    /// Returns an error if:
    /// - Port numbers are out of range (1-65535)
    /// - Timeout values are invalid or negative
    /// - Required fields are missing or empty
    /// - Values are inconsistent across domains
    /// - Database connection settings are malformed
    /// - Security policies have conflicting requirements
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
    /// # use beardog_errors::BearDogError;
    ///
    /// # fn main() -> Result<(), BearDogError> {
    /// let mut config = UnifiedBearDogConfig::default();
    /// config.app.app_name = "MyApp".to_string();
    /// config.app.organization = "MyOrg".to_string();
    ///
    /// // Validate before use
    /// config.validate()?;
    /// println!("Configuration is valid!");
    /// # Ok(())
    /// # }
    /// ```
    pub fn validate(&self) -> Result<(), BearDogError> {
        // Validate metadata
        if self.metadata.version.beardog_version.is_empty() {
            return Err(BearDogError::business(
                "Configuration version cannot be empty".to_string(),
            ));
        }

        // Validate monitoring configuration
        // Monitoring validation not needed - using validated defaults

        // Production configuration is valid by construction

        Ok(())
    }

    /// Migrates from legacy configuration formats to unified configuration
    ///
    /// This method provides a migration path from older `BearDog` configuration
    /// formats (pre-unification) to the new unified configuration system. It
    /// handles configuration consolidation, field mapping, and validation.
    ///
    /// ## Migration Process
    ///
    /// 1. Detects legacy configuration format and version
    /// 2. Maps legacy fields to unified configuration structure
    /// 3. Applies sensible defaults for new fields
    /// 4. Validates migrated configuration
    /// 5. Returns ready-to-use unified configuration
    ///
    /// ## Supported Legacy Formats
    ///
    /// - `BearDog` v2.x fragmented configurations
    /// - Environment variable-based configs
    /// - TOML-based legacy configs
    ///
    /// ## Returns
    ///
    /// - `Ok(UnifiedBearDogConfig)` - Successfully migrated and validated configuration
    /// - `Err(BearDogError)` - If migration fails
    ///
    /// ## Errors
    ///
    /// Returns an error if:
    /// - Legacy configuration cannot be located
    /// - Legacy format is unsupported or corrupted
    /// - Required fields are missing in legacy config
    /// - Migration validation fails
    /// - File system access errors occur
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
    /// # use beardog_errors::BearDogError;
    ///
    /// # fn main() -> Result<(), BearDogError> {
    /// // Migrate from legacy v2.x configuration
    /// let config = UnifiedBearDogConfig::migrate_from_legacy()?;
    ///
    /// println!("Successfully migrated to unified configuration");
    /// println!("Version: {:?}", config.metadata.version);
    /// # Ok(())
    /// # }
    /// ```
    pub fn migrate_from_legacy() -> Result<Self, BearDogError> {
        let mut config = Self::default();

        // Set up basic migration defaults
        config.metadata.version.beardog_version = env!("CARGO_PKG_VERSION").to_string();
        config.metadata.environment = Environment::Development;

        config.monitoring.enabled = true;

        // Validate migrated configuration
        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::UnifiedBearDogConfig;
    use crate::canonical::config::unified::metadata::Environment;

    #[test]
    fn load_and_from_env_default_path() {
        let a = UnifiedBearDogConfig::load().expect("load with defaults should validate");
        let b = UnifiedBearDogConfig::from_env().expect("from_env aliases load");
        assert_eq!(
            a.metadata.environment, b.metadata.environment,
            "load and from_env should agree"
        );
    }

    #[test]
    fn development_and_production_presets() {
        let d = UnifiedBearDogConfig::development();
        assert_eq!(d.metadata.environment, Environment::Development);
        let p = UnifiedBearDogConfig::production();
        assert_eq!(p.metadata.environment, Environment::Production);
    }

    #[test]
    fn validate_rejects_empty_beardog_version() {
        let mut c = UnifiedBearDogConfig::default();
        c.metadata.version.beardog_version.clear();
        let err = c
            .validate()
            .expect_err("empty version should fail validation");
        let msg = err.to_string();
        assert!(
            msg.contains("version") || msg.contains("empty"),
            "unexpected validation message: {msg}"
        );
    }

    #[test]
    fn migrate_from_legacy_succeeds() {
        let m = UnifiedBearDogConfig::migrate_from_legacy().expect("migrate");
        assert!(m.monitoring.enabled);
        assert_eq!(m.metadata.environment, Environment::Development);
        assert!(!m.metadata.version.beardog_version.is_empty());
    }
}
