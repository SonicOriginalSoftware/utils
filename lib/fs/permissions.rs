use std::fmt::Display;

pub enum PermissionMask {
    Unset = 0,
    Read = 0x0040,
    Write = 0x0080,
    Exec = 0x0100,
}

const READ_CHAR: char = 'r';
const WRITE_CHAR: char = 'w';
const EXEC_CHAR: char = 'x';

pub struct Permission(pub PermissionMask);

impl Permission {
    pub fn all() -> [(Self, Self, Self); 3] {
        [
            // FIXME Below needs shifted right 6
            (
                Permission(PermissionMask::Read),
                Permission(PermissionMask::Write),
                Permission(PermissionMask::Exec),
            ),
            // FIXME Below needs shifted right 3
            (
                Permission(PermissionMask::Read),
                Permission(PermissionMask::Write),
                Permission(PermissionMask::Exec),
            ),
            (
                Permission(PermissionMask::Read),
                Permission(PermissionMask::Write),
                Permission(PermissionMask::Exec),
            ),
        ]
    }
}

impl Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self.0 {
            PermissionMask::Exec => EXEC_CHAR,
            PermissionMask::Write => WRITE_CHAR,
            PermissionMask::Read => READ_CHAR,
            PermissionMask::Unset => '-',
        };
        write!(f, "{ch}")
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecialBits {
    Unset = 0,
    Sticky = 0x0200,
    SetGID = 0x0400,
    SetUID = 0x0800,
}

impl Display for SpecialBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Self::Unset => '-',
            Self::Sticky => 't',
            Self::SetGID => 's',
            Self::SetUID => 's',
        };
        write!(f, "{ch}")
    }
}
