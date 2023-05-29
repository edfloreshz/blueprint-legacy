use blueprint_core::preferences::library::Library;
use relm4::{
    adw::{
        self,
        traits::{PreferencesGroupExt, PreferencesPageExt},
    },
    factory::AsyncFactoryVecDeque,
    gtk::traits::WidgetExt,
    ComponentParts, ComponentSender, SimpleComponent,
};

use crate::{factories::library::LibraryModel, models::page::Page};

pub struct LibrariesModel {
    page: Page,
    librarie_factory: AsyncFactoryVecDeque<LibraryModel>,
}

#[derive(Debug)]
pub enum LibrariesInput {
    SelectPage(Page),
}

#[derive(Debug)]
pub enum LibrariesOutput {}

#[derive(Debug, new)]
pub struct LibrariesInit {
    page: Page,
    libraries: Vec<Library>,
}

#[relm4::component(pub)]
impl SimpleComponent for LibrariesModel {
    type Input = LibrariesInput;
    type Output = LibrariesOutput;
    type Init = LibrariesInit;

    view! {
        #[root]
        adw::PreferencesPage {
            #[watch]
            set_visible: model.page == Page::Libraries,
            #[local_ref]
            add = librarie_container -> adw::PreferencesGroup {
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
        let mut librarie_factory =
            AsyncFactoryVecDeque::new(adw::PreferencesGroup::default(), sender.input_sender());
        {
            let mut guard = librarie_factory.guard();

            for (i, library) in init.libraries.iter().enumerate() {
                guard.push_back((i, library.clone()));
            }
        }
        let model = LibrariesModel {
            page: init.page,
            librarie_factory,
        };
        let librarie_container = model.librarie_factory.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            LibrariesInput::SelectPage(page) => self.page = page,
        }
    }
}
