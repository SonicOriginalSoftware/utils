use std::fmt::Display;

use crate::{error::Error, fs::mode::Mode};

pub struct FileTypeMask(pub u32);

const FILE_TYPE_MASK: FileTypeMask = FileTypeMask(0xF000);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Kind {
    Unset = 0,
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
    type Error = Error<'static>;
    fn try_from(mode: Mode) -> Result<Self, Self::Error> {
        Ok(mode & FILE_TYPE_MASK)
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
