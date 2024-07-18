use std::fmt::Display;

use crate::{error::Error, fs::mode::Mode};

pub const FILE_TYPE_MASK: u32 = 0xF000;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Pipe = 0x1000,
    Char = 0x2000,
    Dir = 0x4000,
    Block = 0x6000,
    File = 0x8000,
    Link = 0xA000,
    Sock = 0xC000,
    Exec = 0x49,
}

impl TryFrom<Mode> for Kind {
    type Error = Error;

    fn try_from(value: Mode) -> Result<Self, Self::Error> {
        match value & FILE_TYPE_MASK {
            x if x == Self::Pipe as u32 => Ok(Self::Pipe),
            x if x == Self::Char as u32 => Ok(Self::Char),
            x if x == Self::Dir as u32 => Ok(Self::Dir),
            x if x == Self::Block as u32 => Ok(Self::Block),
            x if x == Self::File as u32 => Ok(Self::File),
            x if x == Self::Link as u32 => Ok(Self::Link),
            x if x == Self::Sock as u32 => Ok(Self::Sock),
            _ => Err(Self::Error::String("Invalid value for FileType".to_owned())),
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Self::Pipe => 'p',
            Self::Char => 'c',
            Self::Dir => 'd',
            Self::Block => 'b',
            Self::File => '-',
            Self::Link => 'l',
            Self::Sock => 's',
            _ => '?',
        };
        write!(f, "{}", ch)
    }
}
