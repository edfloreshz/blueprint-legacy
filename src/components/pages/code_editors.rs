use blueprint_core::preferences::code_editor::CodeEditor;
use relm4::{
    adw::{
        self,
        traits::{PreferencesGroupExt, PreferencesPageExt},
    },
    factory::AsyncFactoryVecDeque,
    gtk::traits::WidgetExt,
    ComponentParts, ComponentSender, SimpleComponent,
};

use crate::{factories::code_editor::CodeEditorModel, models::page::Page};

pub struct CodeEditorsModel {
    page: Page,
    code_editor_factory: AsyncFactoryVecDeque<CodeEditorModel>,
}

#[derive(Debug)]
pub enum CodeEditorsInput {
    SelectPage(Page),
}

#[derive(Debug)]
pub enum CodeEditorsOutput {}

#[derive(Debug, new)]
pub struct CodeEditorsInit {
    page: Page,
    code_editors: Vec<CodeEditor>,
}

#[relm4::component(pub)]
impl SimpleComponent for CodeEditorsModel {
    type Input = CodeEditorsInput;
    type Output = CodeEditorsOutput;
    type Init = CodeEditorsInit;

    view! {
        #[root]
        adw::PreferencesPage {
            #[watch]
            set_visible: model.page == Page::CodeEditors,
            #[local_ref]
            add = code_editor_container -> adw::PreferencesGroup {
                #[watch]
                set_title: model.page.name(),
                #[watch]
                set_description: Some(model.page.description()),
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut code_editor_factory =
            AsyncFactoryVecDeque::new(adw::PreferencesGroup::default(), sender.input_sender());
        {
            let mut guard = code_editor_factory.guard();

            for (i, code_editor) in init.code_editors.iter().enumerate() {
                guard.push_back((i, code_editor.clone()));
            }
        }
        let model = CodeEditorsModel {
            page: init.page,
            code_editor_factory,
        };
        let code_editor_container = model.code_editor_factory.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            CodeEditorsInput::SelectPage(page) => self.page = page,
        }
    }
}
