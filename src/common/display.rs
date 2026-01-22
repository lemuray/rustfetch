use colored::*;
use std::path::Path;

use crate::common::{get_trimmed, get_value_from_file};
use crate::platform::{self, get_power_draw};

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

pub fn display_ram_usage() {
    let (total, used, percentage) = platform::get_ram_usage();
    println!(
        "{} {}GB / {}GB ({} used)",
        "RAM:".bold(),
        used,
        total,
        color_percentage(percentage)
    );
}

pub fn display_swap_usage(){
    let (total, used, percentage) = platform::get_swap_usage();
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
