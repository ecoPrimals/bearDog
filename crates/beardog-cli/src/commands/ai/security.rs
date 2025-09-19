

use super::types::OutputFormat;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Clone)]
        key_id: String,

        output: Option<PathBuf>,

        #[arg(String,
    },

    Decrypt {

        #[arg(Option<String>,

    Sign {

        #[arg(String,

    Verify {

        signature: String,

    GenerateKey {

        #[arg(String,

        #[arg(String,

        key_id: Option<String>,

        export_public: Option<PathBuf>,

        attributes: Vec<String>,

    ListKeys {

        #[arg(OutputFormat,

        key_type: Option<String>,

        usage: Option<String>,

        detailed: bool,

    DeleteKey {

        force: bool,

    ExportKey {

        output: PathBuf,

        #[arg(String,

        include_private: bool,

    ImportKey {

        input: PathBuf,

    Audit {

        from: Option<String>,

        to: Option<String>,

        operation: Option<String>,

}
