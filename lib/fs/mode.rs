use std::{fmt::Display, ops::BitAnd};

use crate::fs::{
    kind::Kind,
    permissions::{Permission, PermissionMask, SpecialBits},
};

type FileTypeMask = u32;

const FILE_TYPE_MASK: FileTypeMask = 0xF000;

const ANY_EXEC_MASK: u32 = 0x49;

#[derive(Debug, PartialEq, Clone, Copy)]
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
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let permission_range = Permission::all();
        for _each in permission_range {
            let read_permission: Permission = *self & &_each.0;
            let write_permission: Permission = *self & &_each.1;
            let execute_permission: Permission = *self & &_each.2;
            match write!(f, "{read_permission}{write_permission}{execute_permission}") {
                Ok(p) => p,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl BitAnd<&SpecialBits> for Mode {
    type Output = SpecialBits;
    fn bitand(self, _rhs: &SpecialBits) -> Self::Output {
        // self.0 & rhs as u32
        SpecialBits::Unset
    }
}

impl BitAnd<&Permission> for Mode {
    type Output = Permission;
    fn bitand(self, _rhs: &Permission) -> Self::Output {
        // self.0 & rhs as u32
        Permission(PermissionMask::Unset)
    }
}
