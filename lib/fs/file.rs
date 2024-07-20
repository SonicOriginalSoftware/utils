use std::{
    cmp::Ordering,
    fmt::Display,
    fs::{read_link, DirEntry},
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
    pub name: String,
    pub path: PathBuf,
    pub mode: Mode,
    pub size: Size,
    pub uid: u32,
    pub gid: u32,
}

impl File {
    pub fn new(entry: DirEntry) -> Result<Self, Error<'static>> {
        let path = entry.path();
        let name = match path.file_name() {
            Some(p) => p.to_string_lossy().to_string(),
            None => return Err(Error::Str("Could not parse file name")),
        };
        let metadata = entry.metadata()?;

        let mode = Mode(metadata.mode());
        let size = Size(metadata.size());

        Ok(Self {
            name,
            size,
            uid: metadata.uid(),
            gid: metadata.gid(),
            path,
            mode,
        })
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let user = match get_user_by_uid(self.uid) {
            Some(p) => p.name().to_owned(),
            None => return Err(std::fmt::Error),
        };
        let group = match get_group_by_gid(self.gid) {
            Some(p) => p.name().to_owned(),
            None => return Err(std::fmt::Error),
        };

        let is_terminal = stdout().is_terminal();
        let kind = self.mode.file_type();
        let name = if kind == Kind::Link {
            let resolved_path = match read_link(&self.path) {
                Ok(p) => p.display().to_string(),
                Err(_e) => return Err(std::fmt::Error),
            };
            if is_terminal {
                &format!("\x1b[36m{:<20}\x1b[0m -> {}", self.name, resolved_path)
            } else {
                &format!("{:<20} -> {}", self.name, resolved_path)
            }
        } else if (kind == Kind::Dir) && is_terminal {
            &format!("\x1b[34m{}\x1b[0m", self.name)
        } else if self.mode.is_executable() && is_terminal {
            &format!("\x1b[35m{}\x1b[0m", self.name)
        } else {
            &self.name
        };

        write!(
            f,
            "{kind}{mode} {size} {user:^8} {group:^8} {name}",
            size = self.size,
            mode = self.mode,
            user = user.to_string_lossy(),
            group = group.to_string_lossy(),
        )
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode
            && self.size == other.size
            && self.uid == other.uid
            && self.gid == other.gid
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
