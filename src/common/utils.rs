//! Common functions shared across all platforms

use std::fs;
use std::path::Path;

/// Gets content from a single line file and trims it
pub fn get_trimmed(path: &Path) -> String {
    let content = fs::read_to_string(path).unwrap_or(String::from("Null"));
    content.trim().to_string() // Remove any whitespace or \n using .trim()
}

/// Converts KiB figures into GB, MB or unchanged based on its size.
/// Returns a formatted String based on the conversion that has happened
///
/// For example: 14_648.4 KiB will return "15 MB"
pub fn convert_to_bytes(memory_kib: f64) -> String {
    if memory_kib >= (1024.0 * 1024.0) {
        // If memory is more than 1GB, transform it to GB
        // 1 GB = (1024 * 1024) KiB
        let memory_gb = round_to_two_decimal(memory_kib / (1024.0 * 1024.0));
        format!("{} GB", memory_gb)
    } else if memory_kib >= 1024.0 {
        // Same as before but with MB
        // 1 MB = 1024 KiB
        let memory_mb = round_to_two_decimal(memory_kib / 1024.0);
        format!("{} MB", memory_mb)
    } else {
        // Else, just return it in KiB
        format!("{} KiB", memory_kib)
    }
}

/// Gets the percentage as a rounded value considering a part and a total
pub fn get_percentage_from_part(part: f64, total: f64) -> u64 {
    (part / total * 100.0).floor() as u64
}

/// Rounds an f64 variable to two decimal points
pub fn round_to_two_decimal(input: f64) -> f64 {
    (input * 100.0).floor() / 100.0
}

/// Extracts the first whitespace-separated numeric value from a string.
///
/// For example, "1234567 kB" returns 1234567.0
pub fn extract_numeric_value(input: &str) -> f64 {
    return (input.split_whitespace().next().unwrap())
        .parse::<f64>()
        .unwrap();
}
