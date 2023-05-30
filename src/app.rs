use blueprint_core::preferences::Preferences;
use relm4::{
    actions::{ActionGroupName, RelmAction, RelmActionGroup},
    adw,
    component::{AsyncComponent, AsyncComponentParts},
    gtk::{
        self,
        traits::{ButtonExt, OrientableExt},
    },
    main_application, AsyncComponentSender, Component, ComponentController, Controller,
    RelmWidgetExt,
};

use gtk::prelude::{ApplicationExt, ApplicationWindowExt, GtkWindowExt, WidgetExt};
use relm4_icons::icon_name;

use crate::{
    components::{content::ContentInput, preferences::PreferencesModel, sidebar::SidebarModel},
    config::PROFILE,
    fl,
    modals::{
        password::{PasswordModel, PasswordOutput},
        summary::{SummaryInput, SummaryModel},
    },
    setup::preferences_path,
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
    preferences: Controller<PreferencesModel>,
    password_modal: Controller<PasswordModel>,
    summary_modal: Controller<SummaryModel>,
    reveal_sidebar: bool,
    show_spinner: bool,
}

#[derive(Debug)]
pub(super) enum AppMsg {
    SelectPage(Page),
    OpenPreferences,
    RevealSidebar,
    PromptPassword,
    ShowSpinner(String),
    ApplyChanges(String),
    Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub async)]
impl AsyncComponent for App {
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
                "_About Blueprint" => AboutAction,
            }
        }
    }

    view! {
        main_window = adw::ApplicationWindow::new(&main_application()) {
            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                gtk::Inhibit(true)
            },

            #[wrap(Some)]
            set_help_overlay: shortcuts = &gtk::Builder::from_resource(
                    "/dev/edfloreshz/Blueprint/gtk/help-overlay.ui"
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
                set_width_request: 700,
                set_height_request: 700,
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar {
                    set_show_start_title_buttons: false,
                    pack_start = &gtk::Button {
                        set_css_classes: &["flat"],
                        set_tooltip: fl!("hide-sidebar"),
                        set_icon_name: icon_name::PANEL_RIGHT_CONTRACT_FILLED,
                        connect_clicked => AppMsg::RevealSidebar,
                    },
                    pack_end = &gtk::Spinner {
                        #[watch]
                        set_visible: model.show_spinner,
                        start: (),
                    },
                    pack_end = &gtk::Button {
                        set_css_classes: &["flat"],
                        set_tooltip: fl!("apply-tooltip"),
                        set_icon_name: icon_name::CHECKBOX_CHECKED_FILLED,
                        connect_clicked => AppMsg::PromptPassword,
                    },
                },
                #[name(flap)]
                adw::Flap {
                    #[watch]
                    set_reveal_flap: model.reveal_sidebar,
                    set_flap: Some(model.sidebar.widget()),
                    set_separator: Some(&gtk::Separator::default()),
                    set_content: Some(model.content.widget()),
                    set_swipe_to_close: true,
                    set_swipe_to_open: true,
                    set_modal: true,
                }
            }
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let about_dialog = AboutDialog::builder()
            .transient_for(root.clone())
            .launch(())
            .detach();

        let sidebar =
            SidebarModel::builder()
                .launch(())
                .forward(sender.input_sender(), |message| match message {
                    SidebarOutput::SelectPage(page) => AppMsg::SelectPage(page),
                    SidebarOutput::OpenPreferences => AppMsg::OpenPreferences,
                });
        let content = ContentModel::builder().launch(Page::Shells).detach();
        let preferences = PreferencesModel::builder().launch(()).detach();
        let password_modal =
            PasswordModel::builder()
                .launch(())
                .forward(sender.input_sender(), |message| match message {
                    PasswordOutput::Password(password) => AppMsg::ShowSpinner(password),
                });
        let summary_modal = SummaryModel::builder().launch(()).detach();
        let model = Self {
            about_dialog,
            sidebar,
            content,
            preferences,
            password_modal,
            summary_modal,
            reveal_sidebar: true,
            show_spinner: false,
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

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AppMsg::RevealSidebar => self.reveal_sidebar = !widgets.flap.reveals_flap(),
            AppMsg::SelectPage(page) => self
                .content
                .sender()
                .send(ContentInput::SelectPage(page))
                .unwrap(),
            AppMsg::OpenPreferences => {
                let preferences = self.preferences.widget();
                preferences.present();
            }
            AppMsg::PromptPassword => self.password_modal.widget().present(),
            AppMsg::ShowSpinner(password) => {
                self.show_spinner = true;
                sender.input(AppMsg::ApplyChanges(password))
            }
            AppMsg::ApplyChanges(password) => {
                let preferences = Preferences::load(preferences_path().display().to_string())
                    .expect("Preferences are not correctly set");

                let (tx, mut rx) = relm4::tokio::sync::mpsc::channel(32);
                relm4::tokio::spawn(async move {
                    match preferences.apply(&password).await {
                        Ok(package) => tx.send(package.install_summary().clone()).await.unwrap(),
                        Err(err) => println!("An error ocurred: {}", err),
                    }
                });
                if let Some(messages) = rx.recv().await {
                    self.show_spinner = false;
                    self.summary_modal
                        .sender()
                        .send(SummaryInput::Open(messages))
                        .unwrap();
                }
            }
            AppMsg::Quit => main_application().quit(),
        }
        self.update_view(widgets, sender)
    }
}
