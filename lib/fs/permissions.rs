use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Permission {
    Unset = 0,
    Exec = 0x0040,
    Write = 0x0080,
    Read = 0x0100,
    Sticky = 0x0200,
    SetGID = 0x0400,
    SetUID = 0x0800,
}

impl Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Self::Unset => '-',
            Self::Read => 'r',
            Self::Write => 'w',
            Self::Exec => 'x',
            Self::Sticky => 't',
            Self::SetGID => 's',
            Self::SetUID => 's',
        };
        write!(f, "{}", ch)
    }
}
