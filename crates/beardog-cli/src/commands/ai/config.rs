

use super::types::OutputFormat;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum ConfigOperation {

    Get {

        #[arg(long)]
        key: String,

        #[arg(long, value_enum, default_value = "json")]
        format: OutputFormat,

        show_path: bool,
    },

    Set {

        value: String,

        value_type: Option<String>,

        validate: bool,

    List {

        prefix: Option<String>,

        modified_only: bool,

        include_descriptions: bool,

    Reset {

        key: Option<String>,

        force: bool,

    Export {

        output: PathBuf,

        #[arg(long, default_value = "toml")]
        format: String,

        include_defaults: bool,

        include_sensitive: bool,

    Import {

        input: PathBuf,

        merge: bool,

        dry_run: bool,

    Validate {

        config: Option<PathBuf>,

        show_warnings: bool,

    Schema {

        section: Option<String>,

        include_examples: bool,

    Diff {

        config1: PathBuf,

        config2: PathBuf,

        differences_only: bool,
}
