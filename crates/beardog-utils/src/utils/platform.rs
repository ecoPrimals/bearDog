// SPDX-License-Identifier: AGPL-3.0-or-later

//! Platform-specific helpers without `unsafe`.

/// Resolve the real UID by reading `/proc/self/status` (Linux) without `unsafe`.
///
/// Returns `None` on non-Linux or on parse failure, allowing callers to fall back.
#[must_use]
pub fn resolve_uid_from_proc() -> Option<u32> {
    #[cfg(target_os = "linux")]
    {
        let status = std::fs::read_to_string("/proc/self/status").ok()?;
        for line in status.lines() {
            if let Some(rest) = line.strip_prefix("Uid:") {
                return rest.split_whitespace().next()?.parse().ok();
            }
        }
        None
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}
