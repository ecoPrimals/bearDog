// SPDX-License-Identifier: AGPL-3.0-or-later

//! G68 Platform Substrate: platform-agnostic file permission operations.
//!
//! Replaces direct `std::os::unix::fs::PermissionsExt` usage with a
//! cross-platform abstraction. On Unix, delegates to `mode()` / `set_mode()`.
//! On non-Unix, provides no-op or best-effort equivalents.

use std::io;
use std::path::Path;

/// Platform-agnostic file permission operations.
///
/// Eliminates L2 violations from the G68 platform substrate audit by
/// wrapping Unix-specific `PermissionsExt` behind a cross-platform API.
pub struct PlatformAccess;

/// Result of reading file permissions.
#[derive(Debug, Clone, Copy)]
pub struct FileMode {
    /// Raw Unix mode bits (0 on non-Unix).
    pub mode: u32,
    /// Whether the file is executable by anyone.
    pub is_executable: bool,
    /// Whether the file is world-readable (readable by "other").
    pub is_world_readable: bool,
    /// Whether the file is owner-readable.
    pub is_owner_readable: bool,
}

impl PlatformAccess {
    /// Read the permission mode of a file.
    ///
    /// On Unix, returns the full mode bits. On non-Unix, returns a synthetic
    /// `FileMode` with `is_executable` inferred from the extension.
    ///
    /// # Errors
    ///
    /// Returns [`io::Error`] if the file metadata cannot be read.
    pub fn read_mode(path: &Path) -> io::Result<FileMode> {
        let metadata = std::fs::metadata(path)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            Ok(FileMode {
                mode,
                is_executable: (mode & 0o111) != 0,
                is_world_readable: (mode & 0o044) != 0,
                is_owner_readable: (mode & 0o400) != 0,
            })
        }

        #[cfg(not(unix))]
        {
            let _ = metadata;
            let is_executable = path
                .extension()
                .is_some_and(|ext| ext == "exe" || ext == "cmd" || ext == "bat");
            Ok(FileMode {
                mode: 0,
                is_executable,
                is_world_readable: false,
                is_owner_readable: true,
            })
        }
    }

    /// Set file permissions to owner-only read/write (0o600 on Unix).
    ///
    /// # Errors
    ///
    /// Returns [`io::Error`] if the file metadata cannot be read or permissions
    /// cannot be set.
    pub fn set_owner_only(path: &Path) -> io::Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
        }

        #[cfg(not(unix))]
        {
            let _ = path;
        }

        Ok(())
    }

    /// Async version of [`Self::set_owner_only`].
    ///
    /// # Errors
    ///
    /// Returns [`io::Error`] if the file metadata cannot be read or permissions
    /// cannot be set.
    pub async fn set_owner_only_async(path: &Path) -> io::Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            tokio::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).await?;
        }

        #[cfg(not(unix))]
        {
            let _ = path;
        }

        Ok(())
    }

    /// Set file permissions to executable (0o755 on Unix).
    ///
    /// # Errors
    ///
    /// Returns [`io::Error`] if the file metadata cannot be read or permissions
    /// cannot be set.
    pub async fn set_executable_async(path: &Path) -> io::Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = tokio::fs::metadata(path).await?.permissions();
            perms.set_mode(0o755);
            tokio::fs::set_permissions(path, perms).await?;
        }

        #[cfg(not(unix))]
        {
            let _ = path;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn read_mode_on_temp_file() {
        let f = NamedTempFile::new().expect("tempfile");
        let mode = PlatformAccess::read_mode(f.path()).expect("read_mode");
        assert!(mode.is_owner_readable);
    }

    #[test]
    fn set_owner_only_sync() {
        let mut f = NamedTempFile::new().expect("tempfile");
        f.write_all(b"secret").expect("write");
        PlatformAccess::set_owner_only(f.path()).expect("set_owner_only");

        let mode = PlatformAccess::read_mode(f.path()).expect("read_mode after set");
        #[cfg(unix)]
        {
            assert_eq!(mode.mode & 0o777, 0o600);
            assert!(!mode.is_world_readable);
        }
    }

    #[tokio::test]
    async fn set_owner_only_async_works() {
        let mut f = NamedTempFile::new().expect("tempfile");
        f.write_all(b"async-secret").expect("write");
        PlatformAccess::set_owner_only_async(f.path())
            .await
            .expect("set_owner_only_async");

        let mode = PlatformAccess::read_mode(f.path()).expect("read_mode after async set");
        #[cfg(unix)]
        {
            assert_eq!(mode.mode & 0o777, 0o600);
        }
    }

    #[tokio::test]
    async fn set_executable_async_works() {
        let f = NamedTempFile::new().expect("tempfile");
        PlatformAccess::set_executable_async(f.path())
            .await
            .expect("set_executable_async");

        let mode = PlatformAccess::read_mode(f.path()).expect("read_mode after executable set");
        #[cfg(unix)]
        {
            assert!(mode.is_executable);
            assert_eq!(mode.mode & 0o777, 0o755);
        }
    }

    #[test]
    fn read_mode_nonexistent_file() {
        let result = PlatformAccess::read_mode(Path::new("/nonexistent/file"));
        assert!(result.is_err());
    }
}
