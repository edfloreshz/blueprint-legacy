use relm4::{
    adw::{self, traits::PreferencesRowExt},
    gtk::{
        self,
        traits::{BoxExt, ButtonExt, EditableExt, GtkWindowExt, OrientableExt, WidgetExt},
    },
    Component, ComponentParts, ComponentSender, RelmWidgetExt,
};
use relm4_icons::icon_name;

pub struct PasswordModel {
    password: String,
}

#[derive(Debug)]
pub enum PasswordInput {
    SetPassword(String),
    Cancel,
    Ok,
}

#[derive(Debug)]
pub enum PasswordOutput {
    Password(String),
}

#[relm4::component(pub)]
impl Component for PasswordModel {
    type Input = PasswordInput;
    type Output = PasswordOutput;
    type Init = ();
    type CommandOutput = ();

    view! {
        #[root]
        adw::Window {
            set_hide_on_close: true,
            set_default_width: 320,
            set_resizable: false,
            set_modal: true,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    set_show_end_title_buttons: true,
                    set_css_classes: &["flat"],
                    set_title_widget: Some(&gtk::Box::default())
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 20,
                    set_spacing: 10,
                    gtk::Image {
                        set_icon_size: gtk::IconSize::Large,
                        set_icon_name: Some(icon_name::WARNING),
                    },
                    gtk::Label {
                    set_css_classes: &["title-4"],
                    set_label: "Sudo Mode",
                    },
                    gtk::Label {
                        set_label: "We require authorization to make changes in your system.",
                    },
                    adw::PreferencesGroup {
                        #[name(password)]
                        adw::PasswordEntryRow {
                            connect_changed[sender] => move |entry| {
                                let buffer = entry.text().to_string();
                                sender.input(PasswordInput::SetPassword(buffer))
                            },
                            set_title: "User password",
                        },
                    },
                    gtk::Button {
                        set_label: "Cancel",
                        connect_clicked => PasswordInput::Cancel,
                    },
                    gtk::Button {
                        set_css_classes: &["suggested-action"],
                        set_label: "Ok",
                        connect_clicked => PasswordInput::Ok,
                    },
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = PasswordModel {
            password: String::default(),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            PasswordInput::Cancel => root.close(),
            PasswordInput::SetPassword(password) => self.password = password,
            PasswordInput::Ok => {
                sender
                    .output(PasswordOutput::Password(self.password.clone()))
                    .unwrap_or_default();
                root.close()
            }
        }
    }
}
