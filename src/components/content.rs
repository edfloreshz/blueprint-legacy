use devx_core::preferences::Preferences;
use relm4::{
    gtk::{
        self,
        traits::{BoxExt, OrientableExt, WidgetExt},
    },
    Component, ComponentController, ComponentParts, ComponentSender, Controller, SimpleComponent,
};

use crate::{components::pages::shells::ShellsInit, models::page::Page, setup::preferences_path};

use super::pages::shells::{ShellsInput, ShellsModel};

pub struct ContentModel {
    page: Page,
    shells: Controller<ShellsModel>,
}

#[derive(Debug)]
pub enum ContentInput {
    SelectPage(Page),
}

#[derive(Debug)]
pub enum ContentOutput {}

#[relm4::component(pub)]
impl SimpleComponent for ContentModel {
    type Input = ContentInput;
    type Output = ContentOutput;
    type Init = Page;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_hexpand: true,
            append: model.shells.widget()
        }
    }

    fn init(
        page: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let preferences =
            Preferences::load(preferences_path().display().to_string()).unwrap_or_default();
        let shells = ShellsModel::builder()
            .launch(ShellsInit::new(page, preferences.shells().clone()))
            .detach();
        let model = ContentModel { page, shells };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            ContentInput::SelectPage(page) => {
                self.page = page;
                self.shells
                    .sender()
                    .send(ShellsInput::SelectPage(page))
                    .unwrap();
            }
        }
    }
}
