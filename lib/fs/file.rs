use std::{
    borrow::Cow,
    cmp::Ordering,
    ffi::OsString,
    fmt::Display,
    fs::DirEntry,
    io::{stdout, IsTerminal},
    os::unix::fs::MetadataExt,
    path::{absolute, PathBuf},
};

use uzers::{get_group_by_gid, get_user_by_uid};

use crate::{
    error::Error,
    fs::{kind::Kind, mode::Mode, permissions::Permissions, size::Size},
};

#[derive(Debug)]
pub struct File {
    pub name: OsString,
    pub mode: Mode,
    pub size: Size,
    pub kind: Kind,
    pub uid: u32,
    pub gid: u32,
    pub path: PathBuf,
}

impl File {
    pub fn new(entry: &DirEntry) -> Result<Self, Error> {
        let name = entry.file_name();
        let metadata = entry.metadata()?;

        let mode = Mode(metadata.mode());
        let size = Size(metadata.size());
        let kind = Kind::try_from(mode)?;

        let path = match absolute(entry.path()) {
            Ok(p) => p,
            Err(e) => return Err(Error::IO(e)),
        };

        Ok(Self {
            name,
            kind,
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
        let lossy_name: Cow<str> = self.name.to_string_lossy();
        let lossy_name = if self.kind == Kind::Link {
            format!("{:<20} -> {}", lossy_name, self.path.to_string_lossy()).into()
        } else if (self.kind == Kind::Dir) && is_terminal {
            format!("\x1b[34m{}\x1b[0m", lossy_name).into()
        } else if self.mode & Permissions::Exec != 0 && is_terminal {
            format!("\x1b[35m{}\x1b[0m", lossy_name).into()
        } else {
            lossy_name
        };

        write!(
            f,
            "{kind}{mode} {size} {user:^8} {group:^8} {lossy_name}",
            size = self.size,
            kind = self.kind,
            mode = self.mode,
            user = user.to_string_lossy(),
            group = group.to_string_lossy(),
        )
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode
            && self.kind == other.kind
            && self.size == other.size
            && self.uid == other.uid
            && self.gid == other.gid
            && self.name == other.name
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
        let file_type_ordering = self.kind.cmp(&other.kind);

        if file_type_ordering == Ordering::Equal {
            self.name.to_ascii_lowercase().cmp(&other.name.to_ascii_lowercase())
            // self.name.cmp(&other.name)
        } else {
            file_type_ordering
        }
    }
}
