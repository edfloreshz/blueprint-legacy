use relm4::{
    actions::{ActionGroupName, RelmAction, RelmActionGroup},
    adw,
    gtk::{
        self,
        traits::{ButtonExt, OrientableExt},
    },
    main_application, Component, ComponentController, ComponentParts, ComponentSender, Controller,
};

use gtk::prelude::{ApplicationExt, ApplicationWindowExt, GtkWindowExt, WidgetExt};
use relm4_icons::icon_name;

use crate::{
    components::{content::ContentInput, sidebar::SidebarModel},
    config::PROFILE,
};
use crate::{
    components::{content::ContentModel, sidebar::SidebarOutput},
    modals::about::AboutDialog,
    models::page::Page,
};

pub(super) struct App {
    about_dialog: Controller<AboutDialog>,
    sidebar: Controller<SidebarModel>,
    content: Controller<ContentModel>,
    reveal_sidebar: bool,
}

#[derive(Debug)]
pub(super) enum AppMsg {
    SelectPage(Page),
    RevealSidebar,
    Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl Component for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;
    type CommandOutput = ();

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About Devx" => AboutAction,
            }
        }
    }

    view! {
        main_window = adw::ApplicationWindow::new(&main_application()) {
            set_default_size: (700, 700),
            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                gtk::Inhibit(true)
            },

            #[wrap(Some)]
            set_help_overlay: shortcuts = &gtk::Builder::from_resource(
                    "/dev/edfloreshz/Devx/gtk/help-overlay.ui"
                )
                .object::<gtk::ShortcutsWindow>("help_overlay")
                .unwrap() -> gtk::ShortcutsWindow {
                    set_transient_for: Some(&main_window),
                    set_application: Some(&main_application()),
            },

            add_css_class?: if PROFILE == "Devel" {
                Some("devel")
            } else {
                None
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar {
                    set_show_start_title_buttons: false,
                    pack_start = &gtk::Button {
                        set_css_classes: &["flat"],
                        set_icon_name: icon_name::PANEL_RIGHT_CONTRACT_FILLED,
                        connect_clicked => AppMsg::RevealSidebar,
                    },
                },
                #[name(flap)]
                adw::Flap {
                    set_flap: Some(model.sidebar.widget()),
                    set_separator: Some(&gtk::Separator::default()),
                    set_content: Some(model.content.widget()),
                    set_swipe_to_close: true,
                    set_swipe_to_open: true,
                    set_modal: true,
                    #[watch]
                    set_reveal_flap: model.reveal_sidebar,
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let about_dialog = AboutDialog::builder()
            .transient_for(root)
            .launch(())
            .detach();

        let sidebar =
            SidebarModel::builder()
                .launch(())
                .forward(sender.input_sender(), |message| match message {
                    SidebarOutput::SelectPage(page) => AppMsg::SelectPage(page),
                });
        let content = ContentModel::builder().launch(Page::Shells).detach();
        let model = Self {
            about_dialog,
            sidebar,
            content,
            reveal_sidebar: true,
        };

        let widgets = view_output!();

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        let shortcuts_action = {
            let shortcuts = widgets.shortcuts.clone();
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                shortcuts.present();
            })
        };

        let about_action = {
            let sender = model.about_dialog.sender().clone();
            RelmAction::<AboutAction>::new_stateless(move |_| {
                sender.send(()).unwrap();
            })
        };

        actions.add_action(shortcuts_action);
        actions.add_action(about_action);

        widgets
            .main_window
            .insert_action_group(WindowActionGroup::NAME, Some(&actions.into_action_group()));

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AppMsg::RevealSidebar => self.reveal_sidebar = !widgets.flap.reveals_flap(),
            AppMsg::SelectPage(page) => self
                .content
                .sender()
                .send(ContentInput::SelectPage(page))
                .unwrap(),
            AppMsg::Quit => main_application().quit(),
        }
        self.update_view(widgets, sender)
    }
}
