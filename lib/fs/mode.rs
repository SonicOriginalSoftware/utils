use std::fmt::Display;

use crate::fs::{kind::Kind, permissions::Permission};

type FileTypeMask = u32;

const FILE_TYPE_MASK: FileTypeMask = 0xF000;

const ANY_EXEC_MASK: u32 = 0b001001001;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Mode(pub u32);

impl Mode {
    pub fn is_executable(&self) -> bool {
        self.0 & ANY_EXEC_MASK != 0
    }

    pub fn is_setuid(&self) -> bool {
        Permission::SetUID & self.0 != Permission::Unset
    }

    pub fn is_setgid(&self) -> bool {
        Permission::SetGID & self.0 != Permission::Unset
    }

    pub fn is_sticky(&self) -> bool {
        Permission::Sticky & self.0 != Permission::Unset
    }

    pub fn file_type(&self) -> Kind {
        let ft = self.0 & FILE_TYPE_MASK;
        match ft {
            x if x == Kind::Block as u32 => Kind::Block,
            x if x == Kind::Char as u32 => Kind::Char,
            x if x == Kind::Link as u32 => Kind::Link,
            x if x == Kind::Pipe as u32 => Kind::Pipe,
            x if x == Kind::Sock as u32 => Kind::Sock,
            x if x == Kind::Dir as u32 => Kind::Dir,
            x if x == Kind::File as u32 => Kind::File,
            _ => Kind::Unset,
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PERMISSION_MASKS: [Permission; 9] = [
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
        let setuid = self.is_setuid();
        let setgid = self.is_setgid();
        let sticky = self.is_sticky();

        for (i, &each_mask) in PERMISSION_MASKS.iter().enumerate() {
            let set = each_mask & self.0;
            let permission: Permission = match i {
                2 if setuid => {
                    if set != Permission::Unset {
                        Permission::SetUID
                    } else {
                        Permission::UnSetUID
                    }
                }
                5 if setgid => {
                    if set != Permission::Unset {
                        Permission::SetGID
                    } else {
                        Permission::UnSetGID
                    }
                }
                8 if sticky => {
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
