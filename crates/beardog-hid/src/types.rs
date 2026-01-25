//! HID Types - Core data structures
//!
//! Defines the platform-agnostic HID device interface.

use beardog_errors::BearDogError;
use std::fmt;

/// USB Vendor ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VendorId(pub u16);

impl fmt::Display for VendorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04x}", self.0)
    }
}

/// USB Product ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProductId(pub u16);

impl fmt::Display for ProductId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04x}", self.0)
    }
}

/// HID Device Information
///
/// Contains metadata about a HID device discovered on the system.
#[derive(Debug, Clone)]
pub struct HidDeviceInfo {
    /// USB Vendor ID
    pub vendor_id: VendorId,
    
    /// USB Product ID
    pub product_id: ProductId,
    
    /// Manufacturer name (e.g., "SoloKeys", "Yubico")
    pub manufacturer: String,
    
    /// Product name (e.g., "Solo 2", "YubiKey 5")
    pub product: String,
    
    /// Serial number (may be empty)
    pub serial: String,
    
    /// System-specific device path
    /// - Linux: `/dev/hidraw0`, `/dev/hidraw1`, etc.
    /// - Android: USB device path
    pub path: String,
}

impl fmt::Display for HidDeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} (VID:{}, PID:{}) @ {}",
            self.manufacturer, self.product, self.vendor_id, self.product_id, self.path
        )
    }
}

/// HID Device Trait
///
/// Provides async read/write interface to HID devices.
/// All implementations must be Pure Rust (ecoBin compliant).
#[async_trait::async_trait]
pub trait HidDevice: Send + Sync {
    /// Write HID report to device
    ///
    /// # Arguments
    ///
    /// * `report` - Raw HID report data to send
    ///
    /// # Returns
    ///
    /// Number of bytes written
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Device is disconnected
    /// - I/O error occurs
    /// - Report is malformed
    async fn write(&mut self, report: &[u8]) -> Result<usize, BearDogError>;

    /// Read HID report from device
    ///
    /// # Arguments
    ///
    /// * `buf` - Buffer to read data into
    ///
    /// # Returns
    ///
    /// Number of bytes read
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Device is disconnected
    /// - I/O error occurs
    /// - Timeout occurs
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, BearDogError>;

    /// Get device information
    fn info(&self) -> &HidDeviceInfo;
}

/// Well-known FIDO2-compatible vendor IDs
pub mod fido2_vendors {
    use super::VendorId;

    /// SoloKeys (Solo 2, etc.)
    pub const SOLOKEYS: VendorId = VendorId(0x1209);

    /// Yubico (YubiKey 5 series, etc.)
    pub const YUBICO: VendorId = VendorId(0x1050);

    /// Google (Titan Security Key)
    pub const GOOGLE: VendorId = VendorId(0x096e);

    /// Feitian Technologies
    pub const FEITIAN: VendorId = VendorId(0x096e);
}

/// Well-known FIDO2-compatible product IDs
pub mod fido2_products {
    use super::ProductId;

    /// Solo 2 Security Key
    pub const SOLO2: ProductId = ProductId(0xbeee);
}

/// Check if a device is FIDO2-compatible based on VID/PID
///
/// # Example
///
/// ```rust
/// use beardog_hid::{VendorId, ProductId, types::is_fido2_device};
///
/// let vid = VendorId(0x1209);  // SoloKeys
/// let pid = ProductId(0xbeee); // Solo 2
/// assert!(is_fido2_device(vid, pid));
/// ```
#[must_use]
pub fn is_fido2_device(vendor_id: VendorId, product_id: ProductId) -> bool {
    use fido2_products::*;
    use fido2_vendors::*;

    match (vendor_id, product_id) {
        // SoloKeys
        (SOLOKEYS, SOLO2) => true,

        // Yubico (all YubiKey models support FIDO2)
        (YUBICO, _) => true,

        // Google Titan (specific products)
        (GOOGLE, ProductId(0x0858)) => true, // Titan Security Key
        (GOOGLE, ProductId(0x0859)) => true, // Titan Security Key (BLE)

        // Feitian
        (FEITIAN, _) => true,

        // Unknown
        _ => false,
    }
}

#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;

