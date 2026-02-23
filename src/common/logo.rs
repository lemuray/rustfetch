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

    Some(lines)
}

/// Gets the lines logos in a vector and returns them
pub fn get_logo_lines(distro_id: &str) -> Vec<String> {
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

        for i in 0..max_lines {
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
    }

    handle.flush()?;
    Ok(())
}
