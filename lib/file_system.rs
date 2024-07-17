pub const EXEC: u32 = 0x40;
pub const WRITE: u32 = 0x80;
pub const READ: u32 = 0x100;

pub const STKY: u32 = 0x200;
pub const SGID: u32 = 0x400;
pub const SUID: u32 = 0x800;

pub const FIFO: u32 = 0x1000;
pub const CHAR: u32 = 0x2000;
pub const DIR: u32 = 0x4000;
pub const BLK: u32 = 0x6000;
pub const FILE: u32 = 0x8000;
pub const LINK: u32 = 0xA000;
pub const SOCK: u32 = 0xC000;

pub const FT: u32 = 0xF000;
