use serde::{Deserialize, Serialize}; // This transforms toml files into structs and viceversa
use std::path::PathBuf;

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
        }
    }
}

pub fn load_config() -> Config {
    // On Linux:  ~/.config/rustfetch/config.toml
    // On Windows: C:\Users\YourName\AppData\Roaming\rustfetch\config.toml
    // On macOS: ~/Library/Application Support/rustfetch/config.toml
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
            eprintln!(
                "Warning: Could not create config file at {:?}: {}",
                config_path, e
            );
            eprintln!("Using default configuration in memory");
        } else {
            println!("Created default config file at:  {:?}", config_path);
        }

        default_config
    }
}
