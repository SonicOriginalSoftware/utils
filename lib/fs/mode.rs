use std::{fmt::Display, ops::BitAnd};

use crate::fs::{
    kind::{FileTypeMask, Kind},
    permissions::Permission,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mode(pub u32);

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let permission_range: [(Permission, Permission, Permission); 3] = [
            (Permission::Read, Permission::Write, Permission::Exec),
            (Permission::Read, Permission::Write, Permission::Exec),
            (Permission::Read, Permission::Write, Permission::Exec),
        ];

        for _each in permission_range {
            let user_permission: Permission = *self & _each.0;
            let group_permission: Permission = *self & _each.1;
            let other_permission: Permission = *self & _each.2;
            match write!(f, "{}{}{}", user_permission, group_permission, other_permission) {
                Ok(p) => p,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl BitAnd<Permission> for Mode {
    type Output = Permission;
    fn bitand(self, _rhs: Permission) -> Self::Output {
        // self.0 & rhs as u32
        Permission::Unset
    }
}

impl BitAnd<FileTypeMask> for Mode {
    type Output = Kind;
    fn bitand(self, _rhs: FileTypeMask) -> Self::Output {
        let _ = self.0 & _rhs.0;
        Kind::Unset
    }
}
