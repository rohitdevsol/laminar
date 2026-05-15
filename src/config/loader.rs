use crate::config::Config;
use crate::config::default::DEFAULT_CONFIG;
use anyhow::Context;
use std::fs;
use std::path::Path;

// Responsible for loading and deserializing YAML configuration files.
// This module should ONLY handle configuration parsing.
// Validation and runtime state initialization happen separately.
pub fn load_config(config_path: &str) -> anyhow::Result<Config> {
    if !Path::new(config_path).exists() {
        fs::write(config_path, DEFAULT_CONFIG).context("failed to create default config")?;

        println!("Created default laminar_config.yaml");
        println!("Please edit it and restart Laminar.");

        std::process::exit(0);
    }
    let content = fs::read_to_string(config_path).context("failed to read config")?;

    let config: Config = serde_yaml::from_str(&content).context("invalid yaml config")?;

    Ok(config)
}
