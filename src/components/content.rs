use relm4::{
    adw,
    gtk::{
        self,
        traits::{OrientableExt, WidgetExt},
    },
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
};

use crate::models::page::Page;

pub struct ContentModel {
    page: Page,
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
        adw::ClampScrollable {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_halign: gtk::Align::Center,
                set_hexpand: true,
                set_margin_all: 20,
                gtk::Image {
                    #[watch]
                    set_icon_name: Some(model.page.icon()),
                    set_icon_size: gtk::IconSize::Large,
                },
                gtk::Label {
                    #[watch]
                    set_label: model.page.name(),
                    add_css_class: "title-header",
                    set_halign: gtk::Align::Center
                }
            }
        }
    }

    fn init(
        page: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ContentModel { page };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            ContentInput::SelectPage(page) => self.page = page,
        }
    }
}
