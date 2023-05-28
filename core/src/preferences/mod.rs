pub mod code_editor;
pub mod language;
pub mod library;
pub mod source;

use anyhow::Result;
use derive_setters::Setters;
use getset::{CopyGetters, Getters, MutGetters};
use serde::{Deserialize, Serialize};

use self::{code_editor::CodeEditor, language::Language, library::Library, shell::Shell};

pub mod shell;

#[derive(
    Debug,
    Default,
    Getters,
    Setters,
    MutGetters,
    CopyGetters,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
)]
#[setters(prefix = "set_")]
pub struct Preferences {
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    author: String,
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    location: String,
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    shells: Vec<Shell>,
    #[setters(borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    languages: Vec<Language>,
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    libraries: Vec<Library>,
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
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

    pub fn load(path: impl ToString) -> Result<Self> {
        let file = std::fs::read_to_string(path.to_string())?;
        let preferences = ron::from_str::<Preferences>(&file)?;
        Ok(preferences)
    }

    pub fn update_shell(&mut self, index: usize, shell: Shell) -> Result<()> {
        self.shells[index] = shell;
        self.save()
    }

    pub fn update_language(&mut self, index: usize, language: Language) -> Result<()> {
        self.languages[index] = language;
        self.save()
    }

    pub fn update_library(&mut self, index: usize, library: Library) -> Result<()> {
        self.libraries[index] = library;
        self.save()
    }

    pub fn update_code_editor(&mut self, index: usize, code_editor: CodeEditor) -> Result<()> {
        self.code_editors[index] = code_editor;
        self.save()
    }
}
