use relm4::{
    adw,
    factory::FactoryVecDeque,
    gtk::{
        self,
        traits::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt},
    },
    Component, ComponentParts, ComponentSender, RelmWidgetExt,
};
use relm4_icons::icon_name;

use crate::factories::change::ChangeModel;

pub struct SummaryModel {
    summary_factory: FactoryVecDeque<ChangeModel>,
}

#[derive(Debug)]
pub enum SummaryInput {
    Open(Vec<String>),
    Close,
}

#[derive(Debug)]
pub enum SummaryOutput {}

#[relm4::component(pub)]
impl Component for SummaryModel {
    type CommandOutput = ();
    type Input = SummaryInput;
    type Output = SummaryOutput;
    type Init = ();

    view! {
        #[root]
        adw::Window {
            set_hide_on_close: true,
            set_default_width: 320,
            set_resizable: false,
            set_modal: true,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar {
                    set_show_end_title_buttons: true,
                    set_css_classes: &["flat"],
                    set_title_widget: Some(&gtk::Box::default())
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 20,
                    set_spacing: 10,
                    gtk::Image {
                        set_icon_size: gtk::IconSize::Large,
                        set_icon_name: Some(icon_name::CHECKMARK_CIRCLE_FILLED),
                    },
                    gtk::Label {
                        set_css_classes: &["title-4"],
                        set_label: "Done",
                    },
                    gtk::Label {
                        set_label: "We have applied this configuration to your system, happy coding!",
                    },
                    #[local_ref]
                    summary -> gtk::ListBox {
                        set_css_classes: &["navigation-sidebar"],
                    },
                    gtk::Button {
                        set_css_classes: &["suggested-action"],
                        set_label: "Ok",
                        connect_clicked => SummaryInput::Close
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let summary_factory = FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender());
        let model = SummaryModel { summary_factory };
        let summary = model.summary_factory.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            SummaryInput::Open(changes) => {
                let mut guard = self.summary_factory.guard();
                for change in changes {
                    guard.push_back(change);
                }
                root.present()
            }
            SummaryInput::Close => root.close(),
        }
    }
}
