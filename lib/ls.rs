use uzers::{get_group_by_gid, get_user_by_uid};

use crate::{
    file_system::{BLK, CHAR, DIR, EXEC, FIFO, FILE, FT, LINK, READ, SGID, SOCK, STKY, SUID, WRITE},
    pwd,
};
use std::{
    fs::{read_dir, DirEntry},
    io::{stdout, IsTerminal},
    os::unix::fs::MetadataExt,
};

fn format_entry(entry: DirEntry) -> Option<String> {
    let metadata = match entry.metadata() {
        Ok(p) => p,
        Err(_) => return None,
    };
    let user = get_user_by_uid(metadata.uid())?;
    let user_name = user.name().to_str()?;
    let group = get_group_by_gid(metadata.gid())?;
    let group_name = group.name().to_str()?;

    let mode = metadata.mode();
    let suid = (mode & SUID) != 0;
    let guid = (mode & SGID) != 0;
    let stky = (mode & STKY) != 0;
    let o = stdout();

    let name = match entry.file_name().into_string() {
        Ok(p) if metadata.is_dir() && o.is_terminal() => format!("\x1b[34m{p}\x1b[0m"),
        Ok(p) if mode & 0o111 != 0 && o.is_terminal() => format!("\x1b[35m{p}\x1b[0m"),
        Ok(p) => p,
        Err(_) => return None,
    };

    let mut perms = String::new();
    let file_type_bit = match mode & FT {
        DIR => 'd',
        CHAR => 'c',
        BLK => 'b',
        FILE => '-',
        FIFO => 'p',
        LINK => 'l',
        SOCK => 's',
        _ => '?',
    };
    perms.push(file_type_bit);

    let permission_bits = [
        (READ, 'r'),
        (WRITE, 'w'),
        (EXEC, 'x'),
        (READ >> 3, 'r'),
        (WRITE >> 3, 'w'),
        (EXEC >> 3, 'x'),
        (READ >> 6, 'r'),
        (WRITE >> 6, 'w'),
        (EXEC >> 6, 'x'),
    ];

    for (i, &(bit, on_char)) in permission_bits.iter().enumerate() {
        let is_set = (mode & bit) != 0;
        let special_char = match i {
            #[rustfmt::skip]
            2 if suid => { if is_set { 's' } else { 'S' } }, // setuid
            #[rustfmt::skip]
            5 if guid => { if is_set { 's' } else { 'S' } }, // setgid
            #[rustfmt::skip]
            8 if stky => { if is_set { 't' } else { 'T' } }, // sticky bit
            #[rustfmt::skip]
            _ => { if is_set { on_char } else { '-' } },
        };
        perms.push(special_char);
    }

    let size = match metadata.size() {
        s if s > 1000000000 => format!("{:5.1} G", s as f32 / 100000.0),
        s if s > 1000000 => format!("{:5.1} M", s as f32 / 10000.0),
        s if s > 1000 => format!("{:5.1} K", s as f32 / 1000.0),
        s => format!("{s:5.1} B"),
    };

    Some(format!("{perms} {size} {user_name:^8} {group_name:^8} {name}"))
}

pub fn run(args: &[String]) -> Result<Vec<String>, String> {
    let target = match args.get(1) {
        Some(p) => p,
        None => &match pwd::run() {
            Err(err) => return Err(err.to_string()),
            Ok(p) => p,
        },
    };

    let dirs = match read_dir(target) {
        Ok(p) => p,
        Err(err) => return Err(err.to_string()),
    };

    let mut directories = Vec::<DirEntry>::new();
    let mut files = Vec::<DirEntry>::new();

    for each_entry in dirs {
        match each_entry {
            Err(err) => return Err(err.to_string()),
            Ok(entry) => match entry.metadata() {
                Err(_) => (),
                Ok(p) => {
                    if p.is_dir() {
                        directories.push(entry);
                    } else {
                        files.push(entry);
                    }
                }
            },
        }
    }

    directories.sort_by_key(|each_entry_a| each_entry_a.path());
    files.sort_by_key(|each_entry_a| each_entry_a.path());

    let mut raw_collection = Vec::<DirEntry>::new();
    raw_collection.append(&mut directories);
    raw_collection.append(&mut files);

    let collection = raw_collection.into_iter().flat_map(format_entry);

    Ok(collection.collect::<Vec<String>>())
}
