pub mod cache;
pub mod cli;
pub mod common;
pub mod config;
pub mod platform;
pub mod sysinfo;

use clap::Parser;

use crate::{
    cli::Cli,
    common::*,
    config::{load_all_config, load_config},
};

// TODO:
// Add CPU, GPU: temps, usage

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = if cli.all {
        load_all_config()
    } else {
        load_config(&cli)
    };
    let sys = sysinfo::create_system(&config);

    let distro_id = platform::get_distro_id();

    let info_lines: Vec<String> = config
        .display
        .identifier
        .then(display_identifier)
        .into_iter()
        .flatten()
        .chain(
            vec![
                config.display.os.then(display_os),
                config.display.kernel.then(display_kernel),
                config.display.kernel.then(display_shell).flatten(),
                config.display.de.then(display_de).flatten(),
                config.display.cpu.then(|| display_cpu(&sys, &config)),
                #[cfg(target_os = "linux")] // yet to be implemented, possible
                config.display.gpu.then(|| display_gpu_name(&cli)).flatten(),
                config.display.screen.then(|| display_screen(&config)).flatten(),
                config.display.ram.then(|| display_ram_usage(&sys)),
                config.display.swap.then(|| display_swap_usage(&sys)),
                config.display.uptime.then(display_uptime),
                config.display.date.then(display_date),
                config.display.time.then(display_time),
                #[cfg(target_os = "linux")]
                config.display.battery.then(display_battery).flatten(),
                #[cfg(target_os = "linux")]
                config.display.power_draw.then(display_power_draw).flatten(),
                config.display.disk.then(display_disk_usage),
            ]
            .into_iter()
            .flatten(),
        )
        .collect();

    let logo_lines = get_logo_lines(&distro_id, &cli);

    if let Some(parsed_lines) = get_logo_style(logo_lines) {
        print_logo(parsed_lines, info_lines, &cli)?;
    }

    Ok(())
}
