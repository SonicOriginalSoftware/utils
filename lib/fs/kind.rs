use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Kind {
    Unset = 0,
    Pipe = 0x1000,
    Char = 0x2000,
    Dir = 0x4000,
    Block = 0x6000,
    File = 0x8000,
    Link = 0xA000,
    Sock = 0xC000,
    Exec = 0x49,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Self::Pipe => 'p',
            Self::Char => 'c',
            Self::Dir => 'd',
            Self::Block => 'b',
            Self::File => '-',
            Self::Link => 'l',
            Self::Sock => 's',
            _ => '?',
        };
        write!(f, "{}", ch)
    }
}
