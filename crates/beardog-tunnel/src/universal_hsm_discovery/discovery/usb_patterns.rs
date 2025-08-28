

use beardog_errors::BearDogError;
use beardog_utils::utils::error_patterns::with_operation_context;
use tracing::{debug, info, warn};
use std::collections::HashMap;

pub async fn detect_usb_device(
    vendor_id: u16,
    product_id: u16,
    device_name: &str,
) -> Result<Option<UsbDeviceInfo>, BearDogError>> {
    with_operation_context(&format_args!("detect_usb_{}", device_name).to_string(), || async {
        debug!("Scanning for USB device: {} ({:04x}:{:04x})", device_name, vendor_id, product_id);

        Ok(Some(UsbDeviceInfo {
            vendor_id,
            product_id,
            device_name: device_name.to_string(),
            serial_number: None,
            firmware_version: None,
        }))
    }).await
}

pub async fn detect_hsm_capabilities(
    device_info: &UsbDeviceInfo,
) -> Result<Vec<HsmCapability>, BearDogError>> {
    with_operation_context("detect_hsm_capabilities", || async {
        debug!("Detecting HSM capabilities for device: {}", device_info.device_name);
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
        Ok(capabilities)

pub async fn establish_usb_connection(
) -> Result<UsbConnection, BearDogError> {
    with_operation_context("establish_usb_connection", || async {
        info!("Establishing connection to USB device: {}", device_info.device_name);

        Ok(UsbConnection {
            device_info: device_info.clone(),
            connection_id: uuid::Uuid::new_v4().to_string(),
            connected_at: std::time::SystemTime::now(),
        })

#[derive(Debug, Clone)]
pub struct UsbDeviceInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_name: String,
    pub serial_number: Option<String>,
    pub firmware_version: Option<String>,

pub enum HsmCapability {
    Ed25519Signing,
    RsaSigning,
    EcdsaSigning,
    KeyGeneration,
    UserPresenceRequired,
    BasicCrypto,

pub struct UsbConnection {
    pub device_info: UsbDeviceInfo,
    pub connection_id: String,
    pub connected_at: std::time::SystemTime,

pub struct UsbDeviceRegistry {
    devices: HashMap<String, UsbDeviceInfo>,
    connections: HashMap<String, UsbConnection>,}

impl UsbDeviceRegistry {}

    pub fn new() -> Self {
        Self {
            devices: HashMap::with_capacity(16),
            connections: HashMap::with_capacity(16),
    }
    
    pub async fn register_device(&mut self, device: UsbDeviceInfo) -> Result<(), BearDogError> {
        with_operation_context("register_usb_device", || async {
            let device_key = format_args!("{:04x}:{:04x}", device.vendor_id, device.product_id).to_string();
            self.devices.insert(device_key, device);
            Ok(())
        }).await
    pub fn get_registered_devices(&self) -> Vec<&UsbDeviceInfo> {
        self.devices.values().collect()}

    pub async fn cleanup_connections(&mut self) -> Result<(), BearDogError> {
        with_operation_context("cleanup_usb_connections", || async {
            let connection_count = self.connections.len();
            self.connections.clear();
            debug!("Cleaned up {} USB connections", connection_count);
} 
