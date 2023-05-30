use std::path::PathBuf;

use blueprint_core::preferences::Preferences;
use dirs::data_dir;
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    gtk::{self, prelude::ApplicationExt},
    main_application,
};

use anyhow::Result;
use gettextrs::{gettext, LocaleCategory};
use gtk::{gdk, gio, glib};

use crate::{
    application::localization,
    config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR},
    AppActionGroup, QuitAction,
};

pub fn setup() -> Result<gtk::Application> {
    let app = main_application();

    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();

    localization::init();

    // Initialize GTK
    gtk::init().unwrap();

    setup_gettext();

    glib::set_application_name(&gettext("Blueprint"));

    gio::resources_register_include!("resources.gresource")?;

    setup_css();

    setup_preferences()?;

    gtk::Window::set_default_icon_name(APP_ID);

    relm4_icons::initialize_icons();

    app.set_resource_base_path(Some("/dev/edfloreshz/Blueprint/"));

    setup_actions(&app);

    Ok(app)
}

fn setup_actions(app: &gtk::Application) {
    let mut actions = RelmActionGroup::<AppActionGroup>::new();
    let quit_action = {
        let app = app.clone();
        RelmAction::<QuitAction>::new_stateless(move |_| {
            app.quit();
        })
    };
    actions.add_action(quit_action);
    app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);
    app.set_action_group(Some(&actions.into_action_group()));
}

fn setup_gettext() {
    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
}

fn setup_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/dev/edfloreshz/Blueprint/style.css");
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
    let preferences = Preferences::new(preferences_file_path.display().to_string());
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
