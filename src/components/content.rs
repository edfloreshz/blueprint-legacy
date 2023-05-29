use blueprint_core::preferences::Preferences;
use relm4::{
    gtk::{
        self,
        traits::{BoxExt, OrientableExt, WidgetExt},
    },
    Component, ComponentController, ComponentParts, ComponentSender, Controller, SimpleComponent,
};

use crate::{
    components::pages::{
        code_editors::CodeEditorsInit, languages::LanguagesInit, libraries::LibrariesInit,
        shells::ShellsInit,
    },
    models::page::Page,
    setup::preferences_path,
};

use super::pages::{
    code_editors::{CodeEditorsInput, CodeEditorsModel},
    languages::{LanguagesInput, LanguagesModel},
    libraries::{LibrariesInput, LibrariesModel},
    shells::{ShellsInput, ShellsModel},
};

pub struct ContentModel {
    page: Page,
    shells: Controller<ShellsModel>,
    languages: Controller<LanguagesModel>,
    libraries: Controller<LibrariesModel>,
    code_editors: Controller<CodeEditorsModel>,
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
            append: model.shells.widget(),
            append: model.languages.widget(),
            append: model.libraries.widget(),
            append: model.code_editors.widget(),
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
        let languages = LanguagesModel::builder()
            .launch(LanguagesInit::new(page, preferences.languages().clone()))
            .detach();
        let libraries = LibrariesModel::builder()
            .launch(LibrariesInit::new(page, preferences.libraries().clone()))
            .detach();
        let code_editors = CodeEditorsModel::builder()
            .launch(CodeEditorsInit::new(
                page,
                preferences.code_editors().clone(),
            ))
            .detach();
        let model = ContentModel {
            page,
            shells,
            languages,
            libraries,
            code_editors,
        };
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
                self.languages
                    .sender()
                    .send(LanguagesInput::SelectPage(page))
                    .unwrap();
                self.libraries
                    .sender()
                    .send(LibrariesInput::SelectPage(page))
                    .unwrap();
                self.code_editors
                    .sender()
                    .send(CodeEditorsInput::SelectPage(page))
                    .unwrap();
            }
        }
    }
}
