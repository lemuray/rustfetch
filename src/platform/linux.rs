use std::{fs, path::Path};

use crate::{
    cache::{create_cache, get_cache},
    cli::Cli,
    common::*,
    sysinfo::*,
};

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
        } else {
            let _ = create_cache();
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
            if let Some(device) = line.split_whitespace().next() {
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
