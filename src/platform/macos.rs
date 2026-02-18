use colored::*;

use crate::sysinfo::*;

// This will never run as it is called only if the program is compiled for linux
// but it makes the compiler shut up
pub fn get_battery() -> (String, String) {
    (String::from("Null"), String::from("Null"))
}

// Same as before
pub fn get_power_draw() -> i32 {
    0
}

pub fn get_disk_usage() -> (u64, u64, u64) {
    // MacOS root directory
    get_directory_usage("/")
}

pub fn format_kernel_version() -> String {
    format!("MacOS {}", get_kernel_version())
}

pub fn get_distro_id() -> String {
    String::from("macos")
}
