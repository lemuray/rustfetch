use crate::common::{get_trimmed, get_value_from_file};
use nix::sys::statvfs::*;
use std::path::Path;

/// Returns total and used memory in GB alongside the percentage of usage
fn get_memory_usage(total_input: &str, available_input: &str) -> (f64, f64, f64) {
    // TODO: Check if memory total and usage is equal or more than 1GB, if not display the value in MB
    let total_string = get_value_from_file(Path::new("/proc/meminfo"), total_input, ":");
    let total_kb = (total_string.split_whitespace().next().unwrap())
        .parse::<f64>()
        .unwrap();
    let total_gb = ((total_kb / (1024.0 * 1024.0)) * 100.0).floor() / 100.0;

    let available_string = get_value_from_file(Path::new("/proc/meminfo"), available_input, ":");
    let available_kb = available_string.split_whitespace().next().unwrap();
    let available_gb = available_kb.parse::<f64>().unwrap() / (1024.0 * 1024.0);

    let used = ((total_gb - available_gb) * 100.0).floor() / 100.0;
    let percentage = if total_gb > 0.0 {
        (used / total_gb * 100.0).floor()
    } else {
        0.0
    };
    (total_gb, used, percentage)
}

/// Returns total and used RAM in GB alongside the percentage of usage -> (total_gb, used, percentage)
pub fn get_ram_usage() -> (f64, f64, f64) {
    get_memory_usage("MemTotal", "MemAvailable")
}

/// Returns total and used RAM in GB alongside the percentage of usage -> (total_gb, used, percentage)
pub fn get_swap_usage() -> (f64, f64, f64) {
    get_memory_usage("SwapTotal", "SwapFree")
}

/// Gets device uptime in HHh MMm SSs format
pub fn get_uptime() -> String {
    let content = get_trimmed(Path::new("/proc/uptime"));
    if let Some((uptime_string, _)) = content.split_once(" ") {
        let uptime = (uptime_string.parse::<f64>().unwrap()).floor();

        // Transform the uptime (Which is in seconds) to hours, minutes and the remainder in seconds
        let hours = (uptime / 3600.0).floor();
        let minutes = ((uptime % 3600.0) / 60.0).floor();
        let seconds = uptime % 60.0;

        if hours < 1.0 {
            return format!("{:02}m {:02}s", minutes.to_string(), seconds.to_string());
        };
        return format!(
            "{:02}h {:02}m {:02}s",
            hours.to_string(),
            minutes.to_string(),
            seconds.to_string()
        );
    }
    String::from("Null")
}

/// Gets battery status as a tuple (Capacity, Status) if available
pub fn get_battery() -> (String, String) {
    let capacity = get_trimmed(Path::new("/sys/class/power_supply/BAT0/capacity"));
    let status = get_trimmed(Path::new("/sys/class/power_supply/BAT0/status"));
    (capacity, status)
}

/// Gets current power draw and returns it as Watts - Only available on battery-powered devices
pub fn get_power_draw() -> i32 {
    let power_draw_mw = get_trimmed(Path::new("/sys/class/power_supply/BAT0/power_now"))
        .parse::<i32>()
        .unwrap_or(0);
    power_draw_mw / 1_000_000 // power_now contains the value in microwatts, we transform it in watts
}

/// Gets disk (root) usage and returns in GB and percentage (floored)
pub fn get_disk_usage() -> (u64, u64, f64) {
    let stats = statvfs("/").unwrap();
    let block_size = stats.block_size();
    let total = stats.blocks() * block_size;
    let free = stats.blocks_available() * block_size;
    let used = total - free;

    let percentage = ((used as f64 / total as f64) * 100.0).floor();

    (total / 1_000_000_000, used / 1_000_000_000, percentage)
}
