//! Idiomatic Rust error patterns and type aliases

/// Type alias for system operation results using BearDog's unified error system
/// 
/// This provides a convenient shorthand for `Result<T, BearDogError>` specifically
/// for system-level operations and infrastructure code.
pub type SystemResult<T> = crate::BearDogResult<T>;
