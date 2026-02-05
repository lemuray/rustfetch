pub mod cli;
pub mod common;
pub mod config;
pub mod platform;
pub mod sysinfo;

use std::io::Write;

use clap::Parser;
use cli::Cli;
use colored::*;
use config::load_config;
use rustfetch::sysinfo::get_gpu_info;

use crate::{common::display_gpu_info, config::load_all_config, platform::colorize_logo_line};

// TODO:
// Add CPU, GPU: temps, usage

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = if cli.all {
        load_all_config()
    } else {
        load_config()
    };
    let sys = sysinfo::create_system(&config);

    let distro_id = platform::get_distro_id();
    let logo_lines = sysinfo::get_logo_lines(&distro_id);

    // Here we're creating an empty vector that'll hold our printing information in different
    // indexes
    let mut info_lines: Vec<String> = Vec::new();

    if config.display.os {
        info_lines.push(common::display_os());
    }
    if config.display.kernel {
        info_lines.push(common::display_kernel());
    }
    if config.display.cpu {
        info_lines.push(common::display_cpu(&sys, &config));
    }
    if config.display.gpu
        && let Some(gpu_info) = display_gpu_info()
    {
        info_lines.push(gpu_info);
    }
    if config.display.ram {
        info_lines.push(common::display_ram_usage(&sys));
    }
    if config.display.swap {
        info_lines.push(common::display_swap_usage(&sys));
    }
    if config.display.uptime {
        info_lines.push(common::display_uptime());
    }

    #[cfg(target_os = "linux")]
    {
        if config.display.battery
            && let Some(battery_info) = common::display_battery()
        {
            info_lines.push(battery_info);
        }
        if config.display.power_draw
            && let Some(power_draw) = common::display_power_draw()
        {
            info_lines.push(power_draw);
        }
    }

    if config.display.disk {
        info_lines.push(common::display_disk_usage());
    }

    let mut stdout = std::io::stdout();
    let max_lines = logo_lines.len().max(info_lines.len());
    // We get the maximum length from the logo using .max()
    let logo_column_width = logo_lines.iter().map(|l| l.len()).max().unwrap_or(0);

    for i in 0..max_lines {
        if i < logo_lines.len() {
            write!(stdout, "{}", colorize_logo_line(&distro_id, &logo_lines[i]))?;
            // TODO: Add a command line argument to increase padding (padding += cli.arg)
            let padding = logo_column_width.saturating_sub(logo_lines[i].len());
            write!(stdout, "{:width$}", "", width = padding)?;
        } else {
            write!(stdout, "{:width$}", "", width = logo_column_width)?;
        }

        if i < info_lines.len() {
            writeln!(stdout, "  {}", info_lines[i].white())?;
        } else {
            writeln!(stdout)?;
        }
    }

    stdout.flush()?;
    Ok(())
}
