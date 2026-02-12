use colored::*;
use sysinfo::System;

use crate::{
    common::{extract_numeric_value, round_to_two_decimal},
    config::Config,
    platform::{self, get_power_draw},
    sysinfo::*,
};

fn color_percentage(percentage: u64) -> ColoredString {
    if percentage < 40 {
        format!("{}%", percentage).green()
    } else if (40..80).contains(&percentage) {
        format!("{}%", percentage).yellow()
    } else {
        format!("{}%", percentage).red()
    }
}

fn color_percentage_inverse(percentage: f64) -> ColoredString {
    if percentage < 30.0 {
        format!("{}%", percentage).red()
    } else if (30.0..70.0).contains(&percentage) {
        format!("{}%", percentage).yellow()
    } else {
        format!("{}%", percentage).green()
    }
}

pub fn display_os() -> String {
    format!(
        "{} {} ({})",
        "OS:".bold(),
        get_os_name(),
        std::env::consts::ARCH // CPU Architecture the program was compiled for
    )
}

pub fn display_kernel() -> String {
    format!("{} {}", "Kernel:".bold(), platform::format_kernel_version())
}

pub fn display_cpu(sys: &System, config: &Config) -> String {
    let cpu_name = get_cpu_name(sys);

    let cpu_frequency;
    if config.display.cpu_frequency {
        let frequency = get_cpu_frequency(sys);
        if frequency >= 1000 {
            cpu_frequency = format!(" @ {} GHz ", round_to_two_decimal(frequency as f64 / 1000.0))
        } else {
            cpu_frequency = format!(" @ {} MHz ", frequency)
        }
    } else {
        cpu_frequency = String::from("");
    }

    format!("{} {}{}", "CPU:".bold(), cpu_name, cpu_frequency)
}

pub fn display_ram_usage(sys: &System) -> String {
    let (total, used, percentage) = get_ram_usage(sys);
    format!("{} {} / {} ({})", "RAM:".bold(), used, total, color_percentage(percentage))
}

pub fn display_swap_usage(sys: &System) -> String {
    let (total, used, percentage) = get_swap_usage(sys);
    if extract_numeric_value(&total).is_ok_and(|v| v == 0.0) {
        format!("{} Disabled", "Swap:".bold())
    } else {
        format!("{} {} / {} ({})", "Swap:".bold(), used, total, color_percentage(percentage))
    }
}

pub fn display_uptime() -> String {
    format!("{} {}", "Uptime:".bold(), get_uptime())
}

pub fn display_battery() -> Option<String> {
    let (capacity, status) = platform::get_battery();
    if capacity != "Unavailable" && status != "Unavailable" {
        Some(format!(
            "{} {} ({})",
            "Battery:".bold(),
            color_percentage_inverse(capacity.parse::<f64>().unwrap_or(0.0)),
            status
        ))
    } else {
        None
    }
}
pub fn display_power_draw() -> Option<String> {
    let power_draw = get_power_draw();
    if power_draw != 0 {
        Some(format!("{} {}W", "Power Draw:".bold(), power_draw))
    } else {
        None
    }
}

pub fn display_disk_usage() -> String {
    let (total, used, percentage) = platform::get_disk_usage();
    format!(
        "{} {}GB / {}GB ({})",
        "Disk (/):".bold(), // FIXME: Shows "/" dir statically
        used,
        total,
        color_percentage(percentage)
    )
}

pub fn display_gpu_info() -> Option<String> {
    get_gpu_info().map(|gpu_info| format!("{} {}", "GPU:".bold(), gpu_info))
}

pub fn display_screen(config: &Config) -> Option<String> {
    if !config.display.resolution && !config.display.refresh_rate {
        // I'm sure theres a better way to do this, but this works as well
        return None;
    }

    let resolution;
    if config.display.resolution
        && let Some((width, height)) = get_screen_resolution()
    {
        resolution = format!("{}x{}", width, height);
    } else {
        resolution = String::from("");
    }

    let refresh_rate;
    if config.display.refresh_rate
        && let Some(rr) = get_screen_refresh_rate()
    {
        refresh_rate = format!("@ {}Hz", rr);
    } else {
        refresh_rate = String::from("");
    }

    Some(format!("{} {} {}", "Screen:".bold(), resolution, refresh_rate))
}
