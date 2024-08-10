use std::fmt::Display;

use crate::fs::{
    kind::Kind,
    permissions::{Permission, PermissionMask},
};

type FileTypeMask = u32;

const FILE_TYPE_MASK: FileTypeMask = 0xF000;

const ANY_EXEC_MASK: u32 = 0b001001001;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Mode(pub u32);

impl Mode {
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
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PermissionMask::new(self))
    }
}

impl From<&str> for Mode {
    fn from(mode: &str) -> Self {
        let mode = u32::from_str_radix(mode, 8).unwrap_or(0);
        Self(mode)
    }
}
