use chrono::*;
use colored::*;
use sysinfo::System;

use crate::{
    cli::Cli,
    common::round_to_two_decimal,
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

pub fn display_os() -> Option<String> {
    Some(format!(
        "{} {} ({})",
        "OS:".bold(),
        get_os_name()?,
        std::env::consts::ARCH // CPU Architecture the program was compiled for
    ))
}

pub fn display_kernel() -> Option<String> {
    let kernel = platform::format_kernel_version()?;
    Some(format!("{} {}", "Kernel:".bold(), kernel))
}

pub fn display_cpu(sys: &System, config: &Config) -> Option<String> {
    if !config.display.cpu_frequency {
        return None;
    }

    let cpu_name = get_cpu_name(sys)?;
    let frequency = get_cpu_frequency(sys)?;

    let cpu_frequency = if frequency >= 1000 {
        format!(" @ {} GHz ", round_to_two_decimal(frequency as f64 / 1000.0))
    } else {
        format!(" @ {} MHz ", frequency)
    };

    Some(format!("{} {}{}", "CPU:".bold(), cpu_name, cpu_frequency))
}

pub fn display_ram_usage(sys: &System) -> Option<String> {
    let (total, used, percentage) = get_ram_usage(sys)?;
    Some(format!(
        "{} {} / {} ({})",
        "RAM:".bold(),
        used,
        total,
        color_percentage(percentage)
    ))
}

pub fn display_swap_usage(sys: &System) -> String {
    if let Some(values) = get_swap_usage(sys) {
        format!(
            "{} {} / {} ({})",
            "Swap:".bold(),
            values.1,
            values.0,
            color_percentage(values.2)
        )
    } else {
        format!("{} Disabled", "Swap:".bold())
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
        tracing::debug!("No battery info detected for device, possibly not a laptop");
        None
    }
}
pub fn display_power_draw() -> Option<String> {
    let power_draw = get_power_draw();
    if power_draw != 0 {
        Some(format!("{} {}W", "Power Draw:".bold(), power_draw))
    } else {
        tracing::debug!("No power draw detected for device, possibly not a laptop");
        None
    }
}

pub fn display_disk_usage() -> Option<String> {
    let (total, used, percentage) = platform::get_disk_usage()?;
    Some(format!(
        "{} {}GB / {}GB ({})",
        "Disk (/):".bold(), // FIXME: Shows "/" dir statically
        used,
        total,
        color_percentage(percentage)
    ))
}

pub fn display_gpu_name(cli: &Cli) -> Option<String> {
    platform::get_gpu_name(cli).map(|gpu_name| format!("{} {}", "GPU:".bold(), gpu_name))
}

pub fn display_screen(config: &Config) -> Option<String> {
    if !config.display.resolution && !config.display.refresh_rate {
        // I'm sure theres a better way to do this, but this works as well
        tracing::warn!(
            "The screen option was active, but both its subparts were deactivated. Skipping \
             screen info fetching..."
        );
        return None;
    }

    let resolution;
    if config.display.resolution
        && let Some((width, height)) = get_screen_resolution()
    {
        resolution = format!("{}x{}", width, height);
    } else {
        tracing::warn!("No resolution found for monitor");
        resolution = String::from("");
    }

    let refresh_rate;
    if config.display.refresh_rate
        && let Some(rr) = get_screen_refresh_rate()
    {
        refresh_rate = format!("@ {}Hz", rr);
    } else {
        tracing::warn!("No refresh rate found for monitor");
        refresh_rate = String::from("");
    }

    if resolution.is_empty() && refresh_rate.is_empty() {
        // if both are empty, i.e. if the system is headless
        // or no screen is detected return None
        tracing::debug!(
            "No resolution and refresh rate found for monitor, possibly running a headless setup. \
             Skipping printing monitor info..."
        );
        return None;
    }

    Some(format!("{} {} {}", "Screen:".bold(), resolution, refresh_rate))
}

pub fn display_identifier() -> Vec<String> {
    let mut host_name = String::from("");
    if let Some(host) = get_host_name() {
        host_name = host;
    }
    let username = get_username();

    let mut identifier = format!("{}@{}", username, host_name);
    let underline = "-".repeat(identifier.chars().count());

    // this may add overhead and i will try to find a better way,
    // though bolding everything up messes with the underline
    identifier = format!("{}", identifier.bold());

    vec![identifier, underline]
}

pub fn display_de() -> Option<String> {
    let de = get_de()?;
    let ds = get_display_system()?;
    Some(format!("{} {} ({})", "DE:".bold(), de, ds))
}

pub fn display_date() -> String {
    let time: DateTime<Local> = Local::now();
    format!("{} {}", "Date:".bold(), time.format("%d/%m/%Y"))
}

pub fn display_time() -> String {
    let time: DateTime<Local> = Local::now();
    format!("{} {}", "Time:".bold(), time.format("%H:%M:%S"))
}

pub fn display_shell() -> Option<String> {
    if let Ok((shell, version)) = get_shell() {
        Some(format!("{} {} {}", "Shell:".bold(), shell, version))
    } else {
        tracing::warn!("Unable to get shell info, skipping printing it...");
        None
    }
}
