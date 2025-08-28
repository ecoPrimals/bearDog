

pub use beardog_core as core;
pub use beardog_types::config as config;
pub use beardog_errors as errors;
pub use beardog_traits as traits;

pub use beardog_types::canonical::*;
pub use beardog_errors::BearDogError;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const MISSION: &str = "Democratizing enterprise-grade security for everyone";
