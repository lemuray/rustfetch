pub mod common;
pub mod platform;

/*
    TODO:
    Error Detection in case function returns "Null"
    Test with other distros
    Finish translating to rust
*/



fn main() {
        common::display_os();
        common::display_kernel();
        common::display_cpu();
        common::display_memory_usage();
        common::display_uptime();
        common::display_battery();
}
