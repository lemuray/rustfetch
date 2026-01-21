use colored::*;
use std::path::Path;

use crate::common::{get_trimmed, get_value_from_file};
use crate::platform;

// TODO: Most get_trimmed or get_value_from_file functions statically refer to linux directories, create functions in platform
//       for this

fn color_percentage(percentage: f64) -> ColoredString {
    if percentage < 40.0 {
        (percentage.to_string() + "%").green()
    } else if (40.0..80.0).contains(&percentage) {
        (percentage.to_string() + "%").yellow()
    } else {
        (percentage.to_string() + "%").red()
    }
}

pub fn display_os() {
    println!(
        "{} {} ({})",
        "OS:".bold(),
        get_value_from_file(Path::new("/etc/os-release"), "PRETTY_NAME", "="),
        std::env::consts::ARCH // CPU Architecture the program was compiled for
    );
}

pub fn display_kernel() {
    println!(
        "{} Linux {}", // FIXME: Only displays linux statically
        "Kernel:".bold(),
        get_trimmed(Path::new("/proc/sys/kernel/osrelease"))
    );
}

pub fn display_cpu() {
    println!(
        "{} {}",
        "CPU:".bold(),
        get_value_from_file(Path::new("/proc/cpuinfo"), "model name", ":")
    );
}

pub fn display_memory_usage() {
    let (total, used, percentage) = platform::get_memory_usage("MemTotal", "MemAvailable");
    println!(
        "{} {}GB / {}GB ({} used)",
        "RAM:".bold(),
        used,
        total,
        color_percentage(percentage)
    );

    let (total, used, percentage) = platform::get_memory_usage("SwapTotal", "SwapFree");
    println!(
        "{} {}GB / {}GB ({} used)",
        "Swap:".bold(),
        used,
        total,
        color_percentage(percentage)
    );
}

pub fn display_uptime() {
    println!("{} {}", "Uptime:".bold(), platform::get_uptime());
}

pub fn display_battery() {
    let (capacity, status) = platform::get_battery();
    if capacity != "Null" && status != "Null" {
        println!("{} {}% ({})", "Battery:".bold(), capacity, status);
    }
}
