use std::{
    cmp::Ordering,
    fmt::Display,
    fs::{read_link, symlink_metadata},
    io::{stdout, IsTerminal},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

use uzers::{get_group_by_gid, get_user_by_uid};

use crate::{
    error::Error,
    fs::{kind::Kind, mode::Mode, size::Size},
};

#[derive(Debug)]
pub struct File {
    name: String,
    path: PathBuf,
    mode: Mode,
    size: Size,
    kind: Kind,
    // uid: u32,
    // gid: u32,
    user: String,
    group: String,
}

impl File {}

impl TryFrom<PathBuf> for File {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let md = match symlink_metadata(path.clone()) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{p} - {e}", p = path.display());
                return Err(Error::IO(e));
            }
        };
        let uid = md.uid();
        let gid = md.gid();
        let size = Size(md.size());
        let mode = Mode(md.mode());
        let kind = mode.file_type();

        let name = match path.file_name() {
            Some(p) => p.to_string_lossy().to_string(),
            None => path.display().to_string(),
        };
        let user = match get_user_by_uid(uid) {
            Some(p) => p.name().to_string_lossy().to_string(),
            None => format!("{}", uid),
        };
        let group = match get_group_by_gid(gid) {
            Some(p) => p.name().to_string_lossy().to_string(),
            None => format!("{}", gid),
        };

        Ok(Self {
            name,
            size,
            path,
            mode,
            kind,
            user,
            group,
        })
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_terminal = stdout().is_terminal();
        let name = if self.kind == Kind::Link {
            let resolved_path = match read_link(&self.path) {
                Ok(p) => p.display().to_string(),
                Err(_e) => return Err(std::fmt::Error),
            };
            if is_terminal {
                &format!("\x1b[36m{:<20}\x1b[0m -> {}", self.name, resolved_path)
            } else {
                &format!("{:<20} -> {}", self.name, resolved_path)
            }
        } else if self.mode.is_sticky() && is_terminal {
            &format!("\x1b[36;44m{}\x1b[0m", self.name)
        } else if self.mode.is_setgid() && is_terminal {
            &format!("\x1b[1;37;44m{}\x1b[0m", self.name)
        } else if self.mode.is_setuid() && is_terminal {
            &format!("\x1b[1;37;45m{}\x1b[0m", self.name)
        } else if self.mode.is_executable() && is_terminal {
            &format!("\x1b[35m{}\x1b[0m", self.name)
        } else if (self.kind == Kind::Dir) && is_terminal {
            &format!("\x1b[34m{}\x1b[0m", self.name)
        } else if (self.kind == Kind::Sock) && is_terminal {
            &format!("\x1b[32m{}\x1b[0m", self.name)
        } else {
            &self.name
        };

        write!(
            f,
            "{kind}{mode} {size} {user:^14} {group:^14} {name}",
            kind = self.kind,
            size = self.size,
            mode = self.mode,
            user = self.user,
            group = self.group,
        )
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode
            && self.size == other.size
            && self.user == other.user
            && self.group == other.group
            && self.path == other.path
    }
}

impl Eq for File {}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for File {
    fn cmp<'a>(&'a self, other: &'a Self) -> std::cmp::Ordering {
        let file_type_ordering = self.mode.file_type().cmp(&other.mode.file_type());

        if file_type_ordering == Ordering::Equal {
            self.path
                .display()
                .to_string()
                .to_ascii_lowercase()
                .cmp(&other.path.display().to_string().to_ascii_lowercase())
        } else {
            file_type_ordering
        }
    }
}
