use std::fs;
use std::path::Path;

/*
    TODO:
    Error Detection in case function returns "Null"
    Test with other distros
    Finish translating to rust
*/

fn get_stripped(path: &Path) -> String {
    let content = fs::read_to_string(path).expect("Null");
    content.trim().to_string() // Remove any whitespace or \n using .trim()
}
fn get_value_from_file(path: &Path, value_to_find: &str, separator: &str) -> String {
    let content = fs::read_to_string(path).expect("Null");
    for line in content.lines() {
        if !(line.contains(value_to_find)) {
            continue;
        }
        if let Some((_, value)) = line.split_once(separator) {
            // FIXME Not understood well
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
fn main() {
    if std::env::consts::OS == "linux" {
        println!(
            "OS: {} ({})",
            get_value_from_file(Path::new("/etc/os-release"), "PRETTY_NAME", "="),
            std::env::consts::ARCH // CPU Architecture the program was compiled for
        );
        println!(
            "Kernel: Linux {}",
            get_stripped(Path::new("/proc/sys/kernel/osrelease"))
        );
        println!(
            "CPU: {}",
            get_value_from_file(Path::new("/proc/cpuinfo"), "model name", ":")
        )
    } else {
        println!("Linux is the only supported platform as of now");
    }
}
