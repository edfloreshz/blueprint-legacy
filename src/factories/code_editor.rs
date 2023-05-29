use blueprint_core::preferences::{code_editor::CodeEditor, source::Source, Preferences};
use relm4::{
    adw::{
        self,
        traits::{EntryRowExt, ExpanderRowExt, PreferencesRowExt},
    },
    factory::{AsyncFactoryComponent, FactoryView},
    gtk::traits::EditableExt,
    gtk::{self, traits::ListBoxRowExt},
    prelude::DynamicIndex,
    AsyncFactorySender,
};

use crate::{components::pages::code_editors::CodeEditorsInput, setup::preferences_path};

pub struct CodeEditorModel {
    index: usize,
    code_editor: CodeEditor,
}

#[derive(Debug)]
pub enum CodeEditorInput {
    SetEnabledState(bool),
    ChangePackageName((Source, String)),
}

#[derive(Debug)]
pub enum CodeEditorOutput {}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for CodeEditorModel {
    type ParentWidget = adw::PreferencesGroup;
    type ParentInput = CodeEditorsInput;
    type Input = CodeEditorInput;
    type Output = CodeEditorOutput;
    type Init = (usize, CodeEditor);
    type CommandOutput = ();

    view! {
        root = adw::ExpanderRow {
            connect_enable_expansion_notify[sender] => move |state| {
                sender.input(CodeEditorInput::SetEnabledState(state.enables_expansion()))
            },
            set_title: self.code_editor.name(),
            set_show_enable_switch: true,
            #[watch]
            set_enable_expansion: *self.code_editor.enabled(),
            add_prefix = &gtk::Image {
                set_icon_name: Some(self.code_editor.icon()),
            },
            add_row = &adw::EntryRow {
                set_title: "apt package name for this code editor",
                #[watch]
                set_text: self.code_editor.sources().apt().package_name(),
                set_show_apply_button: true,
                connect_activate[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(CodeEditorInput::ChangePackageName((Source::Apt, buffer)));
                },
                connect_apply[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(CodeEditorInput::ChangePackageName((Source::Apt, buffer)));
                },
            },
            add_row = &adw::EntryRow {
                set_title: "dnf package name for this code editor",
                #[watch]
                set_text: self.code_editor.sources().dnf().package_name(),
                set_show_apply_button: true,
                connect_activate[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(CodeEditorInput::ChangePackageName((Source::Dnf, buffer)));
                },
                connect_apply[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(CodeEditorInput::ChangePackageName((Source::Dnf, buffer)));
                },
            },
            add_row = &adw::EntryRow {
                set_title: "Flatpak app identifier for this code editor",
                #[watch]
                set_text: self.code_editor.sources().flatpak().app_id(),
                set_show_apply_button: true,
                connect_activate[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(CodeEditorInput::ChangePackageName((Source::Flatpak, buffer)));
                },
                connect_apply[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(CodeEditorInput::ChangePackageName((Source::Flatpak, buffer)));
                },
            },
        }
    }

    async fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: AsyncFactorySender<Self>,
    ) -> Self {
        Self {
            index: init.0,
            code_editor: init.1,
        }
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        sender: AsyncFactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        widgets
    }

    async fn update(&mut self, message: Self::Input, _sender: AsyncFactorySender<Self>) {
        let mut preferences = Preferences::load(preferences_path().display().to_string())
            .expect("Failed to open preferences");
        match message {
            CodeEditorInput::SetEnabledState(state) => {
                self.code_editor.set_enabled(state);
            }
            CodeEditorInput::ChangePackageName((source, name)) => match source {
                Source::Apt => {
                    self.code_editor
                        .sources_mut()
                        .apt_mut()
                        .set_package_name(name);
                }
                Source::Dnf => {
                    self.code_editor
                        .sources_mut()
                        .dnf_mut()
                        .set_package_name(name);
                }
                Source::Flatpak => {
                    self.code_editor
                        .sources_mut()
                        .flatpak_mut()
                        .set_app_id(name);
                }
            },
        }
        if let Err(err) = preferences.update_code_editor(self.index, self.code_editor.clone()) {
            eprintln!("{err}")
        }
    }

    fn forward_to_parent(_output: Self::Output) -> Option<Self::ParentInput> {
        None
    }
}
