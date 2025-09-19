// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod tunnel;

pub use tunnel::{BStpConfig, SecureSession, SecurityLevel, SessionManager};

pub use beardog_errors::BearDogError;
