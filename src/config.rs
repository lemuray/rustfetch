//! Main file for the configuration setup
//! To regenerate the config file and test new setups just run
//! rm "YOUR_OS_FILE_PATH" if on unix-like system

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::cli::Cli; // This transforms toml files into structs and viceversa

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
    pub uptime: bool,
    pub cpu: bool,
    pub cpu_frequency: bool,
    pub gpu: bool,
    pub ram: bool,
    pub swap: bool,
    pub disk: bool,
    pub battery: bool,
    pub power_draw: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            // If the config file is missing or corrupted, default to this
            os: true,
            kernel: true,
            uptime: true,
            cpu: true,
            cpu_frequency: false,
            gpu: true,
            ram: true,
            swap: true,
            disk: true,
            battery: true,
            power_draw: false,
        }
    }
}

impl All for DisplayConfig {
    /// Set all values to true
    fn set_all() -> Self {
        Self {
            os: true,
            kernel: true,
            uptime: true,
            cpu: true,
            cpu_frequency: true,
            gpu: true,
            ram: true,
            swap: true,
            disk: true,
            battery: true,
            power_draw: true,
        }
    }
}

fn get_config_template() -> String {
    r#"# Rustfetch config file

[display]
# Display the OS name
os = true
# Display the kernel version
kernel = true
# Display system uptime
uptime = true

# CPU INFO
# --------
# Display CPU info
cpu = true
    cpu_frequency = false

# GPU INFO
# --------
gpu = true

# MEMORY INFO
# -----------
# Display RAM usage
ram = true
# Display swap usage
swap = true

# DISK INFO
# ---------
# Display disk usage
disk = true

# LAPTOP-RELATED INFO (Linux only)
# If the device is not a laptop, turning
# these on just wont display anything
# --------------------------------------
# Display battery information
battery = true
# Display power draw
power_draw = false
"#
    .to_string()
}

/// Gets the default config path for rustfetch. The default path is ~/.config/rustfetch/config.toml
/// or ./rustfetch.toml as fallback
pub fn get_default_path() -> PathBuf {
    dirs::config_dir()
        .map(|p| p.join("rustfetch/config.toml"))
        .unwrap_or_else(|| PathBuf::from("rustfetch.toml")) // fallback = current directory
}

/// Creates the config file with default options and comments
fn create_config_file(config_path: &PathBuf) -> Config {
    let default_config = Config::default();

    // If parent directory does not exist, create it
    if let Some(parent) = config_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let toml_string = get_config_template();

    if let Err(e) = std::fs::write(config_path, toml_string) {
        eprintln!("Warning: Could not create config file at {:?}: {}", config_path, e);
        eprintln!("Using default configuration in memory");
    } else {
        println!("Created default config file at:  {:?}", config_path);
    }

    default_config
}

pub fn load_config(cli: &Cli) -> Config {
    let config_path = cli.config_file.clone().unwrap_or_else(get_default_path);

    if cli.reset_config {
        return create_config_file(&config_path);
    }

    if let Ok(content) = std::fs::read_to_string(&config_path) {
        // If parsing fails, use defaults instead
        toml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Warning: Failed to parse config file: {}", e);
            eprintln!("Using default configuration");
            Config::default()
        })
    } else {
        create_config_file(&config_path)
    }
}

pub fn load_all_config() -> Config {
    Config {
        display: DisplayConfig::set_all(),
    }
}
