//! Storage Domain Constants
//!
//! Canonical storage-related constants for the BearDog ecosystem.
//! These constants are used across storage operations, backends, and caching.

/// Storage location identifiers
pub mod locations {
    /// Cache storage location
    pub const CACHE: &str = "cache";

    /// Memory storage location
    pub const MEMORY: &str = "memory";

    /// Backend storage location
    pub const BACKEND: &str = "backend";

    /// Deleted items location
    pub const DELETED: &str = "deleted";

    /// List operation location
    pub const LIST: &str = "list";
}

/// Storage error messages
pub mod messages {
    /// No storage backend available error message
    pub const NO_BACKEND_AVAILABLE: &str = "No storage backend available";

    /// Store request missing data error message
    pub const MISSING_DATA: &str = "Store request missing data";

    /// Copy operation not implemented message
    pub const COPY_NOT_IMPLEMENTED: &str = "Copy operation not yet implemented";

    /// Move operation not implemented message
    pub const MOVE_NOT_IMPLEMENTED: &str = "Move operation not yet implemented";

    /// Backup operation not implemented message
    pub const BACKUP_NOT_IMPLEMENTED: &str = "Backup operation not yet implemented";

    /// Restore operation not implemented message
    pub const RESTORE_NOT_IMPLEMENTED: &str = "Restore operation not yet implemented";
}

/// Re-export commonly used constants
pub use locations::{BACKEND, CACHE, DELETED, LIST, MEMORY};
pub use messages::NO_BACKEND_AVAILABLE;
