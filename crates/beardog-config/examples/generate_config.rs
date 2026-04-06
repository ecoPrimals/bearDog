// SPDX-License-Identifier: AGPL-3.0-or-later
//! Generate example configuration file

use beardog_config::BearDogConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create default configuration
    let config = BearDogConfig::default();

    // Serialize to TOML with pretty formatting
    let toml_str = toml::to_string_pretty(&config)?;

    // Print to stdout
    println!("{toml_str}");

    Ok(())
}
