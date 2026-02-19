//! Common functions shared across all platforms

use std::{fs, path::Path};

const KIB_IN_MB: f64 = 1024.0;
const KIB_IN_GB: f64 = 1024.0 * 1024.0; // We are declaring it as f64 as we'll use it as a float in this file to minimize casting

/// Gets content from a single line file and trims it
///
/// # Errors
/// Returns an error if the file cannot be read or if the trimmed content is empty
pub fn get_trimmed(path: &Path) -> Result<String, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let trimmed = content.trim().to_string();

    if trimmed.is_empty() {
        Err("File content is empty".to_string())
    } else {
        Ok(trimmed)
    }
}

/// Converts KiB figures into GB, MB or unchanged based on its size.
/// Returns a formatted String based on the conversion that has happened
///
/// For example: 15_369.0 KiB will return "15 MB"
pub fn convert_to_bytes(memory_kib: f64) -> Result<String, String> {
    if memory_kib < 0.0 {
        return Err("Memory value cannot be negative".to_string());
    }

    if memory_kib >= KIB_IN_GB {
        // If memory is more than 1GB, transform it to GB
        // 1 GB = (1024 * 1024) KiB
        let memory_gb = round_to_two_decimal(memory_kib / (KIB_IN_GB));
        Ok(format!("{} GB", memory_gb))
    } else if memory_kib >= KIB_IN_MB {
        // Same as before but with MB
        // 1 MB = 1024 KiB
        let memory_mb = round_to_two_decimal(memory_kib / KIB_IN_MB);
        Ok(format!("{} MB", memory_mb))
    } else {
        // Else, just return it in KiB
        Ok(format!("{} KiB", memory_kib))
    }
}

/// Gets the percentage as a rounded value considering a part and a total
pub fn get_percentage_from_part(part: f64, total: f64) -> Result<u64, String> {
    if part < 0.0 || total < 0.0 {
        return Err("Part or total cannot be negative".to_string());
    }
    if total == 0.0 {
        return Err("Division by zero error avoided: total cannot be 0".to_string());
    }
    Ok((part / total * 100.0).floor() as u64)
}

/// Rounds an f64 variable to two decimal points, note that the conversion is not accurate.
/// For example: 9.989 will return 9.98
pub fn round_to_two_decimal(input: f64) -> f64 {
    // We truncate the number since floor rounds to -infinity and it behaves differently
    // between positive and negative numbers
    (input * 100.0).trunc() / 100.0
}

/// Extracts the first whitespace-separated numeric value from a string.
///
/// For example, "1234567 kB" returns 1234567.0
pub fn extract_numeric_value(input: &str) -> Result<f64, String> {
    input
        .split_whitespace()
        .next()
        .ok_or_else(|| "No numeric value found in the input".to_string())
        .and_then(|num_str| num_str.parse::<f64>().map_err(|e| e.to_string()))
}

/// normalizes hex codes to remove the x, for example "0xf89" becomes "0f89"
pub fn format_hex(id: &str) -> String {
    id.trim_start_matches("0x").trim_start_matches("0X").to_ascii_lowercase()
}

/// Some AMD and Intel GPUs will return their name as "GPU_NAME (Some unneded stuff)".
/// For example "AMD Ryzen RX 580 Series (RADV POLARIS10)".
/// Here we're just truncating the string to get those parentheses out
pub fn strip_gpu_name(gpu_name: &str) -> String {
    let mut end_pos = gpu_name.len();
    if let Some(parentheses_pos) = gpu_name.find("(") {
        end_pos = end_pos.min(parentheses_pos);
    }

    (gpu_name[..end_pos]).to_string()
}

/// cpu.brand() from sysinfo returns the full name of the CPU, for example:
/// "Ryzen 5 5600X 6-Core Processor".
/// We just find the first occurrence of any known suffix and get anything before it
pub fn strip_cpu_name(full_name: &str) -> String {
    let mut end_pos = full_name.len();
    if let Some(pos) = full_name.find(" with ") {
        end_pos = end_pos.min(pos);
    }
    if let Some(pos) = full_name.find(" @ ") {
        end_pos = end_pos.min(pos);
    }

    // looks for "-Core" pattern like "6-Core Processor"
    if let Some(pos) = full_name.find("-Core")
        && let Some(space_pos) = full_name[..pos].rfind(' ')
    {
        end_pos = end_pos.min(space_pos);
    }

    full_name[..end_pos].trim().to_string()
}
