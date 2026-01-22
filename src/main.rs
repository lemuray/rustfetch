pub mod common;
pub mod platform;

/*
    TODO:
    Error Detection in case function returns "Null"
    Test with other distros
    Add JSON settings file to selectively activate or deactivate specific functions
*/



fn main() {
        common::display_os();
        common::display_kernel();
        common::display_cpu();
        common::display_memory_usage();
        common::display_uptime();
        common::display_battery();
        common::display_disk_usage();
        common::display_power_draw();
}
