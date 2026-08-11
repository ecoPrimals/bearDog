// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used, reason = "expect/unwrap acceptable for invariant failures in tests and bootstrap code"))]

//! Pure Rust HID Interface - ecoBin Compliant
//!
//! Provides universal HID (Human Interface Device) access without C dependencies.
//!
//! # ecoBin Compliance
//!
//! This crate is **100% Pure Rust** with **ZERO C dependencies**:
//! - ✅ No `hidapi` (C library)
//! - ✅ No `libusb` (C library)
//! - ✅ Direct `/dev/hidraw` access on Linux (Pure Rust file I/O)
//! - ✅ JNI bindings for Android (Pure Rust)
//! - ✅ Zero `libc` dependency — pure `std::fs` + `spawn_blocking`
//!
//! # Platform Support
//!
//! - **Linux**: Direct `/dev/hidraw` access (no libusb!)
//! - **Android**: Via existing `beardog-tunnel/android_strongbox` (JNI)
//! - **Other**: Compile error with instructions to add support
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │  beardog-hid (Pure Rust)               │
//! ├─────────────────────────────────────────┤
//! │  Linux:   /dev/hidraw + sysfs          │
//! │  Android: JNI (existing)               │
//! └─────────────────────────────────────────┘
//!           │
//!           ▼
//! ┌─────────────────────────────────────────┐
//! │  OS Kernel (HID subsystem)              │
//! └─────────────────────────────────────────┘
//!           │
//!           ▼
//! ┌─────────────────────────────────────────┐
//! │  Hardware (SoloKey, YubiKey, etc.)      │
//! └─────────────────────────────────────────┘
//! ```
//!
//! # Example
//!
//! ```rust,no_run
//! use beardog_hid::{discover, HidDevice};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Discover all HID devices (Pure Rust!)
//!     let devices = discover().await?;
//!     
//!     for device_info in devices {
//!         println!("Found: {} - {}",
//!             device_info.manufacturer,
//!             device_info.product
//!         );
//!         println!("  VID: 0x{:04x}, PID: 0x{:04x}",
//!             device_info.vendor_id.0,
//!             device_info.product_id.0
//!         );
//!         println!("  Path: {}", device_info.path);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! # FIDO2/CTAP2 Usage
//!
//! This crate is designed specifically for `BearDog`'s FIDO2 support:
//!
//! ```rust,ignore
//! use beardog_hid::{discover, open_device, HidDevice};
//!
//! // Find FIDO2 devices
//! let devices = discover().await?;
//! let fido2_device = devices.into_iter()
//!     .find(|d| d.vendor_id.0 == 0x1209 && d.product_id.0 == 0xbeee) // SoloKey
//!     .ok_or("No SoloKey found")?;
//!
//! // Open device (Pure Rust!)
//! let mut device = open_device(&fido2_device.path).await?;
//!
//! // Send CTAP2 command
//! let command = build_ctap2_command();
//! device.write(&command).await?;
//!
//! // Read response
//! let mut response = vec![0u8; 64];
//! let n = device.read(&mut response).await?;
//! ```

#![warn(clippy::all)]

mod hid_device_backend;
pub mod linux;
pub mod types;

#[cfg(test)]
mod linux_tests;

pub use hid_device_backend::HidDeviceBackend;
pub use types::{HidDevice, HidDeviceInfo, ProductId, VendorId, FIDO_USAGE_PAGE};

/// Discover all HID devices on the system
///
/// This function is platform-specific but always Pure Rust:
/// - **Linux**: Scans `/dev/hidraw*` and reads `/sys/class/hidraw/` (Pure Rust file I/O)
/// - **Android**: Uses existing `StrongBox` integration (JNI, Pure Rust)
///
/// # Errors
///
/// Returns error if:
/// - Unable to read `/dev` directory (Linux)
/// - Unable to parse device information
/// - Platform not supported
///
/// # Example
///
/// ```rust,no_run
/// use beardog_hid::discover;
///
/// #[tokio::main]
/// async fn main() {
///     match discover().await {
///         Ok(devices) => {
///             println!("Found {} HID device(s)", devices.len());
///             for dev in devices {
///                 println!("  {} - {}", dev.manufacturer, dev.product);
///             }
///         }
///         Err(e) => eprintln!("Discovery failed: {}", e),
///     }
/// }
/// ```
pub async fn discover() -> Result<Vec<HidDeviceInfo>, beardog_errors::BearDogError> {
    #[cfg(target_os = "linux")]
    {
        linux::discover_hidraw().await
    }

    #[cfg(not(target_os = "linux"))]
    {
        Err(beardog_errors::BearDogError::unsupported_platform(
            "HID device discovery is only supported on Linux (/dev/hidraw)",
        ))
    }
}

/// Open a HID device by path
///
/// # Arguments
///
/// * `path` - Device path (e.g., `/dev/hidraw0` on Linux)
///
/// # Errors
///
/// Returns error if:
/// - Device path doesn't exist
/// - Insufficient permissions
/// - Device is already open
///
/// # Example
///
/// ```rust,no_run
/// use beardog_hid::{discover, open_device, HidDevice};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let devices = discover().await?;
///     if let Some(dev) = devices.first() {
///         let mut device = open_device(&dev.path).await?;
///         
///         // Use device...
///         let data = vec![0x01, 0x02, 0x03];
///         device.write(&data).await?;
///     }
///     Ok(())
/// }
/// ```
pub async fn open_device(path: &str) -> Result<HidDeviceBackend, beardog_errors::BearDogError> {
    #[cfg(target_os = "linux")]
    {
        let device = linux::LinuxHidDevice::open(path).await?;
        Ok(HidDeviceBackend::Linux(device))
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = path; // Suppress unused warning
        Err(beardog_errors::BearDogError::unsupported_platform(
            "HID device opening not yet implemented for this platform",
        ))
    }
}
