use devx_core::preferences::{library::Library, source::Source, Preferences};
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

use crate::{components::pages::libraries::LibrariesInput, setup::preferences_path};

pub struct LibraryModel {
    index: usize,
    library: Library,
}

#[derive(Debug)]
pub enum LibraryInput {
    SetEnabledState(bool),
    ChangePackageName((Source, String)),
}

#[derive(Debug)]
pub enum LibraryOutput {}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for LibraryModel {
    type ParentWidget = adw::PreferencesGroup;
    type ParentInput = LibrariesInput;
    type Input = LibraryInput;
    type Output = LibraryOutput;
    type Init = (usize, Library);
    type CommandOutput = ();

    view! {
        root = adw::ExpanderRow {
            connect_enable_expansion_notify[sender] => move |state| {
                sender.input(LibraryInput::SetEnabledState(state.enables_expansion()))
            },
            set_title: self.library.name(),
            set_show_enable_switch: true,
            #[watch]
            set_enable_expansion: *self.library.enabled(),
            add_prefix = &gtk::Image {
                set_icon_name: Some(self.library.icon()),
            },
            add_row = &adw::EntryRow {
                set_title: "apt package name for this library",
                #[watch]
                set_text: self.library.sources().apt().package_name(),
                set_show_apply_button: true,
                connect_activate[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(LibraryInput::ChangePackageName((Source::Apt, buffer)));
                },
                connect_apply[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(LibraryInput::ChangePackageName((Source::Apt, buffer)));
                },
            },
            add_row = &adw::EntryRow {
                set_title: "dnf package name for this library",
                #[watch]
                set_text: self.library.sources().dnf().package_name(),
                set_show_apply_button: true,
                connect_activate[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(LibraryInput::ChangePackageName((Source::Dnf, buffer)));
                },
                connect_apply[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(LibraryInput::ChangePackageName((Source::Dnf, buffer)));
                },
            },
            add_row = &adw::EntryRow {
                set_title: "Flatpak app identifier for this library",
                #[watch]
                set_text: self.library.sources().flatpak().app_id(),
                set_show_apply_button: true,
                connect_activate[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(LibraryInput::ChangePackageName((Source::Flatpak, buffer)));
                },
                connect_apply[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(LibraryInput::ChangePackageName((Source::Flatpak, buffer)));
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
            library: init.1,
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
            LibraryInput::SetEnabledState(state) => {
                self.library.set_enabled(state);
            }
            LibraryInput::ChangePackageName((source, name)) => match source {
                Source::Apt => {
                    self.library.sources_mut().apt_mut().set_package_name(name);
                }
                Source::Dnf => {
                    self.library.sources_mut().dnf_mut().set_package_name(name);
                }
                Source::Flatpak => {
                    self.library.sources_mut().flatpak_mut().set_app_id(name);
                }
            },
        }
        if let Err(err) = preferences.update_library(self.index, self.library.clone()) {
            eprintln!("{err}")
        }
    }

    fn forward_to_parent(_output: Self::Output) -> Option<Self::ParentInput> {
        None
    }
}
