//! Test all functions within src/utils

use core::f64;

use rustfetch::common::*;

// extract_numeric_value tests
#[test]
fn test_no_numeric_value() {
    let result = extract_numeric_value("");
    assert_eq!(result, Err("No numeric value found in the input".to_string()));

    let result = extract_numeric_value("abcd");
    assert!(result.is_err());
}

#[test]
fn test_numeric_value_correct_syntax() {
    let result = extract_numeric_value("512 GB");
    assert_eq!(result, Ok(512.0));

    let result = extract_numeric_value("412 mb");
    assert_eq!(result, Ok(412.0));

    // The function splits whitespace values, there must be a space between the value and the unit
    // for the syntax to be valid. Thus this is incorrect syntax and should return an error.
    let result = extract_numeric_value("999KiB");
    assert!(result.is_err());
}

// round_to_two_decimals tests

#[test]
fn test_round_to_two_decimals_correct_input() {
    let result = round_to_two_decimal(14.209839);
    assert_eq!(result, 14.20);

    // This test is crucial since its validity is in the doc comments for the function in utils.rs
    let result = round_to_two_decimal(9.989);
    assert_eq!(result, 9.98);

    // 0s after the last decimal number do not matter, as it is in standard math:
    // 9.1 is equal to 9.10, so the function should work
    let result = round_to_two_decimal(9.1);
    assert_eq!(result, 9.10)
}

#[test]
fn test_round_to_two_decimals_edge_cases() {
    // With our current implementation of the function there should be no reason for an input to be
    // negative, but it's better to test it sooner rather than it crashing later
    let result = round_to_two_decimal(-25.11238);
    assert_eq!(result, -25.11);

    // The following 2 should pass through
    let result = round_to_two_decimal(f64::INFINITY);
    assert!(result.is_infinite());

    let result = round_to_two_decimal(f64::NAN);
    assert!(result.is_nan());
}

// get_percentage_from_part tests

#[test]
fn test_get_percentage_from_part_correct_input() {
    let result = get_percentage_from_part(10.0, 100.0);
    assert_eq!(result, Ok(10));

    let result = get_percentage_from_part(200.0, 10.0);
    assert_eq!(result, Ok(2000));
}

#[test]
fn test_get_percentage_from_part_invalid_input() {
    let result = get_percentage_from_part(-3.0, 30.0);
    assert_eq!(result, Err("Part or total cannot be negative".to_string()));
}

// convert_to_bytes tests

#[test]
fn convert_to_bytes_correct_input() {
    // This test is crucial since its validity is in the doc comments for the function in utils.rs
    let result = convert_to_bytes(15_369.0);
    assert_eq!(result, Ok(String::from("15 MB")));

    let result = convert_to_bytes(28.0);
    assert_eq!(result, Ok(String::from("28 KiB")));
}

#[test]
fn convert_to_bytes_invalid_input() {
    let result = convert_to_bytes(-15.0);
    assert_eq!(result, Err("Memory value cannot be negative".to_string()));
}

#[test]
fn strip_gpu_name_normal_use() {
    let result = strip_gpu_name("AMD Sapphire RX 590 (RADV POLARIS)");
    assert_eq!(result, String::from("AMD Sapphire RX 590 "));

    let result = strip_gpu_name("Intel Arc Pro B50");
    assert_eq!(result, String::from("Intel Arc Pro B50"));

    let result = strip_gpu_name("Nvidia Geforce GTX ( 1070");
    assert_eq!(result, String::from("Nvidia Geforce GTX "))
}

#[test]
fn strip_gpu_name_empty_string() {
    let result = strip_gpu_name("");
    assert_eq!(result, String::from(""));
}

#[test]
fn strip_cpu_name_normal_use() {
    let result = strip_cpu_name("Ryzen 7 7800x with 3D V-Cache");
    assert_eq!(result, String::from("Ryzen 7 7800x"));

    let result = strip_cpu_name("Intel i5 12400KF @ 3.04Ghz");
    assert_eq!(result, String::from("Intel i5 12400KF"));

    let result = strip_cpu_name("Ryzen 5 5600x 6-Core Processor");
    assert_eq!(result, String::from("Ryzen 5 5600x"))
}

#[test]
fn strip_cpu_name_empty_string() {
    let result = strip_cpu_name("");
    assert_eq!(result, String::from(""));
}
