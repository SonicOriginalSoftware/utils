use std::{fmt::Display, ops::BitAnd};

use crate::fs::mode::Mode;

const USER_SHIFT: u32 = 6;
const GROUP_SHIFT: u32 = 3;
const OTHER_SHIFT: u32 = 0;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Permission {
    Unset = 0,
    UserRead = 0b100 << USER_SHIFT,
    UserWrite = 0b010 << USER_SHIFT,
    UserExec = 0b001 << USER_SHIFT,
    GroupRead = 0b100 << GROUP_SHIFT,
    GroupWrite = 0b010 << GROUP_SHIFT,
    GroupExec = 0b001 << GROUP_SHIFT,
    OtherRead = 0b100 << OTHER_SHIFT,
    OtherWrite = 0b010 << OTHER_SHIFT,
    OtherExec = 0b001 << OTHER_SHIFT,
    Sticky = 0x200,
    UnSticky = 0x1000,
    SetGID = 0x400,
    UnSetGID = 0x2000,
    SetUID = 0x0800,
    UnSetUID = 0x4000,
}

impl Permission {
    const READ_CHAR: char = 'r';
    const WRITE_CHAR: char = 'w';
    const EXEC_CHAR: char = 'x';
    const STICKY_CHAR: char = 't';
    const SETGID_CHAR: char = 's';
    const SETUID_CHAR: char = 's';
}

impl Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Self::Sticky => Self::STICKY_CHAR,
            Self::UnSticky => Self::STICKY_CHAR.to_ascii_uppercase(),
            Self::SetGID => Self::SETGID_CHAR,
            Self::UnSetGID => Self::SETGID_CHAR.to_ascii_uppercase(),
            Self::SetUID => Self::SETUID_CHAR,
            Self::UnSetUID => Self::SETUID_CHAR.to_ascii_uppercase(),
            Self::UserRead => Self::READ_CHAR,
            Self::UserWrite => Self::WRITE_CHAR,
            Self::UserExec => Self::EXEC_CHAR,
            Self::GroupRead => Self::READ_CHAR,
            Self::GroupWrite => Self::WRITE_CHAR,
            Self::GroupExec => Self::EXEC_CHAR,
            Self::OtherRead => Self::READ_CHAR,
            Self::OtherWrite => Self::WRITE_CHAR,
            Self::OtherExec => Self::EXEC_CHAR,
            _ => '-',
        };
        write!(f, "{ch}")
    }
}

impl BitAnd<u32> for Permission {
    type Output = Permission;

    fn bitand(self, rhs: u32) -> Self::Output {
        match self as u32 & rhs {
            x if x == Permission::Sticky as u32 => Permission::Sticky,
            x if x == Permission::SetGID as u32 => Permission::SetGID,
            x if x == Permission::SetUID as u32 => Permission::SetUID,
            x if x == Permission::UserRead as u32 => Permission::UserRead,
            x if x == Permission::UserWrite as u32 => Permission::UserWrite,
            x if x == Permission::UserExec as u32 => Permission::UserExec,
            x if x == Permission::GroupRead as u32 => Permission::GroupRead,
            x if x == Permission::GroupWrite as u32 => Permission::GroupWrite,
            x if x == Permission::GroupExec as u32 => Permission::GroupExec,
            x if x == Permission::OtherRead as u32 => Permission::OtherRead,
            x if x == Permission::OtherWrite as u32 => Permission::OtherWrite,
            x if x == Permission::OtherExec as u32 => Permission::OtherExec,
            _ => Permission::Unset,
        }
    }
}

type Mask = [Permission; 9];
const MASK: Mask = [
    Permission::UserRead,
    Permission::UserWrite,
    Permission::UserExec,
    Permission::GroupRead,
    Permission::GroupWrite,
    Permission::GroupExec,
    Permission::OtherRead,
    Permission::OtherWrite,
    Permission::OtherExec,
];

pub struct PermissionMask<'a> {
    mask: Mask,
    mode: &'a Mode,
}

impl<'a> PermissionMask<'a> {
    pub const fn new(mode: &'a Mode) -> Self {
        Self { mask: MASK, mode }
    }
}

impl<'a> Display for PermissionMask<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let setuid = self.mode.is_setuid();
        let setgid = self.mode.is_setgid();
        let sticky = self.mode.is_sticky();

        for (i, &each_mask) in self.mask.iter().enumerate() {
            let set = each_mask & (u32::from(self.mode));
            let permission: Permission = match (i, setuid, setgid, sticky) {
                (2, true, _, _) => {
                    if set != Permission::Unset {
                        Permission::SetUID
                    } else {
                        Permission::UnSetUID
                    }
                }
                (5, _, true, _) => {
                    if set != Permission::Unset {
                        Permission::SetGID
                    } else {
                        Permission::UnSetGID
                    }
                }
                (8, _, _, true) => {
                    if set != Permission::Unset {
                        Permission::Sticky
                    } else {
                        Permission::UnSticky
                    }
                }
                _ => set,
            };
            write!(f, "{}", permission)?;
        }
        Ok(())
    }
}
