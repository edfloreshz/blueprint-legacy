use relm4::{
    gtk::{
        self,
        traits::{BoxExt, ListBoxRowExt, OrientableExt, WidgetExt},
    },
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
};
use relm4_icons::icon_name;

use crate::models::page::Page;

pub struct SidebarModel {}

#[derive(Debug)]
pub enum SidebarInput {
    SelectPage(Page),
}

#[derive(Debug)]
pub enum SidebarOutput {
    SelectPage(Page),
}

#[relm4::component(pub)]
impl SimpleComponent for SidebarModel {
    type Input = SidebarInput;
    type Output = SidebarOutput;
    type Init = ();

    view! {
        #[root]
        &gtk::Box {
            set_css_classes: &["background"],
            set_orientation: gtk::Orientation::Vertical,
            set_vexpand: true,
            set_margin_all: 5,
            #[name(scroll_window)]
            gtk::ScrolledWindow {
                set_policy: (gtk::PolicyType::Never, gtk::PolicyType::Automatic),
                set_vexpand: true,
                gtk::ListBox {
                    set_css_classes: &["navigation-sidebar"],
                    connect_row_selected => move |_, listbox_row| {
                        if let Some(row) = listbox_row {
                            row.activate();
                        }
                    },
                    gtk::ListBoxRow {
                        gtk::Box {
                            set_spacing: 5,
                            gtk::Image {
                                set_icon_name: Some(icon_name::CODE_BLOCK_FILLED),
                            },
                            gtk::Label {
                                set_text: "Shells",
                                set_halign: gtk::Align::Start,
                            },
                        },
                        connect_activate[sender] => move |_| {
                            sender.input(SidebarInput::SelectPage(Page::Shells));
                        },
                    },
                    gtk::ListBoxRow {
                        gtk::Box {
                            set_spacing: 5,
                            gtk::Image {
                                set_icon_name: Some(icon_name::CHAT_FILLED),
                            },
                            gtk::Label {
                                set_text: "Languages",
                                set_halign: gtk::Align::Start,
                            },
                        },
                        connect_activate[sender] => move |_| {
                            sender.input(SidebarInput::SelectPage(Page::Languages));
                        },
                    },
                    gtk::ListBoxRow {
                        gtk::Box {
                            set_spacing: 5,
                            gtk::Image {
                                set_icon_name: Some(icon_name::BOOKMARK_MULTIPLE_FILLED),
                            },
                            gtk::Label {
                                set_text: "Libraries",
                                set_halign: gtk::Align::Start,
                            },
                        },
                        connect_activate[sender] => move |_| {
                            sender.input(SidebarInput::SelectPage(Page::Libraries));
                        },
                    },
                    gtk::ListBoxRow {
                        gtk::Box {
                            set_spacing: 5,
                            gtk::Image {
                                set_icon_name: Some(icon_name::SLIDE_TEXT_FILLED),
                            },
                            gtk::Label {
                                set_text: "Code Editors",
                                set_halign: gtk::Align::Start,
                            }
                        },
                        connect_activate[sender] => move |_| {
                            sender.input(SidebarInput::SelectPage(Page::CodeEditors));
                        },
                    }
                }
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SidebarModel {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            SidebarInput::SelectPage(page) => {
                _sender.output(SidebarOutput::SelectPage(page)).unwrap();
            }
        }
    }
}
