pub mod cli;
pub mod common;
pub mod config;
pub mod platform;
pub mod sysinfo;

use std::io::Write;

use clap::Parser;
use cli::Cli;
use config::load_config;

use crate::{config::load_all_config, platform::colorize_logo_line};

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
    let logo_lines = sysinfo::get_logo_lines(&distro_id);

    let info_lines: Vec<String> = vec![
        config.display.os.then(common::display_os),
        config.display.kernel.then(common::display_kernel),
        config.display.cpu.then(|| common::display_cpu(&sys, &config)),
        config.display.gpu.then(common::display_gpu_info).flatten(),
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
    .flatten()
    .collect();

    let mut stdout = std::io::stdout();

    if logo_lines.is_empty() {
        // If the logo does not match any inside ../ascii/LOGO.txt, just print the info
        for line in info_lines {
            let _ = writeln!(stdout, "{}", line);
        }
    } else {
        let max_lines = logo_lines.len().max(info_lines.len());
        // We get the maximum length from the logo using .max()
        let logo_column_width = logo_lines.iter().map(|l| l.len()).max().unwrap_or(0);

        for i in 0..max_lines {
            if i < logo_lines.len() {
                write!(stdout, "{}", colorize_logo_line(&distro_id, &logo_lines[i]))?;
                let padding =
                    logo_column_width.saturating_sub(logo_lines[i].len()) + cli.padding as usize;
                write!(stdout, "{:width$}", "", width = padding)?;
            } else {
                // when past logo lines, print spaces that are logo_column_width + padding
                let total_width = logo_column_width + cli.padding as usize;
                write!(stdout, "{:width$}", "", width = total_width)?;
            }

            if i < info_lines.len() {
                writeln!(stdout, "  {}", info_lines[i])?;
            } else {
                writeln!(stdout)?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}
