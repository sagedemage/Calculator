use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Button};

pub fn create_button(label: &'static str) -> Button {
    let margin = 6;
    let button = Button::builder()
        .label(label)
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build();

    return button;
}