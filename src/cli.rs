//! This file handles all CLI flags such as -a or --all to display all options.
//! It also manages the --help text with custom comments.

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rustfetch", version)]
pub struct Cli {
    #[arg(short, long, help = "Display all info regardless of config")]
    pub all: bool,

    #[arg(long, help = "Regenerates the .toml config file with standard values")]
    pub reset_config: bool,

    #[arg(
        short,
        long,
        default_value_t = 1,
        help = "Adds padding between the logo and the text"
    )]
    pub padding: u8,

    #[arg(
        short,
        long,
        help = "Uses a different config file. Must provide a valid path"
    )]
    pub config_file: Option<PathBuf>,

    #[arg(long, help = "Forcefully regenerates the cache file")]
    pub clear_cache: bool,
}
