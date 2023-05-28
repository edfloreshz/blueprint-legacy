use devx_core::preferences::language::Language;
use relm4::{
    adw::{
        self,
        traits::{PreferencesGroupExt, PreferencesPageExt},
    },
    factory::AsyncFactoryVecDeque,
    gtk::traits::WidgetExt,
    ComponentParts, ComponentSender, SimpleComponent,
};

use crate::{factories::language::LanguageModel, models::page::Page};

pub struct LanguagesModel {
    page: Page,
    language_factory: AsyncFactoryVecDeque<LanguageModel>,
}

#[derive(Debug)]
pub enum LanguagesInput {
    SelectPage(Page),
}

#[derive(Debug)]
pub enum LanguagesOutput {}

#[derive(Debug, new)]
pub struct LanguagesInit {
    page: Page,
    languages: Vec<Language>,
}

#[relm4::component(pub)]
impl SimpleComponent for LanguagesModel {
    type Input = LanguagesInput;
    type Output = LanguagesOutput;
    type Init = LanguagesInit;

    view! {
        #[root]
        adw::PreferencesPage {
            #[watch]
            set_visible: model.page == Page::Languages,
            #[local_ref]
            add = language_container -> adw::PreferencesGroup {
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
        let mut language_factory =
            AsyncFactoryVecDeque::new(adw::PreferencesGroup::default(), sender.input_sender());
        {
            let mut guard = language_factory.guard();

            for (i, language) in init.languages.iter().enumerate() {
                guard.push_back((i, language.clone()));
            }
        }
        let model = LanguagesModel {
            page: init.page,
            language_factory,
        };
        let language_container = model.language_factory.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            LanguagesInput::SelectPage(page) => self.page = page,
        }
    }
}
