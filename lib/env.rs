use std::fmt::Display;

pub struct Vars(pub Vec<(String, String)>);

impl Display for Vars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.0 {
            writeln!(f, "{:<34} = {}", key, value)?;
        }
        Ok(())
    }
}
