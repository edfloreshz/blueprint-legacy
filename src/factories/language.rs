use blueprint_core::preferences::{language::Language, source::Source, Preferences};
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

use crate::{components::pages::languages::LanguagesInput, fl, setup::preferences_path};

pub struct LanguageModel {
    index: usize,
    language: Language,
}

#[derive(Debug)]
pub enum LanguageInput {
    SetEnabledState(bool),
    ChangePackageName((Source, String)),
}

#[derive(Debug)]
pub enum LanguageOutput {}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for LanguageModel {
    type ParentWidget = adw::PreferencesGroup;
    type ParentInput = LanguagesInput;
    type Input = LanguageInput;
    type Output = LanguageOutput;
    type Init = (usize, Language);
    type CommandOutput = ();

    view! {
        root = adw::ExpanderRow {
            connect_enable_expansion_notify[sender] => move |state| {
                sender.input(LanguageInput::SetEnabledState(state.enables_expansion()))
            },
            set_title: self.language.name(),
            set_show_enable_switch: true,
            #[watch]
            set_enable_expansion: *self.language.enabled(),
            add_prefix = &gtk::Image {
                set_icon_name: Some(self.language.icon()),
            },
            add_row = &adw::EntryRow {
                set_title: fl!("apt-description"),
                #[watch]
                set_text: self.language.sources().apt().package_name(),
                set_show_apply_button: true,
                connect_activate[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(LanguageInput::ChangePackageName((Source::Apt, buffer)));
                },
                connect_apply[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(LanguageInput::ChangePackageName((Source::Apt, buffer)));
                },
            },
            add_row = &adw::EntryRow {
                set_title: fl!("dnf-description"),
                #[watch]
                set_text: self.language.sources().dnf().package_name(),
                set_show_apply_button: true,
                connect_activate[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(LanguageInput::ChangePackageName((Source::Dnf, buffer)));
                },
                connect_apply[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(LanguageInput::ChangePackageName((Source::Dnf, buffer)));
                },
            },
            add_row = &adw::EntryRow {
                set_title: fl!("flatpak-description"),
                #[watch]
                set_text: self.language.sources().flatpak().app_id(),
                set_show_apply_button: true,
                connect_activate[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(LanguageInput::ChangePackageName((Source::Flatpak, buffer)));
                },
                connect_apply[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(LanguageInput::ChangePackageName((Source::Flatpak, buffer)));
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
            language: init.1,
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
            LanguageInput::SetEnabledState(state) => {
                self.language.set_enabled(state);
            }
            LanguageInput::ChangePackageName((source, name)) => match source {
                Source::Apt => {
                    self.language.sources_mut().apt_mut().set_package_name(name);
                }
                Source::Dnf => {
                    self.language.sources_mut().dnf_mut().set_package_name(name);
                }
                Source::Flatpak => {
                    self.language.sources_mut().flatpak_mut().set_app_id(name);
                }
            },
        }
        if let Err(err) = preferences.update_language(self.index, self.language.clone()) {
            eprintln!("{err}")
        }
    }

    fn forward_to_parent(_output: Self::Output) -> Option<Self::ParentInput> {
        None
    }
}
