//! Test the serialization, deserialization and re-extraction of values stored in cache.

use clap::Parser;
use rustfetch::{cache::*, cli::Cli, sysinfo::get_gpu_name};

fn get_incorrect_ids() -> Cache {
    return Cache {
        gpu_name_pretty: String::from(""),
         gpu_vendor_id: String::from("Incorrect ID"),
         gpu_device_id: String::from("Incorrect ID"),
    }
}

#[test]
/// tests that the pretty_name is reextracted if the ids are incorrect
fn test_no_pretty_name() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let cache = get_cache(&cli)?;
    let name_before = cache.gpu_name_pretty;

    let cache_path = get_cache_path();
    let incorrect_ids = get_incorrect_ids();
    let toml_string = toml::to_string(&incorrect_ids)?;
    std::fs::write(cache_path, toml_string)?;

    // using the get_gpu_name function should regenerate the name and ids if they do not match
    get_gpu_name(&cli);

    let cache = get_cache(&cli)?;
    let name_after = cache.gpu_name_pretty;

    assert_eq!(name_before, name_after);

    Ok(())
}