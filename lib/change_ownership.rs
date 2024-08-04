use crate::error::Error;

use uzers::{get_group_by_name, get_user_by_name};

pub fn run(args: Vec<String>) -> Result<(), Error> {
    let target = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No path passed")),
    };

    let ownership = match args.get(2) {
        Some(p) => p,
        None => return Err(Error::Str("No ownership passed")),
    };

    let (user, group) = match ownership.split_once(':') {
        Some((u, g)) => (u, g),
        None => (ownership.as_str(), ""),
    };

    let uid = if user.is_empty() {
        None
    } else {
        get_user_by_name(user).map(|p| p.uid())
    };
    let gid = if group.is_empty() {
        None
    } else {
        get_group_by_name(group).map(|p| p.gid())
    };
    if uid.is_none() && gid.is_none() {
        return Err(Error::String(format!(
            "Unable to parse owner and/or group: '{user}:{group}'"
        )));
    }

    match std::os::unix::fs::chown(target, uid, gid) {
        Ok(_p) => Ok(()),
        Err(e) => Err(Error::IO(e)),
    }
}
