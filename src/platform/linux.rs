use std::fs;
use std::path::Path;

use crate::{common::*, sysinfo::*};

const BATTERY_CAPACITY_DIR: &str = "sys/class/power_supply/BAT0/capacity";
const BATTERY_STATUS_DIR: &str = "sys/class/power_supply/BAT0/capacity";
const BATTERY_POWER_DRAW_DIR: &str = "/sys/class/power_supply/BAT0/power_now";
const ROOT_DIR: &str = "/";

pub fn get_distro_id() -> String {
    fs::read_to_string(Path::new("/etc/os-release"))
        .ok()
        .and_then(|content| {
            content
                .lines()
                .find(|line| line.starts_with("ID="))
                .and_then(|line| line.split('=').nth(1))
                .map(|id| id.trim_matches('"').to_string())
        })
        .unwrap_or_else(|| "unknown".to_string())
}

/// Gets battery status as a tuple (Capacity, Status) if available
pub fn get_battery() -> (String, String) {
    let capacity =
        get_trimmed(Path::new(BATTERY_CAPACITY_DIR)).unwrap_or_else(|_| "Unavailable".to_string());
    let status =
        get_trimmed(Path::new(BATTERY_STATUS_DIR)).unwrap_or_else(|_| "Unavailable".to_string());
    (capacity, status)
}

/// Gets current power draw and returns it as Watts - Only available on battery-powered devices
pub fn get_power_draw() -> u32 {
    match get_trimmed(Path::new(BATTERY_POWER_DRAW_DIR)) {
        // power_now contains the value in microwatts, we transform it in watts
        Ok(content) => content.parse::<u32>().unwrap_or(0) / 1_000_000,
        Err(_) => 0,
    }
}

pub fn get_disk_usage() -> (u64, u64, u64) {
    // Linux root directory
    get_directory_usage(ROOT_DIR)
}

pub fn format_kernel_version() -> String {
    format!("Linux {}", get_kernel_version())
}

pub fn get_logo_lines() -> Vec<String> {
    // TODO: This should be a path.join but fs::read_to_string hates Pathbuf
    let ascii_art_path = format!("ascii/{}.txt", get_distro_id());

    fs::read_to_string(&ascii_art_path)
        .ok()
        .map(|content| content.lines().map(|l| l.to_string()).collect())
        .unwrap_or_default()
}
