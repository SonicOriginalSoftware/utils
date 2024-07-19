use std::{fmt::Display, ops::BitAnd};

use crate::fs::permissions::Permissions;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mode(pub u32);

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let permission_bits = [
        //     (self.mode & Permissions::Read != 0, 'r'),
        //     (self.mode & Permissions::Write != 0, 'w'),
        //     (self.mode & Permissions::Exec != 0, 'x'),
        //     (self.mode & (Permissions::Read >> 3) != 0, 'r'),
        //     (self.mode & (Permissions::Write >> 3) != 0, 'w'),
        //     (self.mode & (Permissions::Exec >> 3) != 0, 'x'),
        //     (self.mode & (Permissions::Read >> 6) != 0, 'r'),
        //     (self.mode & (Permissions::Write >> 6) != 0, 'w'),
        //     (self.mode & (Permissions::Exec >> 6) != 0, 'x'),
        // ];

        // for (i, &(is_set, on_char)) in permission_bits.iter().enumerate() {
        //     let special_char = match i {
        //         #[rustfmt::skip]
        //     2 if self.is_suid => { if is_set { 's' } else { 'S' } },
        //         #[rustfmt::skip]
        //     5 if self.is_guid => { if is_set { 's' } else { 'S' } },
        //         #[rustfmt::skip]
        //     8 if self.is_sticky => { if is_set { 't' } else { 'T' } },
        //         #[rustfmt::skip]
        //     _ => { if is_set { on_char } else { '-' } },
        //     };
        //     perms.push(special_char);
        // }
        write!(f, "")
    }
}

impl BitAnd<u32> for Mode {
    type Output = u32;
    fn bitand(self, rhs: u32) -> Self::Output {
        self.0 & rhs
    }
}

impl BitAnd<Permissions> for Mode {
    type Output = u32;
    fn bitand(self, rhs: Permissions) -> Self::Output {
        self.0 & rhs as u32
    }
}

pub enum _SpecialBit {
    Sticky = 0x0200,
    SetGID = 0x0400,
    SetUID = 0x0800,
}
