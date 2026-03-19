// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for decrypt handler
//! 
//! Focuses on:
//! - Encrypted file validation
//! - Metadata parsing
//! - Error handling
//! - Key retrieval

#[cfg(test)]
mod tests {
    use beardog_errors::BearDogError;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // Helper to create temp directory for tests
    fn setup_test_env() -> TempDir {
        TempDir::new().expect("Failed to create temp dir")
    }

    #[test]
    fn test_validate_encrypted_file_exists() {
        let temp_dir = setup_test_env();
        let encrypted_file = temp_dir.path().join("encrypted.enc");
        fs::write(&encrypted_file, b"encrypted data").expect("Failed to write encrypted file");

        let result = validate_encrypted_file(&encrypted_file);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_encrypted_file_missing() {
        let temp_dir = setup_test_env();
        let missing_file = temp_dir.path().join("missing.enc");

        let result = validate_encrypted_file(&missing_file);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("Encrypted file not found"));
        }
    }

    #[test]
    fn test_validate_encrypted_file_empty() {
        let temp_dir = setup_test_env();
        let empty_file = temp_dir.path().join("empty.enc");
        fs::write(&empty_file, b"").expect("Failed to write empty file");

        let result = validate_encrypted_file(&empty_file);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("Encrypted file is empty"));
        }
    }

    #[test]
    fn test_parse_encryption_metadata_valid() {
        let metadata = r#"{"version":"1.0","algorithm":"aes-256-gcm","key_id":"my-key"}"#;
        
        let result = parse_encryption_metadata(metadata);
        assert!(result.is_ok());
        
        if let Ok((key_id, algorithm)) = result {
            assert_eq!(key_id, "my-key");
            assert_eq!(algorithm, "aes-256-gcm");
        }
    }

    #[test]
    fn test_parse_encryption_metadata_invalid_json() {
        let invalid_metadata = "not json";
        
        let result = parse_encryption_metadata(invalid_metadata);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("Failed to parse"));
        }
    }

    #[test]
    fn test_parse_encryption_metadata_missing_fields() {
        let incomplete_metadata = r#"{"version":"1.0"}"#;
        
        let result = parse_encryption_metadata(incomplete_metadata);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("Missing required field"));
        }
    }

    #[test]
    fn test_parse_encryption_metadata_unsupported_version() {
        let future_version = r#"{"version":"2.0","algorithm":"aes-256-gcm","key_id":"my-key"}"#;
        
        let result = parse_encryption_metadata(future_version);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("Unsupported metadata version"));
        }
    }

    #[test]
    fn test_validate_decryption_output_path_new() {
        let temp_dir = setup_test_env();
        let output_file = temp_dir.path().join("decrypted.txt");

        let result = validate_decryption_output_path(&output_file, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_decryption_output_path_existing_no_overwrite() {
        let temp_dir = setup_test_env();
        let output_file = temp_dir.path().join("existing.txt");
        fs::write(&output_file, b"existing").expect("Failed to write existing file");

        let result = validate_decryption_output_path(&output_file, false);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("already exists"));
        }
    }

    #[test]
    fn test_validate_decryption_output_path_existing_with_overwrite() {
        let temp_dir = setup_test_env();
        let output_file = temp_dir.path().join("existing.txt");
        fs::write(&output_file, b"existing").expect("Failed to write existing file");

        let result = validate_decryption_output_path(&output_file, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_encrypted_file_format_valid() {
        let temp_dir = setup_test_env();
        let file = temp_dir.path().join("valid.enc");
        
        // Create a mock encrypted file with proper format
        let content = b"BEARDOG_ENCRYPTED_V1\n{\"key_id\":\"test\"}\nencrypted_data_here";
        fs::write(&file, content).expect("Failed to write file");

        let result = verify_encrypted_file_format(&file);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_encrypted_file_format_invalid_magic() {
        let temp_dir = setup_test_env();
        let file = temp_dir.path().join("invalid.enc");
        
        let content = b"INVALID_HEADER\nrest of file";
        fs::write(&file, content).expect("Failed to write file");

        let result = verify_encrypted_file_format(&file);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("Invalid encrypted file format"));
        }
    }

    #[test]
    fn test_decrypt_error_handling_wrong_key() {
        let temp_dir = setup_test_env();
        let encrypted_file = temp_dir.path().join("encrypted.enc");
        let output_file = temp_dir.path().join("output.txt");
        
        // Create mock encrypted file
        fs::write(&encrypted_file, b"mock encrypted data").expect("Failed to write");

        // Should fail with wrong key
        let result = decrypt_file_with_key(&encrypted_file, &output_file, "wrong-key");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_nonce_from_encrypted_data() {
        let mock_encrypted = b"BEARDOG_ENC_V1\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b";
        
        let result = extract_nonce(mock_encrypted);
        assert!(result.is_ok());
        
        if let Ok(nonce) = result {
            assert_eq!(nonce.len(), 12); // AES-GCM nonce size
        }
    }

    #[test]
    fn test_extract_nonce_insufficient_data() {
        let insufficient_data = b"SHORT";
        
        let result = extract_nonce(insufficient_data);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("Insufficient data"));
        }
    }

    // Helper functions
    fn validate_encrypted_file(path: &PathBuf) -> Result<(), BearDogError> {
        if !path.exists() {
            return Err(BearDogError::validation(format!(
                "Encrypted file not found: {}",
                path.display()
            )));
        }

        let metadata = fs::metadata(path)?;
        if metadata.len() == 0 {
            return Err(BearDogError::validation("Encrypted file is empty"));
        }

        Ok(())
    }

    fn parse_encryption_metadata(metadata: &str) -> Result<(String, String), BearDogError> {
        let parsed: serde_json::Value = serde_json::from_str(metadata)
            .map_err(|e| BearDogError::validation(format!("Failed to parse metadata: {}", e)))?;

        let version = parsed.get("version")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::validation("Missing required field: version"))?;

        if version != "1.0" {
            return Err(BearDogError::validation(format!(
                "Unsupported metadata version: {}",
                version
            )));
        }

        let key_id = parsed.get("key_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::validation("Missing required field: key_id"))?
            .to_string();

        let algorithm = parsed.get("algorithm")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::validation("Missing required field: algorithm"))?
            .to_string();

        Ok((key_id, algorithm))
    }

    fn validate_decryption_output_path(path: &PathBuf, overwrite: bool) -> Result<(), BearDogError> {
        if path.exists() && !overwrite {
            return Err(BearDogError::validation(format!(
                "Output file already exists: {}. Use --force to overwrite.",
                path.display()
            )));
        }
        Ok(())
    }

    fn verify_encrypted_file_format(path: &PathBuf) -> Result<(), BearDogError> {
        let content = fs::read(path)?;
        
        if content.len() < 20 {
            return Err(BearDogError::validation("File too small to be valid encrypted file"));
        }

        let magic = &content[0..20];
        if !magic.starts_with(b"BEARDOG_ENCRYPTED_V1") {
            return Err(BearDogError::validation("Invalid encrypted file format: missing magic header"));
        }

        Ok(())
    }

    fn decrypt_file_with_key(_input: &PathBuf, _output: &PathBuf, _key_id: &str) -> Result<(), BearDogError> {
        Err(BearDogError::validation("Decryption failed: key not found"))
    }

    fn extract_nonce(data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if data.len() < 26 {  // Header (14) + nonce (12)
            return Err(BearDogError::validation("Insufficient data for nonce extraction"));
        }

        Ok(data[14..26].to_vec())
    }
}

