
/* Editable Entry Text */

use std::str::Chars;

use gtk::glib::GString;

pub fn remove_last_character_of_entry(entry_text: &GString) -> Chars {
    let mut char_text = entry_text.chars(); // char text
    char_text.next_back(); // remove last char
    char_text.next_back(); // remove last char again
    
    return char_text;
}
