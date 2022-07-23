/* Widgets */

use gtk4 as gtk;
use gtk::{Button, Entry};

pub struct NumberButtons {
    pub num0: Button,
    pub num1: Button,
    pub num2: Button,
    pub num3: Button,
    pub num4: Button,
    pub num5: Button,
    pub num6: Button,
    pub num7: Button,
    pub num8: Button,
    pub num9: Button,
}

pub fn create_button(label: &'static str) -> Button {
    let margin = 6;
    
    Button::builder()
        .label(label)
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build()
}

pub fn create_entry() -> Entry {
    let margin = 6;
    
    Entry::builder()
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build()
}
