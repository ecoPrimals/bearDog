// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

pub struct EvictionHandler;

impl Default for EvictionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EvictionHandler {
    pub const fn new() -> Self {
        Self
    }

    /// Evict operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn evict(&self, _key: &str) -> Result<(), BearDogError> {
        Ok(())
    }
}

pub struct WarmingHandler;

impl Default for WarmingHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl WarmingHandler {
    pub const fn new() -> Self {
        Self
    }

    /// Warm Cache operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn warm_cache(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
