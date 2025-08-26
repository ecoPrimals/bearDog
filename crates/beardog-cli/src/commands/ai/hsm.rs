

use super::types::OutputFormat;
use clap::{Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum HsmOperation {

    Initialize {

        #[arg(long, value_enum, default_value = "software")]
        provider: HsmProvider,

        #[arg(long)]
        config: Option<PathBuf>,

        force: bool,

        pin: Option<String>,
    },

    GenerateKey {

        #[arg(long, value_enum, default_value = "rsa2048")]
        key_type: String,

        #[arg(long, value_enum, default_value = "general")]
        usage: String,

        label: String,

        extractable: bool,

        attributes: Vec<String>,

    ListKeys {

        #[arg(long, value_enum, default_value = "json")]
        format: OutputFormat,

        #[arg(long, value_enum)]
        key_type: Option<String>,

        usage: Option<String>,

        detailed: bool,

    DeleteKey {

    CryptoOperation {

        #[arg(long, value_enum, default_value = "encrypt")]
        operation: CryptoOperationType,
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

        #[arg(long, default_value = "json")]
        format: String,

    VerifyAttestation {

        attestation: PathBuf,

        expected_data: String,

        trust_anchor: Option<PathBuf>,

    ExportPublicKey {

        #[arg(long, default_value = "PEM")]

    ImportCertificate {

        certificate: PathBuf,

        #[arg(long, default_value = "X509")]
        cert_type: String,

    Diagnostics {

        comprehensive: bool,

        test_operations: Vec<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum HsmProvider {

    Software,

    Pkcs11,

    AwsCloudHsm,

    AzureDedicatedHsm,

    Hardware,

pub enum CryptoOperationType {

    Encrypt,

    Decrypt,

    Sign,

    Verify,

    Derive,

    Unwrap,

    Wrap,
