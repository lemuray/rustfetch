use crate::common::*;
use nix::sys::statvfs::*;

use sysinfo::*;

const BYTES_TO_GB: u64 = 1_000_000_000;
const KIB_TO_MB: u64 = 1024;

/// Creates a System variable once and refreshes all the needed features
pub fn create_system() -> System {
    let mut sys = System::new();
    sys.refresh_memory();
    sys.refresh_cpu_all();
    sys
}

/// Gets RAM usage values and returns them as a formatted String alongside the usage percentage as unsigned int
pub fn get_ram_usage(sys: &System) -> (String, String, u64) {
    let total_kib = (sys.total_memory() / KIB_TO_MB) as f64;
    let used_kib = (sys.used_memory() / KIB_TO_MB) as f64;
    let percentage = get_percentage_from_part(used_kib, total_kib).unwrap_or(0);

    (
        convert_to_bytes(total_kib).unwrap_or(String::from("0 KiB")),
        convert_to_bytes(used_kib).unwrap_or(String::from("0 KiB")),
        percentage,
    )
}

/// Gets swap usage values and returns them as a formatted String alongside the usage percentage as unsigned int
pub fn get_swap_usage(sys: &System) -> (String, String, u64) {
    let total_kib = (sys.total_swap() / KIB_TO_MB) as f64;
    let used_kib = (sys.used_swap() / KIB_TO_MB) as f64;
    let percentage = get_percentage_from_part(used_kib, total_kib).unwrap_or(0);

    (
        convert_to_bytes(total_kib).unwrap_or(String::from("0 KiB")),
        convert_to_bytes(used_kib).unwrap_or(String::from("0 KiB")),
        percentage,
    )
}

/// Gets system uptime in HHh MMm SSs format
pub fn get_uptime() -> String {
    let uptime_seconds = System::uptime();

    let hours = uptime_seconds / 3600;
    let minutes = (uptime_seconds % 3600) / 60;
    let seconds = uptime_seconds % 60;

    if hours < 1 {
        format!("{:02}m {:02}s", minutes, seconds)
    } else {
        format!("{:02}h {:02}m {:02}s", hours, minutes, seconds)
    }
}

/// Gets disk (root) usage and returns in GB and percentage (floored)
pub fn get_directory_usage(directory: &str) -> (u64, u64, u64) {
    let stats = match statvfs(directory) {
        Ok(stats) => stats,
        Err(e) => {
            eprintln!(
                "Unable to get directory usage for '{}', defaulting to 0: \n{}",
                directory, e
            );
            return (0, 0, 0); // Return zeros if unable to get disk stats
        }
    };

    // Cast these to u64 for closs platform compatibility
    // FIXME: shows absurd numbers on MacOS for some reason
    let block_size = stats.block_size() as u64;
    let total = stats.blocks() as u64 * block_size;
    let free = stats.blocks_available() as u64 * block_size;
    let used = total - free;

    let percentage = get_percentage_from_part(used as f64, total as f64).unwrap_or(0);

    (total / BYTES_TO_GB, used / BYTES_TO_GB, percentage)
}

/// Gets os name on any given system
pub fn get_os_name() -> String {
    System::name().unwrap_or_else(|| String::from("Unknown"))
}

/// Gets kernel version on any given system
pub fn get_kernel_version() -> String {
    System::kernel_version().unwrap_or_else(|| String::from("Unknown"))
}

/// Gets cpu name on any given system
pub fn get_cpu_name(sys: &System) -> String {
    sys.cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| String::from("Unknown CPU"))
}
