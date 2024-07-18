use std::{
    cmp::Ordering,
    fmt::{write, Display},
    fs::{read_dir, DirEntry},
    io::{stdout, IsTerminal},
    os::unix::fs::MetadataExt,
};

use uzers::{get_group_by_gid, get_user_by_uid};

use crate::{
    error::Error,
    file_system::{FileType, Permissions, SpecialBit, FT},
    pwd,
};

#[derive(Debug)]
pub struct Entry {
    mode: u32,
    file_type: FileType,
    is_suid: bool,
    is_guid: bool,
    is_sticky: bool,
    is_executable: bool,
    size: u64,
    user: String,
    group: String,
    name: String,
    path: String,
}

impl Entry {
    fn new(entry: DirEntry) -> Result<Self, Error> {
        let name = match entry.file_name().to_str() {
            Some(p) => p.to_string(),
            None => return Err(Error::String(format!("Could not construct file name for {:?}", entry))),
        };

        let metadata = match entry.metadata() {
            Ok(p) => p,
            Err(_) => return Err(Error::String(format!("Could not construct file metadata for {}", name))),
        };
        let mode = metadata.mode();
        let size = metadata.size();

        let file_type = FileType::try_from(mode & FT)?;
        let is_suid = (SpecialBit::SetUID & mode) != 0;
        let is_guid = (SpecialBit::SetGID & mode) != 0;
        let is_sticky = (SpecialBit::Sticky & mode) != 0;
        let is_executable = (FileType::Exec & mode) != 0;

        let user = match get_user_by_uid(metadata.uid()) {
            Some(p) => match p.name().to_str() {
                Some(l) => l.to_string(),
                None => return Err(Error::String(format!("Could not extract user name for {}", name))),
            },
            None => return Err(Error::String(format!("Could not extract user for {}", name))),
        };
        let group = match get_group_by_gid(metadata.gid()) {
            Some(p) => match p.name().to_str() {
                Some(l) => l.to_string(),
                None => return Err(Error::String(format!("Could not extract user name for {}", name))),
            },
            None => return Err(Error::String(format!("Could not extract user for {}", name))),
        };

        let path = match entry.path().canonicalize()?.to_str() {
            Some(p) => p,
            None => return Err(Error::String(format!("Could not canonicalize path for {}", name))),
        }
        .to_string();

        Ok(Self {
            mode,
            size,
            name,
            user,
            group,
            path,
            file_type,
            is_suid,
            is_guid,
            is_sticky,
            is_executable,
        })
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut perms = String::new();
        perms.push(match self.file_type {
            FileType::Pipe => 'p',
            FileType::Char => 'c',
            FileType::Dir => 'd',
            FileType::Block => 'b',
            FileType::File => '-',
            FileType::Link => 'l',
            FileType::Sock => 's',
            _ => '?',
        });

        // Could do list comprehension
        // do generate list of permission bits
        // using initial
        // (mode & (Exec >> i * 3) != 0, 'x')
        // (mode & (WRITE >> i * 3) != 0, 'w')
        // (mode & (READ >> i * 3) != 0, 'r')
        let permission_bits = [
            (Permissions::Read & self.mode != 0, 'r'),
            (Permissions::Write & self.mode != 0, 'w'),
            (Permissions::Exec & self.mode != 0, 'x'),
            ((Permissions::Read >> 3) & self.mode != 0, 'r'),
            ((Permissions::Write >> 3) & self.mode != 0, 'w'),
            ((Permissions::Exec >> 3) & self.mode != 0, 'x'),
            ((Permissions::Read >> 6) & self.mode != 0, 'r'),
            ((Permissions::Write >> 6) & self.mode != 0, 'w'),
            ((Permissions::Exec >> 6) & self.mode != 0, 'x'),
        ];

        for (i, &(is_set, on_char)) in permission_bits.iter().enumerate() {
            let special_char = match i {
                #[rustfmt::skip]
            2 if self.is_suid => { if is_set { 's' } else { 'S' } },
                #[rustfmt::skip]
            5 if self.is_guid => { if is_set { 's' } else { 'S' } },
                #[rustfmt::skip]
            8 if self.is_sticky => { if is_set { 't' } else { 'T' } },
                #[rustfmt::skip]
            _ => { if is_set { on_char } else { '-' } },
            };
            perms.push(special_char);
        }

        let size = match self.size {
            s if s > 1000000000 => format!("{:5.1} G", s as f32 / 100000.0),
            s if s > 1000000 => format!("{:5.1} M", s as f32 / 10000.0),
            s if s > 1000 => format!("{:5.1} K", s as f32 / 1000.0),
            s => format!("{s:5.1} B"),
        };

        let is_terminal = stdout().is_terminal();

        let file_name: String;
        // if metadata.is_symlink() => {
        if self.file_type == FileType::Link {
            file_name = format!("{:<20} -> {}", self.name, self.path);
        } else if self.file_type == FileType::Dir && is_terminal {
            file_name = format!("\x1b[34m{}\x1b[0m", self.name);
        } else if self.is_executable && is_terminal {
            file_name = format!("\x1b[35m{}\x1b[0m", self.name);
        } else {
            file_name = String::from(&self.name);
        }

        write(
            f,
            format_args!(
                "{perms} {size} {user_name:^8} {group_name:^8} {file_name}",
                user_name = self.user,
                group_name = self.group,
            ),
        )
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode
            && self.file_type == other.file_type
            && self.is_suid == other.is_suid
            && self.is_guid == other.is_guid
            && self.is_sticky == other.is_sticky
            && self.is_executable == other.is_executable
            && self.size == other.size
            && self.user == other.user
            && self.group == other.group
            && self.name == other.name
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp<'a>(&'a self, other: &'a Self) -> std::cmp::Ordering {
        let file_type_ordering = self.file_type.cmp(&other.file_type);

        if file_type_ordering == Ordering::Equal {
            // self.name.to_ascii_lowercase().cmp(&other.name.to_ascii_lowercase())
            self.name.cmp(&other.name)
        } else {
            file_type_ordering
        }
    }
}

pub fn run(args: &[String]) -> Result<Vec<Entry>, Error> {
    let target = match args.get(1) {
        Some(p) => p,
        None => &pwd::run()?,
    };

    let mut entries: Vec<Entry> = read_dir(target)?
        .map(|p| {
            //
            Entry::new(p.map_err(Error::IO)?)
        })
        .collect::<Result<_, _>>()?;

    entries.sort();
    Ok(entries)
}
