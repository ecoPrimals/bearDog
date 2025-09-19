

// MODERNIZATION NOTE: This file contains vendor-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
use super::types::OutputFormat;
use clap::{Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone)]
        #[arg(Option<PathBuf>,

        force: bool,

        pin: Option<String>,
    },

    GenerateKey {

        #[arg(String,

        #[arg(String,

        label: String,

        extractable: bool,

        attributes: Vec<String>,

    ListKeys {

        #[arg(OutputFormat,

        #[arg(Option<String>,

        usage: Option<String>,

        detailed: bool,

    DeleteKey {

    CryptoOperation {

        #[arg(CryptoOperationType,
        key_label: String,

        input: String,

        output: Option<PathBuf>,

        params: Vec<String>,

    Status {

        include_slots: bool,

        include_mechanisms: bool,

    Attest {

        data: String,

        output: PathBuf,

        #[arg(String,

    VerifyAttestation {

        attestation: PathBuf,

        expected_data: String,

        trust_anchor: Option<PathBuf>,

    ExportPublicKey {

        #[arg(PathBuf,

        #[arg(String,

    Diagnostics {

        comprehensive: bool,

        test_operations: Vec<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum HsmProvider {


    /// Represents software variant
    Software,


    /// Represents pkcs11 variant
    Pkcs11,


    /// Represents aws cloud hsm variant
    AwsCloudHsm,

    universal_cloudDedicatedHsm,


    /// Represents hardware variant
    Hardware,
/// Types of crypto operation
pub enum CryptoOperationType {


    /// Represents encrypt variant
    Encrypt,


    /// Represents decrypt variant
    Decrypt,


    /// Represents sign variant
    Sign,


    /// Represents verify variant
    Verify,


    /// Represents derive variant
    Derive,


    /// Represents unwrap variant
    Unwrap,


    /// Represents wrap variant
    Wrap,
