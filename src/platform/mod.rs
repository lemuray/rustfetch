//! Platform specific information functions
//! This module compiles platform-specific implementation at compile time based on the target OS 

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

///////////////////////////
// Yet to be implemented //
///////////////////////////

// #[cfg(target_os = "macos")]
// mod macos;
// #[cfg(target_os = "macos")]
// pub use macos::*;