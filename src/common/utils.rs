//! Common functions shared across all platforms

use std::fs;
use std::path::Path;

/// Gets content from a single line file and trims it
pub fn get_trimmed(path: &Path) -> String {
    let content = fs::read_to_string(path).unwrap_or(String::from("Null"));
    content.trim().to_string() // Remove any whitespace or \n using .trim()
}

/// Gets content from a multiple or single line file and returns the value from the key and separator provided.
///
/// Example:
///``` ignore
/// /* /etc/os-release contains: */ PRETTY_NAME = "Arch Linux"
/// get_value_from_file(Path::new("/etc/os-release"), "PRETTY_NAME", "=");
/// ```
/// returns: Arch Linux
pub fn get_value_from_file(path: &Path, key: &str, separator: &str) -> String {
    let content = fs::read_to_string(path).unwrap_or(String::from("Null"));
    for line in content.lines() {
        if !(line.contains(key)) {
            continue;
        }
        if let Some((_, value)) = line.split_once(separator) {
            let trimmed = value.trim();
            if trimmed.starts_with("\"") && trimmed.ends_with("\"") && trimmed.len() > 1 {
                let mut chars = value.chars(); // Transform the string to chars to enable index-based modifications
                chars.next(); // Removes the first character in the string, in this case the first character will always be quotation marks
                chars.next_back(); // Removes the last character, same as before
                return chars.as_str().to_string();
            } else {
                return trimmed.to_string();
            }
        }
    }
    String::from("Null")
}
