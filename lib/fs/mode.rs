use std::fmt::Display;

use crate::fs::{kind::Kind, permissions::PermissionMask};

type FileTypeMask = u32;

const FILE_TYPE_MASK: FileTypeMask = 0xF000;

const ANY_EXEC_MASK: u32 = 0b001001001;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Mode(pub u32);

impl Mode {
    pub fn is_executable(&self) -> bool {
        self.0 & ANY_EXEC_MASK != 0
    }

    pub fn file_type(&self) -> Kind {
        match self.0 & FILE_TYPE_MASK {
            x if x == Kind::Block as u32 => Kind::Block,
            x if x == Kind::Char as u32 => Kind::Char,
            x if x == Kind::Dir as u32 => Kind::Dir,
            x if x == Kind::File as u32 => Kind::File,
            x if x == Kind::Link as u32 => Kind::Link,
            x if x == Kind::Pipe as u32 => Kind::Pipe,
            x if x == Kind::Sock as u32 => Kind::Sock,
            _ => Kind::Unset,
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PERMISSION_MASKS: [PermissionMask; 9] = [
            PermissionMask::UserRead,
            PermissionMask::UserWrite,
            PermissionMask::UserExec,
            PermissionMask::GroupRead,
            PermissionMask::GroupWrite,
            PermissionMask::GroupExec,
            PermissionMask::OtherRead,
            PermissionMask::OtherWrite,
            PermissionMask::OtherExec,
        ];
        let setuid = PermissionMask::SetUID & self.0 != PermissionMask::Unset;
        let setgid = PermissionMask::SetGID & self.0 != PermissionMask::Unset;
        let sticky = PermissionMask::Sticky & self.0 != PermissionMask::Unset;

        for (i, &each_mask) in PERMISSION_MASKS.iter().enumerate() {
            let set = each_mask & self.0;
            let permission: PermissionMask = match i {
                2 if setuid => {
                    if set != PermissionMask::Unset {
                        PermissionMask::SetUID
                    } else {
                        PermissionMask::UnSetUID
                    }
                }
                5 if setgid => {
                    if set != PermissionMask::Unset {
                        PermissionMask::SetGID
                    } else {
                        PermissionMask::UnSetGID
                    }
                }
                8 if sticky => {
                    if set != PermissionMask::Unset {
                        PermissionMask::Sticky
                    } else {
                        PermissionMask::UnSticky
                    }
                }
                _ => set,
            };
            write!(f, "{}", permission)?;
        }
        Ok(())
    }
}
