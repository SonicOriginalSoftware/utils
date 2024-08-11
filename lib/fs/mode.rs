use std::convert::TryFrom;
use std::fmt::Display;

use crate::{
    error::Error,
    fs::{
        kind::Kind,
        permissions::{Permission, PermissionMask},
    },
};

type FileTypeMask = u32;

const FILE_TYPE_MASK: FileTypeMask = 0xF000;

const ANY_EXEC_MASK: u32 = 0b001001001;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Mode(u32);

impl Mode {
    pub fn new(mode: u32) -> Self {
        Self(mode)
    }

    pub fn is_executable(&self) -> bool {
        self.0 & ANY_EXEC_MASK != 0
    }

    pub fn is_setuid(&self) -> bool {
        Permission::SetUID & self.0 != Permission::Unset
    }

    pub fn is_setgid(&self) -> bool {
        Permission::SetGID & self.0 != Permission::Unset
    }

    pub fn is_sticky(&self) -> bool {
        Permission::Sticky & self.0 != Permission::Unset
    }

    pub fn file_type(&self) -> Kind {
        let ft = self.0 & FILE_TYPE_MASK;
        match ft {
            x if x == Kind::Block as u32 => Kind::Block,
            x if x == Kind::Char as u32 => Kind::Char,
            x if x == Kind::Link as u32 => Kind::Link,
            x if x == Kind::Pipe as u32 => Kind::Pipe,
            x if x == Kind::Sock as u32 => Kind::Sock,
            x if x == Kind::Dir as u32 => Kind::Dir,
            x if x == Kind::File as u32 => Kind::File,
            _ => Kind::Unset,
        }
    }

    fn parse_symbolic_permissions(p: &str) -> Result<u32, Error> {
        let mut mode = 0;

        for c in p.chars() {
            match c {
                'r' => mode |= Permission::UserRead as u32,
                'w' => mode |= Permission::UserWrite as u32,
                'x' => mode |= Permission::UserExec as u32,
                'R' => mode |= Permission::GroupRead as u32,
                'W' => mode |= Permission::GroupWrite as u32,
                'X' => mode |= Permission::GroupExec as u32,
                'o' => mode |= Permission::OtherRead as u32,
                'O' => mode |= Permission::OtherWrite as u32,
                'E' => mode |= Permission::OtherExec as u32,
                _ => return Err(Error::Str("Invalid symbolic permission")),
            }
        }

        Ok(mode)
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PermissionMask::new(self))
    }
}

impl From<&Mode> for u32 {
    fn from(mode: &Mode) -> Self {
        mode.0
    }
}

impl From<Mode> for u32 {
    fn from(mode: Mode) -> Self {
        u32::from(&mode)
    }
}

impl TryFrom<&String> for Mode {
    type Error = Error;

    fn try_from(mode: &String) -> Result<Self, Self::Error> {
        if mode.chars().all(|c| c.is_digit(8)) {
            match u32::from_str_radix(mode, 8) {
                Ok(p) => Ok(Self(p)),
                Err(_) => Err(Error::Str("Invalid mode")),
            }
        } else {
            match Mode::parse_symbolic_permissions(mode) {
                Ok(p) => Ok(Self(p)),
                Err(err) => Err(err),
            }
        }
    }
}
