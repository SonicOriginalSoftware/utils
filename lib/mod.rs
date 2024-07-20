mod fs;

pub mod error;
pub mod ls;
pub mod mv;
pub mod pwd;
pub mod stat;

mod tests;

pub type Return = Result<(), error::Error>;
