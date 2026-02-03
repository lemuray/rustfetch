use std::{fs, path::Path};

use colored::*;

use crate::{common::*, sysinfo::*};

const BATTERY_CAPACITY_DIR: &str = "/sys/class/power_supply/BAT0/capacity";
const BATTERY_STATUS_DIR: &str = "/sys/class/power_supply/BAT0/status";
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

/// Getting a reference to an element of the vector of strings we're printing and matching it with
/// the distro id, we get return its colorized version.
/// Example: if distro_id = ubuntu, then we return line.orange()
pub fn colorize_logo_line(distro_id: &str, line: &str) -> ColoredString {
    match distro_id {
        // The exact colors should be tested on your distro and eventually changed it they do not
        // match the color of the distro's logo excessively
        "arch" => line.truecolor(23, 147, 209),
        "ubuntu" => line.truecolor(255, 156, 0),
        "cachyos" => line.truecolor(0, 184, 148),
        "fedora" => line.truecolor(11, 87, 164),
        "garuda" => line.truecolor(138, 43, 226),
        "gentoo" => line.truecolor(84, 73, 149),
        "endeavouros" => line.truecolor(122, 58, 237),
        "kali" => line.truecolor(38, 139, 210),
        "linuxmint" | "manjaro" => line.green(),
        "debian" => line.red(),
        "alpine" => line.cyan(),
        // if the id is in this list default to white
        _ => line.white(),
    }
}
