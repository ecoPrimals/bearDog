// Discovery systems for BearDog
//
// This module contains various discovery mechanisms, from traditional
// hardcoded systems to the new infant discovery system that starts
// with zero knowledge.

pub mod infant_discovery;
pub mod universal_infant_discovery;
pub mod vendor_agnostic_hsm;

pub use infant_discovery::*;
pub use universal_infant_discovery::*;
pub use vendor_agnostic_hsm::*;
