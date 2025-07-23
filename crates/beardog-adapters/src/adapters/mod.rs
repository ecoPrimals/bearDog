//! Universal Adapters for External System Integration
//!
//! **Universal, agnostic adapters following ecoPrimals architecture principles**
//!
//! This module provides the universal adapter system that enables BearDog to integrate
//! with ANY external system (non-ecoPrimals) through a consistent, capability-based interface.

pub mod nestgate;
pub mod universal;

pub use universal::*;
