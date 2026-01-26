use crate::common::*;
use crate::sysinfo::*;
use std::path::Path;

const BATTERY_CAPACITY_DIR: &str = "sys/class/power_supply/BAT0/capacity";
const BATTERY_STATUS_DIR: &str = "sys/class/power_supply/BAT0/capacity";
const BATTERY_POWER_DRAW_DIR: &str = "/sys/class/power_supply/BAT0/power_now";
const ROOT_DIR: &str = "/";

/// Gets battery status as a tuple (Capacity, Status) if available
pub fn get_battery() -> (String, String) {
    let capacity =
        get_trimmed(Path::new(BATTERY_CAPACITY_DIR)).unwrap_or_else(|_| "Unavailable".to_string());
    let status =
        get_trimmed(Path::new(BATTERY_STATUS_DIR)).unwrap_or_else(|_| "Unavailable".to_string());
    (capacity, status)
}

/// Gets current power draw and returns it as Watts - Only available on battery-powered devices
pub fn get_power_draw() -> i32 {
    match get_trimmed(Path::new(BATTERY_POWER_DRAW_DIR)) {
        // power_now contains the value in microwatts, we transform it in watts
        Ok(content) => content.parse::<i32>().unwrap_or(0) / 1_000_000,
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
