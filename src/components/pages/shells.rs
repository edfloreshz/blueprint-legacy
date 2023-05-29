use blueprint_core::preferences::shell::Shell;
use relm4::{
    adw::{
        self,
        traits::{PreferencesGroupExt, PreferencesPageExt},
    },
    factory::AsyncFactoryVecDeque,
    gtk::traits::WidgetExt,
    ComponentParts, ComponentSender, SimpleComponent,
};

use crate::{factories::shell::ShellModel, models::page::Page};

pub struct ShellsModel {
    page: Page,
    shell_factory: AsyncFactoryVecDeque<ShellModel>,
}

#[derive(Debug)]
pub enum ShellsInput {
    SelectPage(Page),
}

#[derive(Debug)]
pub enum ShellsOutput {}

#[derive(Debug, new)]
pub struct ShellsInit {
    page: Page,
    shells: Vec<Shell>,
}

#[relm4::component(pub)]
impl SimpleComponent for ShellsModel {
    type Input = ShellsInput;
    type Output = ShellsOutput;
    type Init = ShellsInit;

    view! {
        #[root]
        adw::PreferencesPage {
            #[watch]
            set_visible: model.page == Page::Shells,
            #[local_ref]
            add = shell_container -> adw::PreferencesGroup {
                set_title: model.page.name(),
                set_description: Some(model.page.description()),
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut shell_factory =
            AsyncFactoryVecDeque::new(adw::PreferencesGroup::default(), sender.input_sender());
        {
            let mut guard = shell_factory.guard();

            for (i, shell) in init.shells.iter().enumerate() {
                guard.push_back((i, shell.clone()));
            }
        }
        let model = ShellsModel {
            page: init.page,
            shell_factory,
        };
        let shell_container = model.shell_factory.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            ShellsInput::SelectPage(page) => self.page = page,
        }
    }
}
