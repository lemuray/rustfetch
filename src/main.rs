use std::fs;
use std::path::Path;

/*
    TODO:
    Error Detection in case function returns "Null"
    Test with other distros
    Finish translating to rust
*/

/// Gets content from a single line file and trims it
fn get_trimmed(path: &Path) -> String {
    let content = fs::read_to_string(path).expect("Null");
    content.trim().to_string() // Remove any whitespace or \n using .trim()
}

/// Gets content from a multiple or single line file and returns the value from the key and separator provided.
///
/// Example:
///``` ignore
/// /* /etc/os-release contains: */ PRETTY_NAME = "Arch Linux"
/// get_value_from_file(Path::new("/etc/os-release"), "PRETTY_NAME", "=");
/// ```
/// returns: Arch Linux
fn get_value_from_file(path: &Path, key: &str, separator: &str) -> String {
    let content = fs::read_to_string(path).expect("Null");
    for line in content.lines() {
        if !(line.contains(key)) {
            continue;
        }
        if let Some((_, value)) = line.split_once(separator) {
            let trimmed = value.trim();
            if trimmed.starts_with("\"") && trimmed.ends_with("\"") && trimmed.len() > 1 {
                let mut chars = value.chars(); // Transform the string to chars to enable index-based modifications
                chars.next(); // Removes the first character in the string, in this case the first character will always be quotation marks
                chars.next_back(); // Removes the last character, same as before
                return chars.as_str().to_string();
            } else {
                return trimmed.to_string();
            }
        }
    }
    String::from("Null")
}

/// Returns total and used RAM in GB alongside the percentage of usage
fn get_memory_usage(total_input: &str, available_input: &str) -> (f64, f64, f64) {
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
fn get_uptime() -> String {
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
fn get_battery() -> (String, String){
    let capacity = get_trimmed(Path::new("/sys/class/power_supply/BAT0/capacity"));
    let status = get_trimmed(Path::new("/sys/class/power_supply/BAT0/status"));
    (capacity, status)
}


fn main() {
    if std::env::consts::OS == "linux" {
        println!(
            "OS: {} ({})",
            get_value_from_file(Path::new("/etc/os-release"), "PRETTY_NAME", "="),
            std::env::consts::ARCH // CPU Architecture the program was compiled for
        );
        println!(
            "Kernel: Linux {}",
            get_trimmed(Path::new("/proc/sys/kernel/osrelease"))
        );
        println!(
            "CPU: {}",
            get_value_from_file(Path::new("/proc/cpuinfo"), "model name", ":")
        );
        let (total, used, percentage) = get_memory_usage("MemTotal", "MemAvailable");
        println!("RAM: {}GB / {}GB ({}% used)", used, total, percentage);
        let (total, used, percentage) = get_memory_usage("SwapTotal", "SwapFree");
        println!("Swap: {}GB / {}GB ({}% used)", used, total, percentage);
        println!("Uptime: {}", get_uptime());

        let (capacity, status) = get_battery();
        if capacity != String::from("Null") && status != String::from("Null") {
            println!("Battery: {}% ({})", capacity, status);
        }
    } else {
        println!("Linux is the only supported platform as of now");
    }
}
