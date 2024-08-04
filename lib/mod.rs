mod fs;

pub mod error;

pub mod change_mode;
pub mod change_ownership;
pub mod copy;
pub mod create_directory;
pub mod create_file;
pub mod current_directory;
pub mod env;
pub mod read;
pub mod remove_directory;
pub mod remove_file;
pub mod rename_file;
pub mod show_directory;
pub mod show_file;

#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod platform;
#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod platform;
#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;

pub use platform::*;

mod tests;

pub type Return = error::Error;
