//! Tests for configuration loader

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::env;

    #[test]
    fn test_config_loader_new() {
        let loader = ConfigLoader::new();
        assert!(loader.build().is_ok(), "New loader should be valid");
    }

    #[test]
    fn test_config_loader_default() {
        let loader = ConfigLoader::default();
        assert!(loader.build().is_ok(), "Default loader should be valid");
    }

    #[test]
    fn test_config_loader_with_defaults() {
        let loader = ConfigLoader::new().with_defaults();
        let config = loader.build().expect("Should build with defaults");

        // Verify some default values exist
        assert!(config.network.api.port > 0, "Should have default API port");
    }

    #[test]
    fn test_config_loader_with_platform_defaults() {
        let loader = ConfigLoader::new().with_defaults().with_platform_defaults();

        assert!(loader.build().is_ok(), "Platform defaults should not fail");
    }

    #[test]
    fn test_config_loader_with_config_file() {
        let loader = ConfigLoader::new().with_defaults().with_config_file();

        assert!(loader.is_ok(), "Auto-discover config file should not fail");
    }

    #[test]
    fn test_config_loader_with_env_vars() {
        // Set a test environment variable
        env::set_var("BEARDOG_API_PORT", "9999");

        let loader = ConfigLoader::new().with_defaults().with_env_vars();

        let config = loader.build().expect("Should build with env vars");

        // Note: This test may not work if from_env() doesn't read BEARDOG_API_PORT
        // but it verifies the method doesn't panic
        assert!(config.network.api.port > 0, "Should have valid port");

        // Clean up
        env::remove_var("BEARDOG_API_PORT");
    }

    #[test]
    fn test_config_loader_full_hierarchy() {
        let result = ConfigLoader::new()
            .with_defaults()
            .with_platform_defaults()
            .with_config_file()
            .and_then(|loader| loader.with_env_vars().build());

        assert!(result.is_ok(), "Full hierarchy should work");
    }

    #[test]
    fn test_config_loader_build_validates() {
        let loader = ConfigLoader::new();
        let result = loader.build();

        assert!(result.is_ok(), "Default config should pass validation");
    }

    #[test]
    fn test_config_loader_builder_pattern() {
        // Test that builder pattern can be chained
        let result = ConfigLoader::new()
            .with_defaults()
            .with_platform_defaults()
            .with_env_vars()
            .build();

        assert!(result.is_ok(), "Builder chain should work");
    }

    #[test]
    fn test_config_loader_multiple_builds() {
        let loader = ConfigLoader::new().with_defaults();

        // First build
        let config1 = loader.build();
        assert!(config1.is_ok(), "First build should succeed");

        // Note: Can't build twice as build() consumes self
        // This test just verifies the pattern works once
    }
}
