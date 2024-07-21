mod fs;

pub mod error;
pub mod ls;
pub mod mv;
pub mod pwd;
pub mod stat;
pub mod touch;

mod tests;

pub type Return = error::Error;
