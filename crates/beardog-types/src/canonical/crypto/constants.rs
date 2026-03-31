// SPDX-License-Identifier: AGPL-3.0-only

use crate::constants::time;

#[expect(
    clippy::cast_possible_truncation,
    reason = "compile-time assert guarantees value fits u32"
)]
pub(crate) const ROTATION_30_DAYS_SECS: u32 = {
    assert!(30 * time::SECONDS_PER_DAY <= u32::MAX as u64);
    (30 * time::SECONDS_PER_DAY) as u32
};

#[expect(
    clippy::cast_possible_truncation,
    reason = "compile-time assert guarantees value fits u32"
)]
pub(crate) const ROTATION_90_DAYS_SECS: u32 = {
    assert!(90 * time::SECONDS_PER_DAY <= u32::MAX as u64);
    (90 * time::SECONDS_PER_DAY) as u32
};

#[expect(
    clippy::cast_possible_truncation,
    reason = "compile-time assert guarantees value fits u32"
)]
pub(crate) const SECONDS_PER_HOUR_U32: u32 = {
    assert!(time::SECONDS_PER_HOUR <= u32::MAX as u64);
    time::SECONDS_PER_HOUR as u32
};
