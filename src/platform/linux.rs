use std::path::Path;
use crate::common::{get_trimmed, get_value_from_file};


/// Returns total and used RAM in GB alongside the percentage of usage
pub fn get_memory_usage(total_input: &str, available_input: &str) -> (f64, f64, f64) {
    // TODO: Check if memory total and usage is equal or more than 1GB, if not display the value in MB
    let total_string = get_value_from_file(Path::new("/proc/meminfo"), total_input, ":");
    let total_kb = (total_string.split_whitespace().next().unwrap()).parse::<f64>().unwrap();
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
            return format!(
                "{:02}m {:02}s",
                minutes.to_string(),
                seconds.to_string()
            )
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
pub fn get_battery() -> (String, String){
    let capacity = get_trimmed(Path::new("/sys/class/power_supply/BAT0/capacity"));
    let status = get_trimmed(Path::new("/sys/class/power_supply/BAT0/status"));
    (capacity, status)
}
