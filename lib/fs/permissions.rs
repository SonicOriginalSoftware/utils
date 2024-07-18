use std::{
    fmt::Display,
    ops::{BitAnd, Shr},
};

pub enum Permissions {
    Exec = 0x0040,
    Write = 0x0080,
    Read = 0x0100,
}

impl Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Permissions::Exec => 'x',
            Permissions::Write => 'w',
            Permissions::Read => 'r',
        };
        write!(f, "{}", ch)
    }
}

impl Shr<u32> for Permissions {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        match self {
            Self::Exec => Self::Exec >> rhs,
            Self::Write => Self::Write >> rhs,
            Self::Read => Self::Read >> rhs,
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
