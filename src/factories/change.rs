use relm4::{
    factory::FactoryView,
    gtk::{self, traits::OrientableExt},
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender,
};

use crate::modals::summary::SummaryInput;

pub struct ChangeModel(String);

#[derive(Debug)]
pub enum ChangeInput {}

#[derive(Debug)]
pub enum ChangeOutput {}

#[relm4::factory(pub)]
impl FactoryComponent for ChangeModel {
    type ParentWidget = gtk::ListBox;
    type ParentInput = SummaryInput;
    type Input = ChangeInput;
    type Output = ChangeOutput;
    type Init = String;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            gtk::Label {
                set_text: &self.0
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self(init)
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, _sender: FactorySender<Self>) {
        match message {}
    }
}
