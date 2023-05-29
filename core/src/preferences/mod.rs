pub mod code_editor;
pub mod language;
pub mod library;
pub mod source;

use anyhow::Result;
use derive_setters::Setters;
use getset::{CopyGetters, Getters, MutGetters};
use relm4_icons::icon_name;
use serde::{Deserialize, Serialize};

use crate::packages::install::Package;

use self::{
    code_editor::CodeEditor,
    language::Language,
    library::Library,
    shell::Shell,
    source::{Apt, Dnf, Flatpak, Sources},
};

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
    pub fn new(location: impl ToString) -> Self {
        Self::default()
            .set_location(location.to_string())
            .set_shells(vec![
                Shell::default()
                    .set_name("Fish Shell")
                    .set_sources(
                        Sources::default()
                            .set_apt(
                                Apt::default()
                                    .set_package_name("fish")
                                    .set_ppa_repository("fish-shell/release-3")
                                    .clone(),
                            )
                            .set_dnf(Dnf::default().set_package_name("fish").clone())
                            .clone(),
                    )
                    .set_icon(icon_name::CODE_BLOCK_FILLED)
                    .clone(),
                Shell::default()
                    .set_name("Zsh")
                    .set_sources(
                        Sources::default()
                            .set_apt(Apt::default().set_package_name("zsh").clone())
                            .set_dnf(Dnf::default().set_package_name("zsh").clone())
                            .clone(),
                    )
                    .set_icon(icon_name::CODE_BLOCK_FILLED)
                    .clone(),
            ])
            .set_languages(vec![
                Language::default()
                    .set_name("Rust")
                    .set_icon(icon_name::CHAT_FILLED)
                    .set_sources(
                        Sources::default()
                            .set_dnf(Dnf::default().set_package_name("rust").clone())
                            .clone(),
                    )
                    .clone(),
                Language::default()
                    .set_name("Go")
                    .set_icon(icon_name::CHAT_FILLED)
                    .set_sources(
                        Sources::default()
                            .set_dnf(Dnf::default().set_package_name("go").clone())
                            .clone(),
                    )
                    .clone(),
            ])
            .set_libraries(vec![
                Library::default()
                    .set_name("GTK 4")
                    .set_icon(icon_name::BOOKMARK_MULTIPLE_FILLED)
                    .set_sources(
                        Sources::default()
                            .set_dnf(Dnf::default().set_package_name("gtk4").clone())
                            .clone(),
                    )
                    .clone(),
                Library::default()
                    .set_name("Libadwaita")
                    .set_icon(icon_name::BOOKMARK_MULTIPLE_FILLED)
                    .set_sources(
                        Sources::default()
                            .set_dnf(Dnf::default().set_package_name("libadwaita").clone())
                            .clone(),
                    )
                    .clone(),
            ])
            .set_code_editors(vec![
                CodeEditor::default()
                    .set_name("Visual Studio Code")
                    .set_icon(icon_name::SLIDE_TEXT_FILLED)
                    .set_sources(
                        Sources::default()
                            .set_flatpak(
                                Flatpak::default()
                                    .set_app_id("com.visualstudio.code")
                                    .clone(),
                            )
                            .clone(),
                    )
                    .clone(),
                CodeEditor::default()
                    .set_name("Helix")
                    .set_icon(icon_name::SLIDE_TEXT_FILLED)
                    .set_sources(
                        Sources::default()
                            .set_flatpak(
                                Flatpak::default()
                                    .set_app_id("com.helix_editor.Helix")
                                    .clone(),
                            )
                            .clone(),
                    )
                    .clone(),
                CodeEditor::default()
                    .set_name("Vim")
                    .set_icon(icon_name::SLIDE_TEXT_FILLED)
                    .set_sources(
                        Sources::default()
                            .set_apt(Apt::default().set_package_name("vim").clone())
                            .set_dnf(Dnf::default().set_package_name("vim").clone())
                            .set_flatpak(Flatpak::default().set_app_id("org.vim.Vim").clone())
                            .clone(),
                    )
                    .clone(),
                CodeEditor::default()
                    .set_name("Neovim")
                    .set_icon(icon_name::SLIDE_TEXT_FILLED)
                    .set_sources(
                        Sources::default()
                            .set_apt(Apt::default().set_package_name("neovim").clone())
                            .set_dnf(Dnf::default().set_package_name("neovim").clone())
                            .set_flatpak(Flatpak::default().set_app_id("io.neovim.nvim").clone())
                            .clone(),
                    )
                    .clone(),
            ])
            .clone()
    }

    pub async fn apply(&self, password: &impl ToString) -> Result<Package> {
        let mut package = Package::default();
        for shell in self
            .shells
            .iter()
            .filter(|p| *p.enabled())
            .collect::<Vec<&Shell>>()
        {
            package
                .install_from_sources(shell.sources(), password)
                .await?;
        }
        for language in &self
            .languages
            .iter()
            .filter(|p| *p.enabled())
            .collect::<Vec<&Language>>()
        {
            package
                .install_from_sources(language.sources(), password)
                .await?;
        }
        for library in &self
            .libraries
            .iter()
            .filter(|p| *p.enabled())
            .collect::<Vec<&Library>>()
        {
            package
                .install_from_sources(library.sources(), password)
                .await?;
        }
        for code_editor in &self
            .code_editors
            .iter()
            .filter(|p| *p.enabled())
            .collect::<Vec<&CodeEditor>>()
        {
            package
                .install_from_sources(code_editor.sources(), password)
                .await?;
        }
        Ok(package)
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
