# beardog-hid

Pure Rust HID (Human Interface Device) interface for BearDog security primal.

## Features

- **Pure Rust** - Zero C dependencies, ecoBin compliant
- **Cross-platform** - Linux, macOS, Windows support
- **FIDO2 Support** - Hardware security key detection and interaction
- **Async I/O** - Tokio-based asynchronous operations
- **Type-safe** - Strong typing for vendor/product IDs

## Status

**ecoBin Compliant**: ✅ TRUE  
**Cross-Compilation**: ✅ All targets  
**Memory Safety**: ✅ 100% safe Rust

## Usage

```rust
use beardog_hid::{HidDevice, is_fido2_compatible};

// Detect FIDO2-compatible devices
if is_fido2_compatible(vendor_id, product_id) {
    // Device supports FIDO2
}
```

## License

AGPL-3.0-or-later

