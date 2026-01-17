

//! Universal HSM providers module
//!
//! **Philosophy**: Open standards, zero vendor locks!
//! - ✅ Software HSM (universal, pure Rust)
//! - ✅ Android StrongBox (open Android standard)
//! - ✅ iOS Secure Enclave (open iOS standard)
//! - ✅ Cloud HSMs (AWS, Azure, GCP)
//! - ✅ TPM 2.0 (open TCG standard)
//! - ❌ PKCS#11 (vendor lock eliminated!)

pub mod factory;
pub mod software;
pub mod tpm;

// Re-exports from software module
pub use software::core::SoftwareHsmProvider;
pub use software::config::SoftwareHsmConfig;

// Re-exports from other providers
pub use factory::ProviderFactory;
pub use tpm::TpmHsmProvider;

// Type aliases for compatibility
pub type UniversalHsmFactory = ProviderFactory;
