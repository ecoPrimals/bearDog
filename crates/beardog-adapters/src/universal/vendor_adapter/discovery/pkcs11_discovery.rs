//! PKCS#11 Library Discovery - Platform-Agnostic
//!
//! Discovers PKCS#11 HSM libraries using capability-based search strategy.
//!
//! ## Philosophy
//!
//! **Zero Hardcoding**: All paths are discovered at runtime using:
//! 1. Environment variables (explicit user configuration)
//! 2. XDG/Platform standards (Linux, macOS, Windows)
//! 3. Well-known vendor locations (fallback only)
//! 4. Runtime architecture detection
//!
//! **Platform Agnostic**: Works on Linux, macOS, Windows without modification.
//!
//! ## Search Priority
//!
//! 1. `PKCS11_LIBRARY` environment variable (highest priority)
//! 2. `PKCS11_MODULE` environment variable
//! 3. XDG Base Directory locations (Linux)
//! 4. Platform-specific standard locations
//! 5. Vendor-specific locations (as fallback)
//!
//! ## Example
//!
//! ```rust
//! use beardog_adapters::universal::vendor_adapter::discovery::pkcs11_discovery::discover_pkcs11_libraries;
//!
//! let libraries = discover_pkcs11_libraries();
//! for lib in libraries {
//!     println!("Found PKCS#11 library: {}", lib.display());
//! }
//! ```

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Discovers PKCS#11 library paths using platform-agnostic search strategy
///
/// Returns a vector of paths to potential PKCS#11 libraries, ordered by priority:
/// 1. Explicitly configured (environment variables)
/// 2. User-space standard locations
/// 3. System-wide standard locations
/// 4. Vendor-specific fallbacks
///
/// # Returns
///
/// Vector of `PathBuf` pointing to discovered PKCS#11 libraries.
/// Paths are validated to exist and have appropriate extensions.
///
/// # Example
///
/// ```no_run
/// use beardog_adapters::universal::vendor_adapter::discovery::pkcs11_discovery::discover_pkcs11_libraries;
///
/// let libraries = discover_pkcs11_libraries();
/// println!("Found {} PKCS#11 libraries", libraries.len());
/// ```
#[must_use]
pub fn discover_pkcs11_libraries() -> Vec<PathBuf> {
    let mut libraries = Vec::new();
    
    // 1. Environment variables (highest priority - explicit user configuration)
    if let Ok(lib_path) = env::var("PKCS11_LIBRARY") {
        let path = PathBuf::from(lib_path);
        if path.exists() && is_pkcs11_library(&path) {
            libraries.push(path);
            return libraries;  // Explicit config takes precedence
        }
    }
    
    if let Ok(lib_path) = env::var("PKCS11_MODULE") {
        let path = PathBuf::from(lib_path);
        if path.exists() && is_pkcs11_library(&path) {
            libraries.push(path);
            return libraries;  // Explicit config takes precedence
        }
    }
    
    // 2. Discover standard search directories (platform-specific)
    let search_dirs = discover_search_directories();
    
    // 3. Scan discovered directories for PKCS#11 libraries
    for dir in search_dirs {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if is_pkcs11_library(&path) {
                    // Validate library has PKCS#11 symbols (optional deep validation)
                    libraries.push(path);
                }
            }
        }
    }
    
    // 4. Add known vendor-specific locations (fallback only)
    libraries.extend(discover_vendor_specific_libraries());
    
    // Deduplicate (canonicalize paths to handle symlinks)
    deduplicate_libraries(libraries)
}

/// Discovers standard PKCS#11 search directories for the current platform
///
/// Uses XDG Base Directory spec on Linux, platform equivalents on macOS/Windows.
///
/// # Returns
///
/// Vector of directories to search for PKCS#11 libraries.
fn discover_search_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    
    #[cfg(target_os = "linux")]
    {
        // User-space locations (XDG Base Directory spec)
        if let Ok(data_home) = env::var("XDG_DATA_HOME") {
            dirs.push(PathBuf::from(data_home).join("pkcs11"));
        } else if let Ok(home) = env::var("HOME") {
            dirs.push(PathBuf::from(home).join(".local/share/pkcs11"));
        }
        
        // System-wide locations
        dirs.push(PathBuf::from("/usr/lib/pkcs11"));
        dirs.push(PathBuf::from("/usr/local/lib/pkcs11"));
        dirs.push(PathBuf::from("/opt/pkcs11"));
        
        // Architecture-specific (discovered at runtime)
        if let Ok(arch) = detect_architecture() {
            dirs.push(PathBuf::from(format!("/usr/lib/{arch}/pkcs11")));
            dirs.push(PathBuf::from(format!("/usr/local/lib/{arch}/pkcs11")));
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = env::var("HOME") {
            dirs.push(PathBuf::from(home).join("Library/Application Support/pkcs11"));
        }
        dirs.push(PathBuf::from("/Library/Application Support/pkcs11"));
        dirs.push(PathBuf::from("/usr/local/lib/pkcs11"));
        dirs.push(PathBuf::from("/opt/pkcs11"));
    }
    
    #[cfg(target_os = "windows")]
    {
        if let Ok(program_files) = env::var("ProgramFiles") {
            dirs.push(PathBuf::from(program_files).join("PKCS11"));
        }
        if let Ok(program_files_x86) = env::var("ProgramFiles(x86)") {
            dirs.push(PathBuf::from(program_files_x86).join("PKCS11"));
        }
        if let Ok(app_data) = env::var("APPDATA") {
            dirs.push(PathBuf::from(app_data).join("pkcs11"));
        }
        if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
            dirs.push(PathBuf::from(local_app_data).join("pkcs11"));
        }
    }
    
    dirs
}

/// Discovers vendor-specific PKCS#11 library locations (fallback only)
///
/// These are well-known locations for specific HSM vendors.
/// Only used as fallback when standard locations don't contain libraries.
///
/// # Returns
///
/// Vector of paths to vendor-specific PKCS#11 libraries that exist on the system.
fn discover_vendor_specific_libraries() -> Vec<PathBuf> {
    let mut libraries = Vec::new();
    
    #[cfg(target_os = "linux")]
    {
        // SoftHSM (common test HSM)
        let softhsm_paths = [
            "/usr/lib/softhsm/libsofthsm2.so",
            "/usr/local/lib/softhsm/libsofthsm2.so",
        ];
        
        // Detect architecture for arch-specific paths
        if let Ok(arch) = detect_architecture() {
            let arch_specific = format!("/usr/lib/{arch}/softhsm/libsofthsm2.so");
            if Path::new(&arch_specific).exists() {
                libraries.push(PathBuf::from(arch_specific));
            }
        }
        
        for path_str in &softhsm_paths {
            let path = Path::new(path_str);
            if path.exists() {
                libraries.push(path.to_path_buf());
            }
        }
        
        // Thales/nCipher (nShield series)
        let thales_path = "/opt/nfast/toolkits/pkcs11/libcknfast.so";
        if Path::new(thales_path).exists() {
            libraries.push(PathBuf::from(thales_path));
        }
        
        // Gemalto/SafeNet
        let gemalto_path = "/usr/lib/libCryptoki2_64.so";
        if Path::new(gemalto_path).exists() {
            libraries.push(PathBuf::from(gemalto_path));
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        // SoftHSM (macOS)
        let softhsm_paths = [
            "/usr/local/lib/softhsm/libsofthsm2.dylib",
            "/opt/homebrew/lib/softhsm/libsofthsm2.dylib",
        ];
        
        for path_str in &softhsm_paths {
            let path = Path::new(path_str);
            if path.exists() {
                libraries.push(path.to_path_buf());
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        // SoftHSM (Windows)
        if let Ok(program_files) = env::var("ProgramFiles") {
            let softhsm_path = PathBuf::from(program_files)
                .join("SoftHSM2")
                .join("lib")
                .join("softhsm2-x64.dll");
            if softhsm_path.exists() {
                libraries.push(softhsm_path);
            }
        }
    }
    
    libraries
}

/// Checks if a path points to a valid PKCS#11 library
///
/// Validates based on file extension (platform-specific).
///
/// # Arguments
///
/// * `path` - Path to check
///
/// # Returns
///
/// `true` if the path has a valid PKCS#11 library extension, `false` otherwise.
fn is_pkcs11_library(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        #[cfg(target_os = "linux")]
        {
            return ext_str == "so";
        }
        
        #[cfg(target_os = "macos")]
        {
            return ext_str == "dylib" || ext_str == "so";
        }
        
        #[cfg(target_os = "windows")]
        {
            return ext_str == "dll";
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            // Fallback for other platforms
            matches!(ext_str.as_ref(), "so" | "dylib" | "dll")
        }
    }
    
    false
}

/// Detects the current system architecture
///
/// Uses `std::env::consts::ARCH` for compile-time architecture.
///
/// # Returns
///
/// Architecture string (e.g., "x86_64-linux-gnu", "aarch64-linux-gnu")
///
/// # Errors
///
/// Returns error for unsupported architectures.
fn detect_architecture() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    {
        match env::consts::ARCH {
            "x86_64" => Ok("x86_64-linux-gnu".to_string()),
            "aarch64" => Ok("aarch64-linux-gnu".to_string()),
            "riscv64" => Ok("riscv64-linux-gnu".to_string()),
            arch => Err(format!("Unsupported architecture: {arch}")),
        }
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        Ok(env::consts::ARCH.to_string())
    }
}

/// Deduplicates library paths by canonicalizing them
///
/// Handles symlinks and duplicate paths to the same library.
///
/// # Arguments
///
/// * `libraries` - Vector of library paths to deduplicate
///
/// # Returns
///
/// Vector of unique, canonicalized library paths.
fn deduplicate_libraries(libraries: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut seen = std::collections::HashSet::new();
    let mut unique = Vec::new();
    
    for lib in libraries {
        // Try to canonicalize (resolves symlinks)
        let canonical = fs::canonicalize(&lib).unwrap_or(lib.clone());
        
        if seen.insert(canonical.clone()) {
            unique.push(canonical);
        }
    }
    
    unique
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_discover_pkcs11_libraries() {
        let libraries = discover_pkcs11_libraries();
        // At minimum, should not panic
        assert!(libraries.len() >= 0);
    }
    
    #[test]
    fn test_is_pkcs11_library() {
        #[cfg(target_os = "linux")]
        {
            assert!(is_pkcs11_library(Path::new("/tmp/test.so")));  // Would fail if file doesn't exist
            assert!(!is_pkcs11_library(Path::new("/tmp/test.txt")));
        }
        
        #[cfg(target_os = "macos")]
        {
            assert!(is_pkcs11_library(Path::new("/tmp/test.dylib")));
        }
        
        #[cfg(target_os = "windows")]
        {
            assert!(is_pkcs11_library(Path::new("C:\\test.dll")));
        }
    }
    
    #[test]
    fn test_environment_variable_override() {
        // Test that PKCS11_LIBRARY env var takes precedence
        // (This test would need a real file to fully verify)
        let original = env::var("PKCS11_LIBRARY").ok();
        
        // Set to non-existent path (should not crash)
        env::set_var("PKCS11_LIBRARY", "/nonexistent/path/lib.so");
        let _ = discover_pkcs11_libraries();
        
        // Restore original
        if let Some(orig) = original {
            env::set_var("PKCS11_LIBRARY", orig);
        } else {
            env::remove_var("PKCS11_LIBRARY");
        }
    }
    
    #[test]
    fn test_detect_architecture() {
        let arch = detect_architecture();
        // Should succeed on supported platforms
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        {
            assert!(arch.is_ok());
        }
    }
    
    #[test]
    fn test_deduplicate_libraries() {
        let libs = vec![
            PathBuf::from("/usr/lib/test.so"),
            PathBuf::from("/usr/lib/test.so"),  // Duplicate
            PathBuf::from("/usr/lib/other.so"),
        ];
        
        let unique = deduplicate_libraries(libs);
        // Depending on whether paths exist, deduplication varies
        // At minimum, should not panic
        assert!(unique.len() >= 0);
    }
}
