use std::ops::{BitAnd, Shr};

use crate::error::Error;

pub enum Permissions {
    Exec = 0x0040,
    Write = 0x0080,
    Read = 0x0100,
}

impl Shr<u32> for Permissions {
    type Output = Permissions;

    fn shr(self, rhs: u32) -> Self::Output {
        match self {
            Permissions::Exec => Permissions::Exec >> rhs,
            Permissions::Write => Permissions::Write >> rhs,
            Permissions::Read => Permissions::Read >> rhs,
        }
    }
}

impl BitAnd<u32> for Permissions {
    type Output = u32;

    fn bitand(self, rhs: u32) -> Self::Output {
        match self {
            Permissions::Exec => Permissions::Exec as u32 & rhs,
            Permissions::Write => Permissions::Write as u32 & rhs,
            Permissions::Read => Permissions::Read as u32 & rhs,
        }
    }
}

pub enum SpecialBit {
    Sticky = 0x0200,
    SetGID = 0x0400,
    SetUID = 0x0800,
}

impl BitAnd<u32> for SpecialBit {
    type Output = u32;

    fn bitand(self, rhs: u32) -> Self::Output {
        match self {
            SpecialBit::Sticky => SpecialBit::Sticky as u32 & rhs,
            SpecialBit::SetGID => SpecialBit::SetGID as u32 & rhs,
            SpecialBit::SetUID => SpecialBit::SetUID as u32 & rhs,
        }
    }
}

pub const FT: u32 = 0xF000;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileType {
    Pipe = 0x1000,
    Char = 0x2000,
    Dir = 0x4000,
    Block = 0x6000,
    File = 0x8000,
    Link = 0xA000,
    Sock = 0xC000,
    Exec = 0x49,
}

impl TryFrom<u32> for FileType {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == FileType::Pipe as u32 => Ok(FileType::Pipe),
            x if x == FileType::Char as u32 => Ok(FileType::Char),
            x if x == FileType::Dir as u32 => Ok(FileType::Dir),
            x if x == FileType::Block as u32 => Ok(FileType::Block),
            x if x == FileType::File as u32 => Ok(FileType::File),
            x if x == FileType::Link as u32 => Ok(FileType::Link),
            x if x == FileType::Sock as u32 => Ok(FileType::Sock),
            _ => Err(Error::String("Invalid value for FileType".to_owned())),
        }
    }
}

impl BitAnd<u32> for FileType {
    type Output = u32;

    fn bitand(self, rhs: u32) -> Self::Output {
        match self {
            FileType::Pipe => FileType::Pipe as u32 & rhs,
            FileType::Char => FileType::Char as u32 & rhs,
            FileType::Dir => FileType::Dir as u32 & rhs,
            FileType::Block => FileType::Block as u32 & rhs,
            FileType::File => FileType::File as u32 & rhs,
            FileType::Link => FileType::Link as u32 & rhs,
            FileType::Sock => FileType::Sock as u32 & rhs,
            FileType::Exec => FileType::Exec as u32 & rhs,
        }
    }
}
