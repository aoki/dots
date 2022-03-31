use anyhow::anyhow;
use colored::Colorize;
use std::{fmt::Display, path::PathBuf};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DotState {
    Linked,
    Unlinked,
    Ignored,
    LinkedToOtherDirctory,
    Error,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dot {
    from: Option<PathBuf>,
    to: Option<PathBuf>,
    file: Option<String>,
    pub state: DotState,
    // e: Option<Error>,
}

impl Dot {
    pub fn new(
        from: Option<PathBuf>,
        to: Option<PathBuf>,
        file: Option<String>,
        state: DotState,
    ) -> Self {
        Dot {
            from,
            to,
            file,
            state,
        }
    }

    pub fn file_name(&self) -> String {
        self.file.clone().unwrap_or("".to_string())
    }
    pub fn from(&self) -> anyhow::Result<PathBuf> {
        let file = match self.file.clone() {
            Some(f) => Ok(f),
            None => Err(anyhow!("file not found")),
        }?;
        match self.from.clone() {
            Some(from) => Ok(from.join(file)),
            None => Err(anyhow!("path not found")),
        }
    }
    pub fn to(&self) -> anyhow::Result<PathBuf> {
        let file = match self.file.clone() {
            Some(f) => Ok(f),
            None => Err(anyhow!("file not found")),
        }?;
        match self.to.clone() {
            Some(to) => Ok(to.join(file)),
            None => Err(anyhow!("path not found")),
        }
    }
}

impl Display for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.file.as_ref().unwrap_or(&"".to_string()).clone();
        let icon = match self.state {
            DotState::Linked => ("✔︎".green().bold(), file.white()),
            DotState::Unlinked => ("✖︎".red().bold(), file.white()),
            DotState::Ignored => ("-".black().bold(), format!("{}: ignored", file).black()),
            DotState::LinkedToOtherDirctory => (
                "-".black().bold(),
                format!("{}: already linked to other file", file).black(),
            ),
            DotState::Error => ("-".black().bold(), file.red()),
        };
        write!(f, "{} {}", icon.0, icon.1)
    }
}
