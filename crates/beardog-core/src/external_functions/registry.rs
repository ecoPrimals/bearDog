// SPDX-License-Identifier: AGPL-3.0-only

// External Function Registry Implementation

use super::safety::{ParameterValue, SafetyChecker};
use super::types::{
    ExternalFunction, ExternalFunctionsRegistryConfig, FunctionHandle, FunctionMetadata,
    FunctionParameter, FunctionResult, FunctionSignature, FunctionValue, LibraryHandle,
    LibraryMetadata, LibraryStatus, SecurityClearance,
};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

/// External function registry
#[derive(Debug)]
pub struct ExternalFunctionRegistry {
    config: ExternalFunctionsRegistryConfig,
    libraries: HashMap<String, LibraryHandle>,
    functions: HashMap<String, FunctionHandle>,
    safety_checker: Arc<SafetyChecker>,
}

impl Default for ExternalFunctionRegistry {
    fn default() -> Self {
        Self::new(ExternalFunctionsRegistryConfig::default())
    }
}

impl ExternalFunctionRegistry {
    /// Create a new registry
    /// Creates a new instance
    #[must_use]
    pub fn new(config: ExternalFunctionsRegistryConfig) -> Self {
        Self {
            config,
            libraries: HashMap::new(),
            functions: HashMap::new(),
            safety_checker: Arc::new(SafetyChecker::new(SecurityClearance::Public)),
        }
    }

    /// Load a library
    /// Loads library
    /// Loads library
    ///
    /// # Errors
    /// Returns an error if the library loading fails
    pub fn load_library(&mut self, library_path: &str) -> Result<String, BearDogError> {
        let library_id = Uuid::new_v4().to_string();

        // Create library handle
        let library_handle = LibraryHandle {
            id: library_id.clone(),
            name: library_path
                .split('/')
                .next_back()
                .unwrap_or("unknown")
                .to_string(),
            path: library_path.into(),
            metadata: LibraryMetadata {
                version: "1.0.0".to_string(),
                description: format!("Library loaded from {library_path}"),
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
        Ok(library_id)
    }

    /// Unload a library
    ///
    /// # Errors
    /// Returns an error if the library is not found
    pub fn unload_library(&mut self, library_id: &str) -> Result<(), BearDogError> {
        if self.libraries.remove(library_id).is_some() {
            // Also remove all functions from this library
            self.functions
                .retain(|_, function| function.library_id != library_id);
            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Library not found: {library_id}"
            )))
        }
    }

    /// Gets `library_info`
    /// Gets `library_info`
    ///
    /// # Errors
    /// Returns an error if the library is not found
    pub fn get_library_info(&self, library_id: &str) -> Result<LibraryHandle, BearDogError> {
        self.libraries
            .get(library_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Library not found: {library_id}")))
    }

    /// List all loaded libraries
    ///
    /// # Errors
    /// Returns an error if listing fails
    pub fn list_libraries(&self) -> Result<Vec<LibraryHandle>, BearDogError> {
        Ok(self.libraries.values().cloned().collect())
    }

    /// Register a function
    ///
    /// # Errors
    /// Returns an error if the library is not found or registration fails
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
                "Library not found: {library_id}"
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
        Ok(function_id)
    }

    /// Unregister a function
    ///
    /// # Errors
    ///
    /// Returns an error if the function is not found in the registry
    pub fn unregister_function(&mut self, function_id: &str) -> Result<(), BearDogError> {
        if self.functions.remove(function_id).is_some() {
            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Function not found: {function_id}"
            )))
        }
    }

    /// Call a function by ID
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The function is not found
    /// - The function execution fails
    /// - Parameter validation fails
    pub fn call_function(
        &mut self,
        function_id: &str,
        parameters: Vec<FunctionParameter>,
    ) -> Result<FunctionResult, BearDogError> {
        if let Some(function) = self.functions.get_mut(function_id) {
            // Update last called timestamp
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

            self.safety_checker
                .check_function_call(&external_function, &parameter_values)?;

            // Log the call if enabled
            if self.config.enable_logging {
                info!(
                    "Calling function: {} with {} parameters",
                    function_id,
                    parameter_values.len()
                );
            }

            // Simulate function call (in real implementation, this would call the actual function)
            let start_time = std::time::Instant::now();
            let result = FunctionResult {
                success: true,
                value: Some(FunctionValue::String(
                    "Function called successfully".to_string(),
                )),
                error: None,
                #[allow(clippy::cast_possible_truncation)]
                execution_time_us: start_time.elapsed().as_micros().min(u128::from(u64::MAX))
                    as u64,
            };
            Ok(result)
        } else {
            Err(BearDogError::not_found(format!(
                "Function not found: {function_id}"
            )))
        }
    }

    /// Gets function information
    ///
    /// # Errors
    ///
    /// Returns an error if the function is not found in the registry
    pub fn get_function_info(&self, function_id: &str) -> Result<FunctionHandle, BearDogError> {
        self.functions
            .get(function_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Function not found: {function_id}")))
    }

    /// Checks if a library path is allowed
    /// Checks if library path allowed
    #[allow(dead_code)]
    fn is_library_path_allowed(&self, path: &str) -> bool {
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
    use crate::external_functions::types::{
        CallingConvention, FunctionMetadata, FunctionSignature, ParameterType, PerformanceInfo,
        ReturnType, SecurityInfo,
    };

    fn make_signature() -> FunctionSignature {
        FunctionSignature {
            parameters: vec![],
            return_type: ReturnType::Type(ParameterType::Int32),
            calling_convention: CallingConvention::C,
            attributes: vec![],
        }
    }

    fn make_metadata() -> FunctionMetadata {
        FunctionMetadata {
            description: "test".to_string(),
            safety_level: crate::external_functions::types::SafetyLevel::Safe,
            performance: PerformanceInfo::default(),
            security: SecurityInfo::default(),
            custom: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn test_registry_load_library() {
        let mut registry = ExternalFunctionRegistry::default();
        let result = registry.load_library("/path/to/lib.so");
        assert!(result.is_ok());
        let lib_id = result.unwrap();
        assert!(!lib_id.is_empty());
    }

    #[test]
    fn test_registry_get_library_info() {
        let mut registry = ExternalFunctionRegistry::default();
        let lib_id = registry.load_library("/test/lib.so").unwrap();
        let info = registry.get_library_info(&lib_id).unwrap();
        assert_eq!(info.id, lib_id);
        assert_eq!(info.name, "lib.so");
    }

    #[test]
    fn test_registry_get_library_info_not_found() {
        let registry = ExternalFunctionRegistry::default();
        let result = registry.get_library_info("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_unload_library() {
        let mut registry = ExternalFunctionRegistry::default();
        let lib_id = registry.load_library("/path/lib.so").unwrap();
        let result = registry.unload_library(&lib_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_registry_unload_library_not_found() {
        let mut registry = ExternalFunctionRegistry::default();
        let result = registry.unload_library("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_list_libraries() {
        let mut registry = ExternalFunctionRegistry::default();
        registry.load_library("/a/lib1.so").unwrap();
        registry.load_library("/b/lib2.so").unwrap();
        let libs = registry.list_libraries().unwrap();
        assert_eq!(libs.len(), 2);
    }

    #[test]
    fn test_registry_register_function() {
        let mut registry = ExternalFunctionRegistry::default();
        let lib_id = registry.load_library("/path/lib.so").unwrap();
        let sig = make_signature();
        let meta = make_metadata();
        let result = registry.register_function(&lib_id, "my_func", sig, meta);
        assert!(result.is_ok());
    }

    #[test]
    fn test_registry_register_function_library_not_found() {
        let mut registry = ExternalFunctionRegistry::default();
        let sig = make_signature();
        let meta = make_metadata();
        let result = registry.register_function("bad_lib", "my_func", sig, meta);
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_unregister_function() {
        let mut registry = ExternalFunctionRegistry::default();
        let lib_id = registry.load_library("/path/lib.so").unwrap();
        let func_id = registry
            .register_function(&lib_id, "my_func", make_signature(), make_metadata())
            .unwrap();
        let result = registry.unregister_function(&func_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_registry_unregister_function_not_found() {
        let mut registry = ExternalFunctionRegistry::default();
        let result = registry.unregister_function("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_get_function_info() {
        let mut registry = ExternalFunctionRegistry::default();
        let lib_id = registry.load_library("/path/lib.so").unwrap();
        let func_id = registry
            .register_function(&lib_id, "my_func", make_signature(), make_metadata())
            .unwrap();
        let info = registry.get_function_info(&func_id).unwrap();
        assert_eq!(info.name, "my_func");
    }

    #[test]
    fn test_registry_get_function_info_not_found() {
        let registry = ExternalFunctionRegistry::default();
        let result = registry.get_function_info("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_call_function() {
        let mut registry = ExternalFunctionRegistry::default();
        let lib_id = registry.load_library("/path/lib.so").unwrap();
        let func_id = registry
            .register_function(&lib_id, "my_func", make_signature(), make_metadata())
            .unwrap();
        let params = vec![];
        let result = registry.call_function(&func_id, params);
        assert!(result.is_ok());
        let res = result.unwrap();
        assert!(res.success);
    }

    #[test]
    fn test_registry_call_function_not_found() {
        let mut registry = ExternalFunctionRegistry::default();
        let result = registry.call_function("nonexistent", vec![]);
        assert!(result.is_err());
    }
}
