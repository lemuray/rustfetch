pub mod cache;
pub mod cli;
pub mod common;
pub mod config;
pub mod platform;
pub mod sysinfo;

use clap::Parser;

use crate::{
    cli::Cli,
    common::{get_logo_lines, print_logo},
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
        .then(common::display_identifier)
        .into_iter()
        .flatten()
        .chain(
            vec![
                config.display.os.then(common::display_os),
                config.display.kernel.then(common::display_kernel),
                config.display.cpu.then(|| common::display_cpu(&sys, &config)),
                #[cfg(target_os = "linux")]
                config.display.gpu.then(|| common::display_gpu_name(&cli)).flatten(),
                config.display.screen.then(|| common::display_screen(&config)).flatten(),
                config.display.ram.then(|| common::display_ram_usage(&sys)),
                config.display.swap.then(|| common::display_swap_usage(&sys)),
                config.display.uptime.then(common::display_uptime),
                #[cfg(target_os = "linux")]
                config.display.battery.then(common::display_battery).flatten(),
                #[cfg(target_os = "linux")]
                config.display.power_draw.then(common::display_power_draw).flatten(),
                config.display.disk.then(common::display_disk_usage),
            ]
            .into_iter()
            .flatten(),
        )
        .collect();

    let logo_lines = get_logo_lines(&distro_id);

    print_logo(logo_lines, info_lines, &distro_id, &cli)?;

    Ok(())
}
