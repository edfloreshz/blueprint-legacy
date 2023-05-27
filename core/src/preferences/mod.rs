pub mod code_editor;
pub mod language;
pub mod library;
use anyhow::Result;
use derive_setters::*;
use serde::{Deserialize, Serialize};

use self::{code_editor::CodeEditor, language::Language, library::Library, shell::Shell};

pub mod shell;

#[derive(Debug, Default, Setters, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Preferences {
    #[setters(into)]
    author: String,
    #[setters(into)]
    location: String,
    shells: Vec<Shell>,
    languages: Vec<Language>,
    libraries: Vec<Library>,
    code_editors: Vec<CodeEditor>,
}

impl Preferences {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save(&self) -> Result<()> {
        let preferences = ron::to_string(self)?;
        std::fs::write(&self.location, preferences)?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self> {
        let file = std::fs::read_to_string(path)?;
        let preferences = ron::from_str::<Preferences>(&file)?;
        Ok(preferences)
    }
}
