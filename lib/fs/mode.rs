use std::fmt::Display;

use crate::fs::{
    kind::Kind,
    permissions::{Permission, PermissionMask, Permissions},
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

    pub fn file_type(&self) -> Kind {
        match self.0 & FILE_TYPE_MASK {
            x if x == Kind::Block as u32 => Kind::Block,
            x if x == Kind::Char as u32 => Kind::Char,
            x if x == Kind::Dir as u32 => Kind::Dir,
            x if x == Kind::File as u32 => Kind::File,
            x if x == Kind::Link as u32 => Kind::Link,
            x if x == Kind::Pipe as u32 => Kind::Pipe,
            x if x == Kind::Sock as u32 => Kind::Sock,
            _ => Kind::Unset,
        }
    }

    fn permission(&self, mask: PermissionMask) -> Permission {
        let p: Permissions = match self.0 & mask {
            x if x == Permissions::Read as u32 => Permissions::Read,
            x if x == Permissions::Write as u32 => Permissions::Write,
            x if x == Permissions::Exec as u32 => Permissions::Exec,
            _ => Permissions::Unset,
        };
        Permission::from(p)
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let permission_range = Permission::all();
        for each in permission_range {
            let read_permission: Permission = self.permission(each.0.mask());
            let write_permission: Permission = self.permission(each.1.mask());
            let execute_permission: Permission = self.permission(each.2.mask());
            match write!(f, "{read_permission}{write_permission}{execute_permission}") {
                Ok(p) => p,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl PartialOrd for Mode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Mode {
    fn cmp<'a>(&'a self, other: &'a Self) -> std::cmp::Ordering {
        self.file_type().cmp(&other.file_type())
    }
}
