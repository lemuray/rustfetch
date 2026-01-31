pub mod cli;
pub mod common;
pub mod config;
pub mod platform;
pub mod sysinfo;

use clap::Parser;
use cli::Cli;
use config::load_config;

use crate::config::load_all_config;

// TODO:
// Add ASCII art
// Add CPU, GPU: temps, usage

fn main() {
    let cli = Cli::parse();

    // We are creating a System variable which we are sharing across all sysinfo functions to
    // have not have overhead creating that variable inside every function
    let sys = sysinfo::create_system();

    // If the --all flag is active the config variable will be set to the load_all_config function
    let config = if cli.all {
        load_all_config()
    } else {
        load_config()
    };

    if config.display.os {
        common::display_os();
    }
    if config.display.kernel {
        common::display_kernel();
    }
    if config.display.cpu {
        common::display_cpu(&sys);
    }
    if config.display.ram {
        common::display_ram_usage(&sys);
    }
    if config.display.swap {
        common::display_swap_usage(&sys);
    }
    if config.display.uptime {
        common::display_uptime();
    }
    #[cfg(target_os = "linux")]
    {
        // These info are only available on Linux
        if config.display.battery {
            common::display_battery();
        }
        if config.display.power_draw {
            common::display_power_draw();
        }
    }
    if config.display.disk {
        common::display_disk_usage();
    }
    if config.display.cpu_frequency {
        common::display_cpu_frequency(&sys);
    }
}
