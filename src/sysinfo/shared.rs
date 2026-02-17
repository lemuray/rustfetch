use std::path::*;

use display_info::DisplayInfo;
use sysinfo::*;

use crate::{cache::get_cache, common::*, config::Config, cli::Cli};

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
    let disks = Disks::new_with_refreshed_list();

    let disk = match disks
        .iter()
        .find(|disk| disk.mount_point().to_string_lossy() == directory)
        .or_else(|| disks.iter().find(|disk| disk.mount_point() == std::path::Path::new("/")))
    {
        Some(disk) => disk,
        _ => return (0, 0, 0),
    };

    let total = disk.total_space();
    let free = disk.available_space();
    let used = total.saturating_sub(free);

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
    // include_str!() works even in compiled binaries
    let logo = match distro_id {
        "arch" => include_str!("../../ascii/arch.txt"),
        "ubuntu" => include_str!("../../ascii/ubuntu.txt"),
        "fedora" => include_str!("../../ascii/fedora.txt"),
        "manjaro" => include_str!("../../ascii/manjaro.txt"),
        "debian" => include_str!("../../ascii/debian.txt"),
        "opensuse" => include_str!("../../ascii/opensuse.txt"),
        "alpine" => include_str!("../../ascii/alpine.txt"),
        "gentoo" => include_str!("../../ascii/gentoo.txt"),
        "endeavouros" => include_str!("../../ascii/endeavouros.txt"),
        "popos" => include_str!("../../ascii/popos.txt"),
        "cachyos" => include_str!("../../ascii/cachyos.txt"),
        "garuda" => include_str!("../../ascii/garuda.txt"),
        "linuxmint" => include_str!("../../ascii/linuxmint.txt"),
        "kali" => include_str!("../../ascii/kali.txt"),
        "macos" => include_str!("../../ascii/macos.txt"),
        "zorin" => include_str!("../../ascii/zorin.txt"),
        "elementary" => include_str!("../../ascii/elementary.txt"),
        "nixos" => include_str!("../../ascii/nixos.txt"),
        _ => "",
    };

    logo.lines().map(|l| l.to_string()).collect()
}

/// Gets the pretty version of the GPU through wgpu, this function is really slow (~45ms), so its
/// value is stored in the cache and only retrieved at first startup or if the system ids do not
/// match the ones in the cache
pub fn get_gpu_name_pretty() -> Option<String> {
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

/// Gets gpu vendor and device ids and returns them as a tuple: (vendor, device)
pub fn get_gpu_ids() -> Option<(String, String)> {
    // On some systems the indexes might start at 1, so instead of iterating through every single
    // possible "card*" we try the first two which are the most likely
    let gpu_path = Path::new("/sys/class/drm/card0/device");
    let fallback_gpu_path = Path::new("/sys/class/drm/card1/device");

    let vendor = std::fs::read_to_string(gpu_path.join("vendor"))
        .or_else(|_| std::fs::read_to_string(fallback_gpu_path.join("vendor")))
        .ok()?
        .trim()
        .to_string();

    let device = std::fs::read_to_string(gpu_path.join("device"))
        .or_else(|_| std::fs::read_to_string(fallback_gpu_path.join("device")))
        .ok()?
        .trim()
        .to_string();

    Some((format_hex(&vendor), format_hex(&device)))
}

/// Gets subsystem IDs for the GPU, which are used to narrow down the possible names of the GPU
fn get_gpu_subsystem_ids() -> Option<(String, String)> {
    let gpu_path = Path::new("/sys/class/drm/card0/device");
    let fallback_gpu_path = Path::new("/sys/class/drm/card1/device");

    let subsystem_vendor = std::fs::read_to_string(gpu_path.join("subsystem_vendor"))
        .or_else(|_| std::fs::read_to_string(fallback_gpu_path.join("subsystem_vendor")))
        .ok()
        .map(|value| value.trim().to_string());

    let subsystem_device = std::fs::read_to_string(gpu_path.join("subsystem_device"))
        .or_else(|_| std::fs::read_to_string(fallback_gpu_path.join("subsystem_device")))
        .ok()
        .map(|value| value.trim().to_string());

    match (subsystem_vendor, subsystem_device) {
        (Some(subvendor), Some(subdevice)) => Some((subvendor, subdevice)),
        _ => None,
    }
}

/// Gets GPU family and possible names, returns them as string
pub fn get_gpu_name(cli: &Cli) -> Option<String> {
    // TODO: This function is pretty long and, while being significantly faster than WGPU
    // (45ms vs 3ms) it is also less accurate. Shorten it and add accuracy
    let (vendor_id, device_id) = get_gpu_ids()?;
    let subsystem_ids = get_gpu_subsystem_ids();

    let subsystem_ids = subsystem_ids
        .map(|(subvendor, subdevice)| (format_hex(&subvendor), format_hex(&subdevice)));

    if let Ok(cache) = get_cache(cli) {
        // again, this shouldn't be collapsed
        if cache.gpu_device_id == device_id && cache.gpu_vendor_id == vendor_id {
            return Some(cache.gpu_name_pretty);
        }
    }

    let pci_ids = std::fs::read_to_string("/usr/share/hwdata/pci.ids")
        .or_else(|_| std::fs::read_to_string("/usr/share/misc/pci.ids"))
        .ok()?;

    let mut current_vendor = None;
    let mut current_device = None;

    for line in pci_ids.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        // vendor lines start with no leading space
        if !line.starts_with('\t') && !line.starts_with(' ') {
            current_device = None;

            if let Some(vendor) = line.split_whitespace().next() {
                if vendor.eq_ignore_ascii_case(&vendor_id) {
                    current_vendor = Some(line.split_once("  ")?.1.trim());
                } else {
                    current_vendor = None;
                }
            }
        } else if current_vendor.is_some() && line.starts_with('\t') && !line.starts_with("\t\t") {
            if let Some(device) = line.trim().split_whitespace().next() {
                if device.eq_ignore_ascii_case(&device_id) {
                    let name = line.split_once("  ")?.1.trim();
                    current_device = Some(name);
                } else {
                    current_device = None;
                }
            }
        } else if current_vendor.is_some() && current_device.is_some() && line.starts_with("\t\t") {
            // Even though clippy is complaining about it, this already long if statement should not
            // be collapsed. This comment prevents that
            if let Some((target_subvendor, target_subdevice)) = subsystem_ids.as_ref() {
                let trimmed = line.trim();
                let mut parts = trimmed.split_whitespace();
                let subvendor = parts.next()?;
                let subdevice = parts.next()?;

                if subvendor.eq_ignore_ascii_case(target_subvendor)
                    && subdevice.eq_ignore_ascii_case(target_subdevice)
                {
                    let mut split = trimmed.splitn(3, char::is_whitespace);
                    split.next()?;
                    split.next()?;
                    let subsystem_name = split.next()?.trim();
                    if !subsystem_name.is_empty() {
                        return Some(subsystem_name.to_string());
                    }
                }
            }
        }
    }

    Some(format!("{} {}", current_vendor?, current_device?))
}

/// Gets the screen resolution and returns it as (width, height)
pub fn get_screen_resolution() -> Option<(u64, u64)> {
    let displays = DisplayInfo::all().ok()?;
    let display = displays.first()?;
    Some((display.width as u64, display.height as u64))
}

pub fn get_screen_refresh_rate() -> Option<u64> {
    // repeated variable between this and get_screen_resolution, we can pass a reference if display
    // is true later on
    let displays = DisplayInfo::all().ok()?;
    let display = displays.first()?;

    (display.frequency > 0.0).then_some(display.frequency as u64)
}

pub fn get_host_name() -> Option<String> {
    System::host_name()
}

pub fn get_username() -> String {
    std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
}
