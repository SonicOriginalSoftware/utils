use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Size(pub u64);

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            s if s > 1000000000000 => write!(f, "{:5.1} T", s as f32 / 1000000000000.0),
            s if s > 1000000000 => write!(f, "{:5.1} G", s as f32 / 1000000000.0),
            s if s > 1000000 => write!(f, "{:5.1} M", s as f32 / 1000000.0),
            s if s > 1000 => write!(f, "{:5.1} K", s as f32 / 1000.0),
            s => write!(f, "{s:5.1} B"),
        }
    }
}
