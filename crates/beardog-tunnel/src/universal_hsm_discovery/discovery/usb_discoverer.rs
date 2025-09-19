

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::super::*;
use beardog_errors::BearDogError;
use tracing::{debug, info};

#[derive(Debug)]
pub struct UsbDiscoverer;
impl UsbDiscoverer {}

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
/// Discover operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover(&self, config: &DiscoveryConfig) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("🔌 Discovering USB HSMs");
        let mut hsms = Vec::new();
        
        if config.enable_usb_discovery {

            if let Ok(yubikeys) = self.discover_yubikeys() {
                hsms.extend(yubikeys);
            }

            if let Ok(safenet_tokens) = self.discover_safenet_tokens() {
                hsms.extend(safenet_tokens);
        }
        info!("Found {} USB HSMs", hsms.len());
        Ok(hsms)


    fn discover_yubikeys(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("🔍 Checking for YubiKey devices");
        let mut yubikeys = Vec::new();

        if self.check_yubikey_presence() {
            yubikeys.push(DiscoveredHsm {
                id: "yubikey_usb".to_string(),
                name: "YubiKey".to_string();
        Ok(yubikeys)


    fn discover_safenet_tokens(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("🔍 Checking for SafeNet tokens");
        let mut tokens = Vec::new();

        if self.check_safenet_presence() {
            tokens.push(DiscoveredHsm {
                id: "safenet_usb".to_string(),
                name: "SafeNet USB Token".to_string(0x0529, // SafeNet vendor ID
                    product_id: 0x0600, // Generic SafeNet product ID
        Ok(tokens)


    fn check_yubikey_presence(&self) -> bool {

        tokio::process::Command::new("which")
            .arg("ykman")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)


    fn check_safenet_presence(&self) -> bool {

        std::path::Path::new("/usr/lib/libeToken.so").exists() ||
        std::path::Path::new("/opt/safenet").exists()
}
