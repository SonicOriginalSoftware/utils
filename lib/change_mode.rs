use crate::error::Error;
use crate::fs::mode::Mode;
use std::fs::{metadata, set_permissions};
use std::os::unix::fs::PermissionsExt;

pub fn run(args: Vec<String>) -> Result<(), Error> {
    let target = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No path passed")),
    };

    let mode_arg = match args.get(2) {
        Some(p) => p,
        None => return Err(Error::Str("No mode passed")),
    };

    let mode = match Mode::try_from(mode_arg) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };

    let mut permissions = match metadata(target) {
        Ok(it) => it,
        Err(err) => return Err(Error::IO(err)),
    }
    .permissions();

    permissions.set_mode(u32::from(mode));
    match set_permissions(target, permissions) {
        Ok(it) => Ok(it),
        Err(err) => Err(Error::IO(err)),
    }
}
