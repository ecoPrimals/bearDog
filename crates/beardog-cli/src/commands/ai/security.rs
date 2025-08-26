

use super::types::OutputFormat;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum SecurityOperation {

    Encrypt {

        #[arg(long)]
        input: String,

        key_id: String,

        output: Option<PathBuf>,

        #[arg(long, default_value = "AES256")]
        algorithm: String,
    },

    Decrypt {

        #[arg(long, value_enum)]
        algorithm: Option<String>,

    Sign {

        #[arg(long, default_value = "RSA_PSS")]

        #[arg(long, default_value = "SHA256")]
        hash: String,

    Verify {

        signature: String,

    GenerateKey {

        #[arg(long, value_enum, default_value = "rsa2048")]
        key_type: String,

        #[arg(long, default_value = "general")]
        usage: String,

        key_id: Option<String>,

        export_public: Option<PathBuf>,

        attributes: Vec<String>,

    ListKeys {

        #[arg(long, value_enum, default_value = "json")]
        format: OutputFormat,

        key_type: Option<String>,

        usage: Option<String>,

        detailed: bool,

    DeleteKey {

        force: bool,

    ExportKey {

        output: PathBuf,

        #[arg(long, default_value = "PEM")]
        format: String,

        include_private: bool,

    ImportKey {

        input: PathBuf,

    Audit {

        from: Option<String>,

        to: Option<String>,

        operation: Option<String>,

}
