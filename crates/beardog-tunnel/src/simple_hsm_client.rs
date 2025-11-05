//! Simple PKCS#11 Client for MVP
//!
//! Provides direct, no-frills access to PKCS#11 devices for the CLI.
//! This is a pragmatic implementation that bypasses the trait layer for simplicity.

use beardog_errors::{constructors_unified::system_error, BearDogError, SystemErrorCategory};
use cryptoki::context::{CInitializeArgs, Pkcs11};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Simple HSM device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleHsmDevice {
    pub slot_id: u64,
    pub label: String,
    pub manufacturer: String,
    pub model: String,
    pub serial_number: String,
    pub has_token: bool,
}

/// Simple PKCS#11 client for direct hardware access
pub struct SimplePkcs11Client {
    pkcs11: Arc<Mutex<Option<Pkcs11>>>,
    library_path: String,
}

impl SimplePkcs11Client {
    /// Create a new client (not yet initialized)
    pub fn new(library_path: String) -> Self {
        Self {
            pkcs11: Arc::new(Mutex::new(None)),
            library_path,
        }
    }

    /// Initialize the PKCS#11 library
    pub fn initialize(&self) -> Result<(), BearDogError> {
        let pkcs11 = Pkcs11::new(&self.library_path).map_err(|e| {
            system_error(
                &format!(
                    "Failed to load PKCS#11 library '{}': {}",
                    self.library_path, e
                ),
                SystemErrorCategory::General,
            )
        })?;

        pkcs11.initialize(CInitializeArgs::OsThreads).map_err(|e| {
            system_error(
                &format!("Failed to initialize PKCS#11: {}", e),
                SystemErrorCategory::General,
            )
        })?;

        *self.pkcs11.lock().map_err(|e| {
            system_error(
                &format!("Mutex lock poisoned: {}", e),
                SystemErrorCategory::General,
            )
        })? = Some(pkcs11);
        Ok(())
    }

    /// Get list of available HSM devices
    pub fn list_devices(&self) -> Result<Vec<SimpleHsmDevice>, BearDogError> {
        let pkcs11_guard = self.pkcs11.lock().map_err(|e| {
            system_error(
                &format!("Mutex lock poisoned: {}", e),
                SystemErrorCategory::General,
            )
        })?;
        let pkcs11 = pkcs11_guard.as_ref().ok_or_else(|| {
            system_error(
                "PKCS#11 not initialized. Call initialize() first",
                SystemErrorCategory::General,
            )
        })?;

        let slots = pkcs11.get_slots_with_token().map_err(|e| {
            system_error(
                &format!("Failed to get slots: {}", e),
                SystemErrorCategory::General,
            )
        })?;

        let mut devices = Vec::new();

        for slot in slots {
            if let Ok(token_info) = pkcs11.get_token_info(slot) {
                devices.push(SimpleHsmDevice {
                    slot_id: slot.try_into().unwrap_or(0),
                    label: token_info.label().trim().to_string(),
                    manufacturer: token_info.manufacturer_id().trim().to_string(),
                    model: token_info.model().trim().to_string(),
                    serial_number: token_info.serial_number().trim().to_string(),
                    has_token: true,
                });
            }
        }

        Ok(devices)
    }

    /// Collect random entropy from a specific slot
    pub fn collect_entropy(&self, slot_id: u64, size: usize) -> Result<Vec<u8>, BearDogError> {
        let pkcs11_guard = self.pkcs11.lock().map_err(|e| {
            system_error(
                &format!("Mutex lock poisoned: {}", e),
                SystemErrorCategory::General,
            )
        })?;
        let pkcs11 = pkcs11_guard
            .as_ref()
            .ok_or_else(|| system_error("PKCS#11 not initialized", SystemErrorCategory::General))?;

        let slot = cryptoki::slot::Slot::try_from(slot_id).map_err(|e| {
            system_error(
                &format!("Invalid slot ID: {}", e),
                SystemErrorCategory::General,
            )
        })?;

        let session = pkcs11.open_ro_session(slot).map_err(|e| {
            system_error(
                &format!("Failed to open session on slot {}: {}", slot_id, e),
                SystemErrorCategory::General,
            )
        })?;

        // Generate random bytes
        let mut random_data = vec![0u8; size];
        session
            .generate_random_slice(&mut random_data)
            .map_err(|e| {
                system_error(
                    &format!("Failed to generate random data: {}", e),
                    SystemErrorCategory::General,
                )
            })?;

        Ok(random_data)
    }

    /// Finalize and clean up
    pub fn finalize(&self) -> Result<(), BearDogError> {
        let mut pkcs11_guard = self.pkcs11.lock().map_err(|e| {
            system_error(
                &format!("Mutex lock poisoned: {}", e),
                SystemErrorCategory::General,
            )
        })?;
        if let Some(pkcs11) = pkcs11_guard.take() {
            pkcs11.finalize();
        }
        Ok(())
    }
}

impl Drop for SimplePkcs11Client {
    fn drop(&mut self) {
        let _ = self.finalize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = SimplePkcs11Client::new("/usr/lib/opensc-pkcs11.so".to_string());
        // Just verify it creates without panic
        assert!(client.pkcs11.lock().is_ok());
    }
}
