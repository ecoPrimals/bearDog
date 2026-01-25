//! Pure Rust HID for Linux via `/dev/hidraw`
//!
//! Direct access to HID devices without libusb or libhidapi (C libraries).
//! Uses standard Rust file I/O to interact with `/dev/hidraw*` devices.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │  beardog-hid (Pure Rust)               │
//! │  - tokio::fs::File                      │
//! │  - /dev/hidraw* access                  │
//! │  - /sys/class/hidraw/* parsing          │
//! └─────────────────────────────────────────┘
//!           │
//!           ▼ (Pure Rust file I/O)
//! ┌─────────────────────────────────────────┐
//! │  Linux Kernel HID Subsystem             │
//! │  - hidraw driver                        │
//! │  - USB HID class driver                 │
//! └─────────────────────────────────────────┘
//!           │
//!           ▼
//! ┌─────────────────────────────────────────┐
//! │  Hardware (SoloKey, YubiKey, etc.)      │
//! └─────────────────────────────────────────┘
//! ```
//!
//! # Permissions
//!
//! To access `/dev/hidraw*` as non-root, add udev rules:
//!
//! ```text
//! # /etc/udev/rules.d/70-solokey.rules
//! SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1209", ATTRS{idProduct}=="beee", MODE="0660", TAG+="uaccess"
//! ```
//!
//! Then reload: `sudo udevadm control --reload-rules && sudo udevadm trigger`

use super::types::{HidDevice, HidDeviceInfo, ProductId, VendorId};
use beardog_errors::BearDogError;
use std::path::PathBuf;
use tokio::fs::{File, OpenOptions, read_dir, read_to_string};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{debug, trace, warn};

/// Linux HID device via `/dev/hidraw`
///
/// Provides Pure Rust access to HID devices using standard file I/O.
pub struct LinuxHidDevice {
    /// Async file handle to `/dev/hidrawN`
    device: File,

    /// Device metadata
    info: HidDeviceInfo,
}

impl LinuxHidDevice {
    /// Open a HID device by path (Pure Rust)
    ///
    /// # Arguments
    ///
    /// * `path` - Device path (e.g., `/dev/hidraw0`)
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Path doesn't exist
    /// - Insufficient permissions
    /// - Cannot read device metadata
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog_hid::linux::LinuxHidDevice;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let device = LinuxHidDevice::open("/dev/hidraw0").await?;
    ///     println!("Opened: {}", device.info().product);
    ///     Ok(())
    /// }
    /// ```
    pub async fn open(path: &str) -> Result<Self, BearDogError> {
        debug!("Opening HID device: {}", path);

        // Pure Rust file I/O - no C libraries!
        let device = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK) // Only libc usage - for flags
            .open(path)
            .await
            .map_err(|e| {
                BearDogError::io_error(&format!(
                    "Failed to open HID device {}: {}. \
                     Check permissions and udev rules.",
                    path, e
                ))
            })?;

        // Read device metadata from sysfs (Pure Rust)
        let info = Self::read_device_info(path).await?;

        debug!(
            "Opened HID device: {} {} (VID:{}, PID:{})",
            info.manufacturer, info.product, info.vendor_id, info.product_id
        );

        Ok(Self { device, info })
    }

    /// Read device info from `/sys/class/hidraw/` (Pure Rust)
    ///
    /// Parses sysfs files to extract device metadata.
    async fn read_device_info(path: &str) -> Result<HidDeviceInfo, BearDogError> {
        // Extract hidrawN from /dev/hidrawN
        let dev_name = PathBuf::from(path)
            .file_name()
            .ok_or_else(|| BearDogError::invalid_input("Invalid hidraw path"))?
            .to_string_lossy()
            .to_string();

        trace!("Reading device info for: {}", dev_name);

        // Sysfs paths
        let hidraw_path = format!("/sys/class/hidraw/{}", dev_name);
        let device_path = format!("{}/device", hidraw_path);

        // Read IDs from uevent file (Pure Rust file I/O)
        let uevent_path = format!("{}/uevent", device_path);
        let uevent_content = read_to_string(&uevent_path).await.map_err(|e| {
            BearDogError::io_error(&format!(
                "Failed to read uevent file {}: {}. \
                 Device may not be USB HID.",
                uevent_path, e
            ))
        })?;

        // Parse uevent file for HID_ID line
        // Format: HID_ID=0003:00001209:0000BEEE (bus:vendor:product)
        let (vendor_id, product_id) = parse_hid_id(&uevent_content)?;

        // Read string descriptors (may fail for some devices)
        let manufacturer =
            read_string_file(&format!("{}/manufacturer", device_path))
                .await
                .unwrap_or_else(|_| "Unknown".to_string());

        let product = read_string_file(&format!("{}/product", device_path))
            .await
            .unwrap_or_else(|_| "Unknown".to_string());

        let serial = read_string_file(&format!("{}/serial", device_path))
            .await
            .unwrap_or_else(|_| String::new());

        Ok(HidDeviceInfo {
            vendor_id: VendorId(vendor_id),
            product_id: ProductId(product_id),
            manufacturer,
            product,
            serial,
            path: path.to_string(),
        })
    }
}

#[async_trait::async_trait]
impl HidDevice for LinuxHidDevice {
    /// Write HID report to device (Pure Rust)
    async fn write(&mut self, report: &[u8]) -> Result<usize, BearDogError> {
        trace!("Writing {} bytes to HID device", report.len());

        self.device
            .write_all(report)
            .await
            .map_err(|e| BearDogError::io_error(&format!("HID write failed: {}", e)))?;

        Ok(report.len())
    }

    /// Read HID report from device (Pure Rust)
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, BearDogError> {
        trace!("Reading up to {} bytes from HID device", buf.len());

        let n = self
            .device
            .read(buf)
            .await
            .map_err(|e| BearDogError::io_error(&format!("HID read failed: {}", e)))?;

        trace!("Read {} bytes from HID device", n);
        Ok(n)
    }

    fn info(&self) -> &HidDeviceInfo {
        &self.info
    }
}

/// Discover all `/dev/hidraw*` devices (Pure Rust)
///
/// Scans `/dev` for hidraw devices and reads their metadata from sysfs.
///
/// # Errors
///
/// Returns error if unable to read `/dev` directory.
///
/// # Example
///
/// ```rust,no_run
/// use beardog_hid::linux::discover_hidraw;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let devices = discover_hidraw().await?;
///     println!("Found {} HID device(s)", devices.len());
///     for dev in devices {
///         println!("  {} at {}", dev.product, dev.path);
///     }
///     Ok(())
/// }
/// ```
pub async fn discover_hidraw() -> Result<Vec<HidDeviceInfo>, BearDogError> {
    debug!("Discovering HID devices via /dev/hidraw*");

    let mut devices = Vec::new();

    // Pure Rust directory iteration
    let mut entries = read_dir("/dev")
        .await
        .map_err(|e| BearDogError::io_error(&format!("Failed to read /dev: {}", e)))?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| BearDogError::io_error(&format!("Failed to iterate /dev: {}", e)))?
    {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        // Look for hidrawN files
        if name.starts_with("hidraw") {
            let path = entry.path().to_string_lossy().to_string();
            trace!("Found hidraw device: {}", path);

            // Try to read device info
            match LinuxHidDevice::read_device_info(&path).await {
                Ok(info) => {
                    debug!("  {}", info);
                    devices.push(info);
                }
                Err(e) => {
                    // Some hidraw devices may not be accessible or USB HID
                    warn!("Skipping {} (cannot read info): {}", path, e);
                }
            }
        }
    }

    debug!("Discovery complete: found {} HID device(s)", devices.len());
    Ok(devices)
}

// ============================================================================
// Helper Functions (Pure Rust)
// ============================================================================

/// Parse HID_ID from uevent file
///
/// Format: `HID_ID=0003:00001209:0000BEEE` (bus:vendor:product)
///
/// Returns (vendor_id, product_id)
fn parse_hid_id(uevent_content: &str) -> Result<(u16, u16), BearDogError> {
    for line in uevent_content.lines() {
        if let Some(hid_id) = line.strip_prefix("HID_ID=") {
            // Parse format: BBBB:VVVVVVVV:PPPPPPPP
            let parts: Vec<&str> = hid_id.split(':').collect();
            if parts.len() == 3 {
                let vendor_str = parts[1];
                let product_str = parts[2];

                let vendor_id = u16::from_str_radix(vendor_str, 16).map_err(|e| {
                    BearDogError::invalid_input(&format!("Invalid vendor ID {}: {}", vendor_str, e))
                })?;

                let product_id = u16::from_str_radix(product_str, 16).map_err(|e| {
                    BearDogError::invalid_input(&format!("Invalid product ID {}: {}", product_str, e))
                })?;

                return Ok((vendor_id, product_id));
            }
        }
    }

    Err(BearDogError::invalid_input(
        "HID_ID not found in uevent file",
    ))
}

/// Read a string file from sysfs (Pure Rust)
async fn read_string_file(path: &str) -> Result<String, BearDogError> {
    let content = read_to_string(path)
        .await
        .map_err(|e| BearDogError::io_error(&format!("Failed to read {}: {}", path, e)))?;

    Ok(content.trim().to_string())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hid_id_solokey() {
        let uevent = "HID_ID=0003:00001209:0000BEEE\n";
        let (vid, pid) = parse_hid_id(uevent).unwrap();
        assert_eq!(vid, 0x1209); // SoloKeys
        assert_eq!(pid, 0xBEEE); // Solo 2
    }

    #[test]
    fn test_parse_hid_id_yubikey() {
        let uevent = "HID_ID=0003:00001050:00000407\n";
        let (vid, pid) = parse_hid_id(uevent).unwrap();
        assert_eq!(vid, 0x1050); // Yubico
        assert_eq!(pid, 0x0407); // YubiKey 5
    }

    #[test]
    fn test_parse_hid_id_missing() {
        let uevent = "SOME_OTHER_FIELD=value\n";
        assert!(parse_hid_id(uevent).is_err());
    }

    #[test]
    fn test_parse_hid_id_malformed() {
        let uevent = "HID_ID=invalid\n";
        assert!(parse_hid_id(uevent).is_err());
    }
}

