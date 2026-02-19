//! This file handles everything related to displaying the distro logo

use std::io::{BufWriter, Write};

use colored::*;

use crate::cli::Cli;

/// Gets the lines logos in a vector and returns them
pub fn get_logo_lines(distro_id: &str) -> Vec<String> {
    // include_str!() works even in compiled binaries
    let logo = match distro_id {
        "arch" => include_str!("../../ascii/arch.txt"),
        "ubuntu" => include_str!("../../ascii/ubuntu.txt"),
        "fedora" => include_str!("../../ascii/fedora.txt"),
        "manjaro" => include_str!("../../ascii/manjaro.txt"),
        "debian" => include_str!("../../ascii/debian.txt"),
        "opensuse" => include_str!("../../ascii/opensuse.txt"),
        "alpine" => include_str!("../../ascii/alpine.txt"),
        "gentoo" => include_str!("../../ascii/gentoo.txt"),
        "endeavouros" => include_str!("../../ascii/endeavouros.txt"),
        "popos" => include_str!("../../ascii/popos.txt"),
        "cachyos" => include_str!("../../ascii/cachyos.txt"),
        "garuda" => include_str!("../../ascii/garuda.txt"),
        "linuxmint" => include_str!("../../ascii/linuxmint.txt"),
        "kali" => include_str!("../../ascii/kali.txt"),
        "macos" => include_str!("../../ascii/macos.txt"),
        "zorin" => include_str!("../../ascii/zorin.txt"),
        "elementary" => include_str!("../../ascii/elementary.txt"),
        "nixos" => include_str!("../../ascii/nixos.txt"),
        _ => "",
    };

    logo.lines().map(|l| l.to_string()).collect()
}

/// Getting a reference to an element of the vector of strings we're printing and matching it with
/// the distro id, we get return its colorized version.
/// Example: if distro_id = ubuntu, then we return line.orange()
pub fn colorize_logo_line(distro_id: &str, line: &str) -> ColoredString {
    match distro_id {
        // The exact colors should be tested on your distro and eventually changed it they do not
        // match the color of the distro's logo excessively
        "arch" => line.truecolor(23, 147, 209),
        "ubuntu" => line.truecolor(255, 156, 0),
        "cachyos" => line.truecolor(0, 184, 148),
        "fedora" => line.truecolor(11, 87, 164),
        "garuda" => line.truecolor(138, 43, 226),
        "gentoo" => line.truecolor(84, 73, 149),
        "endeavouros" => line.truecolor(122, 58, 237),
        "kali" => line.truecolor(38, 139, 210),
        "linuxmint" | "manjaro" => line.green(),
        "debian" => line.red(),
        "alpine" => line.cyan(),
        "popos" => line.truecolor(72, 149, 239),
        "opensuse" => line.truecolor(115, 186, 37),
        "nixos" => line.truecolor(125, 176, 221),
        "zorin" => line.truecolor(17, 162, 236),
        "elementary" => line.white(),
        _ => line.white(), // this includes macos
    }
}

pub fn print_logo(
    logo_lines: Vec<String>,
    info_lines: Vec<String>,
    distro_id: &str,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = std::io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    if logo_lines.is_empty() {
        // If the logo does not match any inside ../ascii/LOGO.txt, just print the info
        for line in info_lines {
            let _ = writeln!(handle, "{}", line);
        }
    } else {
        let max_lines = logo_lines.len().max(info_lines.len());
        // We get the maximum length from the logo using .max()
        let logo_column_width = logo_lines.iter().map(|l| l.len()).max().unwrap_or(0);

        for i in 0 .. max_lines {
            if i < logo_lines.len() {
                write!(handle, "{}", colorize_logo_line(distro_id, &logo_lines[i]))?;
                let padding =
                    logo_column_width.saturating_sub(logo_lines[i].len()) + cli.padding as usize;
                write!(handle, "{:width$}", "", width = padding)?;
            } else {
                // when past logo lines, print spaces that are logo_column_width + padding
                let total_width = logo_column_width + cli.padding as usize;
                write!(handle, "{:width$}", "", width = total_width)?;
            }

            if i < info_lines.len() {
                writeln!(handle, "  {}", info_lines[i])?;
            } else {
                writeln!(handle)?;
            }
        }
    }

    handle.flush()?;
    Ok(())
}
