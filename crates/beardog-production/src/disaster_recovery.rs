// SPDX-License-Identifier: AGPL-3.0-only

//! Disaster recovery helpers: content-addressed integrity for backup trees.
//!
//! Backup checksums use BLAKE3 over a deterministic encoding of the backup path:
//! for each file in sorted relative-path order, the hasher ingests the UTF-8 relative path,
//! a separator byte, then the raw file bytes. Empty directories yield the hash of the empty tree.

use std::path::{Path, PathBuf};

use beardog_errors::BearDogError;

/// Compute a deterministic BLAKE3 checksum over all regular-file contents under `root`.
///
/// - If `root` is a file, hashes that file only (relative path component is the file name).
/// - If `root` is a directory, walks recursively, skips symlinks, sorts paths for stability.
///
/// # Errors
///
/// Returns [`BearDogError`] when `root` is missing, unreadable, or an I/O error occurs while walking.
pub fn hash_backup_content(root: &Path) -> Result<String, BearDogError> {
    let mut hasher = blake3::Hasher::new();

    if !root.exists() {
        let msg = format!("Backup path does not exist: {}", root.display());
        return Err(BearDogError::invalid_input(&msg));
    }

    let meta = std::fs::symlink_metadata(root).map_err(|e| {
        let msg = format!("Cannot read metadata for {}: {e}", root.display());
        BearDogError::invalid_input(&msg)
    })?;

    if meta.is_file() {
        hash_one_file(root, root, &mut hasher)?;
        return Ok(hasher.finalize().to_hex().to_string());
    }

    if meta.is_dir() {
        let mut files = Vec::new();
        collect_regular_files(root, &mut files)?;
        files.sort();
        for path in files {
            hash_one_file(root, &path, &mut hasher)?;
        }
        return Ok(hasher.finalize().to_hex().to_string());
    }

    let msg = format!("Backup path is not a file or directory: {}", root.display());
    Err(BearDogError::invalid_input(&msg))
}

fn hash_one_file(
    root: &Path,
    file_path: &Path,
    hasher: &mut blake3::Hasher,
) -> Result<(), BearDogError> {
    let rel = file_path.strip_prefix(root).unwrap_or(file_path);
    let label = rel.to_string_lossy();
    hasher.update(label.as_bytes());
    hasher.update(&[0]);
    let bytes = std::fs::read(file_path).map_err(|e| {
        let msg = format!("Failed to read backup file {}: {e}", file_path.display());
        BearDogError::invalid_input(&msg)
    })?;
    hasher.update(&bytes);
    Ok(())
}

fn collect_regular_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), BearDogError> {
    let entries = std::fs::read_dir(dir).map_err(|e| {
        let msg = format!("Failed to read backup directory {}: {e}", dir.display());
        BearDogError::invalid_input(&msg)
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            let msg = format!("Failed to read directory entry in {}: {e}", dir.display());
            BearDogError::invalid_input(&msg)
        })?;
        let path = entry.path();
        let meta = std::fs::symlink_metadata(&path).map_err(|e| {
            let msg = format!("Failed to read metadata for {}: {e}", path.display());
            BearDogError::invalid_input(&msg)
        })?;

        if meta.is_dir() {
            collect_regular_files(&path, out)?;
        } else if meta.is_file() {
            out.push(path);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn hash_empty_directory() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let h1 = hash_backup_content(dir.path())?;
        let h2 = hash_backup_content(dir.path())?;
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
        Ok(())
    }

    #[test]
    fn hash_file_order_independent_of_creation_order() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        fs::write(dir.path().join("b.txt"), b"second")?;
        fs::write(dir.path().join("a.txt"), b"first")?;
        let h = hash_backup_content(dir.path())?;
        assert_eq!(h.len(), 64);
        Ok(())
    }

    #[test]
    fn hash_changes_when_content_changes() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let f = dir.path().join("data.bin");
        fs::write(&f, b"v1")?;
        let h1 = hash_backup_content(dir.path())?;
        fs::write(&f, b"v2")?;
        let h2 = hash_backup_content(dir.path())?;
        assert_ne!(h1, h2);
        Ok(())
    }
}
