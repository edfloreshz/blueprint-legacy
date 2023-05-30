use blueprint_core::preferences::{shell::Shell, source::Source, Preferences};
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

use crate::{components::pages::shells::ShellsInput, fl, setup::preferences_path};

pub struct ShellModel {
    index: usize,
    shell: Shell,
}

#[derive(Debug)]
pub enum ShellInput {
    SetEnabledState(bool),
    ChangeConfigurationFileUrl(String),
    ChangePackageName((Source, String)),
}

#[derive(Debug)]
pub enum ShellOutput {}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for ShellModel {
    type ParentWidget = adw::PreferencesGroup;
    type ParentInput = ShellsInput;
    type Input = ShellInput;
    type Output = ShellOutput;
    type Init = (usize, Shell);
    type CommandOutput = ();

    view! {
        root = adw::ExpanderRow {
            connect_enable_expansion_notify[sender] => move |state| {
                sender.input(ShellInput::SetEnabledState(state.enables_expansion()))
            },
            set_title: self.shell.name(),
            set_show_enable_switch: true,
            #[watch]
            set_enable_expansion: *self.shell.enabled(),
            add_prefix = &gtk::Image {
                set_icon_name: Some(self.shell.icon()),
            },
            add_row = &adw::EntryRow {
                set_title: fl!("apt-description"),
                #[watch]
                set_text: self.shell.sources().apt().package_name(),
                set_show_apply_button: true,
                connect_activate[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(ShellInput::ChangePackageName((Source::Apt, buffer)));
                },
                connect_apply[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(ShellInput::ChangePackageName((Source::Apt, buffer)));
                },
            },
            add_row = &adw::EntryRow {
                set_title: fl!("dnf-description"),
                #[watch]
                set_text: self.shell.sources().dnf().package_name(),
                set_show_apply_button: true,
                connect_activate[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(ShellInput::ChangePackageName((Source::Dnf, buffer)));
                },
                connect_apply[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(ShellInput::ChangePackageName((Source::Dnf, buffer)));
                },
            },
            add_row = &adw::EntryRow {
                set_title: fl!("flatpak-description"),
                #[watch]
                set_text: self.shell.sources().flatpak().app_id(),
                set_show_apply_button: true,
                connect_activate[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(ShellInput::ChangePackageName((Source::Flatpak, buffer)));
                },
                connect_apply[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(ShellInput::ChangePackageName((Source::Flatpak, buffer)));
                },
            },
            add_row = &adw::EntryRow {
                set_title: fl!("url-description"),
                #[watch]
                set_text: self.shell.config(),
                set_show_apply_button: true,
                connect_activate[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(ShellInput::ChangeConfigurationFileUrl(buffer));
                },
                connect_apply[sender] => move |entry| {
                    let buffer = entry.text().to_string();
                    sender.input(ShellInput::ChangeConfigurationFileUrl(buffer));
                },
            }
        }
    }

    async fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: AsyncFactorySender<Self>,
    ) -> Self {
        Self {
            index: init.0,
            shell: init.1,
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
            ShellInput::SetEnabledState(state) => {
                self.shell.set_enabled(state);
            }
            ShellInput::ChangeConfigurationFileUrl(url) => {
                self.shell.set_config(url);
            }
            ShellInput::ChangePackageName((source, name)) => match source {
                Source::Apt => {
                    self.shell.sources_mut().apt_mut().set_package_name(name);
                }
                Source::Dnf => {
                    self.shell.sources_mut().dnf_mut().set_package_name(name);
                }
                Source::Flatpak => {
                    self.shell.sources_mut().flatpak_mut().set_app_id(name);
                }
            },
        }
        if let Err(err) = preferences.update_shell(self.index, self.shell.clone()) {
            eprintln!("{err}")
        }
    }

    fn forward_to_parent(_output: Self::Output) -> Option<Self::ParentInput> {
        None
    }
}
