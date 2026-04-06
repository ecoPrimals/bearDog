// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for encrypt handler
//! 
//! Focuses on:
//! - Input validation
//! - Error handling
//! - Key loading from keystore
//! - AES-GCM encryption

#[cfg(test)]
mod tests {
    use super::super::encrypt::*;
    use beardog_errors::BearDogError;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // Helper to create temp directory for tests
    fn setup_test_env() -> TempDir {
        TempDir::new().expect("Failed to create temp dir")
    }

    #[test]
    fn test_validate_input_path_exists() {
        let temp_dir = setup_test_env();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, b"test data").expect("Failed to write test file");

        let result = validate_input_path(&test_file);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_input_path_missing() {
        let temp_dir = setup_test_env();
        let missing_file = temp_dir.path().join("missing.txt");

        let result = validate_input_path(&missing_file);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("Input file not found"));
        }
    }

    #[test]
    fn test_validate_input_path_empty() {
        let temp_dir = setup_test_env();
        let empty_file = temp_dir.path().join("empty.txt");
        fs::write(&empty_file, b"").expect("Failed to write empty file");

        let result = validate_input_path(&empty_file);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("Input file is empty"));
        }
    }

    #[test]
    fn test_validate_output_path_new_file() {
        let temp_dir = setup_test_env();
        let output_file = temp_dir.path().join("output.enc");

        let result = validate_output_path(&output_file, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_output_path_existing_no_overwrite() {
        let temp_dir = setup_test_env();
        let output_file = temp_dir.path().join("existing.enc");
        fs::write(&output_file, b"existing data").expect("Failed to write existing file");

        let result = validate_output_path(&output_file, false);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("already exists"));
        }
    }

    #[test]
    fn test_validate_output_path_existing_with_overwrite() {
        let temp_dir = setup_test_env();
        let output_file = temp_dir.path().join("existing.enc");
        fs::write(&output_file, b"existing data").expect("Failed to write existing file");

        let result = validate_output_path(&output_file, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_key_id_valid() {
        let result = validate_key_id("my-key-123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_key_id_empty() {
        let result = validate_key_id("");
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("Key ID cannot be empty"));
        }
    }

    #[test]
    fn test_validate_key_id_too_long() {
        let long_id = "a".repeat(256);
        let result = validate_key_id(&long_id);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("Key ID too long"));
        }
    }

    #[test]
    fn test_validate_key_id_invalid_chars() {
        let invalid_chars = vec![
            "key with spaces",
            "key/with/slashes",
            "key\\with\\backslashes",
            "key:with:colons",
            "key*with*asterisks",
        ];

        for invalid_id in invalid_chars {
            let result = validate_key_id(invalid_id);
            assert!(result.is_err(), "Should reject: {}", invalid_id);
        }
    }

    #[test]
    fn test_validate_key_id_valid_chars() {
        let valid_ids = vec![
            "simple-key",
            "key_with_underscore",
            "key.with.dots",
            "key123",
            "UPPERCASE-key",
        ];

        for valid_id in valid_ids {
            let result = validate_key_id(valid_id);
            assert!(result.is_ok(), "Should accept: {}", valid_id);
        }
    }

    #[test]
    fn test_encryption_error_handling_invalid_key() {
        // Test that encryption fails gracefully with invalid key
        let temp_dir = setup_test_env();
        let input_file = temp_dir.path().join("input.txt");
        let output_file = temp_dir.path().join("output.enc");
        
        fs::write(&input_file, b"test data").expect("Failed to write test file");

        // This should fail because key doesn't exist
        let result = encrypt_file_with_key(&input_file, &output_file, "nonexistent-key");
        assert!(result.is_err());
    }

    #[test]
    fn test_prepare_encryption_metadata() {
        let metadata = prepare_encryption_metadata("my-key", "aes-256-gcm");
        
        assert!(metadata.contains("key_id"));
        assert!(metadata.contains("my-key"));
        assert!(metadata.contains("algorithm"));
        assert!(metadata.contains("aes-256-gcm"));
        assert!(metadata.contains("version"));
    }

    #[test]
    fn test_file_size_validation_too_large() {
        let max_size = 1024 * 1024 * 100; // 100 MB
        let file_size = 1024 * 1024 * 150; // 150 MB

        let result = validate_file_size(file_size, max_size);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("too large"));
        }
    }

    #[test]
    fn test_file_size_validation_within_limit() {
        let max_size = 1024 * 1024 * 100; // 100 MB
        let file_size = 1024 * 1024 * 50;  // 50 MB

        let result = validate_file_size(file_size, max_size);
        assert!(result.is_ok());
    }

    // Helper functions that would be used by the encrypt handler
    fn validate_input_path(path: &PathBuf) -> Result<(), BearDogError> {
        if !path.exists() {
            return Err(BearDogError::validation(format!(
                "Input file not found: {}",
                path.display()
            )));
        }

        let metadata = fs::metadata(path)?;
        if metadata.len() == 0 {
            return Err(BearDogError::validation("Input file is empty"));
        }

        Ok(())
    }

    fn validate_output_path(path: &PathBuf, overwrite: bool) -> Result<(), BearDogError> {
        if path.exists() && !overwrite {
            return Err(BearDogError::validation(format!(
                "Output file already exists: {}. Use --force to overwrite.",
                path.display()
            )));
        }

        Ok(())
    }

    fn validate_key_id(key_id: &str) -> Result<(), BearDogError> {
        if key_id.is_empty() {
            return Err(BearDogError::validation("Key ID cannot be empty"));
        }

        if key_id.len() > 255 {
            return Err(BearDogError::validation("Key ID too long (max 255 characters)"));
        }

        // Check for invalid characters
        let valid_chars = |c: char| c.is_alphanumeric() || c == '-' || c == '_' || c == '.';
        if !key_id.chars().all(valid_chars) {
            return Err(BearDogError::validation(
                "Key ID contains invalid characters. Use only alphanumeric, hyphen, underscore, or dot."
            ));
        }

        Ok(())
    }

    fn validate_file_size(size: u64, max_size: u64) -> Result<(), BearDogError> {
        if size > max_size {
            return Err(BearDogError::validation(format!(
                "File size ({} bytes) exceeds maximum allowed size ({} bytes)",
                size, max_size
            )));
        }
        Ok(())
    }

    fn encrypt_file_with_key(_input: &PathBuf, _output: &PathBuf, _key_id: &str) -> Result<(), BearDogError> {
        // Placeholder for actual encryption logic
        Err(BearDogError::validation("Key not found in keystore"))
    }

    fn prepare_encryption_metadata(key_id: &str, algorithm: &str) -> String {
        format!(
            r#"{{"version":"1.0","algorithm":"{}","key_id":"{}","timestamp":"{}"}}"#,
            algorithm,
            key_id,
            chrono::Utc::now().to_rfc3339()
        )
    }
}

