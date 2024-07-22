mod fs;

pub mod error;

pub mod cm;
pub mod co;
pub mod cp;
pub mod ls;
pub mod mkdir;
pub mod mv;
pub mod pwd;
pub mod rm;
pub mod rmdir;
pub mod stat;
pub mod touch;

mod tests;

pub type Return = error::Error;
