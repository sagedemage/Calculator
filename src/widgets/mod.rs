/* Widgets */

use gtk::{Button, Entry};

pub fn create_button(label: &'static str) -> Button {
    let margin = 1;
    
    Button::builder()
        .label(label)
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build()
}

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

impl NumberButtons {
    pub fn new() -> NumberButtons {
        NumberButtons{
            num0: create_button(" 0 "),
            num1: create_button(" 1 "),
            num2: create_button(" 2 "),
            num3: create_button(" 3 "),
            num4: create_button(" 4 "),
            num5: create_button(" 5 "),
            num6: create_button(" 6 "),
            num7: create_button(" 7 "),
            num8: create_button(" 8 "),
            num9: create_button(" 9 "),
        }
    }
} 

pub struct OperatorButtons {
    pub plus: Button,
    pub minus: Button,
    pub multiply: Button,
    pub divide: Button,
}

impl OperatorButtons {
    pub fn new() -> OperatorButtons {
        OperatorButtons{
            plus: create_button("+"),
            minus: create_button("\u{2212}"),
            multiply: create_button("\u{00D7}"),
            divide: create_button("\u{00F7}"),
        }
    }
}

pub struct SpecialButtons {
    pub clear: Button,
    pub equals: Button,
    pub period: Button,
    pub negative: Button,
}

impl SpecialButtons {
    pub fn new() -> SpecialButtons {
        SpecialButtons{
            clear: create_button("clear"),
            equals: create_button(" = "),
            period: create_button(" . "),
            negative: create_button("-/+"),
        }
    }
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

