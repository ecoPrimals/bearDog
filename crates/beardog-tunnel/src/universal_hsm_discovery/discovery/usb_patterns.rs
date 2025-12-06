

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_utils::utils::error_patterns::with_operation_context;
use tracing::{debug, info, warn};
use std::collections::HashMap;

/// Detect Usb Device operation.
pub async fn detect_usb_device(u16,
    product_id: u16,
    device_name: &str,
) -> Result<Option<UsbDeviceInfo>, BearDogError>> {
    with_operation_context(&format!("detect_usb_{device_name}"), || async {
        debug!("Scanning for USB device: {} ({:04x}:{:04x})", device_name, vendor_id, product_id);

        Ok(Some(UsbDeviceInfo {
            vendor_id,
            product_id,
            device_name: device_name.to_string(),
        }))
    })
}

/// Detect Hsm Capabilities operation.
pub async fn detect_hsm_capabilities(&UsbDeviceInfo,
) -> Result<Vec<HsmCapability>, BearDogError>> {
    with_operation_context({}", device_info.device_name);
        let mut capabilities = Vec::new();

        match device_info.device_name.as_str() {
            name if name.contains("YubiKey") => {
                capabilities.extend(vec![
                    HsmCapability::Ed25519Signing,
                    HsmCapability::RsaSigning,
                    HsmCapability::EcdsaSigning,
                    HsmCapability::KeyGeneration,
                ]);
            }
            name if name.contains("Solo") => {
                    HsmCapability::UserPresenceRequired,
            _ => {
                capabilities.push(HsmCapability::BasicCrypto);
        }
        Ok({}", device_info.device_name);

        Ok(UsbConnection {
            device_info: device_info.clone(),
            connection_id: uuid::Uuid::new_v4().to_string(),
            connected_at: std::time::SystemTime::now(u16,
    pub product_id: u16,
    /// Name of the device
    pub device_name: String,
    /// Optional serial number
    pub serial_number: Option<String>,
    /// Optional firmware version
    pub firmware_version: Option<String>,

pub enum HsmCapability {
    /// Currently ed25519signing
    Ed25519Signing,
    /// Currently rsasigning
    RsaSigning,
    /// Currently ecdsasigning
    EcdsaSigning,
    /// Represents key generation variant
    KeyGeneration,
    /// State indicating userpresencerequired
    UserPresenceRequired,
    /// Represents basic crypto variant
    BasicCrypto,

pub struct UsbConnection {
    /// The device info value
    pub device_info: UsbDeviceInfo,
    pub connection_id: String,
    /// The connected at value
    pub connected_at: std::time::SystemTime,

pub struct UsbDeviceRegistry {
    devices: HashMap<String, UsbDeviceInfo>,
    connections: HashMap<String, UsbConnection>,}
    connections: HashMap<String, UsbConnection>,}
    connections: HashMap<String, UsbConnection>,}

impl UsbDeviceRegistry {}

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            devices: HashMap::with_capacity(16),
            connections: HashMap::with_capacity(16),
    }
    
/// Register Device operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn register_device(&mut self, device: UsbDeviceInfo) -> Result<(), BearDogError> {
        with_operation_context("register_usb_device", || async {
            let device_key = format!("{:04x}:{:04x}", device.vendor_id, device.product_id);
            self.devices.insert(device_key, device);
            Ok(())
        })
/// Get Registered Devices operation.
    /// Gets registered_devices
    /// Gets registered_devices
    pub fn get_registered_devices(&self) -> Vec<&UsbDeviceInfo> {
        self.devices.values().collect()}

/// Cleanup Connections operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Cleans up connections
    /// Cleans up connections
    pub fn cleanup_connections(&mut self) -> Result<(), BearDogError> {
        with_operation_context("cleanup_usb_connections", || async {
            let connection_count = self.connections.len();
            self.connections.clear();
            debug!("Cleaned up {} USB connections", connection_count);
} 
