use colored::*;
use sysinfo::System;

use crate::{
    common::extract_numeric_value,
    platform::{self, get_power_draw},
    sysinfo::*,
};

fn color_percentage(percentage: u64) -> ColoredString {
    if percentage < 40 {
        format!("{}%", percentage).green()
    } else if (40 .. 80).contains(&percentage) {
        format!("{}%", percentage).yellow()
    } else {
        format!("{}%", percentage).red()
    }
}

fn color_percentage_inverse(percentage: f64) -> ColoredString {
    if percentage < 30.0 {
        format!("{}%", percentage).red()
    } else if (30.0 .. 70.0).contains(&percentage) {
        format!("{}%", percentage).yellow()
    } else {
        format!("{}%", percentage).green()
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
    println!("{} {} / {} ({})", "RAM:".bold(), used, total, color_percentage(percentage));
}

pub fn display_swap_usage(sys: &System) {
    let (total, used, percentage) = get_swap_usage(sys);
    if extract_numeric_value(&total).is_ok_and(|v| v == 0.0) {
        println!("{} Disabled", "Swap:".bold())
    } else {
        println!("{} {} / {} ({})", "Swap:".bold(), used, total, color_percentage(percentage));
    }
}

pub fn display_uptime() {
    println!("{} {}", "Uptime:".bold(), get_uptime());
}

pub fn display_battery() {
    let (capacity, status) = platform::get_battery();
    if capacity != "Unavailable" && status != "Unavailable" {
        println!(
            "{} {} ({})",
            "Battery:".bold(),
            color_percentage_inverse(capacity.parse::<f64>().unwrap_or(0.0)),
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
        "Disk (/):".bold(), // FIXME: Shows "/" dir statically
        used,
        total,
        color_percentage(percentage)
    )
}

/// Formats frequency to GHz or MHz
pub fn display_cpu_frequency(sys: &System) {
    let frequency = get_cpu_frequency(sys);
    if frequency >= 1000 {
        println!("{} {} GHz", "Frequency".bold(), frequency as f64 / 1000.0)
    } else {
        println!("{} {} MHz", "Frequency".bold(), frequency)
    }
}
