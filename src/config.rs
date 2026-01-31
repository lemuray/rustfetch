//! Main file for the configuration setup
//! To regenerate the config file and test new setups just run
//! rm "YOUR_OS_FILE_PATH" if on unix-like system

// TODO: On command line, add --reset-config and --all-on to make test easier

use std::path::PathBuf;

use serde::{Deserialize, Serialize}; // This transforms toml files into structs and viceversa

trait All {
    fn set_all() -> Self;
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub display: DisplayConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayConfig {
    // The names MUST match the names inside config.toml
    pub os: bool,
    pub kernel: bool,
    pub cpu: bool,
    pub ram: bool,
    pub swap: bool,
    pub uptime: bool,
    pub battery: bool,
    pub disk: bool,
    pub power_draw: bool,
    pub cpu_frequency: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            // If the config file is missing or corrupted, default to this
            os: true,
            kernel: true,
            cpu: true,
            ram: true,
            swap: true,
            uptime: true,
            battery: true,
            disk: true,
            power_draw: false,
            cpu_frequency: false,
        }
    }
}

impl All for DisplayConfig {
    /// Set all values to true
    fn set_all() -> Self {
        Self {
            os: true,
            kernel: true,
            cpu: true,
            ram: true,
            swap: true,
            uptime: true,
            battery: true,
            disk: true,
            power_draw: true,
            cpu_frequency: true,
        }
    }
}

pub fn load_config() -> Config {
    let config_path = dirs::config_dir()
        .map(|p| p.join("rustfetch/config.toml")) // Add file path
        .unwrap_or_else(|| PathBuf::from("rustfetch.toml")); // Fallback = current directory

    if let Ok(content) = std::fs::read_to_string(&config_path) {
        // If parsing fails, use defaults instead
        toml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Warning: Failed to parse config file: {}", e);
            eprintln!("Using default configuration");
            Config::default()
        })
    } else {
        // If file doesn't exist, create it with defaults
        let default_config = Config::default();

        // If parent directory does not exist, create it
        if let Some(parent) = config_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let toml_string =
            toml::to_string_pretty(&default_config).expect("Failed to serialize default config");

        if let Err(e) = std::fs::write(&config_path, toml_string) {
            eprintln!("Warning: Could not create config file at {:?}: {}", config_path, e);
            eprintln!("Using default configuration in memory");
        } else {
            println!("Created default config file at:  {:?}", config_path);
        }

        default_config
    }
}

pub fn load_all_config() -> Config {
    Config {
        display: DisplayConfig::set_all(),
    }
}
