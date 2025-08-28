

use super::super::*;
use beardog_errors::BearDogError;
use tracing::{debug, info};

#[derive(Debug)]
pub struct UsbDiscoverer;
impl UsbDiscoverer {}

    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
    pub async fn discover(&self, config: &DiscoveryConfig) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("🔌 Discovering USB HSMs");
        let mut hsms = Vec::new();
        
        if config.enable_usb_discovery {

            if let Ok(yubikeys) = self.discover_yubikeys().await {
                hsms.extend(yubikeys);
            }

            if let Ok(safenet_tokens) = self.discover_safenet_tokens().await {
                hsms.extend(safenet_tokens);
        }
        info!("Found {} USB HSMs", hsms.len());
        Ok(hsms)

    async fn discover_yubikeys(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("🔍 Checking for YubiKey devices");
        let mut yubikeys = Vec::new();

        if self.check_yubikey_presence().await {
            yubikeys.push(DiscoveredHsm {
                id: "yubikey_usb".to_string(),
                name: "YubiKey".to_string(),
                hsm_type: HsmType::Usb,
                connection_info: ConnectionInfo::Usb {
                    vendor_id: 0x1050, // Yubico vendor ID
                    product_id: 0x0010, // Generic YubiKey product ID
                    serial_number: None,
                },
                capabilities: vec![
                    HsmCapability::KeyGeneration,
                    HsmCapability::Signing,
                    HsmCapability::Encryption,
                    HsmCapability::Authentication,
                ],
                status: HsmStatus::Available,
            });
        Ok(yubikeys)

    async fn discover_safenet_tokens(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("🔍 Checking for SafeNet tokens");
        let mut tokens = Vec::new();

        if self.check_safenet_presence().await {
            tokens.push(DiscoveredHsm {
                id: "safenet_usb".to_string(),
                name: "SafeNet USB Token".to_string(),
                    vendor_id: 0x0529, // SafeNet vendor ID
                    product_id: 0x0600, // Generic SafeNet product ID
        Ok(tokens)

    async fn check_yubikey_presence(&self) -> bool {

        tokio::process::Command::new("which")
            .arg("ykman")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)

    async fn check_safenet_presence(&self) -> bool {

        std::path::Path::new("/usr/lib/libeToken.so").exists() ||
        std::path::Path::new("/opt/safenet").exists()
}
