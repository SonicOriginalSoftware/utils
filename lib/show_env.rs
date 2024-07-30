use std::fmt::Display;

pub struct EnvVars(Vec<(String, String)>);

impl Display for EnvVars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.0 {
            writeln!(f, "{:<34} = {}", key, value)?;
        }
        Ok(())
    }
}

pub fn run() -> Result<EnvVars, crate::error::Error> {
    Ok(EnvVars(std::env::vars().collect()))
}
