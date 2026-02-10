use std::fs;

use nix::sys::statvfs::*;
use sysinfo::*;

use crate::{common::*, config::Config};

const BYTES_TO_GB: u64 = 1_000_000_000;
const KIB_TO_MB: u64 = 1024;
const SECONDS_TO_HOURS: u64 = 3600;
const MINUTES_TO_HOURS: u64 = 60;

/// Creates a System variable once and refreshes features according to what is on in the config file
pub fn create_system(config: &Config) -> System {
    let mut sys = System::new();
    if config.display.ram || config.display.swap {
        sys.refresh_memory();
    }
    if config.display.cpu {
        // Refreshing all cpu info is cheap performance-wise and just freshing the cpu name is not
        // really readable with this crate imo
        sys.refresh_cpu_all();
    }
    sys
}

/// Gets RAM usage values and returns them as a formatted String alongside the usage percentage as
/// unsigned int. Returns 0 on all values in case any error occurs
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

/// Gets swap usage values and returns them as a formatted String alongside the usage percentage as
/// unsigned int. Returns 0 on all values in case any error occurs
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

    let hours = uptime_seconds / SECONDS_TO_HOURS;
    let minutes = (uptime_seconds % SECONDS_TO_HOURS) / MINUTES_TO_HOURS;
    let seconds = uptime_seconds % MINUTES_TO_HOURS;

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
        },
    };

    // Cast these to u64 for cross platform compatibility
    let fragment_size = stats.fragment_size() as u64;

    // MacOS uses fragments, so this will show the correct numbers
    let block_size = if fragment_size > 0 {
        fragment_size
    } else {
        // In case fragment.size fails, fallback to block_size
        // (Shows incorrect numbers on MacOS but fine on Linux)
        stats.block_size() as u64
    };
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

/// Gets cpu name on any given system, filters out any comments such as x-core processor
pub fn get_cpu_name(sys: &System) -> String {
    sys.cpus()
        .first()
        .map(|cpu| {
            let full_name = cpu.brand();
            let mut end_pos = full_name.len();

            // cpu.brand() returns the full name of the CPU, for example:
            // "Ryzen 5 5600X 6-Core Processor"
            // We just find the first occurrence of any known suffix and get anything before it
            if let Some(pos) = full_name.find(" with ") {
                end_pos = end_pos.min(pos);
            }
            if let Some(pos) = full_name.find(" @ ") {
                end_pos = end_pos.min(pos);
            }

            // looks for "-Core" pattern like "6-Core Processor"
            if let Some(pos) = full_name.find("-Core")
                && let Some(space_pos) = full_name[..pos].rfind(' ')
            {
                end_pos = end_pos.min(space_pos);
            }

            full_name[..end_pos].trim().to_string()
        })
        .unwrap_or_else(|| String::from("Unknown CPU"))
}

/// Gets CPU frequency in MHz
pub fn get_cpu_frequency(sys: &System) -> u64 {
    sys.cpus().first().map(|cpu| cpu.frequency()).unwrap_or_else(|| 0)
}

/// Gets the lines logos in a vector and returns them
pub fn get_logo_lines(distro_id: &str) -> Vec<String> {
    // TODO: This should be a path.join but fs::read_to_string hates Pathbuf
    let ascii_art_path = format!("ascii/{}.txt", distro_id);

    fs::read_to_string(&ascii_art_path)
        .ok()
        .map(|content| content.lines().map(|l| l.to_string()).collect())
        .unwrap_or_default()
}

/// Gets the GPU name and filters comments
pub fn get_gpu_info() -> Option<String> {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

    let adapters: Vec<wgpu::Adapter> =
        pollster::block_on(instance.enumerate_adapters(wgpu::Backends::all()));

    if let Some(adapter) = adapters.into_iter().next() {
        let gpu_name = adapter.get_info().name;

        // Some AMD and Intel GPUs will return their name as "GPU_NAME (Some unneded stuff)"
        // For example "AMD Ryzen RX 580 Series (RADV POLARIS10)"
        // Here we're just truncating the string to get those parentheses out
        let mut end_pos = gpu_name.len();
        if let Some(parentheses_pos) = gpu_name.find("(") {
            end_pos = end_pos.min(parentheses_pos);
        }

        return Some((gpu_name[..end_pos]).to_string());
    }

    None
}
