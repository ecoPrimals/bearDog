// SPDX-License-Identifier: AGPL-3.0-or-later

//! HID Types - Core data structures
//!
//! Defines the platform-agnostic HID device interface.

use beardog_errors::BearDogError;
use std::fmt;
use std::future::Future;

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

/// FIDO Alliance CTAPHID usage page (`0xF1D0`).
pub const FIDO_USAGE_PAGE: u16 = 0xF1D0;

/// HID Device Information
///
/// Contains metadata about a HID device discovered on the system.
#[derive(Debug, Clone)]
pub struct HidDeviceInfo {
    /// USB Vendor ID
    pub vendor_id: VendorId,

    /// USB Product ID
    pub product_id: ProductId,

    /// Manufacturer name (e.g., `SoloKeys`, `Yubico`)
    pub manufacturer: String,

    /// Product name (e.g., `Solo 2`, `YubiKey 5`)
    pub product: String,

    /// Serial number (may be empty)
    pub serial: String,

    /// System-specific device path
    /// - Linux: `/dev/hidraw0`, `/dev/hidraw1`, etc.
    /// - Android: USB device path
    pub path: String,

    /// HID usage page from the report descriptor (if parsed).
    /// FIDO2/CTAPHID interfaces use `0xF1D0`.
    pub usage_page: Option<u16>,
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
    fn write<'a>(
        &'a mut self,
        report: &'a [u8],
    ) -> impl Future<Output = Result<usize, BearDogError>> + Send + 'a;

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
    fn read<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> impl Future<Output = Result<usize, BearDogError>> + Send + 'a;

    /// Get device information
    fn info(&self) -> &HidDeviceInfo;
}

/// Well-known FIDO2-compatible vendor IDs
pub mod fido2_vendors {
    use super::VendorId;

    /// `SoloKeys` (Solo 2, etc.)
    pub const SOLOKEYS: VendorId = VendorId(0x1209);

    /// `Yubico` (`YubiKey` 5 series, etc.)
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

    /// Solo 1 / Somu
    pub const SOLO1: ProductId = ProductId(0xb000);

    /// Google Titan Security Key (USB-A)
    pub const TITAN_USB_A: ProductId = ProductId(0x0858);

    /// Google Titan Security Key (USB-C)
    pub const TITAN_USB_C: ProductId = ProductId(0x0859);
}

/// Human-readable FIDO2 vendor and product names
pub mod fido2_names {
    /// `SoloKeys` manufacturer label
    pub const MANUFACTURER_SOLOKEYS: &str = "SoloKeys";
    /// Yubico manufacturer label
    pub const MANUFACTURER_YUBICO: &str = "Yubico";
    /// Google / Feitian shared VID manufacturer label
    pub const MANUFACTURER_GOOGLE_FEITIAN: &str = "Google/Feitian";

    /// Solo 2 Security Key product label
    pub const PRODUCT_SOLO2: &str = "Solo 2 Security Key";
    /// Solo 1 / Somu product label
    pub const PRODUCT_SOLO1: &str = "Solo 1 / Somu";
    /// `YubiKey` product label (all PIDs)
    pub const PRODUCT_YUBIKEY: &str = "YubiKey";
    /// Google Titan Security Key product label
    pub const PRODUCT_TITAN: &str = "Titan Security Key";
}

/// Resolve a FIDO2 manufacturer name from USB vendor ID.
#[must_use]
pub const fn fido2_manufacturer_name(vendor_id: VendorId) -> Option<&'static str> {
    use fido2_names::{
        MANUFACTURER_GOOGLE_FEITIAN, MANUFACTURER_SOLOKEYS, MANUFACTURER_YUBICO,
    };
    use fido2_vendors::{GOOGLE, SOLOKEYS, YUBICO};

    match vendor_id {
        SOLOKEYS => Some(MANUFACTURER_SOLOKEYS),
        YUBICO => Some(MANUFACTURER_YUBICO),
        GOOGLE => Some(MANUFACTURER_GOOGLE_FEITIAN),
        _ => None,
    }
}

/// Resolve a FIDO2 product name from USB vendor/product IDs.
#[must_use]
pub const fn fido2_product_name(vendor_id: VendorId, product_id: ProductId) -> Option<&'static str> {
    use fido2_names::{PRODUCT_SOLO1, PRODUCT_SOLO2, PRODUCT_TITAN, PRODUCT_YUBIKEY};
    use fido2_products::{SOLO1, SOLO2, TITAN_USB_A, TITAN_USB_C};
    use fido2_vendors::{GOOGLE, SOLOKEYS, YUBICO};

    match (vendor_id, product_id) {
        (SOLOKEYS, SOLO2) => Some(PRODUCT_SOLO2),
        (SOLOKEYS, SOLO1) => Some(PRODUCT_SOLO1),
        (YUBICO, _) => Some(PRODUCT_YUBIKEY),
        (GOOGLE, TITAN_USB_A | TITAN_USB_C) => Some(PRODUCT_TITAN),
        _ => None,
    }
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
#[expect(
    clippy::unnested_or_patterns,
    reason = "clarity: separate match arms for distinct HID types"
)]
pub const fn is_fido2_device(vendor_id: VendorId, product_id: ProductId) -> bool {
    use fido2_products::{SOLO2, TITAN_USB_A, TITAN_USB_C};
    use fido2_vendors::{FEITIAN, GOOGLE, SOLOKEYS, YUBICO};

    match (vendor_id, product_id) {
        // SoloKeys Solo 2, Yubico (all models), Google Titan (specific), Feitian (all models)
        (SOLOKEYS, SOLO2) | (YUBICO, _) | (GOOGLE, TITAN_USB_A) | (GOOGLE, TITAN_USB_C)
        | (FEITIAN, _) => true,
        _ => false,
    }
}

/// Check if a [`HidDeviceInfo`] represents a FIDO2 CTAPHID interface.
///
/// When the usage page is available, only the FIDO Alliance usage page (`0xF1D0`)
/// qualifies. When the usage page is unknown (older kernels / non-Linux), falls
/// back to VID/PID matching via [`is_fido2_device`].
#[must_use]
pub fn is_fido2_interface(info: &HidDeviceInfo) -> bool {
    if let Some(page) = info.usage_page {
        page == FIDO_USAGE_PAGE
    } else {
        is_fido2_device(info.vendor_id, info.product_id)
    }
}

/// Parse the HID usage page from a raw report descriptor.
///
/// Scans for the Usage Page item (`0x06 LO HI` for 2-byte, `0x05 VAL` for 1-byte)
/// at the top level and returns the first one found.
#[must_use]
pub fn parse_usage_page_from_descriptor(descriptor: &[u8]) -> Option<u16> {
    let mut i = 0;
    while i < descriptor.len() {
        let prefix = descriptor[i];
        let size = (prefix & 0x03) as usize;
        let tag = prefix & 0xFC;

        // Usage Page short item: tag 0x04 (bType=Global, bTag=0x00)
        // 1-byte: prefix=0x05, 2-byte: prefix=0x06
        if tag == 0x04 {
            if size == 1 && i + 1 < descriptor.len() {
                return Some(u16::from(descriptor[i + 1]));
            }
            if size == 2 && i + 2 < descriptor.len() {
                return Some(u16::from_le_bytes([descriptor[i + 1], descriptor[i + 2]]));
            }
        }

        // Long items (prefix 0xFE) have a different layout
        if prefix == 0xFE && i + 2 < descriptor.len() {
            let data_size = descriptor[i + 1] as usize;
            i += 3 + data_size;
        } else {
            i += 1 + size;
        }
    }
    None
}

#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;
