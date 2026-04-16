// SPDX-License-Identifier: AGPL-3.0-or-later

//! Enum dispatch for [`crate::types::HidDevice`] (replaces `Box<dyn HidDevice>`).

use crate::linux::LinuxHidDevice;
use crate::types::{HidDevice, HidDeviceInfo};
use beardog_errors::BearDogError;
use std::future::Future;

/// Owned HID device backend.
#[derive(Debug)]
pub enum HidDeviceBackend {
    /// Linux `/dev/hidraw` device (Pure Rust).
    Linux(LinuxHidDevice),
}

impl HidDevice for HidDeviceBackend {
    fn write<'a>(
        &'a mut self,
        report: &'a [u8],
    ) -> impl Future<Output = Result<usize, BearDogError>> + Send + 'a {
        match self {
            Self::Linux(d) => d.write(report),
        }
    }

    fn read<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> impl Future<Output = Result<usize, BearDogError>> + Send + 'a {
        match self {
            Self::Linux(d) => d.read(buf),
        }
    }

    fn info(&self) -> &HidDeviceInfo {
        match self {
            Self::Linux(d) => d.info(),
        }
    }
}
