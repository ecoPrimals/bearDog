// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for HID types
//!
//! Tests `VendorId`, `ProductId`, `HidDeviceInfo`, and FIDO2 device detection.

#[cfg(test)]
use super::*;

#[test]
fn test_vendor_id_display() {
    let vid = VendorId(0x1209);
    assert_eq!(format!("{vid}"), "0x1209");

    let vid2 = VendorId(0x00ab);
    assert_eq!(format!("{vid2}"), "0x00ab");
}

#[test]
fn test_product_id_display() {
    let pid = ProductId(0xbeee);
    assert_eq!(format!("{pid}"), "0xbeee");

    let pid2 = ProductId(0x0001);
    assert_eq!(format!("{pid2}"), "0x0001");
}

#[test]
fn test_vendor_id_equality() {
    let vid1 = VendorId(0x1209);
    let vid2 = VendorId(0x1209);
    let vid3 = VendorId(0x1050);

    assert_eq!(vid1, vid2);
    assert_ne!(vid1, vid3);
}

#[test]
fn test_product_id_equality() {
    let pid1 = ProductId(0xbeee);
    let pid2 = ProductId(0xbeee);
    let pid3 = ProductId(0x0001);

    assert_eq!(pid1, pid2);
    assert_ne!(pid1, pid3);
}

#[test]
fn test_hid_device_info_creation() {
    let info = HidDeviceInfo {
        vendor_id: VendorId(0x1209),
        product_id: ProductId(0xbeee),
        manufacturer: "SoloKeys".to_string(),
        product: "Solo 2".to_string(),
        serial: "ABC123".to_string(),
        path: "/dev/hidraw0".to_string(),
        usage_page: None,
    };

    assert_eq!(info.vendor_id, VendorId(0x1209));
    assert_eq!(info.product_id, ProductId(0xbeee));
    assert_eq!(info.manufacturer, "SoloKeys");
    assert_eq!(info.product, "Solo 2");
    assert_eq!(info.serial, "ABC123");
    assert_eq!(info.path, "/dev/hidraw0");
}

#[test]
fn test_hid_device_info_display() {
    let info = HidDeviceInfo {
        vendor_id: VendorId(0x1209),
        product_id: ProductId(0xbeee),
        manufacturer: "SoloKeys".to_string(),
        product: "Solo 2".to_string(),
        serial: "ABC123".to_string(),
        path: "/dev/hidraw0".to_string(),
        usage_page: None,
    };

    let display = format!("{info}");
    assert!(display.contains("SoloKeys"));
    assert!(display.contains("Solo 2"));
    assert!(display.contains("0x1209"));
    assert!(display.contains("0xbeee"));
    assert!(display.contains("/dev/hidraw0"));
}

#[test]
fn test_hid_device_info_empty_serial() {
    let info = HidDeviceInfo {
        vendor_id: VendorId(0x1050),
        product_id: ProductId(0x0407),
        manufacturer: "Yubico".to_string(),
        product: "YubiKey 5".to_string(),
        serial: String::new(),
        path: "/dev/hidraw1".to_string(),
        usage_page: None,
    };

    assert!(info.serial.is_empty());
}

#[test]
fn test_fido2_device_solokeys() {
    let vid = fido2_vendors::SOLOKEYS;
    let pid = fido2_products::SOLO2;

    assert!(is_fido2_device(vid, pid));
}

#[test]
fn test_fido2_device_yubico() {
    let vid = fido2_vendors::YUBICO;

    // All Yubico devices support FIDO2
    assert!(is_fido2_device(vid, ProductId(0x0407)));
    assert!(is_fido2_device(vid, ProductId(0x0410)));
    assert!(is_fido2_device(vid, ProductId(0xffff)));
}

#[test]
fn test_fido2_device_google_titan() {
    let vid = fido2_vendors::GOOGLE;

    // Specific Titan models
    assert!(is_fido2_device(vid, ProductId(0x0858)));
    assert!(is_fido2_device(vid, ProductId(0x0859)));

    // Note: VID 0x096e is shared with Feitian, so other products may also be FIDO2
}

#[test]
fn test_fido2_device_feitian() {
    let vid = fido2_vendors::FEITIAN;

    // Feitian devices support FIDO2
    assert!(is_fido2_device(vid, ProductId(0x0850)));
}

#[test]
fn test_fido2_device_unknown() {
    let vid = VendorId(0x9999);
    let pid = ProductId(0x8888);

    assert!(!is_fido2_device(vid, pid));
}

#[test]
fn test_fido2_vendors_constants() {
    assert_eq!(fido2_vendors::SOLOKEYS.0, 0x1209);
    assert_eq!(fido2_vendors::YUBICO.0, 0x1050);
    assert_eq!(fido2_vendors::GOOGLE.0, 0x096e);
    assert_eq!(fido2_vendors::FEITIAN.0, 0x096e);
}

#[test]
fn test_fido2_products_constants() {
    assert_eq!(fido2_products::SOLO2.0, 0xbeee);
}

#[test]
fn test_vendor_id_hash() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(VendorId(0x1209), "SoloKeys");
    map.insert(VendorId(0x1050), "Yubico");

    assert_eq!(map.get(&VendorId(0x1209)), Some(&"SoloKeys"));
    assert_eq!(map.get(&VendorId(0x1050)), Some(&"Yubico"));
}

#[test]
fn test_product_id_hash() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(ProductId(0xbeee), "Solo 2");
    map.insert(ProductId(0x0407), "YubiKey 5");

    assert_eq!(map.get(&ProductId(0xbeee)), Some(&"Solo 2"));
    assert_eq!(map.get(&ProductId(0x0407)), Some(&"YubiKey 5"));
}

#[test]
fn test_vendor_id_clone() {
    let vid1 = VendorId(0x1209);
    let vid2 = vid1;

    assert_eq!(vid1, vid2);
}

#[test]
fn test_product_id_clone() {
    let pid1 = ProductId(0xbeee);
    let pid2 = pid1;

    assert_eq!(pid1, pid2);
}

#[test]
fn test_hid_device_info_clone() {
    let info1 = HidDeviceInfo {
        vendor_id: VendorId(0x1209),
        product_id: ProductId(0xbeee),
        manufacturer: "SoloKeys".to_string(),
        product: "Solo 2".to_string(),
        serial: "ABC123".to_string(),
        path: "/dev/hidraw0".to_string(),
        usage_page: None,
    };

    let info2 = info1.clone();

    assert_eq!(info1.vendor_id, info2.vendor_id);
    assert_eq!(info1.product_id, info2.product_id);
    assert_eq!(info1.manufacturer, info2.manufacturer);
    assert_eq!(info1.product, info2.product);
    assert_eq!(info1.serial, info2.serial);
    assert_eq!(info1.path, info2.path);
}

#[test]
fn test_is_fido2_device_comprehensive() {
    // SoloKeys
    assert!(is_fido2_device(VendorId(0x1209), ProductId(0xbeee)));

    // Yubico (any product)
    assert!(is_fido2_device(VendorId(0x1050), ProductId(0x0001)));
    assert!(is_fido2_device(VendorId(0x1050), ProductId(0xFFFF)));

    // Google Titan (specific) - VID 0x096e is shared with Feitian
    assert!(is_fido2_device(VendorId(0x096e), ProductId(0x0858)));
    assert!(is_fido2_device(VendorId(0x096e), ProductId(0x0859)));

    // Feitian (any product) - shares VID 0x096e with Google
    assert!(is_fido2_device(VendorId(0x096e), ProductId(0x0850)));

    // Unknown vendors
    assert!(!is_fido2_device(VendorId(0x0000), ProductId(0x0000)));
    assert!(!is_fido2_device(VendorId(0xFFFF), ProductId(0xFFFF)));
}

#[test]
fn test_vendor_id_debug() {
    let vid = VendorId(0x1209);
    let debug_str = format!("{vid:?}");
    assert!(debug_str.contains("VendorId"));
    assert!(debug_str.contains("1209") || debug_str.contains("4617")); // hex or decimal
}

#[test]
fn test_product_id_debug() {
    let pid = ProductId(0xbeee);
    let debug_str = format!("{pid:?}");
    assert!(debug_str.contains("ProductId"));
    assert!(debug_str.contains("beee") || debug_str.contains("48878")); // hex or decimal
}

#[test]
fn test_parse_usage_page_fido2() {
    // Real Solo 2 report descriptor prefix: 06 D0 F1 = usage page 0xF1D0
    let descriptor = [0x06, 0xD0, 0xF1, 0x09, 0x01, 0xA1, 0x01];
    assert_eq!(parse_usage_page_from_descriptor(&descriptor), Some(FIDO_USAGE_PAGE));
}

#[test]
fn test_parse_usage_page_generic_desktop() {
    // Generic Desktop usage page (1-byte): 05 01
    let descriptor = [0x05, 0x01, 0x09, 0x06];
    assert_eq!(parse_usage_page_from_descriptor(&descriptor), Some(0x0001));
}

#[test]
fn test_parse_usage_page_empty() {
    assert_eq!(parse_usage_page_from_descriptor(&[]), None);
}

#[test]
fn test_is_fido2_interface_with_usage_page() {
    let fido_info = HidDeviceInfo {
        vendor_id: VendorId(0x9999),
        product_id: ProductId(0x8888),
        manufacturer: "Unknown".to_string(),
        product: "Unknown".to_string(),
        serial: String::new(),
        path: "/dev/hidraw0".to_string(),
        usage_page: Some(FIDO_USAGE_PAGE),
    };
    assert!(is_fido2_interface(&fido_info));

    let non_fido_info = HidDeviceInfo {
        usage_page: Some(0x0001),
        ..fido_info.clone()
    };
    assert!(!is_fido2_interface(&non_fido_info));
}

#[test]
fn test_is_fido2_interface_fallback_to_vid_pid() {
    let info = HidDeviceInfo {
        vendor_id: VendorId(0x1209),
        product_id: ProductId(0xbeee),
        manufacturer: "SoloKeys".to_string(),
        product: "Solo 2".to_string(),
        serial: String::new(),
        path: "/dev/hidraw0".to_string(),
        usage_page: None,
    };
    assert!(is_fido2_interface(&info));
}
