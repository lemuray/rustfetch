//! This file handles everything related to displaying the distro logo

use std::io::{BufWriter, Write};

use colored::*;
use unicode_width::*;

use crate::cli::Cli;

#[derive(Clone, Default)]
pub struct Style {
    // using this struct, features like bold or underline
    // can be easily added later
    color_id: u8,
}

pub struct Segment {
    text: String,
    style: Style,
    visible_width: usize,
}

#[derive(Default)]
pub struct ParsedLine {
    segments: Vec<Segment>,
}

pub fn get_logo_style(logo: Vec<String>) -> Option<Vec<ParsedLine>> {
    if logo.is_empty() {
        tracing::warn!("Logo is empty, skipping printing...");
        return None;
    }

    let mut lines: Vec<ParsedLine> = Vec::new();
    // style carries over between lines
    let mut carried_style = Style::default();

    for raw_line in logo {
        let mut line = ParsedLine::default();
        let mut is_in_style = false;
        let mut style_buf = String::new();
        // whatever style was left by the previous line
        let mut current_segment = Segment {
            text: String::new(),
            style: carried_style.clone(),
            visible_width: 0,
        };

        for ch in raw_line.chars() {
            if ch == '{' {
                is_in_style = true;
                style_buf.clear();
                continue;
            } else if ch == '}' {
                is_in_style = false;

                // close and push the segment that was open before this flag,
                // only if it contains text
                if !current_segment.text.is_empty() {
                    current_segment.visible_width =
                        UnicodeWidthStr::width(current_segment.text.as_str());
                    line.segments.push(current_segment);
                }

                // the color prefix is c:X where x is a color in
                // color_id_to_color
                if let Some(id_str) = style_buf.strip_prefix("c:") {
                    // dont collapse
                    if let Ok(id) = id_str.parse::<u8>() {
                        carried_style.color_id = id;
                    }
                }

                current_segment = Segment {
                    text: String::new(),
                    style: carried_style.clone(),
                    visible_width: 0,
                };
                style_buf.clear();
                continue;
            } else if is_in_style {
                // accumulate whats in the style
                style_buf.push(ch);
            } else {
                current_segment.text.push(ch);
            }
        }

        if !current_segment.text.is_empty() {
            current_segment.visible_width = UnicodeWidthStr::width(current_segment.text.as_str());
            line.segments.push(current_segment);
        }

        lines.push(line);
    }

    tracing::info!("Succesfully parsed logo");
    Some(lines)
}

/// Gets the lines logos in a vector and returns them
pub fn get_logo_lines(distro_id: &str, cli: &Cli) -> Vec<String> {
    // the following variable is used in case the small_logo
    // flag is active and the logo doesn't have a small equivalent,
    // it becomes negative when the small logo is not found so the
    // function parses the bigger logos instead
    let mut found = true;
    let mut logo = "";
    if cli.small_logo {
        logo = match distro_id {
            "arch" => include_str!("../../ascii/small/arch.txt"),
            "ubuntu" => include_str!("../../ascii/small/ubuntu.txt"),
            "fedora" => include_str!("../../ascii/small/fedora.txt"),
            "manjaro" => include_str!("../../ascii/small/manjaro.txt"),
            "debian" => include_str!("../../ascii/small/debian.txt"),
            "opensuse" => include_str!("../../ascii/small/opensuse.txt"),
            "alpine" => include_str!("../../ascii/small/alpine.txt"),
            "gentoo" => include_str!("../../ascii/small/gentoo.txt"),
            "endeavouros" => include_str!("../../ascii/small/endeavouros.txt"),
            "popos" => include_str!("../../ascii/small/popos.txt"),
            "cachyos" => include_str!("../../ascii/small/cachyos.txt"),
            "garuda" => include_str!("../../ascii/small/garuda.txt"),
            "linuxmint" => include_str!("../../ascii/small/linuxmint.txt"),
            "kali" => include_str!("../../ascii/small/kali.txt"),
            "macos" => include_str!("../../ascii/small/macos.txt"),
            "nixos" => include_str!("../../ascii/small/nixos.txt"),
            _ => {
                tracing::warn!(
                    "--small-logo flag active but no small logo found for user's distro, \
                     resorting to bigger ones..."
                );
                found = false;
                ""
            },
        };
    }
    if !cli.small_logo || !found {
        logo = match distro_id {
            "arch" => include_str!("../../ascii/big/arch.txt"),
            "ubuntu" => include_str!("../../ascii/big/ubuntu.txt"),
            "fedora" => include_str!("../../ascii/big/fedora.txt"),
            "manjaro" => include_str!("../../ascii/big/manjaro.txt"),
            "debian" => include_str!("../../ascii/big/debian.txt"),
            "opensuse" => include_str!("../../ascii/big/opensuse.txt"),
            "alpine" => include_str!("../../ascii/big/alpine.txt"),
            "gentoo" => include_str!("../../ascii/big/gentoo.txt"),
            "endeavouros" => include_str!("../../ascii/big/endeavouros.txt"),
            "popos" => include_str!("../../ascii/big/popos.txt"),
            "cachyos" => include_str!("../../ascii/big/cachyos.txt"),
            "garuda" => include_str!("../../ascii/big/garuda.txt"),
            "linuxmint" => include_str!("../../ascii/big/linuxmint.txt"),
            "kali" => include_str!("../../ascii/big/kali.txt"),
            "macos" => include_str!("../../ascii/big/macos.txt"),
            "zorin" => include_str!("../../ascii/big/zorin.txt"),
            "elementary" => include_str!("../../ascii/big/elementary.txt"),
            "void" => include_str!("../../ascii/big/void.txt"),
            "lubuntu" => include_str!("../../ascii/big/lubuntu.txt"),
            "kubuntu" => include_str!("../../ascii/big/kubuntu.txt"),
            "truenas-scale" => include_str!("../../ascii/big/truenas-scale.txt"),
            "nixos" => include_str!("../../ascii/big/nixos.txt"),
            _ => {
                tracing::warn!("No logo found for user's distro");
                ""
            },
        };
    }

    logo.lines().map(|l| l.to_string()).collect()
}

#[rustfmt::skip]
/// match a u8 id value to its assigned color
fn color_id_to_color(id: u8) -> Option<Color> {
    match id {
        0 => None, // unstyled
        1 => Some(Color::Blue),
        2 => Some(Color::Yellow),
        3 => Some(Color::Red),
        4 => Some(Color::Green),
        5 => Some(Color::TrueColor {r: (96), g: (96), b: (96)}), // grey
        6 => Some(Color::Cyan),
        7 => Some(Color::TrueColor { r: (148), g: (0), b: (211) }), // purple
        8 => Some(Color::TrueColor {r: (255), g: (156), b: (0)}), // orange
        _ => None,
    }
}

/// Takes a parsed line and returns each segment as a ColoredString,
/// ready to be written to stdout. distro_id is kept for future use,
/// e.g. distro-specific palette overrides.
fn color_logo_line(line: &ParsedLine) -> Vec<ColoredString> {
    line.segments
        .iter()
        .map(|seg| {
            let cs = ColoredString::from(seg.text.as_str());
            match color_id_to_color(seg.style.color_id) {
                Some(color) => cs.color(color),
                None => cs,
            }
        })
        .collect()
}

pub fn print_logo(
    logo_lines: Vec<ParsedLine>,
    info_lines: Vec<String>,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = std::io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    if logo_lines.is_empty() {
        for line in info_lines {
            tracing::warn!("Logo lines are empty, printing the information only");
            writeln!(handle, "{}", line)?;
        }
    } else {
        let max_lines = logo_lines.len().max(info_lines.len());

        // sum each line's segment widths and take the max
        let logo_column_width = logo_lines
            .iter()
            .map(|line| line.segments.iter().map(|seg| seg.visible_width).sum::<usize>())
            .max()
            .unwrap_or(0);

        for i in 0 .. max_lines {
            if i < logo_lines.len() {
                for colored_segment in color_logo_line(&logo_lines[i]) {
                    write!(handle, "{}", colored_segment)?;
                }

                let line_visible_width: usize =
                    logo_lines[i].segments.iter().map(|seg| seg.visible_width).sum();
                let padding =
                    logo_column_width.saturating_sub(line_visible_width) + cli.padding as usize;
                write!(handle, "{:width$}", "", width = padding)?;
            } else {
                let total_width = logo_column_width + cli.padding as usize;
                write!(handle, "{:width$}", "", width = total_width)?;
            }

            if i < info_lines.len() {
                writeln!(handle, "  {}", info_lines[i])?;
            } else {
                writeln!(handle)?;
            }
        }
        tracing::info!("Succesfully printed logo and info");
    }

    handle.flush()?;
    Ok(())
}
