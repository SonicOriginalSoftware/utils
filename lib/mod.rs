mod fs;

pub mod error;

pub mod change_mode;
pub mod change_ownership;
pub mod copy;
pub mod create_directory;
pub mod create_file;
pub mod current_directory;
pub mod remove_directory;
pub mod remove_file;
pub mod rename_file;
pub mod show;
pub mod show_file;

mod tests;

pub type Return = error::Error;
