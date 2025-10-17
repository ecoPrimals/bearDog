//! Universal HSM Provider Module
//!
//! Provider implementations for universal HSMs

use beardog_errors::BearDogError;
use std::fmt::Debug;

/// Universal provider trait
pub trait UniversalProvider: Debug + Send + Sync {
    /// Provider name
    fn name(&self) -> &str;
    
    /// Check if available
    fn is_available(&self) -> bool;
}

/// Get available providers
pub fn get_providers() -> Vec<Box<dyn UniversalProvider>> {
    // TODO: Implement provider discovery
    Vec::new()
}

