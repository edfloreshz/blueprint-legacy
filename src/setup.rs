use relm4::gtk;

use anyhow::Result;
use gettextrs::{gettext, LocaleCategory};
use gtk::{gdk, gio, glib};

use crate::config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR};

pub fn setup() -> Result<()> {
    // Initialize GTK
    gtk::init().unwrap();

    setup_gettext();

    glib::set_application_name(&gettext("Devx"));

    gio::resources_register_include!("resources.gresource")?;

    setup_css();

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
