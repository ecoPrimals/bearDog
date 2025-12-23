

//! Universal HSM providers module

pub mod factory;
pub mod software;
pub mod pkcs11;
pub mod tpm;

// Re-exports from software module
pub use software::core::SoftwareHsmProvider;
pub use software::config::SoftwareHsmConfig;

// Re-exports from other providers
pub use factory::ProviderFactory;
pub use pkcs11::Pkcs11HsmProvider;
pub use tpm::TpmHsmProvider;

// Type aliases for compatibility
pub type UniversalHsmFactory = ProviderFactory;
