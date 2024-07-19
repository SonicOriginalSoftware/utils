use std::fmt::Display;

pub type PermissionMask = u32;

pub enum Permissions {
    Unset = 0,
    Read = 0b100,
    Write = 0b010,
    Exec = 0b001,
}

pub struct Permission(u32);

impl Permission {
    const USER_SHIFT: u32 = 6;
    const GROUP_SHIFT: u32 = 3;
    const OTHER_SHIFT: u32 = 0;

    pub fn mask(&self) -> PermissionMask {
        self.0
    }

    pub fn all() -> [(Self, Self, Self); 3] {
        [
            (
                Permission((Permissions::Read as u32) << Self::USER_SHIFT),
                Permission((Permissions::Write as u32) << Self::USER_SHIFT),
                Permission((Permissions::Exec as u32) << Self::USER_SHIFT),
            ),
            (
                Permission((Permissions::Read as u32) << Self::GROUP_SHIFT),
                Permission((Permissions::Write as u32) << Self::GROUP_SHIFT),
                Permission((Permissions::Exec as u32) << Self::GROUP_SHIFT),
            ),
            (
                Permission((Permissions::Read as u32) << Self::OTHER_SHIFT),
                Permission((Permissions::Write as u32) << Self::OTHER_SHIFT),
                Permission((Permissions::Exec as u32) << Self::OTHER_SHIFT),
            ),
        ]
    }
}

impl From<Permissions> for Permission {
    fn from(mask: Permissions) -> Self {
        Permission(mask as u32)
    }
}

impl Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self.0 {
            x if x == Permissions::Read as u32 => 'r',
            x if x == Permissions::Write as u32 => 'w',
            x if x == Permissions::Exec as u32 => 'x',
            _ => '-',
        };
        write!(f, "{ch}")
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecialBits {
    _Unset = 0,
    _Sticky = 0x0200,
    _SetGID = 0x0400,
    _SetUID = 0x0800,
}

impl Display for SpecialBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Self::_Unset => '-',
            Self::_Sticky => 't',
            Self::_SetGID => 's',
            Self::_SetUID => 's',
        };
        write!(f, "{ch}")
    }
}
