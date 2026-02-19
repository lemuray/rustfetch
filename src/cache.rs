//! File for handling cache, default path is:
//! Linux: ~/.cache
//! MacOS: ~/Library/Caches

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{cli::Cli, platform::get_gpu_ids, sysinfo::get_gpu_name_pretty};

#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub gpu_name_pretty: String,
    pub gpu_vendor_id: String,
    pub gpu_device_id: String,
}

fn get_default_values() -> Cache {
    let gpu_name;
    if let Some(gpu) = get_gpu_name_pretty() {
        gpu_name = gpu;
    } else {
        gpu_name = String::from("");
    }

    let gpu_ids;
    if let Some(ids) = get_gpu_ids() {
        gpu_ids = ids;
    } else {
        gpu_ids = (String::from(""), String::from(""));
    }

    Cache {
        gpu_name_pretty: gpu_name,
        gpu_vendor_id: gpu_ids.0,
        gpu_device_id: gpu_ids.1,
    }
}

pub fn get_cache_path() -> PathBuf {
    dirs::cache_dir()
        .map(|p| p.join("rustfetch/cache.toml"))
        .unwrap_or_else(|| PathBuf::from("cache.toml")) // as with config.rs, the fallback is the current directory
}

pub fn create_cache() -> Result<(), Box<dyn std::error::Error>> {
    let cache_path = get_cache_path();

    if let Some(parent) = cache_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let cache_defaults = get_default_values();
    let toml_string = toml::to_string(&cache_defaults)?;

    std::fs::write(&cache_path, toml_string)?;

    // apparently parsing a pathbuf to string needs 3 methods lol
    eprintln!(
        "Created cache at {}",
        cache_path.into_os_string().into_string().unwrap_or(String::from(""))
    );

    Ok(())
}

pub fn get_cache(cli: &Cli) -> Result<Cache, Box<dyn std::error::Error>> {
    let cache_path = get_cache_path();
    if std::fs::read_to_string(&cache_path).is_err() || cli.clear_cache {
        create_cache()?;
    }
    let contents = std::fs::read_to_string(&cache_path)?;
    let cache: Cache = toml::from_str(&contents)?;
    Ok(cache)
}
