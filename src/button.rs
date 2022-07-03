/* Create Button implementation */

use gtk4 as gtk;
use gtk::{Button, Entry};

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

pub fn create_entry() -> Entry {
    let margin = 6;
    let entry = Entry::builder()
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build();

    return entry;
}