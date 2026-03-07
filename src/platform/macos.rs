use crate::{cli::Cli, sysinfo::*};

// Same as before
pub fn get_power_draw() -> Option<u32> {
    None
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

// the following functions will never run since they're for linux only features but the compiler
// complains if they're not in macos
pub fn get_battery() -> (String, String) {
    None
}
pub fn get_gpu_ids() -> Option<(String, String)> {
    None
}
pub fn get_gpu_subsystem_ids() -> Option<(String, String)> {
    None
}
pub fn get_gpu_name(_cli: &Cli) -> Option<String> {
    None
}
