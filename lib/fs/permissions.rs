use std::{fmt::Display, ops::BitAnd};

const USER_SHIFT: u32 = 6;
const GROUP_SHIFT: u32 = 3;
const OTHER_SHIFT: u32 = 0;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PermissionMask {
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

impl PermissionMask {
    const READ_CHAR: char = 'r';
    const WRITE_CHAR: char = 'w';
    const EXEC_CHAR: char = 'x';
    const STICKY_CHAR: char = 't';
    const SETGID_CHAR: char = 's';
    const SETUID_CHAR: char = 's';
}

impl Display for PermissionMask {
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

impl BitAnd<u32> for PermissionMask {
    type Output = PermissionMask;

    fn bitand(self, rhs: u32) -> Self::Output {
        let p = self as u32 & rhs;
        match p {
            x if x == PermissionMask::Sticky as u32 => PermissionMask::Sticky,
            x if x == PermissionMask::SetGID as u32 => PermissionMask::SetGID,
            x if x == PermissionMask::SetUID as u32 => PermissionMask::SetUID,
            x if x == PermissionMask::UserRead as u32 => PermissionMask::UserRead,
            x if x == PermissionMask::UserWrite as u32 => PermissionMask::UserWrite,
            x if x == PermissionMask::UserExec as u32 => PermissionMask::UserExec,
            x if x == PermissionMask::GroupRead as u32 => PermissionMask::GroupRead,
            x if x == PermissionMask::GroupWrite as u32 => PermissionMask::GroupWrite,
            x if x == PermissionMask::GroupExec as u32 => PermissionMask::GroupExec,
            x if x == PermissionMask::OtherRead as u32 => PermissionMask::OtherRead,
            x if x == PermissionMask::OtherWrite as u32 => PermissionMask::OtherWrite,
            x if x == PermissionMask::OtherExec as u32 => PermissionMask::OtherExec,
            _ => PermissionMask::Unset,
        }
    }
}
