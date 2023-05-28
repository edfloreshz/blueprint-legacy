use relm4::{
    adw::{
        self,
        traits::{
            ActionRowExt, AdwWindowExt, ComboRowExt, PreferencesGroupExt, PreferencesPageExt,
            PreferencesRowExt,
        },
        ColorScheme,
    },
    gtk::{
        self,
        traits::{BoxExt, GtkWindowExt, OrientableExt, WidgetExt},
    },
    Component, ComponentParts, ComponentSender,
};
use relm4_icons::icon_name;

use crate::models::app_preferences::AppPreferences;

pub struct PreferencesModel {
    preferences: AppPreferences,
}

#[derive(Debug)]
pub enum PreferencesInput {
    SetColorScheme(ColorScheme),
}

#[derive(Debug)]
pub enum PreferencesOutput {}

#[relm4::component(pub)]
impl Component for PreferencesModel {
    type CommandOutput = ();
    type Input = PreferencesInput;
    type Output = PreferencesOutput;
    type Init = ();

    view! {
        #[root]
        adw::PreferencesWindow {
            set_title: Some("Preferences"),
            set_hide_on_close: true,
            #[wrap(Some)]
            #[name = "overlay"]
            set_content = &adw::ToastOverlay {
                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append = &adw::HeaderBar {
                        set_show_end_title_buttons: true
                    },
                    append = &adw::Clamp {
                        #[wrap(Some)]
                        set_child = &adw::PreferencesPage {
                            set_vexpand: true,
                            add = &adw::PreferencesGroup {
                                set_title: "Appearance",
                                adw::ComboRow {
                                    set_title: "Color scheme",
                                    set_subtitle: "Set the color scheme of the app",
                                    set_icon_name: Some(icon_name::DARK_MODE),
                                    set_model: Some(&gtk::StringList::new(&[
                                        "Light",
                                        "Dark",
                                        "Default",
                                    ])),
                                    set_selected: match model.preferences.color_scheme {
                                        ColorScheme::PreferLight => 0,
                                        ColorScheme::PreferDark => 1,
                                        ColorScheme::Default => 2,
                                        _ => 0,
                                    },
                                    connect_selected_notify[sender] => move |combo_row| {
                                        match combo_row.selected() {
                                            0 => sender.input_sender().send(PreferencesInput::SetColorScheme(ColorScheme::PreferLight)).unwrap(),
                                            1 => sender.input_sender().send(PreferencesInput::SetColorScheme(ColorScheme::PreferDark)).unwrap(),
                                            _ => sender.input_sender().send(PreferencesInput::SetColorScheme(ColorScheme::Default)).unwrap(),
                                        }
                                    },
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = PreferencesModel {
            preferences: AppPreferences {
                color_scheme: ColorScheme::Default,
            },
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            PreferencesInput::SetColorScheme(color_scheme) => {
                set_color_scheme(self, color_scheme);
            }
        }
    }
}

pub fn set_color_scheme(model: &mut PreferencesModel, color_scheme: ColorScheme) {
    match color_scheme {
        ColorScheme::PreferDark => {
            adw::StyleManager::default().set_color_scheme(ColorScheme::ForceDark);
            model.preferences.color_scheme = ColorScheme::PreferDark;
        }
        ColorScheme::PreferLight => {
            adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceLight);
            model.preferences.color_scheme = ColorScheme::PreferLight;
        }
        ColorScheme::Default => {
            adw::StyleManager::default().set_color_scheme(adw::ColorScheme::Default);
            model.preferences.color_scheme = ColorScheme::Default;
        }
        _ => model.preferences.color_scheme = ColorScheme::Default,
    }
}
