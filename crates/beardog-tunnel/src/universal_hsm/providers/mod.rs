

pub mod factory;
pub mod software;

mod android;
mod ios;
mod pkcs11;
mod tpm;

pub use software::{
    AttestationEngine, CryptoEngine, EntropyCollector, KeyStore, SecureMemory, SoftwareHsmConfig,
    SoftwareHsmProvider,
};
pub use factory::ProviderFactory;

pub use android::MobileHardwareProvider;
pub use ios::DesktopHardwareProvider;
pub use pkcs11::Pkcs11Provider;
pub use tpm::TpmProvider;

pub type UniversalHsmFactory = ProviderFactory;
