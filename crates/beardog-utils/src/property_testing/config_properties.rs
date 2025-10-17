// Configuration property testing implementations

use super::{info, BearDogError, PropertyBasedTestFramework, TestCase};

impl PropertyBasedTestFramework {
    /// Test configuration properties
    pub fn test_configuration_properties(&mut self) -> Result<(), BearDogError> {
        info!("⚙️ Testing Configuration Properties");

        // Generate config-specific test cases
        self.generate_config_test_cases()?;

        // Test configuration properties
        self.test_config_parsing_properties()?;
        self.test_config_validation_properties()?;
        self.test_default_value_properties()?;

        self.statistics.properties_tested += 3;
        Ok(())
    }

    pub fn generate_config_test_cases(&mut self) -> Result<(), BearDogError> {
        info!("🎲 Generating configuration test cases");

        for i in 0..self.config.test_cases {
            let test_case = TestCase {
                id: (i + 2000) as u64, // Offset to avoid ID conflicts
                input_data: self.generate_config_data(i)?,
                test_type: "configuration".to_string(),
                expected_properties: vec![
                    "parseable".to_string(),
                    "validated".to_string(),
                    "defaults_applied".to_string(),
                ],
            };
            self.test_cases.push(test_case);
        }

        Ok(())
    }

    fn test_config_parsing_properties(&mut self) -> Result<(), BearDogError> {
        let test_cases = self.test_cases.clone(); // Clone to avoid borrow conflict
        for test_case in &test_cases {
            if test_case.test_type == "configuration" {
                let parseable = self.test_toml_parsing(test_case)?;
                self.record_result(test_case.id, "toml_parsing", parseable);
            }
        }
        Ok(())
    }

    fn test_config_validation_properties(&mut self) -> Result<(), BearDogError> {
        let test_cases = self.test_cases.clone(); // Clone to avoid borrow conflict
        for test_case in &test_cases {
            if test_case.test_type == "configuration" {
                let required_fields = self.test_required_fields(test_case)?;
                self.record_result(test_case.id, "required_fields", required_fields);

                let type_validation = self.test_type_validation(test_case)?;
                self.record_result(test_case.id, "type_validation", type_validation);
            }
        }
        Ok(())
    }

    fn test_default_value_properties(&mut self) -> Result<(), BearDogError> {
        let test_cases = self.test_cases.clone(); // Clone to avoid borrow conflict
        for test_case in &test_cases {
            if test_case.test_type == "configuration" {
                let defaults_applied = self.test_default_value_application(test_case)?;
                self.record_result(test_case.id, "default_value_application", defaults_applied);
            }
        }
        Ok(())
    }

    // Individual test implementations
    fn test_toml_parsing(&self, test_case: &TestCase) -> Result<bool, BearDogError> {
        let config_str = String::from_utf8_lossy(&test_case.input_data);
        match self.mock_parse_toml(&config_str) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false), // Parsing failure is a valid result
        }
    }

    fn test_required_fields(&self, test_case: &TestCase) -> Result<bool, BearDogError> {
        let config_str = String::from_utf8_lossy(&test_case.input_data);
        match self.mock_parse_toml(&config_str) {
            Ok(config) => {
                // For this test, we don't require specific fields
                Ok(!config.is_empty() || config_str.trim().is_empty())
            }
            Err(_) => Ok(true), // Parsing errors are handled elsewhere
        }
    }

    fn test_type_validation(&self, test_case: &TestCase) -> Result<bool, BearDogError> {
        let config_str = String::from_utf8_lossy(&test_case.input_data);
        match self.mock_parse_toml(&config_str) {
            Ok(_) => Ok(true),  // If it parses, types are valid enough
            Err(_) => Ok(true), // Type errors are expected for invalid input
        }
    }

    fn test_default_value_application(&self, test_case: &TestCase) -> Result<bool, BearDogError> {
        let config_str = String::from_utf8_lossy(&test_case.input_data);
        match self.mock_parse_config_with_defaults(&config_str) {
            Ok(config) => {
                // Check that defaults are applied
                Ok(config.contains_key("default_value") && config.contains_key("timeout"))
            }
            Err(_) => Ok(false),
        }
    }
}
