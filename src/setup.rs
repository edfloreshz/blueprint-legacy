use std::path::PathBuf;

use devx_core::preferences::{
    shell::Shell,
    source::{Apt, Dnf, Sources},
    Preferences,
};
use dirs::data_dir;
use relm4::gtk;

use anyhow::Result;
use gettextrs::{gettext, LocaleCategory};
use gtk::{gdk, gio, glib};
use relm4_icons::icon_name;

use crate::config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR};

pub fn setup() -> Result<()> {
    // Initialize GTK
    gtk::init().unwrap();

    setup_gettext();

    glib::set_application_name(&gettext("Devx"));

    gio::resources_register_include!("resources.gresource")?;

    setup_css();

    setup_preferences()?;

    gtk::Window::set_default_icon_name(APP_ID);

    relm4_icons::initialize_icons();

    Ok(())
}

fn setup_gettext() {
    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
}

fn setup_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/dev/edfloreshz/Devx/style.css");
    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn setup_preferences() -> Result<()> {
    let preferences_file_path = preferences_path();
    let app_directory = preferences_file_path.parent().unwrap();
    if !preferences_file_path.exists() {
        std::fs::create_dir_all(app_directory)?;
    }
    let preferences = Preferences::new()
        .set_location(preferences_file_path.display().to_string())
        .set_shells(vec![
            Shell::default()
                .set_name("Fish Shell")
                .set_enabled(true)
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
        .clone();
    if !preferences_file_path.exists() {
        preferences.save()?;
    }
    Ok(())
}

pub fn preferences_path() -> PathBuf {
    data_dir()
        .unwrap()
        .join(APP_ID)
        .join(format!("{APP_ID}.ron"))
}
