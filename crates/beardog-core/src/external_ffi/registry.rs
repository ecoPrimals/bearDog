// SPDX-License-Identifier: AGPL-3.0-only

//! # External Function Registry
//!
//! Implementation of the FFI registry for managing external libraries and functions.

use super::safety::{ParameterValue, SafetyChecker};
use super::types::{
    ExternalFunction, FfiRegistryConfig, FunctionHandle, FunctionMetadata, FunctionParameter,
    FunctionResult, FunctionSignature, FunctionValue, LibraryHandle, LibraryMetadata,
    LibraryStatus, SecurityClearance,
};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

// Re-export for backward compatibility
pub use super::types::FfiRegistryConfig as RegistryConfig;

/// External function registry
#[derive(Debug)]
pub struct ExternalFunctionRegistry {
    /// Registry configuration
    config: FfiRegistryConfig,

    /// Loaded libraries by ID
    libraries: HashMap<String, LibraryHandle>,

    /// Registered functions by ID
    functions: HashMap<String, FunctionHandle>,

    /// Safety checker for function calls
    safety_checker: Arc<SafetyChecker>,
}

impl Default for ExternalFunctionRegistry {
    fn default() -> Self {
        Self::new(FfiRegistryConfig::default())
    }
}

impl ExternalFunctionRegistry {
    /// Create a new registry with the given configuration
    pub fn new(config: FfiRegistryConfig) -> Self {
        Self {
            config,
            libraries: HashMap::new(),
            functions: HashMap::new(),
            safety_checker: Arc::new(SafetyChecker::new(SecurityClearance::Public)),
        }
    }

    /// Load a library from the given path
    ///
    /// # Arguments
    /// * `library_path` - Path to the library to load
    ///
    /// # Returns
    /// The unique ID of the loaded library
    pub fn load_library(&mut self, library_path: &str) -> Result<String, BearDogError> {
        // Verify path is allowed
        if self.config.enable_safety_checks && !self.is_library_path_allowed(library_path) {
            return Err(BearDogError::permission_denied(format!(
                "Library path not in allowed directories: {}",
                library_path
            )));
        }

        // Check library limit
        if self.libraries.len() >= self.config.max_libraries {
            return Err(BearDogError::resource_exhausted(format!(
                "Maximum library limit reached: {}",
                self.config.max_libraries
            )));
        }

        let library_id = Uuid::new_v4().to_string();

        // Extract library name from path
        let name = library_path
            .split('/')
            .last()
            .unwrap_or("unknown")
            .to_string();

        let library_handle = LibraryHandle {
            id: library_id.clone(),
            name,
            path: library_path.into(),
            metadata: LibraryMetadata {
                version: "1.0.0".to_string(),
                description: format!("Library loaded from {}", library_path),
                author: "Unknown".to_string(),
                license: "Unknown".to_string(),
                exported_functions: vec![],
                dependencies: vec![],
                custom: HashMap::new(),
            },
            status: LibraryStatus::Loaded,
            loaded_at: chrono::Utc::now(),
        };

        self.libraries.insert(library_id.clone(), library_handle);

        if self.config.enable_logging {
            info!("Loaded library: {} from {}", library_id, library_path);
        }

        Ok(library_id)
    }

    /// Unload a library by ID
    ///
    /// # Arguments
    /// * `library_id` - ID of the library to unload
    pub fn unload_library(&mut self, library_id: &str) -> Result<(), BearDogError> {
        if self.libraries.remove(library_id).is_some() {
            // Also remove all functions from this library
            self.functions
                .retain(|_, function| function.library_id != library_id);

            if self.config.enable_logging {
                info!("Unloaded library: {}", library_id);
            }

            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Library not found: {}",
                library_id
            )))
        }
    }

    /// Get information about a library
    ///
    /// # Arguments
    /// * `library_id` - ID of the library
    pub fn get_library_info(&self, library_id: &str) -> Result<LibraryHandle, BearDogError> {
        self.libraries
            .get(library_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Library not found: {}", library_id)))
    }

    /// List all loaded libraries
    pub fn list_libraries(&self) -> Result<Vec<LibraryHandle>, BearDogError> {
        Ok(self.libraries.values().cloned().collect())
    }

    /// Register a function from a library
    ///
    /// # Arguments
    /// * `library_id` - ID of the library containing the function
    /// * `function_name` - Name of the function
    /// * `signature` - Function signature
    /// * `metadata` - Function metadata
    ///
    /// # Returns
    /// The unique ID of the registered function
    pub fn register_function(
        &mut self,
        library_id: &str,
        function_name: &str,
        signature: FunctionSignature,
        metadata: FunctionMetadata,
    ) -> Result<String, BearDogError> {
        // Verify library exists
        if !self.libraries.contains_key(library_id) {
            return Err(BearDogError::not_found(format!(
                "Library not found: {}",
                library_id
            )));
        }

        let function_id = Uuid::new_v4().to_string();

        let function_handle = FunctionHandle {
            id: function_id.clone(),
            name: function_name.to_string(),
            library_id: library_id.to_string(),
            signature,
            call_count: 0,
            last_called: None,
            metadata,
        };

        self.functions.insert(function_id.clone(), function_handle);

        if self.config.enable_logging {
            info!(
                "Registered function: {} in library {}",
                function_name, library_id
            );
        }

        Ok(function_id)
    }

    /// Unregister a function
    ///
    /// # Arguments
    /// * `function_id` - ID of the function to unregister
    pub fn unregister_function(&mut self, function_id: &str) -> Result<(), BearDogError> {
        if self.functions.remove(function_id).is_some() {
            if self.config.enable_logging {
                info!("Unregistered function: {}", function_id);
            }
            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Function not found: {}",
                function_id
            )))
        }
    }

    /// Call a function by ID
    ///
    /// # Arguments
    /// * `function_id` - ID of the function to call
    /// * `parameters` - Function parameters
    ///
    /// # Returns
    /// The result of the function call
    pub fn call_function(
        &mut self,
        function_id: &str,
        parameters: Vec<FunctionParameter>,
    ) -> Result<FunctionResult, BearDogError> {
        let function = self.functions.get_mut(function_id).ok_or_else(|| {
            BearDogError::not_found(format!("Function not found: {}", function_id))
        })?;

        // Update call statistics
        function.call_count += 1;
        function.last_called = Some(chrono::Utc::now());

        // Convert function handle to external function for safety check
        let external_function = ExternalFunction {
            id: function.id.clone(),
            name: function.name.clone(),
            signature: function.signature.clone(),
            metadata: function.metadata.clone(),
            library_id: function.library_id.clone(),
        };

        // Convert FunctionParameter to ParameterValue for safety check
        let parameter_values: Vec<ParameterValue> = parameters
            .into_iter()
            .map(|param| {
                let value = param.default_value.clone().unwrap_or(FunctionValue::Null);
                ParameterValue {
                    value,
                    parameter: param,
                }
            })
            .collect();

        // Perform safety check
        if self.config.enable_safety_checks {
            self.safety_checker
                .check_function_call(&external_function, &parameter_values)?;
        }

        // Log the call if enabled
        if self.config.enable_logging {
            info!(
                "Calling function: {} with {} parameters",
                function_id,
                parameter_values.len()
            );
        }

        // Simulate function call
        // In a real implementation, this would call the actual function
        let start_time = std::time::Instant::now();

        let result = FunctionResult {
            success: true,
            value: Some(FunctionValue::String(
                "Function called successfully".to_string(),
            )),
            error: None,
            execution_time_us: start_time.elapsed().as_micros() as u64,
        };

        Ok(result)
    }

    /// Get information about a function
    ///
    /// # Arguments
    /// * `function_id` - ID of the function
    pub fn get_function_info(&self, function_id: &str) -> Result<FunctionHandle, BearDogError> {
        self.functions
            .get(function_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Function not found: {}", function_id)))
    }

    /// Check if a library path is in the allowed directories
    fn is_library_path_allowed(&self, path: &str) -> bool {
        // If no directories are configured, allow all (for development)
        if self.config.allowed_directories.is_empty() {
            return true;
        }

        let path_buf = std::path::PathBuf::from(path);

        for allowed_dir in &self.config.allowed_directories {
            if path_buf.starts_with(allowed_dir) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ExternalFunctionRegistry::default();
        assert!(registry.libraries.is_empty());
        assert!(registry.functions.is_empty());
    }

    #[test]
    fn test_load_library() {
        let mut registry = ExternalFunctionRegistry::default();
        let result = registry.load_library("/test/library.so");
        assert!(result.is_ok());

        let library_id = result.expect("load test library");
        let info = registry.get_library_info(&library_id);
        assert!(info.is_ok());
    }

    #[test]
    fn test_unload_nonexistent_library() {
        let mut registry = ExternalFunctionRegistry::default();
        let result = registry.unload_library("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_libraries() {
        let mut registry = ExternalFunctionRegistry::default();
        registry
            .load_library("/test/lib1.so")
            .expect("load test lib1");
        registry
            .load_library("/test/lib2.so")
            .expect("load test lib2");

        let libraries = registry.list_libraries().expect("list libraries");
        assert_eq!(libraries.len(), 2);
    }
}
