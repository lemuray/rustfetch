use crate::common::extract_numeric_value;
use crate::platform::{self, get_power_draw};
use crate::sysinfo::*;
use colored::*;
use sysinfo::System;

fn color_percentage(percentage: u64) -> ColoredString {
    if percentage < 40 {
        (percentage.to_string() + "%").green()
    } else if (40..80).contains(&percentage) {
        (percentage.to_string() + "%").yellow()
    } else {
        (percentage.to_string() + "%").red()
    }
}

fn color_percentage_inverse(percentage: f64) -> ColoredString {
    if percentage < 30.0 {
        (percentage.to_string() + "%").red()
    } else if (30.0..70.0).contains(&percentage) {
        (percentage.to_string() + "%").yellow()
    } else {
        (percentage.to_string() + "%").green()
    }
}

pub fn display_os() {
    println!(
        "{} {} ({})",
        "OS:".bold(),
        get_os_name(),
        std::env::consts::ARCH // CPU Architecture the program was compiled for
    );
}

pub fn display_kernel() {
    println!("{} {}", "Kernel:".bold(), platform::format_kernel_version());
}

pub fn display_cpu(sys: &System) {
    println!("{} {}", "CPU:".bold(), get_cpu_name(sys));
}

pub fn display_ram_usage(sys: &System) {
    let (total, used, percentage) = get_ram_usage(sys);
    println!(
        "{} {} / {} ({})",
        "RAM:".bold(),
        used,
        total,
        color_percentage(percentage)
    );
}

pub fn display_swap_usage(sys: &System) {
    let (total, used, percentage) = get_swap_usage(sys);
    if extract_numeric_value(&total) == 0.0 {
        println!("{} Disabled", "Swap:".bold())
    }
    println!(
        "{} {} / {} ({})",
        "Swap:".bold(),
        used,
        total,
        color_percentage(percentage)
    );
}

pub fn display_uptime() {
    println!("{} {}", "Uptime:".bold(), get_uptime());
}

pub fn display_battery() {
    let (capacity, status) = platform::get_battery();
    if capacity != "Null" && status != "Null" {
        println!(
            "{} {} ({})",
            "Battery:".bold(),
            color_percentage_inverse(capacity.parse::<f64>().unwrap()),
            status
        );
    }
}
pub fn display_power_draw() {
    let power_draw = get_power_draw();
    if power_draw != 0 {
        println!("{} {}W", "Power Draw:".bold(), power_draw)
    }
}

pub fn display_disk_usage() {
    let (total, used, percentage) = platform::get_disk_usage();
    println!(
        "{} {}GB / {}GB ({})",
        "Disk (/):".bold(),
        used,
        total,
        color_percentage(percentage)
    )
}
